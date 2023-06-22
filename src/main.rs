use std::{
    fs,
    io::Read,
    io::{self, BufRead, BufReader},
    println,
};

use clap::{Parser, ValueEnum};
use indexmap::IndexMap;

#[derive(Debug, Clone, ValueEnum)]
pub enum Format {
    Yaml,
    Json,
}

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long, default_value = "yaml")]
    format: Format,
    #[arg(default_value = "-")]
    file: String,
}

#[derive(Debug)]
struct Entry {
    typ: Option<String>,
    msg: Option<String>,
    file: Option<String>,
    hash: Option<String>,
    ppid: Option<u32>,
    pid: Option<u32>,
    auid: Option<u32>,
    uid: Option<u32>,
    gid: Option<u32>,
    euid: Option<u32>,
    suid: Option<u32>,
    fsuid: Option<u32>,
    egid: Option<u32>,
    sgid: Option<u32>,
    fsgid: Option<u32>,
    sig: Option<u32>,
    tty: Option<String>,
    ses: Option<String>,
    comm: Option<String>,
    exe: Option<String>,
}

fn find_next_split(line: &str) -> usize {
    let mut parens = 0;
    let mut quoted = false;
    let mut double_quoted = false;
    for (i, c) in line.chars().enumerate() {
        if c == ' ' && !double_quoted && !quoted && parens == 0 {
            return i;
        }
        if c == '"' && !quoted && parens == 0 {
            double_quoted = !double_quoted;
        }
        if c == '\'' && !double_quoted && parens == 0 {
            quoted = !quoted;
        }
        if c == '(' && !quoted && !double_quoted {
            parens += 1;
        }
        if c == ')' && !quoted && !double_quoted {
            parens -= 1;
        }
    }
    0
}

fn parse_line(line: &str) -> Entry {
    let mut line = line.trim();
    let mut next = find_next_split(line);
    let mut map = IndexMap::new();
    while next != 0 {
        let element = line[..next].trim();
        let mut split = element.split('=');
        let mut key = split.next().unwrap();
        while key.starts_with('"')
            || key.starts_with('\'')
            || key.starts_with('(')
            || key.ends_with('"')
            || key.ends_with('\'')
            || key.ends_with(')')
        {
            key = key.trim_matches('"');
            key = key.trim_matches('\'');
            key = key.trim_matches('(');
            key = key.trim_matches(')');
        }
        let mut value = split.next().unwrap();
        while value.starts_with('"')
            || value.starts_with('\'')
            || value.ends_with('"')
            || value.ends_with('\'')
        {
            value = value.trim_matches('"');
            value = value.trim_matches('\'');
        }
        map.insert(key, value);
        line = line[next..].trim();
        next = find_next_split(line);
    }
    Entry {
        typ: map
            .get("type")
            .map(|v| v.to_owned().parse().unwrap_or_default()),
        msg: map
            .get("msg")
            .map(|v| v.to_owned().parse().unwrap_or_default()),
        file: map
            .get("file")
            .map(|v| v.to_owned().parse().unwrap_or_default()),
        hash: map
            .get("hash")
            .map(|v| v.to_owned().parse().unwrap_or_default()),
        ppid: map
            .get("ppid")
            .map(|v| v.to_owned().parse().unwrap_or_default()),
        pid: map
            .get("pid")
            .map(|v| v.to_owned().parse().unwrap_or_default()),
        auid: map
            .get("auid")
            .map(|v| v.to_owned().parse().unwrap_or_default()),
        uid: map
            .get("uid")
            .map(|v| v.to_owned().parse().unwrap_or_default()),
        sig: map
            .get("sig")
            .map(|v| v.to_owned().parse().unwrap_or_default()),
        gid: map
            .get("gid")
            .map(|v| v.to_owned().parse().unwrap_or_default()),
        euid: map
            .get("euid")
            .map(|v| v.to_owned().parse().unwrap_or_default()),
        suid: map
            .get("suid")
            .map(|v| v.to_owned().parse().unwrap_or_default()),
        fsuid: map
            .get("fsuid")
            .map(|v| v.to_owned().parse().unwrap_or_default()),
        egid: map
            .get("egid")
            .map(|v| v.to_owned().parse().unwrap_or_default()),
        sgid: map
            .get("sgid")
            .map(|v| v.to_owned().parse().unwrap_or_default()),
        fsgid: map
            .get("fsgid")
            .map(|v| v.to_owned().parse().unwrap_or_default()),
        tty: map
            .get("tty")
            .map(|v| v.to_owned().parse().unwrap_or_default()),
        ses: map
            .get("ses")
            .map(|v| v.to_owned().parse().unwrap_or_default()),
        comm: map
            .get("comm")
            .map(|v| v.to_owned().parse().unwrap_or_default()),
        exe: map
            .get("exe")
            .map(|v| v.to_owned().parse().unwrap_or_default()),
    }
}

type Stats = IndexMap<String, IndexMap<String, IndexMap<String, usize>>>;

fn increment_stat(stats: &mut IndexMap<String, IndexMap<String, usize>>, name: &str, value: &str) {
    *stats
        .entry(name.into())
        .or_default()
        .entry(value.into())
        .or_insert(0) += 1;
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut reader: Box<dyn BufRead> = match cli.file.as_str() {
        "-" => Box::new(BufReader::new(io::stdin())),
        filename => Box::new(BufReader::new(fs::File::open(filename).unwrap())),
    };

    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    let mut stats: Stats = IndexMap::new();
    for line in content.lines() {
        let Entry {
            typ,
            msg,
            file,
            hash,
            ppid,
            pid,
            auid,
            uid,
            gid,
            sig,
            euid,
            suid,
            fsuid,
            egid,
            sgid,
            fsgid,
            tty,
            ses,
            comm,
            exe,
        } = parse_line(line);

        let name = file.clone().unwrap_or_else(|| {
            exe.clone().unwrap_or_else(|| {
                comm.clone()
                    .unwrap_or_else(|| hash.unwrap_or_else(|| "UNKNOWN".to_string()))
            })
        });

        if !stats.contains_key(&name) {
            stats.insert(name.clone(), IndexMap::new());
        }
        for (key, value) in [
            ("type", &typ),
            ("exe", &exe),
            ("msg", &msg),
            ("file", &file),
            // ("hash", &hash),
            ("sig", &sig.map(|v| v.to_string())),
            ("ppid", &ppid.map(|v| v.to_string())),
            ("pid", &pid.map(|v| v.to_string())),
            ("auid", &auid.map(|v| v.to_string())),
            ("uid", &uid.map(|v| v.to_string())),
            ("gid", &gid.map(|v| v.to_string())),
            ("euid", &euid.map(|v| v.to_string())),
            ("suid", &suid.map(|v| v.to_string())),
            ("fsuid", &fsuid.map(|v| v.to_string())),
            ("egid", &egid.map(|v| v.to_string())),
            ("sgid", &sgid.map(|v| v.to_string())),
            ("fsgid", &fsgid.map(|v| v.to_string())),
            ("tty", &tty),
            ("ses", &ses),
            ("comm", &comm),
        ] {
            if let Some(value) = value {
                increment_stat(stats.get_mut(&name).unwrap(), key, value);
            }
        }
    }

    println!(
        "{}",
        match cli.format {
            Format::Yaml => serde_yaml::to_string(&stats)?,
            Format::Json => serde_json::to_string_pretty(&stats)?,
        }
    );

    Ok(())
}

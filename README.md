# audit-stats
Take a linux audit log and transform it into a file representing stats.

e.g.: which signal got raised how often for which process

## Usage
```bash
➜ sudo cat /var/log/audit/audit.log | audit-stats > audit.log.stats.yml
```

## Audit Log Spec
https://access.redhat.com/documentation/de-de/red_hat_enterprise_linux/7/html/security_guide/sec-understanding_audit_log_files

## Format Description (YAML)

```yml
<identifier>:
    <key>:
        <value>: <count>
```

| Field          | Description                                                                                                          |
|----------------|----------------------------------------------------------------------------------------------------------------------|
| \<identifier\> | An identifier to group similar things together. Can be exe,file or hash. If no identifier is found, UNKNOWN is used. |
| \<key\>        | The key from one of the audit-log entries in a line.                                                                 |
| \<value\>      | The value of the audit-log entry, according to the key.                                                              |
| \<count\>      | The amount of occurences of this value.                                                                              |

## Example

The tool converts from this:
```audit
type=SYSCALL msg=audit(1522927552.749:917): arch=c000003e syscall=2 success=yes exit=3 a0=7ffe2ce05793 a1=0 a2=1fffffffffff0000 a3=7ffe2ce043a0 items=1 ppid=2906 pid=4668 auid=1000 uid=0 gid=0 euid=0 suid=0 fsuid=0 egid=0 sgid=0 fsgid=0 tty=pts4 ses=1 comm="cat" exe="/bin/cat" key="passwd"
type=CWD msg=audit(1522927552.749:917):  cwd="/root"
type=PATH msg=audit(1522927552.749:917): item=0 name="/etc/passwd" inode=3147443 dev=08:01 mode=0100644 ouid=0 ogid=0 rdev=00:00 nametype=NORMAL
type=UNKNOWN[1327] msg=audit(1522927552.749:917): proctitle=636174002F6574632F706173737764
```

to this:
```YAML
/bin/cat:
  type:
    SYSCALL: 1
  exe:
    /bin/cat: 1
  msg:
    'audit(1522927552.749:917):': 1
  ppid:
    '2906': 1
  pid:
    '4668': 1
  auid:
    '1000': 1
  uid:
    '0': 1
  gid:
    '0': 1
  euid:
    '0': 1
  suid:
    '0': 1
  fsuid:
    '0': 1
  egid:
    '0': 1
  sgid:
    '0': 1
  fsgid:
    '0': 1
  tty:
    pts4: 1
  ses:
    '1': 1
  comm:
    cat: 1
UNKNOWN:
  type:
    CWD: 1
    PATH: 1
    UNKNOWN[1327]: 1
  msg:
    'audit(1522927552.749:917):': 3
```

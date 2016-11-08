# axe

> log redaction and encryption

## about

Logs should never contain secrets. Unfortunately sometimes mistakes are made
and they sneak in. This program can be placed between your application and
logging system as a final attempt to stop them going any further.

## usage

### detection

The command `axe detect` reads from standard input and exits with failure if
any credentials are detected.

### redaction

The command `axe filter` reads from standard input and will redact any lines
containing credentials. Regular lines are output unchanged.

#### encryption

An encryption key may be passed to `axe filter` using the `-k` argument. If
this is set then lines containing credentials will be encrypted rather than
redacted. This is useful for debugging a problem which requires viewing
sensitive information that should not enter your logging system.

## installation

Not yet.

## benchmarks

These are some are some empirical performance figures from my 4 year old
Macbook Air:

| Task                                      | Lines processed/second  |
|-------------------------------------------|------------------------:|
| Re-outputting when there are no passwords | 271,000                 |
| Redacting when every line is a password   | 371,000                 |
| Encrypting when evey line is a password   | 102,000                 |

You can run these bencharks for yourself by using the `benchmark/check.sh`
script. Make sure to compile the binary in release mode with `cargo build
--release`.

# AXIS

A open source Office-Automation solution.

## Usage

```bash
$ sudo apt-get install rsync git openssh-client
$ cargo install --git https://github.com/saturn-xiv/axis.git
$ ssh-copy-id deploy@xxx.xxx.xxx.xxx
$ RUST_LOG=info axis -i staging -r ping
```

## Test

```bash
$ cargo test -- --nocapture
```

## Documents

- [Toml](https://github.com/toml-lang/toml)
- [Handlebars templating language](https://handlebarsjs.com/guide/)

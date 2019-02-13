AXIS - A radically simple IT automation platform.
---

## Usage

```bash
$ git clone https://github.com/saturn-xiv/axis.git
$ cd axis 
$ make
$ cd dist
$ ./axis -h
```

## Build deb package

```bash
$ sudo apt-get install libzmq3-dev libsqlite3-dev libsodium-dev
$ cargo install cargo-deb
$ cargo deb
```
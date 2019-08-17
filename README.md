AXIS - A radically simple IT automation platform.
---

## Build local docker hub

```bash
$ sudo apt-get install nmap
$ cargo install cargo-deb
$ git clone https://github.com/saturn-xiv/axis.git
$ cd axis 
$ npm install
$ cargo build --release
$ docker build -t axis .
$ docker run --rm -it --network host axis
```

## Run in docker hub

```bash
$ docker run --rm -it --network host chonglou/axis:latest
```
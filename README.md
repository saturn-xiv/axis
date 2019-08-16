AXIS - A radically simple IT automation platform.
---

## Build deb package

```bash
$ sudo apt-get install nmap
$ cargo install cargo-deb
$ git clone https://github.com/saturn-xiv/axis.git
$ cd axis 
$ cargo deb
```

## Run in docker

```bash
$ docker run --rm -it --network host chonglou/axis:latest
```
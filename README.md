# `scdx`
## Simple Columnar inDeX

![Python](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54)![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

A tool for querying the Common Crawl CDX.  Versions in both Python and Rust are included in this repository.  The command–line syntax is identical in both versions.

### Installation:

1. Clone this repository
2. To run the Rust version, compile and run via:
```
$ cargo build --release
```

```
$ cd target/release
$ chmod +x scdx
```

### Usage:

```
scdx --sleep 2 --domain commoncrawl.org --crawls CC-MAIN-2021-04 CC-MAIN-2024-10
```

```
scdx -s 10 -d wikipedia.org -c CC-MAIN-2023-50
```

If no crawls are specified, all crawls will be queried.

The Python version uses [`tqdm`](https://tqdm.github.io/) to display a progress bar, and the Rust version uses [`indicatif`](https://docs.rs/indicatif/latest/indicatif/).

### Licence
[MIT License](LICENSE)

### Thanks
- **[Greg Lindahl](https://github.com/wumpus)**
- **[Pedro Ortiz Suarez](https://github.com/pjox)**

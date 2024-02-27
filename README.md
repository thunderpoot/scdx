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

```bash
scdx --sleep 2 --domain commoncrawl.org --crawls CC-MAIN-2021-04 CC-MAIN-2024-10
```

```bash
scdx -s 10 -d '*.wikipedia.org' -c CC-MAIN-2023-50
```

The program will display a progress bar and output a file with a timestamp (e.g `2024-02-27_18-34-50_output.jsonl`) to the working directory.

The default sleep time is 2 seconds. Please be polite! Polling multiple times a second will make the index server sad.

If no crawls are specified, all crawls will be queried.

The API used supports two methods of wildcarding, like the (more advanced and mature) [cdx-toolkit](https://github.com/cocrawler/cdx_toolkit) by Greg Lindahl.

- **Prefixed asterisk**

    The query `*.example.com`, in CDX jargon sets `matchType='domain'`, and will return captures for `blog.example.com`, `support.example.com`, etc.

- **Appended asterisk**

    The query `example.com/*` will return captures for any page on `example.com`.

The Python version uses [`tqdm`](https://tqdm.github.io/) to display a progress bar, and the Rust version uses [`indicatif`](https://docs.rs/indicatif/latest/indicatif/).

### Licence
[MIT License](LICENSE)

### Thanks
- **[Greg Lindahl](https://github.com/wumpus)**
- **[Pedro Ortiz Suarez](https://github.com/pjox)**

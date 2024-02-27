use clap::{App, Arg};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use chrono::Local;

// Usage:
// scdx -c CC-MAIN-2023-50 -d wikipedia.org -s 10

fn main() {
    let matches = App::new("SCDX: Simple Columnar Index Tool")
        .version("1.0")
        .author("T E Vaughan <thom@commoncrawl.org>")
        .about("A simple tool for querying the Common Crawl CDX for a given domain")
        .arg(
            Arg::with_name("sleep")
                .short("s")
                .long("sleep")
                .value_name("SLEEP")
                .help("Sleep duration in seconds")
                .takes_value(true)
                .default_value("2"),
        )
        .arg(
            Arg::with_name("domain")
                .short("d")
                .long("domain")
                .value_name("DOMAIN")
                .help("Domain to search for")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("crawls")
                .short("c")
                .long("crawls")
                .value_name("CRAWLS")
                .help("Specify which crawl(s) to query. Default is all.")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("latest")
                .short("l")
                .long("latest")
                .help("Check only the latest crawl")
                .takes_value(false)
                .conflicts_with("crawls"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILENAME")
                .help("Specify the output filename")
                .takes_value(true),
        )
        .get_matches();

    let is_latest = matches.is_present("latest");
    let output_filename = matches.value_of("output").map(|s| s.to_string()).unwrap_or_else(|| {
        Local::now().format("%Y-%m-%d_%H-%M-%S_output.jsonl").to_string()
    });
    let sleep_duration = matches.value_of("sleep").unwrap().parse::<u64>().unwrap();
    let domain = matches.value_of("domain").unwrap();
    let crawls: Vec<&str> = matches.values_of("crawls").unwrap_or_default().collect();

    let client = Client::new();
    let collinfo_url = "https://index.commoncrawl.org/collinfo.json";
    let response = client.get(collinfo_url).send().unwrap();

    if response.status().is_success() {
        let crawls_data: Vec<Value> = response.json().unwrap();
        let filtered_crawls: Vec<&Value> = if is_latest {
            crawls_data.iter().take(1).collect() // Assuming the first one is the latest
        } else if !crawls.is_empty() {
            crawls_data.iter().filter(|crawl| crawls.contains(&crawl["id"].as_str().unwrap())).collect()
        } else {
            crawls_data.iter().collect()
        };

        let mut file = File::create(&output_filename).unwrap();

        let pb = ProgressBar::new(filtered_crawls.len() as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
            .progress_chars("#>-"));

        for crawl in filtered_crawls {
            let api_url = format!("{}?url={}/*&output=json", crawl["cdx-api"].as_str().unwrap(), domain);
            let mut success = false;

            while !success {
                let response = client.get(&api_url).send().unwrap();

                match response.status().as_u16() {
                    200 => {
                        let data_lines = response.text().unwrap();
                        for line in data_lines.split('\n') {
                            if !line.is_empty() {
                                let data: Value = serde_json::from_str(line).unwrap();
                                let record = serde_json::to_string(&data).unwrap();
                                writeln!(file, "{}", record).unwrap();
                            }
                        }
                        success = true;
                        pb.inc(1);
                    },
                    503 => {
                        pb.println(format!("Service unavailable for {}, retrying in {} seconds...", crawl["id"].as_str().unwrap(), sleep_duration));
                        std::thread::sleep(std::time::Duration::from_secs(sleep_duration));
                    },
                    404 => {
                        pb.println(format!("No data found for {} in {}. HTTP status code: 404", domain, crawl["id"].as_str().unwrap()));
                        break;
                    },
                    _ => {
                        pb.println(format!("Failed to fetch data for {}. Retrying...", crawl["id"].as_str().unwrap()));
                        std::thread::sleep(std::time::Duration::from_secs(sleep_duration));
                    }
                }
            }
        }
        pb.finish_with_message("Data collection complete.");
        println!("Results saved to {}.", output_filename);
    } else {
        println!("Failed to fetch collinfo.json");
    }
}

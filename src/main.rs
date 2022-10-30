use std::collections::HashSet;
use std::fs;
use std::sync::Arc;

use clap::{Arg, ArgAction, Command};
use futures::StreamExt;
use tokio::sync::{mpsc, Barrier};

mod publisher;
mod url;
mod worker;

#[tokio::main]
async fn main() {
    let cli = Command::new("check")
        .arg(
            Arg::new("mounting-type")
                .long("mounting-type")
                .short('m')
                .action(ArgAction::Set)
                .required(true)
                .help("mounting type of calendar"),
        )
        .arg(
            Arg::new("input-file")
                .long("input-file")
                .short('i')
                .action(ArgAction::Set)
                .required(true)
                .help("path to file with calendar ids to be checked"),
        )
        .arg(
            Arg::new("num-workers")
                .long("num-workers")
                .short('n')
                .action(ArgAction::Set)
                .default_value("10")
                .help("number of workers running in parallel, the number of parallel requests will be squared, DO NOT set this too high, it can flood your tcp/tls handshake pool"),
        );

    let matches = cli.get_matches();

    let mounting_type = matches.get_one::<String>("mounting-type").unwrap();
    let input_file = matches.get_one::<String>("input-file").unwrap();
    let concurrency_str = matches.get_one::<String>("num-workers").unwrap();
    let concurrency = concurrency_str
        .parse::<usize>()
        .expect("could not parse num-workers into usize");

    let ids = fs::read_to_string(input_file)
        .unwrap()
        .split("\n")
        .filter(|number| !number.is_empty())
        .map(|number| number.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let (jobs_tx, jobs_rx) = mpsc::channel(concurrency);
    let (results_tx, mut results_rx) = mpsc::channel(concurrency);
    let barrier = Arc::new(Barrier::new(1));

    let b = Arc::clone(&barrier);
    tokio::spawn(async move {
        let mut missing_ids = HashSet::<u32>::new();
        while let Some(id) = results_rx.recv().await {
            if missing_ids.insert(id) {
                println!("{}\tall: {}", id, missing_ids.len());
            }
        }
        b.wait().await;
    });

    publisher::start_publisher(jobs_tx, ids);
    let worker = worker::Worker::new(results_tx, mounting_type.to_string(), concurrency);
    worker.start(jobs_rx).await;
}

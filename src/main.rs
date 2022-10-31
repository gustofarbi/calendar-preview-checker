use std::fs::File;
use std::io;
use std::io::BufRead;
use std::{collections::HashSet, sync::Arc};

use clap::{Arg, ArgAction, Command};
use futures::StreamExt;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;

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
        )
        .arg(
            Arg::new("refinement")
                .long("refinement")
                .short('r')
                .action(ArgAction::SetTrue)
                .help("requests image with _ref suffix")
        );

    let matches = cli.get_matches();

    let mounting_type = matches.get_one::<String>("mounting-type").unwrap();
    let input_file = matches.get_one::<String>("input-file").unwrap();
    let concurrency_str = matches.get_one::<String>("num-workers").unwrap();
    let concurrency = concurrency_str
        .parse::<usize>()
        .expect("could not parse num-workers into usize");
    let refinement = matches.get_flag("refinement");

    let ids = io::BufReader::new(File::open(input_file).expect("file not found"))
        .lines()
        .filter_map(|number_maybe| number_maybe.ok())
        .filter_map(|number_string| number_string.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let (jobs_tx, jobs_rx) = mpsc::channel(concurrency);
    let (results_tx, mut results_rx) = mpsc::channel(concurrency);

    tokio::spawn(async move {
        let mut missing_ids = HashSet::<u32>::new();

        loop {
            let id = match results_rx.try_recv() {
                Ok(id) => id,
                Err(e) => match e {
                    TryRecvError::Empty => {
                        continue;
                    }
                    TryRecvError::Disconnected => {
                        println!("receiver finished");
                        return;
                    }
                },
            };
            if missing_ids.insert(id) {
                println!("{}\tall:{}", id, missing_ids.len());
            }
        }
    });

    publisher::start_publisher(jobs_tx, ids);

    let worker = worker::Worker::new(mounting_type.to_string(), refinement, concurrency);
    let results_tx = &Arc::from(results_tx);

    worker.start(jobs_rx, results_tx).await;
}

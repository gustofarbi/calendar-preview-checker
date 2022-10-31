use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::sync::Arc;

use clap::ArgMatches;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;

use async_trait::async_trait;
use indicatif::ProgressBar;

use crate::handler::Handler;
use crate::progress_bar;

mod publisher;
mod url;
mod worker;

pub struct Preview {}

impl Preview {
    pub fn new() -> Self {
        Preview {}
    }
}

#[async_trait]
impl Handler for Preview {
    async fn handle(&self, matches: &ArgMatches) {
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
        let (progress_bar_tx, progress_bar_rx) = mpsc::channel(concurrency);

        let progress_bar = ProgressBar::new(ids.len() as u64);

        progress_bar::start(progress_bar_rx, progress_bar);

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
                            break;
                        }
                    },
                };
                if missing_ids.insert(id) {
//                    println!("{}\tall:{}", id, missing_ids.len());
                }
            }

            let mut missing_ids = Vec::from_iter(missing_ids);
            missing_ids.sort();
            println!("{:?}", missing_ids);
        });

        publisher::start_publisher(jobs_tx, ids);

        let worker = worker::Worker::new(mounting_type.to_string(), refinement, concurrency);
        let results_tx = &Arc::from(results_tx);
        let progress_bar_tx = &Arc::from(progress_bar_tx);

        worker.start(jobs_rx, results_tx, progress_bar_tx).await;
    }
}

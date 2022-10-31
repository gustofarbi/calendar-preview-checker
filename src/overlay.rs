use std::{collections::HashSet, fs::File, sync::Arc};

use tokio::sync::mpsc::{self, error::TryRecvError};

use async_trait::async_trait;
use indicatif::ProgressBar;
use serde::Deserialize;

use crate::handler::Handler;

mod publisher;
mod url;
mod worker;
mod progress_bar;

#[derive(Deserialize)]
pub struct Item {
    id: u32,
    path: String,
    hash: String,
}

pub struct Overlay {}

impl Overlay {
    pub fn new() -> Self {
        Overlay {}
    }
}

#[async_trait]
impl Handler for Overlay {
    async fn handle(&self, matches: &clap::ArgMatches) {
        let year = matches
            .get_one::<String>("year")
            .expect("could not parse arg")
            .parse::<u32>()
            .expect("arg is not a u32 integer");

        // todo this is common, put it elsewhere
        let input_file = matches.get_one::<String>("input-file").unwrap();
        let concurrency_str = matches.get_one::<String>("num-workers").unwrap();
        let concurrency = concurrency_str
            .parse::<usize>()
            .expect("could not parse num-workers into usize");
        let refinement = matches.get_flag("refinement");

        let items: Vec<Item> =
            serde_json::from_reader(File::open(input_file).expect("file not found")).unwrap();

        let (jobs_tx, jobs_rx) = mpsc::channel(concurrency);
        let (results_tx, mut results_rx) = mpsc::channel(concurrency);
        let (progress_bar_tx, progress_bar_rx) = mpsc::channel(concurrency);

        let progress_bar = ProgressBar::new(items.len() as u64);

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

        publisher::start_publisher(jobs_tx, items);

        let worker = worker::Worker::new(year, refinement, concurrency);
        let results_tx = &Arc::from(results_tx);
        let progress_bar_tx = &Arc::from(progress_bar_tx);

        worker.start(jobs_rx, results_tx, progress_bar_tx).await;
    }
}

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::sync::Arc;

use async_trait::async_trait;
use clap::ArgMatches;
use tokio::sync::mpsc;

use crate::{progress_bar, receiver};
use crate::handler::Handler;

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

        let progress_bar = progress_bar::get(ids.len() as u64);

        progress_bar::start(progress_bar_rx, progress_bar);

        tokio::spawn(async move {
            receiver::start(&mut results_rx);
        });

        publisher::start(jobs_tx, ids);

        worker::Worker::new(mounting_type.to_string(), refinement, concurrency)
            .start(jobs_rx, &Arc::from(results_tx), &Arc::from(progress_bar_tx))
            .await;
    }
}

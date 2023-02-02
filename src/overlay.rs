use std::{fs::File, sync::Arc};

use async_trait::async_trait;
use serde::Deserialize;
use tokio::sync::mpsc;

use crate::{handler::Handler, progress_bar, receiver};

mod publisher;
mod url;
mod worker;

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

        let progress_bar = progress_bar::get(items.len() as u64);

        progress_bar::start(progress_bar_rx, progress_bar);

        tokio::spawn(async move {
            receiver::start(&mut results_rx);
        });

        publisher::start(jobs_tx, items);

        worker::Worker::new(year, refinement, concurrency)
            .start(jobs_rx, &Arc::from(results_tx), &Arc::from(progress_bar_tx))
            .await;
    }
}

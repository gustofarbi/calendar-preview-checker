use std::{sync::Arc, time::Duration};
use std::sync::atomic::AtomicBool;

use futures::stream;
use reqwest::Client;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::overlay::url::build_urls;
use crate::StreamExt;
use crate::url_handler::UrlHandler;

pub struct Worker {
    year: u32,
    refinement: bool,
    concurrency: usize,
}

impl Worker {
    pub fn new(year: u32, refinement: bool, concurrency: usize) -> Self {
        Worker {
            year,
            refinement,
            concurrency,
        }
    }

    pub async fn start(
        &self,
        jobs_rx: Receiver<(u32, String, String)>,
        results_tx: &Arc<Sender<u32>>,
        progress_bar_tx: &Arc<Sender<()>>,
    ) {
        tokio_stream::wrappers::ReceiverStream::new(jobs_rx)
            .for_each_concurrent(self.concurrency, |(id, path, hash)| async move {
                let client = Client::builder()
                    .timeout(Duration::from_secs(3))
                    .build()
                    .unwrap();
                let break_loop = AtomicBool::new(false);
                progress_bar_tx.send(()).await.unwrap();

                let url_handler = UrlHandler::new(id, client, break_loop);

                stream::iter(build_urls(self.year, path, hash, self.refinement))
                    .for_each_concurrent(self.concurrency, |url| async {
                        url_handler.try_one(url, results_tx).await;
                    })
                    .await;
            })
            .await;

        println!("workers finished");
    }
}

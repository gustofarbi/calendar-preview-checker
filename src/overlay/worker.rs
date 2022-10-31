use std::{sync::Arc, time::Duration};
use std::sync::atomic::{AtomicBool, Ordering};

use futures::stream;
use reqwest::Client;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::overlay::url::build_urls;
use crate::StreamExt;

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

    pub async fn start(&self, jobs_rx: Receiver<(u32, String, String)>, results_tx: &Arc<Sender<u32>>) {
        tokio_stream::wrappers::ReceiverStream::new(jobs_rx)
            .for_each_concurrent(self.concurrency, |(id, path, hash)| async move {
                let client = Client::builder()
                    .timeout(Duration::from_secs(3))
                    .build()
                    .unwrap();
                let break_loop = AtomicBool::new(false);

                stream::iter(build_urls(self.year, path, hash, self.refinement))
                    .for_each_concurrent(self.concurrency, |url| async {
                        if break_loop.load(Ordering::SeqCst) {
                            return;
                        }
                        let response = client.head(url).send().await;
                        if response.is_err() {
                            break_loop.store(true, Ordering::SeqCst);
                            results_tx.send(id).await.unwrap();
                            return;
                        }
                        let response = response.unwrap();
                        if !response.status().is_success() {
                            break_loop.store(true, Ordering::SeqCst);
                            results_tx.send(id).await.unwrap();
                            return;
                        }
                    })
                    .await;
            })
            .await;

        println!("workers finished");
    }
}

use std::fmt::Write;

use futures::StreamExt;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use tokio::sync::mpsc::Receiver;
use tokio_stream::wrappers::ReceiverStream;

pub fn get(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));
    pb
}

pub fn start(progress_bar_rx: Receiver<()>, progress_bar: ProgressBar) {
    tokio::spawn(async move {
        ReceiverStream::new(progress_bar_rx)
            .for_each(|_| async {
                progress_bar.inc(1);
            })
            .await;
    });
}

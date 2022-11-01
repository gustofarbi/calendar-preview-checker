use std::collections::HashSet;

use tokio::sync::mpsc::{error::TryRecvError, Receiver};

pub fn start(results_rx: &mut Receiver<u32>) {
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
        if missing_ids.insert(id) {}
    }

    let mut missing_ids = Vec::from_iter(missing_ids);
    missing_ids.sort();
    println!("{:?}", missing_ids);
}

use restaurant::{startup::spawn_app, tablet::start_tablet, types::Request};
use std::{sync::mpsc::channel, thread};

const TABLET_COUNT: usize = 10;
const TABLET_WAIT_MS: u64 = 0;
const TABLE_COUNT: usize = 100;
const PRINT_REQUEST_COUNT_INTERVAL: u64 = 10000;

fn main() {
    let (tx, rx) = channel::<Request>();

    [0; TABLET_COUNT].iter().for_each(|_| {
        let requester = tx.clone();
        thread::spawn(move || {
            start_tablet(requester, TABLET_WAIT_MS);
        });
    });

    spawn_app(rx, TABLE_COUNT, PRINT_REQUEST_COUNT_INTERVAL);
}

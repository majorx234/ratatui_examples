use crossbeam_channel::{unbounded, Receiver, Sender};
use std::{thread, time::Duration};
pub struct Dummy {
    rx_close: Receiver<bool>,
    tx_status: Sender<f64>,
}

impl Dummy {
    pub fn start() -> (thread::JoinHandle<()>, Sender<bool>, Receiver<f64>) {
        let (tx_close, rx_close) = unbounded();
        let (tx_status, rx_status) = unbounded();
        let dummy_obj = Self {
            rx_close,
            tx_status,
        };
        let thread_join_handle = std::thread::spawn(move || {
            let mut run = true;
            while run {
                thread::sleep(Duration::from_millis(100));
                match dummy_obj.rx_close.recv() {
                    Ok(running) => run = running,
                    Err(_) => run = false,
                }
            }
        });
        (thread_join_handle, tx_close, rx_status)
    }
}

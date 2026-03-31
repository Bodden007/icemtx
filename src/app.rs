pub mod protocol;
pub mod services;
pub mod transport;

use crate::app::protocol::flecs::{FlecsField, FlecsFrame};
use crate::app::services::poller::Poller;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub fn run() {
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = shutdown.clone();

    // Ctrl+C handler
    if let Err(e) = ctrlc::set_handler(move || {
        eprintln!("\nВыход по сигналу...");
        shutdown_clone.store(true, Ordering::Relaxed);
    }) {
        eprintln!("Не удалось поставить хендлер: {}", e);
    }

    // Запускаем поллер
    match Poller::new(shutdown, "COM3", 9600) {
        Ok(mut poller) => poller.run(),
        Err(e) => eprintln!("Поллер не запустился: {}", e),
    }
}

use crate::app::protocol::flecs::{FlecsField, FlecsFrame};
use crate::app::transport::serial::SerialSession;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

pub struct Poller {
    shutdown: Arc<AtomicBool>,
    session: SerialSession,
    interval: Duration,
}

impl Poller {
    pub fn new(shutdown: Arc<AtomicBool>, port: &str, baud: u32) -> Result<Self, String> {
        let session = SerialSession::new(port, baud)?;
        Ok(Self {
            shutdown,
            session,
            interval: Duration::from_secs(1),
        })
    }

    pub fn run(&mut self) {
        let mut next_tick = Instant::now();

        while !self.shutdown.load(Ordering::Relaxed) {
            let mut frame = FlecsFrame::new_zero();
            frame.set(FlecsField::RecircDensity, "12.34");
            frame.set(FlecsField::MixWaterRate, "8");

            let data = frame.build_line();

            if let Err(e) = self.session.send(data.as_bytes()) {
                eprintln!("Отправка ебнулась: {}", e);
            }

            next_tick = next_tick + self.interval;
            let sleep_time = next_tick.saturating_duration_since(Instant::now());
            if sleep_time > Duration::ZERO {
                std::thread::sleep(sleep_time);
            }
        }

        println!("Поллер остановлен, порт освобождён.");
    }
}

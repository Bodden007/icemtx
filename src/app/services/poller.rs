//! Сервис периодического опроса устройств.
//!
//! Этот модуль содержит структуру [`Poller`], которая выполняет циклический
//! опрос внешних устройств через последовательный порт с заданным интервалом.
//!
//! # Основные функции
//!
//! - Создание сессии опроса с указанием порта и скорости
//! - Периодическая отправка FLECS-фреймов
//! - Обработка сигнала завершения (Ctrl+C)
//! - Точное соблюдение временных интервалов
//!
//! # Пример использования
//!
//! ```rust
//! use std::sync::Arc;
//! use std::sync::atomic::AtomicBool;
//! use crate::app::services::poller::Poller;
//!
//! let shutdown = Arc::new(AtomicBool::new(false));
//! let mut poller = Poller::new(shutdown, "COM3", 9600).unwrap();
//! poller.run();
//! ```

use crate::app::protocol::flecs::{FlecsField, FlecsFrame};
use crate::app::transport::serial::SerialSession;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// Сервис периодического опроса устройств.
///
/// Отправляет FLECS-фреймы через последовательный порт с фиксированным
/// интервалом (по умолчанию 1 секунда). Работает до получения сигнала
/// завершения через атомарный флаг `shutdown`.
///
/// # Поля
///
/// - `shutdown` - атомарный флаг для graceful shutdown
/// - `session` - сессия последовательного порта
/// - `interval` - интервал между отправками фреймов
pub struct Poller {
    shutdown: Arc<AtomicBool>,
    session: SerialSession,
    interval: Duration,
}

impl Poller {
    /// Создаёт новый экземпляр Poller.
    ///
    /// # Аргументы
    ///
    /// * `shutdown` - атомарный флаг для управления завершением работы
    /// * `port` - имя последовательного порта
    /// * `baud` - скорость передачи данных
    ///
    /// # Возвращает
    ///
    /// `Result<Self, String>` - успешный Poller или сообщение об ошибке
    ///
    /// # Пример
    ///
    /// ```rust
    /// let shutdown = Arc::new(AtomicBool::new(false));
    /// let poller = Poller::new(shutdown, "COM3", 9600).unwrap();
    /// ```
    pub fn new(shutdown: Arc<AtomicBool>, port: &str, baud: u32) -> Result<Self, String> {
        let session = SerialSession::new(port, baud)?;
        Ok(Self {
            shutdown,
            session,
            interval: Duration::from_secs(1),
        })
    }

    /// Запускает основной цикл опроса.
    ///
    /// В бесконечном цикле:
    /// 1. Создаёт FLECS-фрейм с тестовыми данными
    /// 2. Отправляет фрейм через последовательный порт
    /// 3. Ожидает до следующего тика с учётом интервала
    ///
    /// Цикл прерывается при установке флага `shutdown` в `true`.
    ///
    /// # Обработка ошибок
    ///
    /// Ошибки отправки выводятся в stderr, но не прерывают цикл опроса.
    pub fn run(&mut self) {
        let mut next_tick = Instant::now();

        while !self.shutdown.load(Ordering::Relaxed) {
            let mut frame = FlecsFrame::new_zero();
            frame.set(FlecsField::RecircDensity, "12.34");
            frame.set(FlecsField::MixWaterRate, "8");

            let data = frame.build_line();

            if let Err(e) = self.session.send(data.as_bytes()) {
                eprintln!("Ошибка отправки: {}", e);
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

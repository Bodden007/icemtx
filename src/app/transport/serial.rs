//! Реализация работы с последовательным портом.
//!
//! Этот модуль предоставляет структуру [`SerialSession`] для управления
//! соединением через последовательный порт (RS-232/RS-485).
//!
//! # Пример использования
//!
//! ```rust
//! use crate::app::transport::serial::SerialSession;
//!
//! // Открытие порта
//! let mut session = SerialSession::new("COM3", 9600)?;
//!
//! // Отправка данных
//! session.send(b"Hello, world!")?;
//! ```
//!
//! # Обработка ошибок
//!
//! Все методы возвращают `Result<_, String>`, где ошибка представлена в виде
//! человекочитаемого сообщения.

use serialport::SerialPort;
use std::time::Duration;

/// Сессия последовательного порта.
///
/// Инкапсулирует соединение с последовательным портом и предоставляет
/// методы для отправки данных. При удалении автоматически закрывает порт.
///
/// # Поля
///
/// - `port` - внутренний объект порта из библиотеки `serialport`
pub struct SerialSession {
    port: Box<dyn SerialPort>,
}

impl SerialSession {
    /// Создаёт новую сессию последовательного порта.
    ///
    /// # Аргументы
    ///
    /// * `port_name` - имя порта (например, "COM3" на Windows или "/dev/ttyUSB0" на Linux)
    /// * `baud_rate` - скорость передачи данных в бодах
    ///
    /// # Возвращает
    ///
    /// `Result<Self, String>` - успешная сессия или сообщение об ошибке
    ///
    /// # Пример
    ///
    /// ```rust
    /// let session = SerialSession::new("COM3", 9600).unwrap();
    /// ```
    pub fn new(port_name: &str, baud_rate: u32) -> Result<Self, String> {
        let port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(100))
            .open()
            .map_err(|e| format!("Не удалось открыть порт: {}", e))?;

        Ok(Self { port })
    }

    /// Отправляет данные через последовательный порт.
    ///
    /// # Аргументы
    ///
    /// * `data` - байтовый срез для отправки
    ///
    /// # Возвращает
    ///
    /// `Result<(), String>` - успех или сообщение об ошибке записи
    ///
    /// # Пример
    ///
    /// ```rust
    /// session.send(b"Hello, world!").unwrap();
    /// ```
    pub fn send(&mut self, data: &[u8]) -> Result<(), String> {
        self.port
            .write_all(data)
            .map_err(|e| format!("Ошибка записи в порт: {}", e))
    }

    /// Переподключается к порту с новыми параметрами.
    ///
    /// Закрывает текущее соединение и открывает новое с указанными
    /// именем порта и скоростью передачи.
    ///
    /// # Аргументы
    ///
    /// * `port_name` - новое имя порта
    /// * `baud_rate` - новая скорость передачи
    ///
    /// # Возвращает
    ///
    /// `Result<(), String>` - успех или сообщение об ошибке
    ///
    /// # Примечание
    ///
    /// Этот метод помечен как `#[expect(dead_code)]`, так как в текущей
    /// реализации он не используется, но может быть полезен для будущих
    /// улучшений.
    #[expect(dead_code)]
    pub fn reconnect(&mut self, port_name: &str, baud_rate: u32) -> Result<(), String> {
        // Старый порт закроется сам через drop поля port
        self.port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(100))
            .open()
            .map_err(|e| format!("Не удалось переподключиться к порту: {}", e))?;
        Ok(())
    }
}

impl Drop for SerialSession {
    /// Освобождает ресурсы порта при удалении сессии.
    ///
    /// Автоматически закрывает порт и очищает выходной буфер.
    fn drop(&mut self) {
        // Порт закроется сам, когда умрёт box<dyn SerialPort>
        // Но можно явно очистить буфер, чтобы данные ушли
        let _ = self.port.clear(serialport::ClearBuffer::Output);
    }
}

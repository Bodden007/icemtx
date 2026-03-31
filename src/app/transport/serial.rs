use serialport::{SerialPort, SerialPortType};
use std::time::Duration;

pub struct SerialSession {
    port: Box<dyn SerialPort>,
}

impl SerialSession {
    pub fn new(port_name: &str, baud_rate: u32) -> Result<Self, String> {
        let port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(100))
            .open()
            .map_err(|e| format!("Порт не открылся, сука: {}", e))?;

        Ok(Self { port })
    }

    pub fn send(&mut self, data: &[u8]) -> Result<(), String> {
        self.port
            .write_all(data)
            .map_err(|e| format!("Запись в порт ебнулась: {}", e))
    }

    pub fn reconnect(&mut self, port_name: &str, baud_rate: u32) -> Result<(), String> {
        // Старый порт закроется сам через drop поля port
        self.port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(100))
            .open()
            .map_err(|e| format!("Ребилд порта проебался: {}", e))?;
        Ok(())
    }
}

impl Drop for SerialSession {
    fn drop(&mut self) {
        // Порт закроется сам, когда умрёт box<dyn SerialPort>
        // Но можно явно флешнуть, чтобы данные ушли
        let _ = self.port.clear(serialport::ClearBuffer::Output);
    }
}

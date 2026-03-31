//! Структура FLECS-фрейма.
//!
//! Этот модуль содержит структуру [`FlecsFrame`], которая представляет
//! полный фрейм данных протокола FLECS. Фрейм состоит из 32 полей,
//! каждое из которых хранит строковое значение.
//!
//! # Использование
//!
//! 1. Создать фрейм с нулевыми значениями через [`new_zero()`](FlecsFrame::new_zero)
//! 2. Установить значения полей через метод [`set()`](FlecsFrame::set)
//! 3. Построить строку для передачи через метод [`build_line()`](FlecsFrame::build_line)
//!
//! # Пример
//!
//! ```rust
//! use crate::app::protocol::flecs::{FlecsField, FlecsFrame};
//!
//! let mut frame = FlecsFrame::new_zero();
//! frame.set(FlecsField::RecircDensity, "12.34");
//! frame.set(FlecsField::MixWaterRate, "8");
//! let line = frame.build_line();
//! ```

use crate::app::protocol::flecs::FlecsField;
use crate::app::protocol::flecs::format::format_field;

/// FLECS-фрейм, содержащий значения всех 32 полей.
///
/// Фрейм хранит значения в виде строк, что позволяет гибко
/// работать с различными форматами чисел. При создании все
/// поля инициализируются значением "0", как того требует
/// протокол iCem для отсутствующих значений.
pub struct FlecsFrame {
    values: [String; FlecsField::COUNT],
}

impl FlecsFrame {
    /// Создаёт новый фрейм со всеми полями, установленными в "0".
    ///
    /// # Возвращает
    ///
    /// Новый экземпляр [`FlecsFrame`] с нулевыми значениями.
    ///
    /// # Пример
    ///
    /// ```rust
    /// let frame = FlecsFrame::new_zero();
    /// assert_eq!(frame.get(FlecsField::RecircDensity), "0");
    /// ```
    pub fn new_zero() -> Self {
        Self {
            values: std::array::from_fn(|_| "0".to_string()),
        }
    }

    /// Устанавливает значение указанного поля.
    ///
    /// # Аргументы
    ///
    /// * `field` - поле для установки
    /// * `value` - значение, которое будет преобразовано в строку
    ///
    /// # Пример
    ///
    /// ```rust
    /// frame.set(FlecsField::RecircDensity, "12.34");
    /// frame.set(FlecsField::MixWaterRate, 8); // автоматическое преобразование
    /// ```
    pub fn set(&mut self, field: FlecsField, value: impl Into<String>) {
        self.values[field.idx()] = value.into();
    }

    /// Возвращает значение указанного поля.
    ///
    /// # Аргументы
    ///
    /// * `field` - поле для получения значения
    ///
    /// # Возвращает
    ///
    /// Ссылку на строку с значением поля.
    ///
    /// # Пример
    ///
    /// ```rust
    /// let value = frame.get(FlecsField::RecircDensity);
    /// println!("Плотность: {}", value);
    /// ```
    pub fn get(&self, field: FlecsField) -> &str {
        &self.values[field.idx()]
    }

    /// Строит строку для передачи через последовательный порт.
    ///
    /// Метод форматирует все поля фрейма в соответствии с их
    /// метаданными (точность, ширина, единицы измерения) и
    /// добавляет завершающие символы `\r\n`.
    ///
    /// # Возвращает
    ///
    /// Отформатированную строку, готовую для отправки.
    ///
    /// # Пример
    ///
    /// ```rust
    /// let line = frame.build_line();
    /// // Пример результата: "12.34 ppg     8 gpm       ...\r\n"
    /// ```
    pub fn build_line(&self) -> String {
        let mut out = String::new();

        for idx in 0..FlecsField::COUNT {
            let field = unsafe {
                // безопасно, потому что enum жёстко 0..31
                std::mem::transmute::<u8, FlecsField>(idx as u8)
            };

            out.push_str(&format_field(field, self.get(field)));
        }

        out.push_str("\r\n");
        out
    }
}

//! Модуль протокола FLECS (Flexible Embedded Communication Standard).
//!
//! Этот модуль предоставляет структуры и функции для работы с протоколом FLECS,
//! используемым для обмена данными с промышленными контроллерами.
//!
//! # Компоненты
//!
//! - [`fieldmeta`] - метаданные полей FLECS
//! - [`flecsfield`] - перечисление полей FLECS
//! - [`flecsframe`] - структура FLECS-фрейма
//! - [`mod@format`] - форматирование данных для передачи
//!
//! # Основные типы
//!
//! - [`FlecsField`] - перечисление всех поддерживаемых полей
//! - [`FlecsFrame`] - контейнер для данных фрейма
//!
//! # Пример использования
//!
//! ```rust
//! use crate::app::protocol::flecs::{FlecsField, FlecsFrame};
//!
//! let mut frame = FlecsFrame::new_zero();
//! frame.set(FlecsField::RecircDensity, "12.34");
//! let line = frame.build_line();
//! ```

pub mod fieldmeta;
pub mod flecsfield;
pub mod flecsframe;
pub mod format;

// pub use fieldmeta::FieldMeta;
pub use flecsfield::FlecsField;
pub use flecsframe::FlecsFrame;

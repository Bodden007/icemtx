//! Метаданные полей FLECS.
//!
//! Этот модуль содержит структуру [`FieldMeta`], которая описывает
//! форматирование и единицы измерения для каждого поля FLECS.
//!
//! # Структура метаданных
//!
//! Каждое поле FLECS имеет:
//! - Единицу измерения (unit) - например, "ppg", "psi", "bpm"
//! - Точность (precision) - количество знаков после запятой (0-3)
//! - Ширину (width) - общая ширина поля при форматировании
//!
//! # Использование
//!
//! Метод `FlecsField::meta()` возвращает метаданные для конкретного поля.
//! Эти метаданные используются при форматировании значений для передачи.

use crate::app::protocol::flecs::FlecsField;

/// Метаданные поля FLECS.
///
/// Описывает как значение поля должно быть отформатировано
/// при преобразовании в строку для передачи.
///
/// # Поля
///
/// - `unit` - единица измерения (например, "ppg" для плотности)
/// - `precision` - количество знаков после запятой (0, 1, 2, 3)
/// - `width` - общая ширина поля при выравнивании пробелами
#[derive(Debug, Clone, Copy)]
pub struct FieldMeta {
    pub unit: Option<&'static str>,
    pub precision: u8, // 0,1,2,3
    pub width: usize,  // выравнивание пробелами 0..20
}

impl FlecsField {
    /// Возвращает метаданные для текущего поля FLECS.
    ///
    /// Метод предоставляет информацию о форматировании значения поля,
    /// включая единицу измерения, точность и ширину.
    ///
    /// # Возвращает
    ///
    /// Структуру [`FieldMeta`] с метаданными поля.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use crate::app::protocol::flecs::FlecsField;
    ///
    /// let meta = FlecsField::RecircDensity.meta();
    /// assert_eq!(meta.unit, Some("ppg"));
    /// assert_eq!(meta.precision, 2);
    /// ```
    pub fn meta(self) -> FieldMeta {
        match self {
            // 1..2 density
            FlecsField::RecircDensity => FieldMeta {
                unit: Some("ppg"),
                precision: 2,
                width: 12,
            }, // 0.00
            FlecsField::DownholeDensity => FieldMeta {
                unit: Some("ppg"),
                precision: 2,
                width: 12,
            }, // 0.00

            // 3 water rate
            FlecsField::MixWaterRate => FieldMeta {
                unit: Some("gpm"),
                precision: 0,
                width: 12,
            }, // 0

            // 4..6 rates/press
            FlecsField::CombPumpRate => FieldMeta {
                unit: Some("bpm"),
                precision: 1,
                width: 12,
            }, // 0.0

            FlecsField::PsPumpPress => FieldMeta {
                unit: Some("psi"),
                precision: 0,
                width: 12,
            }, // 0

            FlecsField::DsPumpPress => FieldMeta {
                unit: Some("psi"),
                precision: 0,
                width: 12,
            }, // 0

            // 7..8 totals (gal)
            FlecsField::WaterStageTotal => FieldMeta {
                unit: Some("gal"),
                precision: 0,
                width: 12,
            }, // 0

            FlecsField::MixWaterTotal => FieldMeta {
                unit: Some("gal"),
                precision: 0,
                width: 12,
            }, // 0

            // 9..10 totals (bbl)
            FlecsField::PumpStageTotal => FieldMeta {
                unit: Some("bbl"),
                precision: 1,
                width: 12,
            }, // 0.0
            FlecsField::CombinedPumpTotal => FieldMeta {
                unit: Some("bbl"),
                precision: 1,
                width: 12,
            }, // 0.0

            // 11..12 valve position
            FlecsField::CmntValvePosition => FieldMeta {
                unit: Some("cmt%"),
                precision: 2,
                width: 12,
            }, // 0.00
            FlecsField::WaterValvePosition => FieldMeta {
                unit: Some("wtr%"),
                precision: 2,
                width: 12,
            }, // 0.00

            // 13..14 pump rates
            FlecsField::PsPumpRate => FieldMeta {
                unit: Some("bpm"),
                precision: 1,
                width: 12,
            }, // 0.0
            FlecsField::DsPumpRate => FieldMeta {
                unit: Some("bpm"),
                precision: 1,
                width: 12,
            }, // 0.0

            // 15..17 misc
            FlecsField::DigitalOuts => FieldMeta {
                unit: Some("DigOut"),
                precision: 0,
                width: 12,
            }, // 0
            FlecsField::HalWinEvent => FieldMeta {
                unit: None,
                precision: 0,
                width: 12,
            }, // 0 (unit может отсутствовать)

            FlecsField::Temperature => FieldMeta {
                unit: Some("F"),
                precision: 0,
                width: 12,
            }, // 0

            // 18..20 unused but still transmitted
            FlecsField::Rho12Hat => FieldMeta {
                unit: Some("ppg"),
                precision: 2,
                width: 12,
            }, // 0.00

            FlecsField::H2Hat => FieldMeta {
                unit: Some("ft"),
                precision: 2,
                width: 12,
            }, // 0.00

            FlecsField::VhpDensityHp => FieldMeta {
                unit: Some("ppg"),
                precision: 2,
                width: 12,
            }, // 0.00

            // 21..27 extra channels
            FlecsField::TubHeight => FieldMeta {
                unit: Some("ft"),
                precision: 2,
                width: 12,
            }, // 0.00

            FlecsField::ExtraPress1 => FieldMeta {
                unit: Some("psi"),
                precision: 0,
                width: 12,
            }, // 0
            FlecsField::ExtraPress2 => FieldMeta {
                unit: Some("psi"),
                precision: 0,
                width: 12,
            }, // 0

            FlecsField::ExtraRate1 => FieldMeta {
                unit: Some("bpm"),
                precision: 1,
                width: 12,
            }, // 0.0

            FlecsField::ExtraTotal1 => FieldMeta {
                unit: Some("bbl"),
                precision: 1,
                width: 12,
            }, // 0.0

            FlecsField::ExtraRate2 => FieldMeta {
                unit: Some("bpm"),
                precision: 1,
                width: 12,
            }, // 0.0

            FlecsField::ExtraTotal2 => FieldMeta {
                unit: Some("bbl"),
                precision: 1,
                width: 12,
            }, // 0.0

            // 28..32
            FlecsField::WaterSlurryRatio => FieldMeta {
                unit: Some("%"),
                precision: 2,
                width: 12,
            }, // 0.00

            FlecsField::MudCupDensity => FieldMeta {
                unit: Some("ppg"),
                precision: 3,
                width: 12,
            }, // 0.000

            FlecsField::PsPumpStageTotal => FieldMeta {
                unit: Some("bbl"),
                precision: 1,
                width: 12,
            }, // 0.0
            FlecsField::DsPumpStageTotal => FieldMeta {
                unit: Some("bbl"),
                precision: 1,
                width: 12,
            }, // 0.0

            FlecsField::VolumetricMeter => FieldMeta {
                unit: Some("bpm"),
                precision: 1,
                width: 12,
            }, // 0.0
        }
    }
}

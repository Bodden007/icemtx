//! Перечисление полей протокола FLECS.
//!
//! Этот модуль содержит перечисление [`FlecsField`], которое представляет
//! все возможные поля данных в протоколе FLECS. Каждое поле имеет
//! числовой идентификатор (0-31), соответствующий его позиции в фрейме.
//!
//! # Структура полей
//!
//! Поля сгруппированы по категориям:
//! - Плотность (Density)
//! - Расход (Rate)
//! - Давление (Pressure)
//! - Суммарные значения (Totals)
//! - Позиции клапанов (Valve Positions)
//! - Дополнительные каналы (Extra Channels)
//!
//! # Использование
//!
//! Перечисление используется для доступа к данным в [`FlecsFrame`](super::flecsframe::FlecsFrame) и
//! для получения метаданных через метод [`meta()`](FlecsField::meta).

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[expect(dead_code)]
pub enum FlecsField {
    /// Плотность рециркуляции (ppg)
    RecircDensity = 0, // 1
    /// Плотность в скважине (ppg)
    DownholeDensity = 1, // 2
    /// Расход смешиваемой воды (gpm)
    MixWaterRate = 2, // 3
    /// Совокупный расход насоса (bpm)
    CombPumpRate = 3, // 4
    /// Давление насоса PS (psi)
    PsPumpPress = 4, // 5
    /// Давление насоса DS (psi)
    DsPumpPress = 5, // 6
    /// Суммарный расход воды по стадиям (gal)
    WaterStageTotal = 6, // 7
    /// Суммарный расход смешиваемой воды (gal)
    MixWaterTotal = 7, // 8
    /// Суммарный расход насоса по стадиям (bbl)
    PumpStageTotal = 8, // 9
    /// Совокупный суммарный расход насоса (bbl)
    CombinedPumpTotal = 9, // 10
    /// Позиция клапана цемента (cmt%)
    CmntValvePosition = 10, // 11
    /// Позиция водяного клапана (wtr%)
    WaterValvePosition = 11, // 12
    /// Расход насоса PS (bpm)
    PsPumpRate = 12, // 13
    /// Расход насоса DS (bpm)
    DsPumpRate = 13, // 14
    /// Цифровые выходы
    DigitalOuts = 14, // 15
    /// Событие HalWin
    HalWinEvent = 15, // 16
    /// Температура (F)
    Temperature = 16, // 17
    /// Оценочная плотность Rho12 (ppg, не используется)
    Rho12Hat = 17, // 18 (unused)
    /// Оценочная высота H2 (ft, не используется)
    H2Hat = 18, // 19 (unused)
    /// Плотность высокого давления VHP (ppg, не используется)
    VhpDensityHp = 19, // 20 (unused)
    /// Высота трубы (ft)
    TubHeight = 20, // 21
    /// Дополнительное давление 1 (psi)
    ExtraPress1 = 21, // 22
    /// Дополнительное давление 2 (psi)
    ExtraPress2 = 22, // 23
    /// Дополнительный расход 1 (bpm)
    ExtraRate1 = 23, // 24
    /// Дополнительный суммарный расход 1 (bbl)
    ExtraTotal1 = 24, // 25
    /// Дополнительный расход 2 (bpm)
    ExtraRate2 = 25, // 26
    /// Дополнительный суммарный расход 2 (bbl)
    ExtraTotal2 = 26, // 27
    /// Соотношение вода/шлам (%)
    WaterSlurryRatio = 27, // 28
    /// Плотность в грязеуловителе (ppg)
    MudCupDensity = 28, // 29
    /// Суммарный расход насоса PS по стадиям (bbl)
    PsPumpStageTotal = 29, // 30
    /// Суммарный расход насоса DS по стадиям (bbl)
    DsPumpStageTotal = 30, // 31
    /// Объёмный расходомер (bpm)
    VolumetricMeter = 31, // 32
}

impl FlecsField {
    /// Общее количество полей в протоколе FLECS.
    pub const COUNT: usize = 32;

    /// Возвращает индекс поля в диапазоне 0..[`COUNT`](Self::COUNT).
    ///
    /// Индекс соответствует числовому значению варианта перечисления
    /// и используется для доступа к данным в массивах.
    ///
    /// # Возвращает
    ///
    /// Индекс поля как `usize`.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use crate::app::protocol::flecs::FlecsField;
    ///
    /// let idx = FlecsField::RecircDensity.idx();
    /// assert_eq!(idx, 0);
    /// ```
    #[inline]
    pub fn idx(self) -> usize {
        self as usize
    }
}

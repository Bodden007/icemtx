use super::{FieldMeta, FlecsField};

pub const GAP_AFTER_COMMA: usize = 7;

/// value_raw — то, что ты положил в frame.set(...):
/// "0", "12.3", "1.23e-4" и т.п.
/// Мы приводим к нужной точности, если это число.
pub fn format_field(field: FlecsField, value_raw: &str) -> String {
    let meta = field.meta();

    let value_str = format_value_by_precision(value_raw, meta.precision);

    let core = match meta.unit {
        Some(unit) => format!("{value_str} {unit}"),
        None => value_str,
    };

    // Выравниваем вправо по ширине (ведущие пробелы)
    // В спецификации разрешены пробелы для выравнивания [0..20] :contentReference[oaicite:0]{index=0}
    let mut out = format!("{core:>width$},", width = meta.width);

    // "CSV между значениями отступ 7 пробелов"
    out.push_str(&" ".repeat(GAP_AFTER_COMMA));
    out
}

fn format_value_by_precision(value_raw: &str, precision: u8) -> String {
    // Если это число — форматируем.
    // Если не число (пусто/мусор) — возвращаем как есть.
    if let Ok(v) = value_raw.trim().parse::<f64>() {
        match precision {
            0 => format!("{:.0}", v),
            1 => format!("{:.1}", v),
            2 => format!("{:.2}", v),
            3 => format!("{:.3}", v),
            _ => value_raw.to_string(),
        }
    } else {
        value_raw.to_string()
    }
}

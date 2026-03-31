use crate::app::protocol::flecs::FlecsField;
use crate::app::protocol::flecs::format::format_field;

pub struct FlecsFrame {
    values: [String; FlecsField::COUNT],
}

impl FlecsFrame {
    /// Все поля по умолчанию "0" (как требует iCem при недостающих значениях)
    pub fn new_zero() -> Self {
        Self {
            values: std::array::from_fn(|_| "0".to_string()),
        }
    }

    pub fn set(&mut self, field: FlecsField, value: impl Into<String>) {
        self.values[field.idx()] = value.into();
    }

    pub fn get(&self, field: FlecsField) -> &str {
        &self.values[field.idx()]
    }

    pub fn build_line(&self) -> String {
        let mut out = String::new();

        for idx in 0..FlecsField::COUNT {
            let field = unsafe {
                // безопасно, потому что enum жёстко 0..31
                std::mem::transmute::<u8, FlecsField>(idx as u8)
            };

            out.push_str(&format_field(field, self.get(field)));
        }

        out.push('\n');
        out
    }
}

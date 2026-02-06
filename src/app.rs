pub mod protocol;
use crate::app::protocol::flecs::format::format_field;
use crate::app::protocol::flecs::{FlecsField, FlecsFrame};

pub fn run() {
    let mut f = FlecsFrame::new_zero();
    f.set(FlecsField::RecircDensity, "12.34");
    f.set(FlecsField::MixWaterRate, "8");

    println!("{}", f.build_line());
}

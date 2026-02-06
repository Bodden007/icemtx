#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FlecsField {
    RecircDensity = 0,       // 1
    DownholeDensity = 1,     // 2
    MixWaterRate = 2,        // 3
    CombPumpRate = 3,        // 4
    PsPumpPress = 4,         // 5
    DsPumpPress = 5,         // 6
    WaterStageTotal = 6,     // 7
    MixWaterTotal = 7,       // 8
    PumpStageTotal = 8,      // 9
    CombinedPumpTotal = 9,   // 10
    CmntValvePosition = 10,  // 11
    WaterValvePosition = 11, // 12
    PsPumpRate = 12,         // 13
    DsPumpRate = 13,         // 14
    DigitalOuts = 14,        // 15
    HalWinEvent = 15,        // 16
    Temperature = 16,        // 17
    Rho12Hat = 17,           // 18 (unused)
    H2Hat = 18,              // 19 (unused)
    VhpDensityHp = 19,       // 20 (unused)
    TubHeight = 20,          // 21
    ExtraPress1 = 21,        // 22
    ExtraPress2 = 22,        // 23
    ExtraRate1 = 23,         // 24
    ExtraTotal1 = 24,        // 25
    ExtraRate2 = 25,         // 26
    ExtraTotal2 = 26,        // 27
    WaterSlurryRatio = 27,   // 28
    MudCupDensity = 28,      // 29
    PsPumpStageTotal = 29,   // 30
    DsPumpStageTotal = 30,   // 31
    VolumetricMeter = 31,    // 32
}

impl FlecsField {
    pub const COUNT: usize = 32;

    #[inline]
    pub fn idx(self) -> usize {
        self as usize
    }
}

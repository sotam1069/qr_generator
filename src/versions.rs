use crate::{encoder::ErrorCorrectionLevel, encoder::CapacityInfo};

#[derive(Debug)]
pub struct VersionInfo {
    pub size: u32,
    pub capacity_by_ec: [CapacityInfo; 4]
}

pub const VERSION_CAPACITIES: &[VersionInfo] = &[
    
    VersionInfo {
        size: 21,
        capacity_by_ec: [
            CapacityInfo { numeric: 41, alphanumeric: 25, byte: 17, kanji: 10 },  // L
            CapacityInfo { numeric: 34, alphanumeric: 20, byte: 14, kanji: 8 },   // M
            CapacityInfo { numeric: 27, alphanumeric: 16, byte: 11, kanji: 7 },   // Q
            CapacityInfo { numeric: 17, alphanumeric: 10, byte: 7,  kanji: 4 },   // H
        ],
    },
];
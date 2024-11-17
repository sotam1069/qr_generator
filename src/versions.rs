use crate::encoder::ErrorCorrectionLevel;

#[derive(Debug)]
pub struct VersionInfo {
    pub size: u32,
    pub capacity_by_ec: [CapacityInfo; 4],
}

#[derive(Debug)]
pub struct CapacityInfo {
    pub numeric: usize,
    pub alphanumeric: usize,
    pub byte: usize,
    pub kanji: usize,
}

pub const VERSION_CAPACITIES: &[VersionInfo] = &[
    // Version 1
    VersionInfo {
        size: 21,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 41,
                alphanumeric: 25,
                byte: 17,
                kanji: 10,
            }, // L
            CapacityInfo {
                numeric: 34,
                alphanumeric: 20,
                byte: 14,
                kanji: 8,
            }, // M
            CapacityInfo {
                numeric: 27,
                alphanumeric: 16,
                byte: 11,
                kanji: 7,
            }, // Q
            CapacityInfo {
                numeric: 17,
                alphanumeric: 10,
                byte: 7,
                kanji: 4,
            }, // H
        ],
    },
    // Version 2
    VersionInfo {
        size: 25,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 77,
                alphanumeric: 47,
                byte: 32,
                kanji: 20,
            }, // L
            CapacityInfo {
                numeric: 63,
                alphanumeric: 38,
                byte: 26,
                kanji: 16,
            }, // M
            CapacityInfo {
                numeric: 48,
                alphanumeric: 29,
                byte: 20,
                kanji: 12,
            }, // Q
            CapacityInfo {
                numeric: 34,
                alphanumeric: 20,
                byte: 14,
                kanji: 8,
            }, // H
        ],
    },
    // Version 3
    VersionInfo {
        size: 29,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 127,
                alphanumeric: 77,
                byte: 53,
                kanji: 32,
            }, // L
            CapacityInfo {
                numeric: 101,
                alphanumeric: 61,
                byte: 42,
                kanji: 26,
            }, // M
            CapacityInfo {
                numeric: 77,
                alphanumeric: 47,
                byte: 32,
                kanji: 20,
            }, // Q
            CapacityInfo {
                numeric: 58,
                alphanumeric: 35,
                byte: 24,
                kanji: 15,
            }, // H
        ],
    },
    // Version 4
    VersionInfo {
        size: 33,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 187,
                alphanumeric: 114,
                byte: 78,
                kanji: 48,
            }, // L
            CapacityInfo {
                numeric: 149,
                alphanumeric: 90,
                byte: 62,
                kanji: 38,
            }, // M
            CapacityInfo {
                numeric: 111,
                alphanumeric: 67,
                byte: 46,
                kanji: 28,
            }, // Q
            CapacityInfo {
                numeric: 82,
                alphanumeric: 50,
                byte: 34,
                kanji: 21,
            }, // H
        ],
    },
    // Version 5
    VersionInfo {
        size: 37,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 255,
                alphanumeric: 154,
                byte: 106,
                kanji: 65,
            }, // L
            CapacityInfo {
                numeric: 202,
                alphanumeric: 122,
                byte: 84,
                kanji: 52,
            }, // M
            CapacityInfo {
                numeric: 144,
                alphanumeric: 87,
                byte: 60,
                kanji: 37,
            }, // Q
            CapacityInfo {
                numeric: 106,
                alphanumeric: 64,
                byte: 44,
                kanji: 27,
            }, // H
        ],
    },
    // Version 6
    VersionInfo {
        size: 41,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 322,
                alphanumeric: 195,
                byte: 134,
                kanji: 82,
            }, // L
            CapacityInfo {
                numeric: 255,
                alphanumeric: 154,
                byte: 106,
                kanji: 65,
            }, // M
            CapacityInfo {
                numeric: 178,
                alphanumeric: 108,
                byte: 74,
                kanji: 45,
            }, // Q
            CapacityInfo {
                numeric: 139,
                alphanumeric: 84,
                byte: 58,
                kanji: 36,
            }, // H
        ],
    },
    // Version 7
    VersionInfo {
        size: 45,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 370,
                alphanumeric: 224,
                byte: 154,
                kanji: 95,
            }, // L
            CapacityInfo {
                numeric: 293,
                alphanumeric: 178,
                byte: 122,
                kanji: 75,
            }, // M
            CapacityInfo {
                numeric: 207,
                alphanumeric: 125,
                byte: 86,
                kanji: 53,
            }, // Q
            CapacityInfo {
                numeric: 154,
                alphanumeric: 93,
                byte: 64,
                kanji: 39,
            }, // H
        ],
    },
    // Version 8
    VersionInfo {
        size: 49,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 461,
                alphanumeric: 279,
                byte: 192,
                kanji: 118,
            }, // L
            CapacityInfo {
                numeric: 365,
                alphanumeric: 221,
                byte: 152,
                kanji: 93,
            }, // M
            CapacityInfo {
                numeric: 259,
                alphanumeric: 157,
                byte: 108,
                kanji: 66,
            }, // Q
            CapacityInfo {
                numeric: 202,
                alphanumeric: 122,
                byte: 84,
                kanji: 52,
            }, // H
        ],
    },
    // Version 9
    VersionInfo {
        size: 53,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 552,
                alphanumeric: 335,
                byte: 230,
                kanji: 141,
            }, // L
            CapacityInfo {
                numeric: 432,
                alphanumeric: 262,
                byte: 180,
                kanji: 111,
            }, // M
            CapacityInfo {
                numeric: 312,
                alphanumeric: 189,
                byte: 130,
                kanji: 80,
            }, // Q
            CapacityInfo {
                numeric: 235,
                alphanumeric: 143,
                byte: 98,
                kanji: 60,
            }, // H
        ],
    },
    // Version 10
    VersionInfo {
        size: 57,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 652,
                alphanumeric: 395,
                byte: 271,
                kanji: 167,
            }, // L
            CapacityInfo {
                numeric: 513,
                alphanumeric: 311,
                byte: 213,
                kanji: 131,
            }, // M
            CapacityInfo {
                numeric: 364,
                alphanumeric: 221,
                byte: 151,
                kanji: 93,
            }, // Q
            CapacityInfo {
                numeric: 288,
                alphanumeric: 174,
                byte: 119,
                kanji: 74,
            }, // H
        ],
    },
    // Version 11
    VersionInfo {
        size: 61,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 772,
                alphanumeric: 468,
                byte: 321,
                kanji: 198,
            }, // L
            CapacityInfo {
                numeric: 604,
                alphanumeric: 366,
                byte: 251,
                kanji: 155,
            }, // M
            CapacityInfo {
                numeric: 427,
                alphanumeric: 259,
                byte: 177,
                kanji: 109,
            }, // Q
            CapacityInfo {
                numeric: 331,
                alphanumeric: 200,
                byte: 137,
                kanji: 85,
            }, // H
        ],
    },
    // Version 12
    VersionInfo {
        size: 65,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 883,
                alphanumeric: 535,
                byte: 367,
                kanji: 226,
            }, // L
            CapacityInfo {
                numeric: 691,
                alphanumeric: 419,
                byte: 287,
                kanji: 177,
            }, // M
            CapacityInfo {
                numeric: 489,
                alphanumeric: 296,
                byte: 203,
                kanji: 125,
            }, // Q
            CapacityInfo {
                numeric: 374,
                alphanumeric: 227,
                byte: 155,
                kanji: 96,
            }, // H
        ],
    },
    // Version 13
    VersionInfo {
        size: 69,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 1022,
                alphanumeric: 619,
                byte: 425,
                kanji: 262,
            }, // L
            CapacityInfo {
                numeric: 796,
                alphanumeric: 483,
                byte: 331,
                kanji: 204,
            }, // M
            CapacityInfo {
                numeric: 580,
                alphanumeric: 352,
                byte: 241,
                kanji: 149,
            }, // Q
            CapacityInfo {
                numeric: 427,
                alphanumeric: 259,
                byte: 177,
                kanji: 109,
            }, // H
        ],
    },
    // Version 14
    VersionInfo {
        size: 73,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 1101,
                alphanumeric: 667,
                byte: 458,
                kanji: 282,
            }, // L
            CapacityInfo {
                numeric: 871,
                alphanumeric: 528,
                byte: 362,
                kanji: 223,
            }, // M
            CapacityInfo {
                numeric: 621,
                alphanumeric: 376,
                byte: 258,
                kanji: 159,
            }, // Q
            CapacityInfo {
                numeric: 468,
                alphanumeric: 283,
                byte: 194,
                kanji: 120,
            }, // H
        ],
    },
    // Version 15
    VersionInfo {
        size: 77,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 1250,
                alphanumeric: 758,
                byte: 520,
                kanji: 320,
            }, // L
            CapacityInfo {
                numeric: 991,
                alphanumeric: 600,
                byte: 412,
                kanji: 254,
            }, // M
            CapacityInfo {
                numeric: 703,
                alphanumeric: 426,
                byte: 292,
                kanji: 180,
            }, // Q
            CapacityInfo {
                numeric: 530,
                alphanumeric: 321,
                byte: 220,
                kanji: 136,
            }, // H
        ],
    },
    // Version 16
    VersionInfo {
        size: 81,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 1408,
                alphanumeric: 854,
                byte: 586,
                kanji: 361,
            }, // L
            CapacityInfo {
                numeric: 1082,
                alphanumeric: 656,
                byte: 450,
                kanji: 277,
            }, // M
            CapacityInfo {
                numeric: 775,
                alphanumeric: 470,
                byte: 322,
                kanji: 198,
            }, // Q
            CapacityInfo {
                numeric: 602,
                alphanumeric: 365,
                byte: 250,
                kanji: 154,
            }, // H
        ],
    },
    // Version 17
    VersionInfo {
        size: 85,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 1548,
                alphanumeric: 938,
                byte: 644,
                kanji: 397,
            }, // L
            CapacityInfo {
                numeric: 1212,
                alphanumeric: 734,
                byte: 504,
                kanji: 310,
            }, // M
            CapacityInfo {
                numeric: 876,
                alphanumeric: 531,
                byte: 364,
                kanji: 224,
            }, // Q
            CapacityInfo {
                numeric: 674,
                alphanumeric: 408,
                byte: 280,
                kanji: 173,
            }, // H
        ],
    },
    // Version 18
    VersionInfo {
        size: 89,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 1725,
                alphanumeric: 1046,
                byte: 718,
                kanji: 442,
            }, // L
            CapacityInfo {
                numeric: 1346,
                alphanumeric: 816,
                byte: 560,
                kanji: 345,
            }, // M
            CapacityInfo {
                numeric: 948,
                alphanumeric: 574,
                byte: 394,
                kanji: 243,
            }, // Q
            CapacityInfo {
                numeric: 746,
                alphanumeric: 452,
                byte: 310,
                kanji: 191,
            }, // H
        ],
    },
    // Version 19
    VersionInfo {
        size: 93,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 1903,
                alphanumeric: 1153,
                byte: 792,
                kanji: 488,
            }, // L
            CapacityInfo {
                numeric: 1500,
                alphanumeric: 909,
                byte: 624,
                kanji: 384,
            }, // M
            CapacityInfo {
                numeric: 1063,
                alphanumeric: 644,
                byte: 442,
                kanji: 272,
            }, // Q
            CapacityInfo {
                numeric: 813,
                alphanumeric: 493,
                byte: 338,
                kanji: 208,
            }, // H
        ],
    },
    // Version 20
    VersionInfo {
        size: 97,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 2061,
                alphanumeric: 1249,
                byte: 858,
                kanji: 528,
            }, // L
            CapacityInfo {
                numeric: 1600,
                alphanumeric: 970,
                byte: 666,
                kanji: 410,
            }, // M
            CapacityInfo {
                numeric: 1159,
                alphanumeric: 702,
                byte: 482,
                kanji: 297,
            }, // Q
            CapacityInfo {
                numeric: 919,
                alphanumeric: 557,
                byte: 382,
                kanji: 235,
            }, // H
        ],
    },
    // Version 21
    VersionInfo {
        size: 101,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 2232,
                alphanumeric: 1352,
                byte: 929,
                kanji: 572,
            }, // L
            CapacityInfo {
                numeric: 1708,
                alphanumeric: 1035,
                byte: 711,
                kanji: 438,
            }, // M
            CapacityInfo {
                numeric: 1224,
                alphanumeric: 742,
                byte: 509,
                kanji: 314,
            }, // Q
            CapacityInfo {
                numeric: 969,
                alphanumeric: 587,
                byte: 403,
                kanji: 248,
            }, // H
        ],
    },
    // Version 22
    VersionInfo {
        size: 105,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 2409,
                alphanumeric: 1460,
                byte: 1003,
                kanji: 618,
            }, // L
            CapacityInfo {
                numeric: 1872,
                alphanumeric: 1134,
                byte: 779,
                kanji: 480,
            }, // M
            CapacityInfo {
                numeric: 1358,
                alphanumeric: 823,
                byte: 565,
                kanji: 348,
            }, // Q
            CapacityInfo {
                numeric: 1056,
                alphanumeric: 640,
                byte: 439,
                kanji: 270,
            }, // H
        ],
    },
    // Version 23
    VersionInfo {
        size: 109,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 2620,
                alphanumeric: 1588,
                byte: 1091,
                kanji: 672,
            }, // L
            CapacityInfo {
                numeric: 2059,
                alphanumeric: 1248,
                byte: 857,
                kanji: 528,
            }, // M
            CapacityInfo {
                numeric: 1468,
                alphanumeric: 890,
                byte: 611,
                kanji: 376,
            }, // Q
            CapacityInfo {
                numeric: 1108,
                alphanumeric: 672,
                byte: 461,
                kanji: 284,
            }, // H
        ],
    },
    // Version 24
    VersionInfo {
        size: 113,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 2812,
                alphanumeric: 1704,
                byte: 1171,
                kanji: 721,
            }, // L
            CapacityInfo {
                numeric: 2188,
                alphanumeric: 1326,
                byte: 911,
                kanji: 561,
            }, // M
            CapacityInfo {
                numeric: 1588,
                alphanumeric: 963,
                byte: 661,
                kanji: 407,
            }, // Q
            CapacityInfo {
                numeric: 1228,
                alphanumeric: 744,
                byte: 511,
                kanji: 315,
            }, // H
        ],
    },
    // Version 25
    VersionInfo {
        size: 117,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 3057,
                alphanumeric: 1853,
                byte: 1273,
                kanji: 784,
            }, // L
            CapacityInfo {
                numeric: 2395,
                alphanumeric: 1451,
                byte: 997,
                kanji: 614,
            }, // M
            CapacityInfo {
                numeric: 1718,
                alphanumeric: 1041,
                byte: 715,
                kanji: 440,
            }, // Q
            CapacityInfo {
                numeric: 1286,
                alphanumeric: 779,
                byte: 535,
                kanji: 330,
            }, // H
        ],
    },
    // Version 26
    VersionInfo {
        size: 121,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 3283,
                alphanumeric: 1990,
                byte: 1367,
                kanji: 842,
            }, // L
            CapacityInfo {
                numeric: 2544,
                alphanumeric: 1542,
                byte: 1059,
                kanji: 652,
            }, // M
            CapacityInfo {
                numeric: 1804,
                alphanumeric: 1094,
                byte: 751,
                kanji: 462,
            }, // Q
            CapacityInfo {
                numeric: 1425,
                alphanumeric: 864,
                byte: 593,
                kanji: 365,
            }, // H
        ],
    },
    // Version 27
    VersionInfo {
        size: 125,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 3517,
                alphanumeric: 2132,
                byte: 1465,
                kanji: 902,
            }, // L
            CapacityInfo {
                numeric: 2701,
                alphanumeric: 1637,
                byte: 1125,
                kanji: 692,
            }, // M
            CapacityInfo {
                numeric: 1933,
                alphanumeric: 1172,
                byte: 805,
                kanji: 496,
            }, // Q
            CapacityInfo {
                numeric: 1501,
                alphanumeric: 910,
                byte: 625,
                kanji: 385,
            }, // H
        ],
    },
    // Version 28
    VersionInfo {
        size: 129,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 3669,
                alphanumeric: 2223,
                byte: 1528,
                kanji: 940,
            }, // L
            CapacityInfo {
                numeric: 2857,
                alphanumeric: 1732,
                byte: 1190,
                kanji: 732,
            }, // M
            CapacityInfo {
                numeric: 2085,
                alphanumeric: 1263,
                byte: 868,
                kanji: 534,
            }, // Q
            CapacityInfo {
                numeric: 1581,
                alphanumeric: 958,
                byte: 658,
                kanji: 405,
            }, // H
        ],
    },
    // Version 29
    VersionInfo {
        size: 133,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 3909,
                alphanumeric: 2369,
                byte: 1628,
                kanji: 1002,
            }, // L
            CapacityInfo {
                numeric: 3035,
                alphanumeric: 1839,
                byte: 1264,
                kanji: 778,
            }, // M
            CapacityInfo {
                numeric: 2181,
                alphanumeric: 1322,
                byte: 908,
                kanji: 559,
            }, // Q
            CapacityInfo {
                numeric: 1677,
                alphanumeric: 1016,
                byte: 698,
                kanji: 430,
            }, // H
        ],
    },
    // Version 30
    VersionInfo {
        size: 137,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 4158,
                alphanumeric: 2520,
                byte: 1732,
                kanji: 1066,
            }, // L
            CapacityInfo {
                numeric: 3289,
                alphanumeric: 1994,
                byte: 1370,
                kanji: 843,
            }, // M
            CapacityInfo {
                numeric: 2358,
                alphanumeric: 1429,
                byte: 982,
                kanji: 604,
            }, // Q
            CapacityInfo {
                numeric: 1782,
                alphanumeric: 1080,
                byte: 742,
                kanji: 457,
            }, // H
        ],
    },
    // Version 31
    VersionInfo {
        size: 141,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 4417,
                alphanumeric: 2677,
                byte: 1840,
                kanji: 1132,
            }, // L
            CapacityInfo {
                numeric: 3486,
                alphanumeric: 2113,
                byte: 1452,
                kanji: 894,
            }, // M
            CapacityInfo {
                numeric: 2473,
                alphanumeric: 1499,
                byte: 1030,
                kanji: 634,
            }, // Q
            CapacityInfo {
                numeric: 1897,
                alphanumeric: 1150,
                byte: 790,
                kanji: 486,
            }, // H
        ],
    },
    // Version 32
    VersionInfo {
        size: 145,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 4686,
                alphanumeric: 2840,
                byte: 1952,
                kanji: 1201,
            }, // L
            CapacityInfo {
                numeric: 3693,
                alphanumeric: 2238,
                byte: 1538,
                kanji: 947,
            }, // M
            CapacityInfo {
                numeric: 2670,
                alphanumeric: 1618,
                byte: 1112,
                kanji: 684,
            }, // Q
            CapacityInfo {
                numeric: 2022,
                alphanumeric: 1226,
                byte: 842,
                kanji: 518,
            }, // H
        ],
    },
    // Version 33
    VersionInfo {
        size: 149,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 4965,
                alphanumeric: 3009,
                byte: 2068,
                kanji: 1273,
            }, // L
            CapacityInfo {
                numeric: 3909,
                alphanumeric: 2369,
                byte: 1628,
                kanji: 1002,
            }, // M
            CapacityInfo {
                numeric: 2805,
                alphanumeric: 1700,
                byte: 1168,
                kanji: 719,
            }, // Q
            CapacityInfo {
                numeric: 2157,
                alphanumeric: 1307,
                byte: 898,
                kanji: 553,
            }, // H
        ],
    },
    // Version 34
    VersionInfo {
        size: 153,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 5253,
                alphanumeric: 3183,
                byte: 2188,
                kanji: 1347,
            }, // L
            CapacityInfo {
                numeric: 4134,
                alphanumeric: 2506,
                byte: 1722,
                kanji: 1060,
            }, // M
            CapacityInfo {
                numeric: 2949,
                alphanumeric: 1787,
                byte: 1228,
                kanji: 756,
            }, // Q
            CapacityInfo {
                numeric: 2301,
                alphanumeric: 1394,
                byte: 958,
                kanji: 590,
            }, // H
        ],
    },
    // Version 35
    VersionInfo {
        size: 157,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 5529,
                alphanumeric: 3351,
                byte: 2303,
                kanji: 1417,
            }, // L
            CapacityInfo {
                numeric: 4343,
                alphanumeric: 2632,
                byte: 1809,
                kanji: 1113,
            }, // M
            CapacityInfo {
                numeric: 3081,
                alphanumeric: 1867,
                byte: 1283,
                kanji: 790,
            }, // Q
            CapacityInfo {
                numeric: 2361,
                alphanumeric: 1431,
                byte: 983,
                kanji: 605,
            }, // H
        ],
    },
    // Version 36
    VersionInfo {
        size: 161,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 5836,
                alphanumeric: 3537,
                byte: 2431,
                kanji: 1496,
            }, // L
            CapacityInfo {
                numeric: 4588,
                alphanumeric: 2780,
                byte: 1911,
                kanji: 1176,
            }, // M
            CapacityInfo {
                numeric: 3244,
                alphanumeric: 1966,
                byte: 1351,
                kanji: 832,
            }, // Q
            CapacityInfo {
                numeric: 2524,
                alphanumeric: 1530,
                byte: 1051,
                kanji: 647,
            }, // H
        ],
    },
    // Version 37
    VersionInfo {
        size: 165,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 6153,
                alphanumeric: 3729,
                byte: 2563,
                kanji: 1577,
            }, // L
            CapacityInfo {
                numeric: 4775,
                alphanumeric: 2894,
                byte: 1989,
                kanji: 1224,
            }, // M
            CapacityInfo {
                numeric: 3417,
                alphanumeric: 2071,
                byte: 1423,
                kanji: 876,
            }, // Q
            CapacityInfo {
                numeric: 2625,
                alphanumeric: 1591,
                byte: 1093,
                kanji: 673,
            }, // H
        ],
    },
    // Version 38
    VersionInfo {
        size: 169,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 6479,
                alphanumeric: 3927,
                byte: 2699,
                kanji: 1661,
            }, // L
            CapacityInfo {
                numeric: 5039,
                alphanumeric: 3054,
                byte: 2099,
                kanji: 1292,
            }, // M
            CapacityInfo {
                numeric: 3599,
                alphanumeric: 2181,
                byte: 1499,
                kanji: 923,
            }, // Q
            CapacityInfo {
                numeric: 2735,
                alphanumeric: 1658,
                byte: 1139,
                kanji: 701,
            }, // H
        ],
    },
    // Version 39
    VersionInfo {
        size: 173,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 6743,
                alphanumeric: 4087,
                byte: 2809,
                kanji: 1729,
            }, // L
            CapacityInfo {
                numeric: 5313,
                alphanumeric: 3220,
                byte: 2213,
                kanji: 1362,
            }, // M
            CapacityInfo {
                numeric: 3791,
                alphanumeric: 2298,
                byte: 1579,
                kanji: 972,
            }, // Q
            CapacityInfo {
                numeric: 2927,
                alphanumeric: 1774,
                byte: 1219,
                kanji: 750,
            }, // H
        ],
    },
    // Version 40
    VersionInfo {
        size: 177,
        capacity_by_ec: [
            CapacityInfo {
                numeric: 7089,
                alphanumeric: 4296,
                byte: 2953,
                kanji: 1817,
            }, // L
            CapacityInfo {
                numeric: 5596,
                alphanumeric: 3391,
                byte: 2331,
                kanji: 1435,
            }, // M
            CapacityInfo {
                numeric: 3993,
                alphanumeric: 2420,
                byte: 1663,
                kanji: 1024,
            }, // Q
            CapacityInfo {
                numeric: 3057,
                alphanumeric: 1852,
                byte: 1273,
                kanji: 784,
            }, // H
        ],
    },
];

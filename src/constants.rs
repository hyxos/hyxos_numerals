pub mod constants {
    pub const DIACRITIC_CHARS: [char; 5] = ['t', 's', 'r', 'j', 'w'];
    pub const DIACRITIC_NAME: [&str; 5] = ["ta", "shey", "ree", "jo", "wu"];
    pub const DUODECIMALS: [char; 12] =
        ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b'];
    pub const DUODECIMAL_NAME: [&str; 12] = [
        "zo", "zee", "bey", "tree", "kat", "pen", "hyx", "sep", "awk", "neyn", "dek", "lev",
    ];
    pub const COLORS: [&str; 5] = ["yellow", "blue", "red", "purple", "green"];
    pub const PLANETS: [&str; 5] = ["Saturn", "Mercury", "Mars", "Venus", "Jupiter"];
    pub const ELEMENTS: [&str; 5] = ["earth", "water", "fire", "metal", "wood"];
    pub const GENERATING_INDECES: [u8; 5] = [2, 4, 1, 3, 0];
    pub const ANIMALS: [&str; 12] = [
        "pig", "rat", "ox", "tiger", "rabbit", "dragon", "snake", "horse", "sheep", "monkey",
        "rooster", "dog",
    ];
    pub const WESTERN_SIGNS: [&str; 12] = [
        "pisces",
        "aries",
        "taurus",
        "gemini",
        "cancer",
        "leo",
        "virgo",
        "libra",
        "scorpio",
        "sagitarius",
        "capricorn",
        "aquarius",
    ];
    pub const ELEMENTS_CN: [&str; 5] = ["木", "火", "土", "金", "水"];
    pub const EARTHLY_BRANCHES_CN: [&str; 12] = [
        "亥", "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌",
    ];
    pub const HEAVENLY_STEMS_CN: [&str; 10] =
        ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
    pub const ANIMALS_CN: [&str; 12] = [
        "猪", "鼠", "牛", "虎", "兔", "龍", "蛇", "馬", "羊", "猴", "雞", "狗",
    ];
    pub const POLARITY: [&str; 2] = ["yin", "yang"];
    pub const POLARITY_CN: [&str; 2] = ["陰", "陽"];
    pub const POLARITY_LUMINARIES: [&str; 2] = ["Moon", "Sun"];
    
    // Suffix tier system for powers of 60
    pub const SUFFIX_NAMES: [&str; 13] = [
        "ma", "fe", "gi", "tho", "vu", "ya", "fre", "gli", "dra", "ske", "pri", "kro", "zru"
    ];
    pub const SUFFIX_POWERS: [u128; 13] = [
        60,                 // 60^1
        3_600,              // 60^2
        216_000,            // 60^3
        12_960_000,         // 60^4
        777_600_000,        // 60^5
        46_656_000_000,     // 60^6
        2_799_360_000_000,  // 60^7
        167_961_600_000_000, // 60^8
        10_077_696_000_000_000, // 60^9
        604_661_760_000_000_000, // 60^10
        36_279_705_600_000_000_000, // 60^11
        2_176_782_336_000_000_000_000, // 60^12
        130_606_940_160_000_000_000_000, // 60^13
    ];
    
    // Grammatical suffixes
    pub const GRAMMATICAL_SUFFIXES: [&str; 11] = [
        "as", "es", "is", "os", "us", "ema", "ek", "an", "at", "ar", "al"
    ];
    
    // Fractional suffix rules
    pub const FRACTIONAL_TOS: [&str; 12] = [
        "zotos", "zeetos", "beytos", "treetos", "katos", "pentos", 
        "hyxos", "septos", "awktos", "neynos", "dekos", "levos"
    ];
}

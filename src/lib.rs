#[derive(Debug, Copy, Clone)]
pub struct Numeral(u8);

mod constants;

use constants::constants::*;

impl Numeral {
    pub fn new(i: u8) -> Numeral {
        if i > 59 {
            panic!("Can only create numerals from values 0 through 59!")
        } else {
            Numeral(i)
        }
    }
    pub fn new_from_enc(n: &str) -> Numeral {
        let mut di: Option<usize> = Some(0);
        let mut dd: Option<usize> = Some(0);
        for (i, c) in n.chars().enumerate() {
            match i {
                0 => di = DIACRITIC_CHARS.iter().position(|&t| t == c),
                1 => dd = DUODECIMALS.iter().position(|&u| u == c),
                _ => panic!("Invalid encoding!"),
            }
        }
        fn check_encoding(o: Option<usize>) -> usize {
            match o {
                Some(o) => o,
                None => panic!("Invalid encoding!"),
            }
        }
        Numeral(((check_encoding(di) * 12) + check_encoding(dd)) as u8)
    }
    pub fn i(&self) -> u8 {
        self.0
    }
    pub fn diacritic_index(&self) -> u8 {
        self.0 / 12
    }
    pub fn duodecimal_index(&self) -> u8 {
        self.0 % 12
    }
    pub fn diacritic_char(&self) -> char {
        DIACRITIC_CHARS[self.diacritic_index() as usize]
    }
    pub fn diacritic_name(&self) -> &str {
        DIACRITIC_NAME[self.diacritic_index() as usize]
    }
    pub fn duodecimal_char(&self) -> char {
        DUODECIMALS[self.duodecimal_index() as usize]
    }
    pub fn duodecimal_name(&self) -> &str {
        DUODECIMAL_NAME[self.duodecimal_index() as usize]
    }
    pub fn sexagesimal_name(&self) -> String {
        self.diacritic_name().to_owned() + self.duodecimal_name()
    }
    pub fn encoding(&self) -> String {
        self.diacritic_char().to_string() + &self.duodecimal_char().to_string()
    }
    pub fn row_index(&self) -> u8 {
        (self.0 * 3) % 4
    }
    pub fn col_index(&self) -> u8 {
        (self.0 * 2) % 3
    }
    pub fn polarity_index(&self) -> u8 {
        self.0 % 2
    }
    pub fn polarity(&self) -> &str {
        POLARITY[self.polarity_index() as usize]
    }
    pub fn polarity_cn(&self) -> &str {
        POLARITY_CN[self.polarity_index() as usize]
    }
    pub fn polarity_lum(&self) -> &str {
        POLARITY_LUMINARIES[self.polarity_index() as usize]
    }
    pub fn color(&self) -> &str {
        COLORS[self.diacritic_index() as usize]
    }
    pub fn planet(&self) -> &str {
        PLANETS[self.diacritic_index() as usize]
    }
    pub fn element(&self) -> &str {
        ELEMENTS[self.diacritic_index() as usize]
    }
    pub fn animal(&self) -> &str {
        ANIMALS[self.duodecimal_index() as usize]
    }
    pub fn nickname(&self) -> String {
        self.element().to_string() + &" ".to_string() + &self.animal()
    }
    pub fn earthly_branch(&self) -> &str {
        EARTHLY_BRANCHES_CN[self.duodecimal_index() as usize]
    }
    pub fn generating_index(&self) -> u8 {
        GENERATING_INDECES[self.diacritic_index() as usize]
    }
    pub fn heavenly_stem_index(&self) -> u8 {
        self.generating_index() * 2 + (if self.polarity_index() == 0 { 1 } else { 0 })
    }
    pub fn heavenly_stem(&self) -> &str {
        HEAVENLY_STEMS_CN[self.heavenly_stem_index() as usize]
    }
    pub fn element_cn(&self) -> &str {
        ELEMENTS_CN[self.generating_index() as usize]
    }
    pub fn animal_cn(&self) -> &str {
        ANIMALS_CN[self.duodecimal_index() as usize]
    }
    pub fn nickname_cn(&self) -> String {
        self.element_cn().to_string() + &self.animal_cn()
    }
    pub fn ganzhi(&self) -> String {
        self.heavenly_stem().to_string() + &self.earthly_branch()
    }
    pub fn western_sign(&self) -> &str {
        WESTERN_SIGNS[self.duodecimal_index() as usize]
    }
    pub fn zee_index(&self) -> u8 {
        if self.duodecimal_index() > 0 {
            self.duodecimal_index() - 1
        } else {
            11
        }
    }
    pub fn hex_index(&self) -> u8 {
        self.zee_index() / 2
    }
    pub fn cohort_index(&self) -> u8 {
        if self.generating_index() >= self.hex_index() {
            self.generating_index() - self.hex_index()
        } else {
            (self.generating_index() + 5) - self.hex_index()
        }
    }
    pub fn natural_order_index(&self) -> u8 {
        self.cohort_index() * 12 + self.zee_index()
    }
    pub fn description(&self) -> String {
        format!(
            "
        
Integer: {}
Name: {}
Encoding: {}
Sexagenary Rank: {}
Ganzhi: {}
Nickname: {}
Nickname (CN): {}

",
            self.i(),
            self.sexagesimal_name(),
            self.encoding(),
            self.natural_order_index() + 1,
            self.ganzhi(),
            self.nickname(),
            self.nickname_cn()
        )
    }
}

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone)]
pub struct Set(Vec<Numeral>);

impl Set {
    pub fn new() -> Set {
        Set((0..60).into_iter().map(|i| Numeral::new(i)).collect())
    }
    pub fn values(&self) -> Vec<Numeral> {
        self.clone().0
    }
    pub fn shuffle(&self) -> Set {
        let mut set: Vec<Numeral> = self.values();
        set.shuffle(&mut thread_rng());
        Set(set)
    }
    pub fn nat_sort(&self) -> Set {
        let mut set: Vec<Numeral> = self.values();
        set.sort_by(|a, b| a.natural_order_index().cmp(&b.natural_order_index()));
        Set(set)
    }
}

#[derive(Debug, Clone)]
pub struct Uint(Vec<Numeral>);

fn powsof60(a: Vec<usize>) -> Vec<usize> {
    let v: usize = 59;
    let mut c = a;
    while &c[0] > &v {
        let d = c[0] / 60;
        let e = c[0] - d * 60;
        c.remove(0);
        c.insert(0, d);
        c.insert(1, e);
        powsof60(c.clone());
    }
    c
}

pub fn u(v: Vec<Numeral>) -> usize {
    let mut sum: u32 = 0;
    let base: u32 = 60;
    for (i, c) in v.iter().rev().enumerate() {
        sum += base.pow(i as u32) * (c.i() as u32);
    }
    sum as usize
}

impl Uint {
    pub fn new(u: usize) -> Uint {
        let v = powsof60(vec![u]);
        let g = v.iter().map(|a| Numeral::new(*a as u8)).collect();
        Uint(g)
    }
    pub fn new_from_numerals(v: Vec<Numeral>) -> Uint {
        Uint::new(u(v))
    }
    pub fn values(&self) -> Vec<Numeral> {
        self.clone().0
    }
    pub fn u(&self) -> usize {
        u(self.values())
    }
}

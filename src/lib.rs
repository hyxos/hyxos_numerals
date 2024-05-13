#[derive(Debug)]
pub struct Numeral(u8);

mod constants;
use constants::constants::*;

impl Numeral {
    pub fn new(i: u8) -> Numeral {
        if i > 59 {
            panic!("Can only create numerals from values 0 through 59!")
        }
        else {
            Numeral(i)
        }
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
    pub fn trine_index(&self) -> u8 {
        (self.0 * 3) % 4
    }
    pub fn square_index(&self) -> u8 {
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
        POLARITY_LUMINARIES[self.polarity_index()as usize]
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
    pub fn earthly_branch(&self) -> &str {
        EARTHLY_BRANCHES_CN[self.duodecimal_index() as usize]
    }
    pub fn generating_index(&self) -> u8 {
        GENERATING_INDECES[self.diacritic_index() as usize]
    }
    pub fn natural_order(&self) -> u8 {
      if self.duodecimal_index() > 1 {
        self.duodecimal_index() - 1
      }
      else { 11 }
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
    pub fn western_sign(&self) -> &str {
        WESTERN_SIGNS[self.duodecimal_index() as usize]
    }
    pub fn zee_index(&self) -> u8 {
        if self.duodecimal_index() > 0 {
          self.duodecimal_index() - 1
        }
        else { 11 }
    }
    pub fn hex_index(&self) -> u8 {
        self.zee_index() / 2
    }
    pub fn cohort_index(&self) -> u8 {
      if self.generating_index() >= self.hex_index() {
        self.generating_index() - self.hex_index()
      }
      else {
        (self.generating_index() + 5) - self.hex_index()
      }
    }
    pub fn natural_order_index(&self) -> u8 {
      self.cohort_index() * 12 + self.zee_index()
    }

}

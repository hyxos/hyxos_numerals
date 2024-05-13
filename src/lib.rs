
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
      pub fn polarity_chin(&self) -> &str {
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
      pub fn element_chin(&self) -> &str {
          ELEMENTS_CN[self.generating_index() as usize]
      }
      pub fn animal_chin(&self) -> &str {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
      let tazo = Numeral::new(0);
      assert_eq!(tazo.diacritic_index(), 0);
      assert_eq!(tazo.duodecimal_index(), 0);
      assert_eq!(tazo.diacritic_char(), 't');
      assert_eq!(tazo.duodecimal_char(), '0');
      assert_eq!(tazo.encoding(), "t0");
      assert_eq!(tazo.trine_index(), 0);
      assert_eq!(tazo.square_index(), 0);
      assert_eq!(tazo.polarity_index(), 0);
      assert_eq!(tazo.color(), "yellow");
      assert_eq!(tazo.planet(), "Saturn");
      assert_eq!(tazo.element(), "earth");
      assert_eq!(tazo.animal(), "pig");
      assert_eq!(tazo.earthly_branch(), "亥");
      assert_eq!(tazo.generating_index(), 2);
      assert_eq!(tazo.natural_order(), 11);
      assert_eq!(tazo.heavenly_stem_index(), 5);
      assert_eq!(tazo.heavenly_stem(), "己");
      assert_eq!(tazo.element_chin(), "土");
      assert_eq!(tazo.animal_chin(), "猪");
      assert_eq!(tazo.diacritic_name(), "ta");
      assert_eq!(tazo.duodecimal_name(), "zo");
      assert_eq!(tazo.sexagesimal_name(), "tazo");
      assert_eq!(tazo.polarity(), "yin");
      assert_eq!(tazo.polarity_chin(), "陰");
      assert_eq!(tazo.polarity_lum(), "Moon");
      assert_eq!(tazo.western_sign(), "pisces");
      assert_eq!(tazo.hex_index(), 5);
      assert_eq!(tazo.zee_index(), 11);
      assert_eq!(tazo.cohort_index(), 2);
      assert_eq!(tazo.natural_order_index(), 35);

      let shenen = Numeral::new(21);
      assert_eq!(shenen.diacritic_index(), 1);
      assert_eq!(shenen.duodecimal_index(), 9);
      assert_eq!(shenen.diacritic_char(), 's');
      assert_eq!(shenen.duodecimal_char(), '9');
      assert_eq!(shenen.encoding(), "s9");
      assert_eq!(shenen.trine_index(), 3);
      assert_eq!(shenen.square_index(), 0);
      assert_eq!(shenen.polarity_index(), 1);
      assert_eq!(shenen.color(), "blue");
      assert_eq!(shenen.element(), "water");
      assert_eq!(shenen.animal(), "monkey");
      assert_eq!(shenen.planet(), "Mercury");
      assert_eq!(shenen.earthly_branch(), "申");
      assert_eq!(shenen.generating_index(), 4);
      assert_eq!(shenen.natural_order(), 8);
      assert_eq!(shenen.heavenly_stem_index(), 8);
      assert_eq!(shenen.heavenly_stem(), "壬");
      assert_eq!(shenen.hex_index(), 4);
      assert_eq!(shenen.zee_index(),8);
      assert_eq!(shenen.cohort_index(), 0);
      assert_eq!(shenen.natural_order_index(), 8);
      
      let wulev = Numeral::new(59);
      assert_eq!(wulev.diacritic_index(), 4);
      assert_eq!(wulev.duodecimal_index(), 11);
      assert_eq!(wulev.diacritic_char(), 'w');
      assert_eq!(wulev.duodecimal_char(), 'B');
      assert_eq!(wulev.encoding(), "wB");
      assert_eq!(wulev.trine_index(), 1);
      assert_eq!(wulev.square_index(), 1);
      assert_eq!(wulev.polarity_index(), 1);
      assert_eq!(wulev.color(), "green");
      assert_eq!(wulev.planet(), "Jupiter");
      assert_eq!(wulev.element(), "wood");
      assert_eq!(wulev.animal(), "dog");
      assert_eq!(wulev.earthly_branch(), "戌");
      assert_eq!(wulev.generating_index(), 0);
      assert_eq!(wulev.natural_order(), 10);
      assert_eq!(wulev.heavenly_stem_index(), 0);
      assert_eq!(wulev.heavenly_stem(), "甲");
      assert_eq!(wulev.hex_index(), 5 );
      assert_eq!(wulev.zee_index(), 10);
      assert_eq!(wulev.cohort_index(), 0);
      assert_eq!(wulev.natural_order_index(), 10);

      let shebey = Numeral::new(14);
      println!("{}", shebey.sexagesimal_name());
      println!("{} {} {}", shebey.duodecimal_index(), shebey.generating_index(), shebey.hex_index() % 5);
      println!("{} {} {}", shebey.cohort_index(), shebey.zee_index(), shebey.natural_order_index());
      println!("{}", shebey.natural_order_index());

      let shetree = Numeral::new(27);
      println!("{}", shetree.sexagesimal_name());
      println!("{} {} {}", shetree.duodecimal_index(), shetree.generating_index(), shetree.hex_index() % 5);
      println!("{} {} {}", shetree.cohort_index(), shetree.zee_index(), shetree.natural_order_index());
      println!("{}", shetree.natural_order_index());

      let jox = Numeral::new(42);
      println!("{}", jox.natural_order_index());
      }
}

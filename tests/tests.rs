#[cfg(test)]
mod tests {
    use hyxos_numerals::Numeral;

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
      assert_eq!(tazo.element_cn(), "土");
      assert_eq!(tazo.animal_cn(), "猪");
      assert_eq!(tazo.diacritic_name(), "ta");
      assert_eq!(tazo.duodecimal_name(), "zo");
      assert_eq!(tazo.sexagesimal_name(), "tazo");
      assert_eq!(tazo.polarity(), "yin");
      assert_eq!(tazo.polarity_cn(), "陰");
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
    }
}
#[cfg(test)]
mod grammar_tests {
    use hyxos_numerals::*;

    #[test]
    fn test_base_glyph_names() {
        // Test all base glyphs (0-11)
        let names = vec![
            "zo", "zee", "bey", "tree", "kat", "pen", 
            "hyx", "sep", "awk", "neyn", "dek", "lev"
        ];
        
        for (i, expected) in names.iter().enumerate() {
            let n = Numeral::new(i as u8);
            assert_eq!(n.spoken_name(), *expected);
            assert_eq!(n.duodecimal_name(), *expected);
        }
    }

    #[test]
    fn test_diacritic_names() {
        // Test diacritic names match our grammar
        assert_eq!(Numeral::new(12).spoken_name(), "shey");
        assert_eq!(Numeral::new(24).spoken_name(), "ree");
        assert_eq!(Numeral::new(36).spoken_name(), "jo");
        assert_eq!(Numeral::new(48).spoken_name(), "wu");
    }

    #[test]
    fn test_contractions() {
        // Test hyx contractions
        assert_eq!(Numeral::new(18).spoken_name(), "shex");  // she + hyx
        assert_eq!(Numeral::new(30).spoken_name(), "reex");  // ree + hyx
        assert_eq!(Numeral::new(42).spoken_name(), "jox");   // jo + hyx
        assert_eq!(Numeral::new(54).spoken_name(), "wux");   // wu + hyx
        
        // Test awk contractions
        assert_eq!(Numeral::new(20).spoken_name(), "shek");  // she + awk
        assert_eq!(Numeral::new(32).spoken_name(), "reek");  // ree + awk
        assert_eq!(Numeral::new(44).spoken_name(), "jok");   // jo + awk
        assert_eq!(Numeral::new(56).spoken_name(), "wuk");   // wu + awk
    }

    #[test]
    fn test_encoding() {
        // Test two-character encoding
        assert_eq!(Numeral::new(0).encoding(), "t0");   // ta + zo
        assert_eq!(Numeral::new(12).encoding(), "s0");  // she + zo
        assert_eq!(Numeral::new(15).encoding(), "s3");  // she + tree
        assert_eq!(Numeral::new(32).encoding(), "r8");  // ree + awk
        assert_eq!(Numeral::new(59).encoding(), "wb");  // wu + lev
    }

    #[test]
    fn test_fractional_names() {
        // Test fractional forms
        assert_eq!(Numeral::new(0).fractional_name(), "zotos");
        assert_eq!(Numeral::new(1).fractional_name(), "zeetos");
        assert_eq!(Numeral::new(2).fractional_name(), "beytos");
        assert_eq!(Numeral::new(3).fractional_name(), "treetos");
        assert_eq!(Numeral::new(4).fractional_name(), "katos");
        assert_eq!(Numeral::new(5).fractional_name(), "pentos");
        assert_eq!(Numeral::new(6).fractional_name(), "hyxos");
        assert_eq!(Numeral::new(7).fractional_name(), "septos");
        assert_eq!(Numeral::new(8).fractional_name(), "awktos");
        assert_eq!(Numeral::new(9).fractional_name(), "neynos");
        assert_eq!(Numeral::new(10).fractional_name(), "dekos");
        assert_eq!(Numeral::new(11).fractional_name(), "levos");
    }

    #[test]
    fn test_from_encoding() {
        // Test parsing from encoding
        assert_eq!(Numeral::new_from_enc("t0").u(), 0);
        assert_eq!(Numeral::new_from_enc("s0").u(), 12);
        assert_eq!(Numeral::new_from_enc("s3").u(), 15);
        assert_eq!(Numeral::new_from_enc("r8").u(), 32);
        assert_eq!(Numeral::new_from_enc("wb").u(), 59);
    }

    #[test]
    fn test_tier_offset_calculation() {
        // Test [tier, offset] breakdown
        let n = Numeral::new(15); // shetree = [1, 3]
        assert_eq!(n.diacritic_index(), 1);  // tier
        assert_eq!(n.duodecimal_index(), 3); // offset
        
        let n = Numeral::new(32); // reek = [2, 8]
        assert_eq!(n.diacritic_index(), 2);
        assert_eq!(n.duodecimal_index(), 8);
    }

    #[test]
    fn test_spoken_names_0_to_60() {
        // Test critical transitions
        let expected = vec![
            (0, "zo"), (11, "lev"), (12, "shey"), (13, "shezee"),
            (18, "shex"), (20, "shek"), (24, "ree"), (30, "reex"),
            (32, "reek"), (36, "jo"), (42, "jox"), (44, "jok"),
            (48, "wu"), (54, "wux"), (56, "wuk"), (59, "wulev")
        ];
        
        for (num, name) in expected {
            assert_eq!(Numeral::new(num).spoken_name(), name);
        }
    }
}
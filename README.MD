# Hyxos Numerals

A Rust library for working with the Hyxos numeral system - a base-60 (sexagesimal) counting system with linguistic elegance and mathematical precision.

📖 **[Read the complete Grammar Guide](./GRAMMAR.md)** - The authoritative reference for the Hyxos numeral system

## Overview

The Hyxos system is designed as a pure numeral system that can be embedded within any existing language, similar to how terms like "dozen" or "score" function in English. It features:

- **Base-60 structure** with 12 glyphs and 5 diacritics
- **Phonetic contractions** for smoother pronunciation
- **Suffix tiers** for large numbers (powers of 60)
- **Grammatical suffixes** for plurals, ordinals, fractions, etc.
- **Self-referential naming** - "hyxos" means 1/6 in the system itself

## Quick Start

```rust
use hyxos_numerals::Numeral;

// Create numerals
let six = Numeral::new(6);
assert_eq!(six.spoken_name(), "hyx");

let thirty = Numeral::new(30);
assert_eq!(thirty.spoken_name(), "reex"); // Contraction of ree + hyx

// Encoding
let fifteen = Numeral::new(15);
assert_eq!(fifteen.encoding(), "s3"); // she (tier 1) + tree (offset 3)

// Fractions
assert_eq!(six.fractional_name(), "hyxos"); // One-sixth
```

## Grammar Guide

### Terminology

- **Hyxamel**: A number expressed in the Hyxos system
- **Hyxadekimal/Maal**: The base-60 system itself
- **[tier, offset]**: Two-part representation where tier (0-4) × 12 + offset (0-11) = decimal

### Base Glyphs (0-11)

| Decimal | Glyph | Encoding |
|---------|-------|----------|
| 0       | zo    | 0        |
| 1       | zee   | 1        |
| 2       | bey   | 2        |
| 3       | tree  | 3        |
| 4       | kat   | 4        |
| 5       | pen   | 5        |
| 6       | hyx   | 6        |
| 7       | sep   | 7        |
| 8       | awk   | 8        |
| 9       | neyn  | 9        |
| 10      | dek   | a        |
| 11      | lev   | b        |

### Diacritics (12-59)

| Tier | Diacritic | Starts At | Encoding |
|------|-----------|-----------|----------|
| 0    | ta        | 0         | t        |
| 1    | she       | 12        | s        |
| 2    | ree       | 24        | r        |
| 3    | jo        | 36        | j        |
| 4    | wu        | 48        | w        |

Examples:
- 12 = **she** (not shezo)
- 15 = **shetree**
- 32 = **reek** (contraction)
- 59 = **wulev**

### Contractions

For smoother pronunciation, `hyx` and `awk` contract with diacritics:

| Diacritic | + hyx → | + awk → |
|-----------|---------|---------|
| she       | shex    | shek    |
| ree       | reex    | reek    |
| jo        | jox     | jok     |
| wu        | wux     | wuk     |

### Numbers ≥ 60

Numbers follow the pattern: **[multiplier].[suffix].[remainder]**

- 60 = **ma** (implicit ta.ma.zo)
- 120 = **beyma** (2×60)
- 150 = **beymareex** (2×60 + 30)
- 3600 = **fe** (60²)

### Fractions

Add **-os** or **-tos** suffix:

| Form    | Meaning | Decimal |
|---------|---------|---------|
| hyxos   | 1/6     | ~0.167  |
| treetos | 1/3     | ~0.333  |
| beytos  | 1/2     | 0.5     |

### Grammatical Suffixes

| Suffix | Function    | Example | Meaning        |
|--------|-------------|---------|----------------|
| -es    | plural      | hyxes   | sixes         |
| -is    | trio        | hyxis   | group of three |
| -us    | ordinal     | hyxus   | sixth         |
| -an    | multiplicative | hyxan | sixfold      |
| -al    | adjectival  | maal    | sexagesimal   |

## API Reference

### Creating Numerals

```rust
// From decimal (0-59)
let n = Numeral::new(15);

// From encoding
let n = Numeral::new_from_enc("s3"); // 15
```

### Properties

```rust
let n = Numeral::new(32); // reek

// Basic properties
n.u();                    // 32 (decimal value)
n.spoken_name();          // "reek" (with contractions)
n.encoding();             // "r8"

// Tier/offset breakdown
n.diacritic_index();      // 2 (tier)
n.duodecimal_index();     // 8 (offset)

// Names
n.diacritic_name();       // "ree"
n.duodecimal_name();      // "awk"
n.fractional_name();      // "awktos"

// Cultural associations
n.element();              // "fire"
n.animal();               // "horse"
n.color();                // "red"
```

## Contributing

This library implements the Hyxos grammar as documented. When making changes:

1. Ensure consistency with the grammar guide
2. Add tests for new features
3. Update documentation

## License

[License information here]
# Hyxos Numeral System Grammar Guide

## Overview

This document outlines the grammar rules of the **Hyxos numeral system**, enabling counting from `zo` (0) to `zru` (~60^13). It defines the base glyphs, diacritics, suffix tiers, and contraction rules used in spoken and symbolic representations. This guide is designed for use by humans or AI systems to understand, generate, and parse Hyxos symbolic numerals.

**Important**: The Hyxos system is designed as a pure numeral system that can be embedded within any existing language, similar to how terms like "dozen" or "score" function in English. This guide covers only numerical expressions - no verbs, sentence structure, or non-numeric vocabulary. Hyxos numbers can be used within English, Spanish, Mandarin, or any other language as a specialized counting system.

### Terminology
- **Hyxamel** (noun): A number expressed in the Hyxos numeral system, analogous to how "decimal" refers to base-10 numbers
- **Decimal**: Traditional base-10 representation (e.g., 4689)
- **Hyxadekimal** (adjective/noun): The base-60 numeral system itself, from hyxa (six) + dek (ten) + -imal, literally "six-ten-al" (Hyxos-native term using the system's own vocabulary)
- **Maal** (adjective/noun): Pure Hyxos term for base-60/sexagesimal, from ma (60) + -al suffix
- **Sexagesimal**: Latin-derived term for base-60 (traditional mathematical term)

---

## 1. Base Glyphs (0–11)

| Decimal | Glyph  | Encoding |
|---------|--------|----------|
| 0       | zo     | 0        |
| 1       | zee    | 1        |
| 2       | bey    | 2        |
| 3       | tree   | 3        |
| 4       | kat    | 4        |
| 5       | pen    | 5        |
| 6       | hyx    | 6        |
| 7       | sep    | 7        |
| 8       | awk    | 8        |
| 9       | neyn   | 9        |
| 10      | dek    | a        |
| 11      | lev    | b        |

---

## 2. Diacritics (12–59)

Each diacritic represents a base-12 tier:

| Tier | Diacritic | Starts At | Encoding |
|------|-----------|-----------|----------|
| 0    | ta        | 0         | t        |
| 1    | she       | 12        | s        |
| 2    | ree       | 24        | r        |
| 3    | jo        | 36        | j        |
| 4    | wu        | 48        | w        |

### Encoding Methods

1. **Two-Part Array Encoding**: [tier, offset] where tier is 0-4 and offset is 0-11
   - `[1, 0]` = she (tier 1, offset 0 = decimal 12)
   - `[1, 3]` = shetree (tier 1, offset 3 = decimal 15)
   - `[2, 8]` = reek (tier 2, offset 8 = decimal 32)
   - `[4, 0]` = wu (tier 4, offset 0 = decimal 48)
   - `[4, 11]` = wulev (tier 4, offset 11 = decimal 59)

2. **Decimal Calculation**: Decimal = (tier × 12) + offset
   - `[1, 3]` = (1 × 12) + 3 = 15 = shetree
   - `[2, 8]` = (2 × 12) + 8 = 32 = reek
   - `[4, 11]` = (4 × 12) + 11 = 59 = wulev

3. **Two-Character String Encoding**: Diacritic letter + glyph encoding
   - 12 = **she** → `s0`
   - 15 = **shetree** → `s3`
   - 18 = **shex** → `s6` (with contraction)
   - 32 = **reek** → `r8` (with contraction)
   - 59 = **wulev** → `wb`

---

## 3. Contraction Rules

### Contraction Glyphs

For smoother pronunciation, glyphs `hyx` and `awk` contract with diacritics:

| Diacritic | + hyx → | + awk →  |
|-----------|---------|----------|
| she       | shex    | shek     |
| ree       | reex    | reek     |
| jo        | jox     | jok      |
| wu        | wux     | wuk      |

---

## 4. Suffix Tier System (60ⁿ and above)

Suffixes denote powers of 60. Applied after 59:

| Power | Decimal Value  | Suffix | Vowel   | 
|-------|----------------|--------|--------|
| 60^1  | 60             | ma     | a      |
| 60^2  | 3,600          | fe     | e      |
| 60^3  | 216,000        | gi     | i      |
| 60^4  | 12.96M         | tho    | o      |
| 60^5  | 777.6M         | vu     | u      |
| 60^6  | 46.656B        | ya     | a      |
| 60^7  | 2.799T         | fre    | e      |
| 60^8  | 167.9T         | gli    | i      |
| 60^9  | 10.1P          | dra    | a      |
| 60^10 | 604P           | ske    | e      |
| 60^11 | 36E            | pri    | i      |
| 60^12 | 2.1Z           | kro    | o      |
| 60^13 | 126.1Z         | zru    | u      |

---

## 5. Forming Numbers

### Numbers 0–59:
- Use base glyphs and diacritics.
- No suffixes.
- Apply contractions where needed.

### Numbers ≥ 60:
- Numbers follow the pattern: **[multiplier].[suffix].[remainder]**
- When spoken, implicit elements are dropped:
  - Implicit `ta` (multiplier = 1) is not spoken
  - Implicit `zo` (remainder = 0) is not spoken
  
#### Deep Structure Examples:
- 60 = `ta.ma.zo` → **ma** (drop implicit ta and zo)
- 61 = `ta.ma.zee` → **mazee** (drop implicit ta)
- 62 = `ta.ma.bey` → **mabey**
- 71 = `ma.she.lev` → **mashelev** (60 + 11)
- 84 = `ma.ree.awk` → **mareek** (60 + 24, with contraction)
- 119 = `ma.wu.lev` → **mawulev** (60 + 59)
- 120 = `bey.ma.zo` → **beyma** (2×60, drop zo)
- 121 = `bey.ma.zee` → **beymazee**
- 132 = `bey.ma.she.zo` → **beymashe** (2×60 + 12, drop zo)
- 144 = `bey.ma.she.she` → **beymasheshe** (2×60 + 24)
- 180 = `tree.ma.zo` → **treema** (3×60)
- 3600 = `ta.fe.zo` → **fe** (1×3600)
- 3660 = `ta.fe.ta.ma.zo` → **fema** (3600 + 60)
- 3661 = `ta.fe.ta.ma.zee` → **femazee**
- 7200 = `bey.fe.zo` → **beyfe** (2×3600)

### Fundamental Pattern for Large Numbers

The Hyxos system builds all numbers using a recursive pattern. Every number can be decomposed into individual tokens separated by periods:

```
[glyph].[suffix].[diacritic].[glyph]...
```

More specifically, for numbers with suffixes:
- **Before suffix**: `[diacritic].[glyph]` or just `[glyph]` (multiplier)
- **Suffix**: The power of 60 (ma, fe, gi, etc.)
- **After suffix**: `[diacritic].[glyph]` or just `[glyph]` (remainder)

Token separation rules:
- Each diacritic is a separate token: `ta`, `she`, `ree`, `jo`, `wu`
- Each glyph is a separate token: `zo`, `zee`, `bey`, etc.
- Each suffix is a separate token: `ma`, `fe`, `gi`, etc.
- In technical notation, ALL tokens are shown separated by periods
- In spoken form, tokens merge following the contraction and dropping rules

The remainder itself can contain another suffix, creating nested structures:
- 3661 = `ta.fe.ta.ma.zee` → **femazee** (3600 + 60 + 1)

### Suffix Interaction Rules

When a suffix appears:
1. It "captures" everything to its left as its multiplier
2. Everything to its right becomes the remainder
3. The pattern repeats recursively for nested suffixes

Examples with explicit tokenization:
- **beymashe** = `bey.ma.she.zo` = 2×60 + 12 = 132
- **femabey** = `ta.fe.ta.ma.bey` = 1×3600 + 1×60 + 2 = 3662
- **treefebeyma** = `tree.fe.bey.ma.zo` = 3×3600 + 2×60 = 10,920
- **wulevmawulev** = `wu.lev.ma.wu.lev` = 59×60 + 59 = 3599

---

## 6. Counting Examples

### Count by 3 (Tree) to 60:
- 3 = **tree**
- 6 = **hyx**
- 9 = **neyn**
- 12 = **she**
- 15 = **shetree**
- 18 = **shex**
- ...
- 60 = **ma**

---

## 7. Fractions

### The Fraction Suffix: -os / -tos

Fractions in Hyxos are formed by adding the suffix **-os** or **-tos** to form "one-nth". The choice between -os and -tos appears to follow euphonic patterns:

Pattern observations:
- **-tos**: zotos, zeetos, beytos, treetos, pentos, septos, awktos
- **-os**: katos, hyxos, neynos, dekos, levos, shetos
- The distribution may reflect phonetic harmony or historical linguistic patterns

| Hyxamel | Meaning | Decimal |
|---------|---------|---------|
| zotos   | 1/0     | ∞       |
| zeetos  | 1/1     | 1.0     |
| beytos  | 1/2     | 0.5     |
| treetos | 1/3     | ~0.333  |
| katos   | 1/4     | 0.25    |
| pentos  | 1/5     | 0.2     |
| hyxos   | 1/6     | ~0.167  |
| septos  | 1/7     | ~0.143  |
| awktos  | 1/8     | 0.125   |
| neynos  | 1/9     | ~0.111  |
| dekos   | 1/10    | 0.1     |
| levos   | 1/11    | ~0.091  |
| shetos  | 1/12    | ~0.083  |

### Compound Fractions

For numerators greater than 1:
- **beytreetos** = 2/3 = ~0.667
- **treekatos** = 3/4 = 0.75
- **shehyxos** = 12/6 = 2

### Etymology Note

The name "Hyxos" for the numeral system itself comes from **hyxos** (1/6), reflecting the hexagonal/six-fold symmetry that appears throughout the system's design.

---

## 8. Grammatical Suffixes (Provisional)

### Core Suffix Pattern

Following the diacritic vowel sequence (a, e, i, o, u), grammatical suffixes modify number meaning:

| Suffix | Vowel   | Function          | Example      | Meaning                |
|-------|---------|-------------------|--------------|------------------------|
| -as   | a (ta)  | singular/cardinal | hyxas → hyx  | six (usually implicit) |
| -es   | e (she) | plural            | hyxes        | sixes                  |
| -is   | i (ree) | trio              | hyxis        | group of three         |
| -os   | o (jo)  | fractional        | hyxos        | one-sixth              |
| -us   | u (wu)  | ordinal           | hyxus        | sixth (in order)       |

### Extended Patterns (Experimental)

**Compound suffixes for numerical operations:**
- **-ema** - iterative/repeated (e.g., beyema = "twice/repeatedly")
- **-ek** - reciprocal/pairwise (e.g., beyek = "in pairs", treek = "in triples")
- **-an** - multiplicative (e.g., hyxan = "sixfold", dekan = "tenfold")
- **-at** - distributive (e.g., hyxat = "by sixes", mawat = "by sixties")
- **-ar** - approximative (e.g., dekar = "about ten", fear = "roughly 3600")
- **-al** - adjectival/system descriptor (e.g., dekal = "decimal/ten-based", maal = "sexagesimal")

### System Naming Conventions

The -al suffix creates terms for number systems and properties:
- **hyxadekimal** = base-60 system (traditional mathematical style)
- **hyxadekal** = base-60 system (pure Hyxos grammar)
- **maal** = sexagesimal/sixty-based (from ma + al)
- **sheal** = duodecimal/twelve-based
- **dekal** = decimal/ten-based

### Usage in Natural Languages

These numerical expressions integrate into any language like specialized counting terms:
- English: "Give me treeat apples" (Give me three each)
- Spanish: "Necesito beyoos" (I need two each/in pairs)
- Japanese: "Hyxan onegaishimasu" (Six times, please)
- French: "J'en veux dekun" (I want tenfold)

### Usage Notes

- These suffixes modify meaning, not value: **beyma** (2×60=120) vs **beyema** (repeatedly two)
- The -as (singular) suffix is typically implicit, like the ta diacritic
- This system is provisional and may evolve with usage

---

## 9. Notes

- Diacritic vowel order mirrors symbolic logic: a, e, i, o, u
- Root glyphs can combine recursively with suffix tiers for massive numbers
- Contractions enhance fluidity and spoken cadence
- Hyxamel representations are typically much more concise than decimal equivalents (e.g., "feshexmaneyn" = 4 syllables vs "four thousand six hundred eighty-nine" = 9 syllables)

---

## 10. Diacritic Semantics and Silent `ta`

### 🔹 The Silent Diacritic: `ta`

The first 12 numerals (`zo` through `lev`, values 0–11) fall under an **implied diacritic**:

- This diacritic is named **`ta`**
- **It is not spoken or written**
- **It defines the zeroth tier** of counting in the system

Examples:
- `ta.zo` = **zo**
- `ta.zee` = **zee**
- ...
- `ta.lev` = **lev**

> The `ta` diacritic is always **silent** and **invisible** in spoken or written form.

### 🔹 The Zero Parallel: `ta` and `zo`

There is a fundamental symmetry in the Hyxos system between zeros:

- **`ta`** = Zero diacritic (represents tier 0)
- **`zo`** = Zero glyph (represents value 0)

This parallel creates a conceptual foundation:
- Just as `zo` is the starting point for glyphs (0-11)
- `ta` is the starting point for diacritics (tier 0)

Both represent "nothing" or "zero" in their respective domains:
- `ta` + any glyph = just the glyph (no diacritic modifier)
- any diacritic + `zo` = just the diacritic value (e.g., `she` = 12, not `shezo`)

---

### 🔹 Diacritics as Powers of 12

The five vowel-based diacritics reflect **powers of 12**:

| Power of 12 | Decimal | Diacritic | Notes                     |
|-------------|---------|-----------|---------------------------|
| 12⁰         | 1       | `ta`      | (implied) base glyphs     |
| 12¹         | 12      | `shey`    | `shezo` → spoken as `she` |
| 12²         | 144     | `ree`     | `reezo` → spoken as `ree` |
| 12³         | 1,728   | `jo`      | `jozo` → spoken as `jo`   |
| 12⁴         | 20,736  | `wu`      | `wuzo` → spoken as `wu`   |

Thus, the **diacritic alone** may serve as a numeral in multiples of 12:

- **12** = `she` (not `shezo`)
- **24** = `ree` (not `reezo`)
- **36** = `jo`  (not `jozo`)
- **48** = `wu`  (not `wuzo`)

These shortenings improve clarity and rhythm in both speech and symbolic notation.

---

## 11. Practical Example: Math Problem

Here's how Hyxos works in a real mathematical context:

**Problem**: "If you have treema apples and need to distribute them hyxat to some baskets, then give away beyus through hyxus baskets, how many apples remain?"

| Step | Description       | Hyxos | Calculation | Decimal |
|------|-------------------|--------|-------------|---------|
| 1    | Starting apples   | treema | tree × ma | 180 |
| 2    | Distribute by     | hyxat | (by sixes) | ÷6 |
| 3    | Number of baskets | reex | treema ÷ hyx | 30 |
| 4    | Apples per basket | hyx | - | 6 |
| 5    | Give away baskets | beyus through hyxus | - | 2nd-6th |
| 6    | Baskets given     | pen | hyxus - zeeus | 5 |
| 7    | Apples given away | reex | pen × hyx | 30 |
| 8    | Apples remaining  | beymareex | treema - reex | 150 |

**Answer**: beymareex apples remain

This example demonstrates:
- Cardinal numbers (treema, hyx, pen)
- Ordinal suffixes (beyus, hyxus)
- Distributive suffix (hyxat)
- Arithmetic operations in hyxadekimal

---

## 12. Encoding

The Hyxos numeral system supports two primary symbolic encodings:

* **Base-12 Encoding** — for values `0` to `11`
* **Sexagesimal Encoding** — for values `0` to `59`

Encoding representations are designed for compact symbolic notation in digital or script-based contexts.

---

### 🔹 Base-12 Encoding (0–11)

Each of the twelve root glyphs maps to a single hexadecimal-like character:

| Decimal | Glyph | Encoding |
| ------- | ----- | -------- |
| 0       | zo    | `0`      |
| 1       | zee   | `1`      |
| 2       | bey   | `2`      |
| 3       | tree  | `3`      |
| 4       | kat   | `4`      |
| 5       | pen   | `5`      |
| 6       | hyx   | `6`      |
| 7       | sep   | `7`      |
| 8       | awk   | `8`      |
| 9       | neyn  | `9`      |
| 10      | dek   | `a`      |
| 11      | lev   | `b`      |

> ⚠️ These encodings are used standalone only for values 0–11.

---

### 🔹 Sexagesimal Encoding (0–59)

All values from `0` to `59` are encoded using **two characters**, where:

* The **first character** encodes the **diacritic tier** (in base-12, multiples of 12)
* The **second character** encodes the **base glyph** (0–11)

| Tier | Decimal Range | Diacritic | Encoding Char |
| ---- | ------------- | --------- | ------------- |
| 0    | 0–11          | ta        | `t`           |
| 1    | 12–23         | she       | `s`           |
| 2    | 24–35         | ree       | `r`           |
| 3    | 36–47         | jo        | `j`           |
| 4    | 48–59         | wu        | `w`           |

#### Examples

| Value | Spoken Form | Encoding |
| ----- | ----------- | -------- |
| 0     | zo          | `t0`     |
| 3     | tree        | `t3`     |
| 15    | shetree     | `s3`     |
| 18    | shex        | `s6`     |
| 32    | reek        | `r8`     |
| 48    | wu          | `w0`     |
| 59    | wulev       | `wb`     |

> 💡 **Note:** Even single-digit values (like `zo`, `zee`) use two-character encodings when in sexagesimal context.

---
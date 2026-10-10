# Hyxos Sixty

Hyxos Sixty is the typeface for the sixty Hyxos numerals, the values 0 to 59. `HyxosSixty-Regular.ttf` is the font and `HyxosSixty-Regular.woff2` is the same face compressed for the web. Each numeral is a composite glyph, and the sixty composites share one set of seventeen outlines: twelve bodies and five suit marks.

## Provenance

Emitted from the exact outline source, text-face set, build of 2026-07-23. One face is canonical; tuning variants are not published.

The ttf is 19468 bytes. Its SHA-256 is `2528e53dd57746e337d455f00497d642089fa2846783f912467dc84ff3bf384a` and its CRC-32 is `0c846ed5`.

The woff2 is converted from the ttf with fontTools 4.66.1 and brotli 1.2.0. The ttf is loaded with `TTFont(path, recalcTimestamp=False)`, its `flavor` is set to `"woff2"`, and it is saved. With those two versions the conversion produces the same bytes on every run.

## Codepoints

The font maps 82 codepoints, all in Supplementary Private Use Area-B (plane 16). The numeral for value `v` is the suit mark of tier `floor(v / 12)` with the body of offset `v mod 12`, in the terms of the two-part encoding in the [grammar guide](../GRAMMAR.md).

| Codepoints | Count | Contents |
|---|---|---|
| U+100000 to U+10003B | 60 | The numerals, values 0 to 59 in order. U+100000 is 0 and U+10003B is 59. |
| U+100040 to U+10004B | 12 | The bodies, offsets 0 to 11 in order. |
| U+100050 to U+100054 | 5 | The suit marks, tiers 0 to 4 in order. |
| U+100060 to U+100064 | 5 | The five suit marks again, enlarged and centred on the advance width and on the middle of the ascent and descent, in tier order. No `ccmp` rule uses them. |

A suit mark followed by a body composes into the numeral through the font's `ccmp` ligature: U+100051 followed by U+100043 renders as U+10000F, the numeral for 15.

## Use

```css
@font-face {
  font-family: "Hyxos Sixty";
  src: url("HyxosSixty-Regular.woff2") format("woff2"),
       url("HyxosSixty-Regular.ttf") format("truetype");
}

.numerals {
  font-family: "Hyxos Sixty", sans-serif;
}
```

Put `Hyxos Sixty` first in the font stack, because system fonts swallow characters they do not map.

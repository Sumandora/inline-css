# inline-css

A rust proc-macro that lets you write CSS in your rust code

Example
---

```rust
use inline_css::*;

fn main() {
    let my_precious_font_size = 36;
    assert_eq!(
        css! {
            body {
                margin: 0;
                padding: 12 em;
            }

            * {
                font-family: "Times New Roman";
                /* It even supports splicing! */
                font-size: ${format!("{}pt", (my_precious_font_size * 2 - 4) / 2 + 2)};
            }
        },
        "body{margin:0;padding:12em}*{font-family:Times New Roman;font-size:36pt}"
    )
}
```

As you can see the css is also minified.  

Caveats
---

Apart from not being well tested, the biggest issue is the Rust-Lexer.
Writing something like `12em` is not tokenized as `Int(12) Ident(em)`, but as `HexInt(0x12e) Erroneous(m)`.
So you are required to put a space after the number to separate it from the unit. The space is then removed in the proc-macro.  

Internals
---

This crate relies heavily on lightningcss, which in turn relies on servo-css-parser, which is the same CSS parser as used by Firefox.
Lightningcss then also performs the minification.


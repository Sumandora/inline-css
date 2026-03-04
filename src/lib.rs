//! A rust proc-macro that lets you write CSS in your rust code
//!
//! Example
//! ---
//!
//! ```rust
//! use inline_css::*;
//!
//! fn main() {
//!     let my_precious_font_size = 36;
//!     assert_eq!(
//!         css! {
//!             body {
//!                 margin: 0;
//!                 padding: 12 em;
//!             }
//!
//!             * {
//!                 font-family: "Times New Roman";
//!                 /* It even supports splicing! */
//!                 font-size: ${(my_precious_font_size * 2 - 4) / 2 + 2}pt;
//!             }
//!         },
//!         "body{margin:0;padding:12em}*{font-family:Times New Roman;font-size:36pt}"
//!     )
//! }
//! ```
//!
//! As you can see the css is also minified.  
//!
//! Caveats
//! ---
//!
//! Apart from not being well tested, the biggest issue is the Rust Lexer.
//! Writing something like `12em` is not tokenized as `Int(12) Ident(em)`, but as `HexInt(0x12e) Erroneous(m)`.
//! So you are required to put a space after the number to separate it from the unit. The space is then removed in the proc-macro.  
//!
//! Internals
//! ---
//!
//! This crate relies heavily on lightningcss, which in turn relies on servo-css-parser, which is the same CSS parser as used by Firefox.
//! Lightningcss then also performs the minification.
//!

use lightningcss::{
    printer::PrinterOptions,
    stylesheet::{MinifyOptions, ParserOptions, StyleSheet},
};
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use std::{collections::HashSet, iter::Peekable, sync::LazyLock};

static UNITS: LazyLock<HashSet<&str>> = LazyLock::new(|| {
    HashSet::from_iter([
        "cm", "mm", "in", "px", "pt", "pc", // Absolute Lengths
        "em", "ex", "ch", "rem", "vw", "vh", "vmin", "vmax", "%", // Relative Lengths
    ])
});

// Lets just pray that this wont conflict with anything...
const SPLICE_TAG: &str = "c8a4e0311e4dd7976ad9f29d21e2eace555b2b5ae089b06586ae38d7c892fb72"; // SHA256("SPLICE")

fn construct_splice_tag(index: usize) -> String {
    String::new() + "[" + SPLICE_TAG + "#" + &index.to_string() + "]"
}

fn tokens_to_string<Iter>(
    mut ts: Peekable<Iter>,
    mut acc: String,
    splices: &mut Vec<TokenStream>,
    skip_next_space: bool,
) -> String
where
    Iter: Iterator<Item = TokenTree>,
{
    let Some(tok) = ts.next() else {
        return acc;
    };

    if let TokenTree::Group(g) = tok {
        let (left, right) = match g.delimiter() {
            Delimiter::Parenthesis => ("(", ")"),
            Delimiter::Brace => ("{", "}"),
            Delimiter::Bracket => ("[", "]"),
            Delimiter::None => (" ", " "),
        };
        acc += left;
        let mut acc = tokens_to_string(g.stream().into_iter().peekable(), acc, splices, false);
        acc += right;
        return tokens_to_string(ts, acc, splices, false);
    }

    let next = ts.peek().cloned();

    match (&tok, &next) {
        (TokenTree::Punct(p), Some(TokenTree::Group(g)))
            if p.as_char() == '$' && g.delimiter() == Delimiter::Brace =>
        {
            let _ = ts.next();

            // Pray that lightningcss wont pull this apart...
            acc = acc + &construct_splice_tag(splices.len());
            splices.push(g.stream());

            return tokens_to_string(ts, acc, splices, true);
        }
        _ => (),
    };

    let skip_next_space = match tok {
        TokenTree::Punct(p) if p.as_char() == '-' || p.as_char() == ':' || p.as_char() == '.' => {
            acc = acc + &p.to_string();
            true
        }
        TokenTree::Punct(p) if p.as_char() == '#' || p.as_char() == '@' => {
            // Still append the space for cases where the pound appears in the selector
            acc = acc + " " + &p.to_string();
            true
        }
        _ if skip_next_space => {
            acc = acc + &tok.to_string();
            false
        }
        _ => {
            acc = acc + " " + &tok.to_string();
            false
        }
    };

    if let Some(next_string) = next.map(|t| t.to_string()) {
        // Remove the space
        if UNITS.contains(next_string.as_str()) {
            let _ = ts.next();

            acc += &next_string;
        }
    }

    tokens_to_string(ts, acc, splices, skip_next_space)
}

#[proc_macro]
pub fn css(ts: TokenStream) -> TokenStream {
    let mut splices = Vec::new();
    let text = tokens_to_string(
        ts.into_iter().peekable(),
        String::new(),
        &mut splices,
        false,
    );

    let mut stylesheet =
        StyleSheet::parse(&text, ParserOptions::default()).expect("Failed to parse stylesheet");

    stylesheet
        .minify(MinifyOptions::default())
        .expect("Failed to minify stylesheet");

    let minified_css = stylesheet
        .to_css(PrinterOptions {
            minify: true,
            ..PrinterOptions::default()
        })
        .expect("Failed to emit minified css");

    let mut token_stream = TokenStream::new();
    token_stream.extend([Literal::string(&minified_css.code)]);
    for (i, tokens) in splices.into_iter().enumerate() {
        token_stream.extend([
            TokenTree::Punct(Punct::new('.', Spacing::Alone)),
            TokenTree::Ident(Ident::new("replace", Span::mixed_site())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, {
                let mut ts = TokenStream::from_iter([
                    TokenTree::Literal(Literal::string(&construct_splice_tag(i))),
                    TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                ]);
                ts.extend([TokenTree::Punct(Punct::new('&', Spacing::Alone))]);
                ts.extend([TokenTree::Group(Group::new(Delimiter::Parenthesis, tokens))]);
                ts.extend([
                    TokenTree::Punct(Punct::new('.', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("to_string", Span::mixed_site())),
                    TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
                ]);
                ts
            })),
        ]);
    }
    token_stream
}

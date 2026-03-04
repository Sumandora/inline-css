#[cfg(test)]
mod tests {
    use inline_css::css;

    #[test]
    fn full() {
        let x = 15;
        let y = 14;

        assert_eq!(
            css! {
                body {
                    margin: 0;
                    padding: 12 em;
                }

                * {
                    font-family: "Times New Roman";
                    font-size: 36pt;
                }

                a {
                    font-size: ${x}${y}em;
                    font-weight: ${(69 + 420 + 1337) / 3}deg;
                }
            },
            "body{margin:0;padding:12em}*{font-family:Times New Roman;font-size:36pt}a{font-size:1514em;font-weight:608deg}"
        )
    }
}

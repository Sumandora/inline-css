#[cfg(test)]
mod tests {
    use inline_css::css;

    #[test]
    fn pound_signs() {
        assert_eq!(
            css! {
                x #y {
                    color: #aaaaaa;
                }
            },
            "x #y{color:#aaa}"
        );
    }
}

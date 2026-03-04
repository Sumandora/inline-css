#[cfg(test)]
mod tests {
    use inline_css::css;

    #[test]
    fn classes() {
        assert_eq!(
            css! {
                .important {
                    color: red;
                }
            },
            ".important{color:red}"
        );
    }
}

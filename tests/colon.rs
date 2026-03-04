#[cfg(test)]
mod tests {
    use inline_css::css;

    #[test]
    fn colon() {
        assert_eq!(
            css! {
                a:hover {
                    color: red;
                }
            },
            "a:hover{color:red}"
        );
    }
}

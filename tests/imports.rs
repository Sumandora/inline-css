#[cfg(test)]
mod tests {
    use inline_css::css;

    #[test]
    fn classes() {
        assert_eq!(
            css! {
                @import url("https://fonts.googleapis.com/css2?family=Itim&family=Noto+Sans+Mono:wght@100..900&display=swap");
            },
            "@import \"https://fonts.googleapis.com/css2?family=Itim&family=Noto+Sans+Mono:wght@100..900&display=swap\";"
        );
    }
}

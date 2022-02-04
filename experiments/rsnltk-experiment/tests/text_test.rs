#[cfg(test)]
mod tests {
    use rsnltk::native::text::*;
    use std::collections::HashMap;

    #[test]
    fn test_text_if_english() {
        let text = "I like you!";
        println!("English probability: {}", similar_with_english(text))
    }

    #[test]
    fn test_trim_to_words() {
        let words = "I like you, do you like me?".to_string();
        let trimmed = trim_to_words(words);
        println!("{:?}", trimmed)
    }

    #[test]
    fn test_count_words() {
        let words = vec![
            "one".to_string(),
            "two".to_string(),
            "two".to_string(),
            "three".to_string(),
            "three".to_string(),
            "three".to_string(),
        ];
        let counted = count_words(&words);

        println!("{:?}",counted);


    }
}
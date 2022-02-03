pub mod annotated_document;
pub mod english_rules;
pub mod node_label;
pub mod regex_tokenizer;
pub mod tree_sequence;
pub mod regex;
mod readme;
mod text_source;

#[cfg(test)]
mod test{
    use crate::native::nlpsvc::annotated_document::*;
    use crate::native::nlpsvc::english_rules::EnglishTokenizer;
    use crate::native::nlpsvc::regex_tokenizer::RegexTokenizer;
    # [test]
    fn test1(){
        let text="A Rust library to support natural language processing with pure Rust implementation and Python bindings!";
        let mut tokenizer = EnglishTokenizer::new();   // compile regex patterns
        let mut doc = AnnotatedDocument::new(text);
        tokenizer.apply_to(&mut doc);
    }

}
/// english_rules.rs
///
/// `EnglishTokenizer` wraps a ThompsonInterpreter around a set of regex
/// patterns for ordinary English token types. It also implements the
/// `RegexTokenizer` trait, which in turn requires it to implement
/// the `TokenReactor` and `TokenRecognizer` traits.

use crate::native::nlpsvc::regex::reinterp::ThompsonInterpreter;
use crate::native::nlpsvc::regex::reinterp::TokenRecognizer;
use crate::native::nlpsvc::regex::reinterp::MatchRecord;
use crate::native::nlpsvc::regex_tokenizer::TokenReactor;
use crate::native::nlpsvc::regex_tokenizer::ThompsonProgramBuilder;
use crate::native::nlpsvc::regex_tokenizer::RegexTokenizer;
use crate::native::nlpsvc::annotated_document::*;

pub struct EnglishTokenizer {
    matcher: ThompsonInterpreter,
}

impl EnglishTokenizer {

    pub fn new() -> EnglishTokenizer {
        let english_patterns = ThompsonProgramBuilder::new()
            .add_rule(r"(?i)[a-z]+")           // [0] words
            .add_rule(r"[0-9,.]*[0-9]+")       // [1] numbers
            .add_rule(r"[.,?!]")               // [2] punctuation
            .build();
        EnglishTokenizer {
            matcher: ThompsonInterpreter::new(english_patterns),
        }
    }

    fn word_action(&mut self, _begin: usize, _end: usize, token: &mut NodeLabel) {
        //println!("WORD [{}] at {}", &doc.get_text()[begin..end], begin);
        token.set_sym_val("toktype", "WORD");
    }

    fn num_action(&mut self, _begin:usize, _end: usize, token: &mut NodeLabel) {
        //println!("NUMBER [{}] at {}", &doc.get_text()[begin..end], begin);
        token.set_sym_val("toktype", "NUMBER");
    }

    fn punct_action(&mut self, _begin: usize, _end: usize, token: &mut NodeLabel) {
        //println!("PUNCT [{}] at {}", &doc.get_text()[begin..end], begin);
        token.set_sym_val("toktype", "PUNCT");
    }
}

impl TokenRecognizer for EnglishTokenizer {
    fn next_token(&mut self, text: &str, pos: usize) -> Option<MatchRecord> {
        self.matcher.next_token(text, pos)
    }
}


impl TokenReactor for EnglishTokenizer {
    /// Append a token
    ///
    /// Append a token starting at `begin` with text `text`, that
    /// matched rule #`rule_id`.
    fn append(&mut self,
              begin: usize,
              end:usize,
              rule_id: usize,
              doc: &mut AnnotatedDocument
    ) {
        let mut token = NodeLabel::new();
        token.set_span(begin, end);
        match rule_id {
            0 => { self.word_action(begin, end, &mut token); }
            1 => { self.num_action(begin, end, &mut token); }
            2 => { self.punct_action(begin, end, &mut token); }
            _ => { panic!("Unrecognized rule ID {} at pos {}", rule_id, begin); }
        };
        println!(
            "{} [{}] at {}",
            token.get_sym_val("toktype"),
            &doc.get_text()[begin..end],
            begin
        );
        doc.get_trees_mut().push_back(token);
    }

    /// Skip an unhandled character
    ///
    /// The character at `begin` is not the first character of any pattern
    /// that this tokenizer knows about. For symmetry with `append()`,
    /// the text is passed in as a &str, but in general it should only be
    /// one character long.
    fn skip(&mut self, begin: usize, text: &str) {
        println!("No rule matched at pos {} ('{}')", begin, &text[0..1]);
    }
}

impl RegexTokenizer for EnglishTokenizer {}
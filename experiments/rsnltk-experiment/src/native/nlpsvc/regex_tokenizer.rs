use crate::native::nlpsvc::regex::reinterp::TokenRecognizer;
use crate::native::nlpsvc::regex::retrans::RegexTranslator;
use crate::native::nlpsvc::regex::reparse;
use crate::native::nlpsvc::regex::reprog::Program;

use crate::native::nlpsvc::annotated_document::*;

/// Trait for holding actions to take upon token recognition
///
///
pub trait TokenReactor {
    /// Append a token
    ///
    /// Append a token starting at `begin` with text `text`, that
    /// matched rule #`rule_id`.
    fn append(&mut self, begin: usize, end: usize, rule_id: usize, doc: &mut AnnotatedDocument);

    /// Skip an unhandled character
    ///
    /// The character at `begin` is not the first character of any pattern
    /// that this tokenizer knows about. For symmetry with `append()`,
    /// the text is passed in as a &str, but in general it should only be
    /// one character long.
    fn skip(&mut self, begin: usize, text: &str);
}


pub trait RegexTokenizer: TokenRecognizer + TokenReactor {

    fn apply_to(&mut self, doc: &mut AnnotatedDocument) {
        let mut pos: usize = 0;
        //let text = doc.get_text();
        while pos < doc.get_text().len() {
            match self.next_token(doc.get_text(), pos) {
                None => {
                    self.skip(pos, &doc.get_text()[pos..pos + 1]);
                    pos += 1;
                }
                Some(match_rec) => {
                    let new_pos = pos + match_rec.len;
                    self.append(pos, new_pos, match_rec.rule, doc);
                    pos = new_pos;
                }
            }
        }
    }

}

/// Designed to apply a regex compiler to a sequence of regexes
///
/// This way we can run them all in parallel, and keep track of which ones
/// matched.
pub struct ThompsonProgramBuilder {
    compiler: RegexTranslator,
    rule_nbr: usize,
}

impl ThompsonProgramBuilder {

    pub fn new() -> ThompsonProgramBuilder {
        ThompsonProgramBuilder {
            compiler: RegexTranslator::new(),
            rule_nbr: 0,
        }
    }

    /// Compile the pattern and add to the current program.
    pub fn add_rule(mut self, pattern: &str) -> ThompsonProgramBuilder {
        let tree = reparse::parse(pattern);
        self.compiler.compile(&tree, self.rule_nbr);
        self.rule_nbr += 1;
        self
    }

    pub fn build(mut self) -> Program {
        self.compiler.finish();       // ground instruction labels
        self.compiler.print_prog();
        self.compiler.prog
    }

}
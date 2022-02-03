use crate::native::nlpsvc::regex::reterm::*;


struct ParseContext {
    no_case: bool,
}

impl ParseContext {
    pub fn new() -> ParseContext {
        ParseContext { no_case: false, }
    }
}

pub fn parse(text: &str) -> Term
{
    let mut ctx = ParseContext::new();
    match parse_regex(text, &mut ctx) {
        Some((t, s)) => {
            if !s.is_empty() {
                println!("Did not parse the whole regex string. Remainder: '{}'", s);
            }
            t
        },
        None => panic!("Parse failed!")
    }
}

/**
 * <regex> ::= <alt>
 * <alt> ::= <conc> OR <conc> '|' <alt>
 * <conc> ::= <iter> OR <iter> <conc>
 * <iter> ::= <base> OR <iter> '*' OR <iter> '+' OR <iter> '?'
 * <base> ::= <char> OR '(' <regex> ')' OR '\' <char> OR '.'
 */
fn parse_regex<'a>(text: &'a str, ctx: &mut ParseContext) -> Option<(Term, &'a str)>
{
    //println!("parse_regex '{}'", text);
    parse_alt(text, ctx)
}

fn parse_alt<'a>(text: &'a str, ctx: &mut ParseContext) -> Option<(Term, &'a str)> {
    //println!("parse_alt '{}'", text);
    match parse_conc(text, ctx) {
        None => None,
        Some((t1, rmdr1)) => {
            if !rmdr1.starts_with("|") {
                Some((t1, rmdr1))
            } else {
                match parse_alt(&rmdr1[1..], ctx) {
                    None => None,
                    Some((t2, rmdr2)) => 
                        Some((Term::new(TermType::Alternation, vec!(t1, t2)), rmdr2))
                }
            }
        }
    }
}

fn parse_conc<'a>(text: &'a str, ctx: &mut ParseContext) -> Option<(Term, &'a str)> {
    //println!("parse_conc '{}'", text);
    match parse_iter(text, ctx) {
        None => None,
        Some((t1, rmdr1)) => {
            if rmdr1.is_empty() || is_operator(rmdr1.chars().next().unwrap()) {
                // 'is_operator' really means 'is_not_a_character_literal'
                Some((t1, rmdr1))
            } else {
                match parse_conc(rmdr1, ctx) {
                    None => None,
                    Some((t2, rmdr2)) =>
                        Some((Term::new(TermType::Concatenation, vec!(t1, t2)), rmdr2))
                }
            }
        }
    }
}

/**
 * Because of expressions like 'b**', the rule has to be:
 *    <iter> -> <iter> '*'
 * But this is left-recursive.
 */
fn parse_iter<'a>(text: &'a str, ctx: &mut ParseContext) -> Option<(Term, &'a str)> {
    //println!("parse_iter '{}'", text);
    match parse_atom(text, ctx) {
        None => None,
        Some((mut t1, mut rmdr1)) => {
            loop {
                match rmdr1.chars().next() {
                    None => break,
                    Some(c1) => match c1 {
                        '*' => t1 = Term::new(TermType::Iteration, vec!(t1)),
                        '+' => t1 = Term::new(TermType::PositiveIteration, vec!(t1)),
                        '?' => t1 = Term::new(TermType::Optional, vec!(t1)),
                        _ => break
                    }
                }
                rmdr1 = &rmdr1[1..];
            }
            Some((t1, rmdr1))
        }
    }
}

fn parse_atom<'a>(text: &'a str, ctx: &mut ParseContext) -> Option<(Term, &'a str)> {
    //println!("parse_atom '{}'", text);
    if text.starts_with("(") {
        if text[1..].starts_with("?") {
            let rmdr = scan_flags(&text[2..], ctx);
            parse_atom(rmdr, ctx)
        } else {
            match parse_regex(&text[1..], ctx) {
                None => None,
                Some((t, rmdr)) => {
                    if !rmdr.starts_with(")") {
                        None
                    } else {
                        Some((t, &rmdr[1..]))
                    }
                }
            }
        }
    } else if text.starts_with("\\") {
        let optc = text.chars().nth(1);
        match optc {
            None => panic!("PARSE ERROR: String ends in a backslash"),
            Some(c) => Some((Term::new(TermType::Atom(c, ctx.no_case), 
                                       vec!()), 
                             &text[2..]))
        }
    } else if text.starts_with("[") {
        parse_char_class(&text[1..], ctx)
    } else {
        let c = text.chars().next().unwrap();
        if c == '.' {
            Some((Term::new(TermType::AnyCharTerm, vec!()), &text[1..]))
        } else {
            Some((Term::new(TermType::Atom(c, ctx.no_case), vec!()), 
                  &text[1..]))
        }
    }
}

/**
 * Used to tell when something is a boundary for concatenation.
 * No string that starts with one of these can be concatenated
 * with the preceding term.
 */
fn is_operator(ch: char) -> bool {
    match ch {
        '|' | '*' | '+' | '?' | ')'  => true,
        _ => false
    }
}

/**
 * The caller has already consumed the leading '[', so text[0] is either
 * '^' or a single char or the start of a char range.
 */
fn parse_char_class<'a>(text: &'a str, ctx: &mut ParseContext) -> Option<(Term, &'a str)> {
    //let mut i = 0;
    let mut rmdr = text;
    let mut negated = false;
    if scan_given("^", rmdr) {
        negated = true;
        rmdr = &rmdr[1..];
    }
    // There must be a character at text[i],
    // but we don't know whether it is a singleton, or the start of a range.
    let mut preds: Vec<CharClassPredicate> = vec![];
    loop { 
        match scan_class_elt(rmdr) {
            None => { break; },
            Some((pred, nxt)) => {
                preds.push(pred);
                rmdr = nxt;
            }
        }
    }
    rmdr = &rmdr[1..];

    let ccd = CharClassData::new(!negated, preds);
    Some((Term::new(TermType::CharClassTerm(ccd, ctx.no_case), vec![]),
          rmdr))
}

/**
 * Scan text for singleton chars and char ranges.
 * Return a char range (in either case), and the position of the 
 * next unread byte in text.
 * Probably needs to be wrapped in an Option.
 * Note that a character might be represented as an escape sequence!
 * E.g., to include ']' or maybe '^'.
 *
 * Someday there will be named classes, but this is not that day.
 */
fn scan_class_elt(text: &str) -> Option<(CharClassPredicate, &str)> {
    let mut rmdr = text;
    if scan_given("]", rmdr) {
        return None;
    }
    match scan_class_elt_char(rmdr) {
        None => { return None; }
        Some((ch1, rmdr1)) => {
            rmdr = rmdr1;
            if ch1 == '[' {
                // Might be a named character class...
            }
            if !scan_given("-", rmdr) {
                return Some((CharClassPredicate::Individual(ch1), rmdr));
            }
            rmdr = &rmdr[1..];
            match scan_class_elt_char(rmdr) {
                None => { None }
                Some((ch2, rmdr2)) => {
                    Some((CharClassPredicate::Range(ch1, ch2), rmdr2))
                }
            }
        }
    }
}

fn scan_class_elt_char(text: &str) -> Option<(char, &str)> {
    let mut bytes = text.bytes();
    let mut first: u8 = bytes.next().unwrap();
    let mut start = 0;
    let end;
    if first == b'\\' {
        first = bytes.next().unwrap();
        start += 1;
    }
    if first & 0b1000_0000 == 0b0000_0000 {
        end = start + 1;
    } else if first & 0b1110_0000 == 0b1100_0000 {
        end = start + 2;
    } else if first & 0b1111_0000 == 0b1110_0000 {
        end = start + 3;
    } else if first & 0b1111_1000 == 0b1111_0000 {
        end = start + 4;
    } else {
        unreachable!("UTF8 char scan failed!");
    }

    let c = text[start..].chars().next().unwrap();

    Some((c, &text[end..]))
}


/**
 * When this is called, we have already consumed the "(?" prefix
 *
 * <flags> ::= '(' '?' <flag-setting>+ ')'
 * <flag-setting> ::= '-'? <flag-letter>
 * <flag-letter> ::= 'i'
 */
fn scan_flags<'a>(text: &'a str, ctx: &mut ParseContext) -> &'a str /*-> Option<(Term, &str)>*/ {
    let mut unset = false;
    let mut i = 0;
    for c in text.bytes() {
        i += 1;
        match c {
            b')' => { break; }
            b'-' => { unset = true; }
            b'i' => {
                if unset {
                    ctx.no_case = false; 
                } else {
                    ctx.no_case = true; 
                }
            }
            _ => { panic!("PARSE ERROR: Unrecognized flag character {}", c); }
        }
    }
    &text[i..]
}


/**
 * The expectation here is that ch will be a one ASCII-char string.
 * This is for scanning for syntactically active characters, not general
 * unicode code points.
 * The character is not consumed from text even if it matches.
 * The caller has to manage that.
 */
fn scan_given(ch: &str, text: &str) -> bool {
    return ch.as_bytes()[0] == text.as_bytes()[0];
}


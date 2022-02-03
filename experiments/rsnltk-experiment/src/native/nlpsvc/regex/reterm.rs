use std::fmt;

#[derive(Debug)]
pub enum TermType {
    Alternation,
    Concatenation,
    Iteration,
    PositiveIteration,
    Optional,
    Atom(char, bool),
    CharClassTerm(CharClassData, bool),
    AnyCharTerm,
}

#[derive(Debug)]
pub struct Term {
    pub op: TermType,
    pub subs: Vec<Term>,
}

impl Term {

    /**
     * Note that there's no arity checking between the op and the
     * sub-term array. So far all our operators have strict arity 
     * requirements, so such a check should probably be added.
     */
    pub fn new(op: TermType, subs: Vec<Term>) -> Term {
        Term {
            op: op,
            subs: subs
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        pretty_print(&self, 0)
    }
}

/**
 * There has to be a better way to do this, but for the life of me,
 * I can't find it.
 */
fn tab_over(n: usize) {
    for _ in 0..n {
        print!(" ");
    }
}

fn print_label(t: &Term) {
    use self::TermType::*;
    match t.op {
        Concatenation => { print!("CONCATENATION"); },
        Alternation => { print!("ALTERNATION"); },
        Iteration => { print!("FREE_ITERATION"); },
        PositiveIteration => { print!("POSITIVE_ITERATION"); },
        Optional => { print!("OPTIONAL"); },
        Atom(c, nocase) => { 
            print!("ATOM '{}'", c); 
            if nocase {
                print!(" (?i)");
            } 
        },
        CharClassTerm(ref ccd, nocase) => { 
            print!("CHAR_CLASS {}", ccd); 
            if nocase {
                print!(" (?i)");
            }  
        },
        AnyCharTerm => { print!("ANY_CHAR"); },
    }
}


fn pretty_print(t: &Term, tab: usize) -> fmt::Result {
    tab_over(tab);
    print_label(t);
    println!("");
    for sb in &t.subs {
        pretty_print(sb, tab + 4).unwrap();
    }
    Ok(())
}


#[derive(Debug, Clone)]
pub struct CharClassData {
    positive: bool,
    ranges: Vec<CharClassPredicate>,
}


/**
 * The implementation of matches() doesn't really belong here.
 * It has to harmonize with other matches() methods used by the interpreter.
 * So probably there needs to be a trait defined somewhere that 
 * allows us to extend CharClassData with what we need to interpret it.
 * This is all because this struct is shared between the char class term
 * and the char class instruction.
 */
impl CharClassData {

    pub fn new(pos: bool, preds: Vec<CharClassPredicate>) -> CharClassData {
        CharClassData {
            positive: pos,
            ranges: preds,       // take ownership
        }
    }
    
    pub fn matches(&self, ch: char) -> bool {
        use self::CharClassPredicate::*;
        for pred in &self.ranges {
            match *pred {
                Range(c1, c2) => {
                    //println!("Range({}, {})", c1, c2);
                    if ch >= c1 && ch <= c2 && self.positive {
                         return true;
                    }
                }
                Individual(c1) => {
                    //println!("Individual({})", c1);
                    if c1 == ch && self.positive {
                        return true;
                    }
                }
                Named(_) => {
                    panic!("matches() unimplemented for Named");
                }
            } 
        }
        !self.positive
    }
    
}

impl fmt::Display for CharClassData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.positive {
            r#try!(write!(f, "NOT "));
        }
        for rng in &self.ranges {
            r#try!(write!(f, "{} ", rng));
        }
        Ok(())
    }
}


#[derive(Debug, Clone)]
pub enum CharClassPredicate {
    Range(char, char),
    Individual(char),
    Named(String),
}

impl fmt::Display for CharClassPredicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::CharClassPredicate::*;
        match *self {
            Range(c1, c2) => {
                write!(f, "{}-{}", c1, c2)
            }
            Individual(c) => {
                write!(f, "{}", c)
            }
            Named(ref nm) => {
                write!(f, "[:{}:]", nm)
            }
        }
    }
}




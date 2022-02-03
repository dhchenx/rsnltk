/**
 * Thompson style "breadth first" NFA interpreter.
 * Add dynamic programming, and you get a "just in time" DFA compiler.
 *
 * Multiple patterns:
 * Append all the programs? Each one has 1 start instruction and 1 match.
 * Ideally we want to keep track of which Match instructions we encounter,
 * not just which string positions we are in when we hit a Match.
 * Appending all programs means we still just have one clist and one nlist.
 */

use std::mem::swap;
use std::cmp::{PartialOrd, Ordering};
use crate::native::nlpsvc::regex::reprog::*;
use crate::native::nlpsvc::regex::sparse::SparseSet; // cribbed from regex crate, and from its ancestors
use crate::native::nlpsvc::regex::reprog::Instruction::*;
use crate::native::nlpsvc::regex::util::char_at;




/// Record of candidate matches.
///
/// The engine does not keep track of where the match starts, since all 
/// candidates start at the same place. 
///
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MatchRecord {
    pub len: usize,
    pub rule: usize,
}

impl MatchRecord {
    pub fn new(len: usize, rule: usize) -> MatchRecord {
        MatchRecord { len, rule }
    }
}

impl PartialOrd for MatchRecord {

    /// A MatchRecord is bigger if it is longer, or same length but its rule is lower numbered
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.len > other.len {
            return Some(Ordering::Greater);
            //best = m.clone();
        } else if self.len == other.len {
            if self.rule < other.rule {
                return Some(Ordering::Greater);
            } else if self.rule == other.rule {
                return Some(Ordering::Equal);
            }
        }
        // self.len < other.len || equal && self.rule > other.rule
        return Some(Ordering::Less);
    }
}



pub trait TokenRecognizer {
    fn next_token(&mut self, text: &str, pos: usize) -> Option<MatchRecord>;
}



struct TaskList {
    t: SparseSet,
}

impl TaskList {
    pub fn new(len: usize) -> TaskList {
        TaskList { t: SparseSet::new(len) }
    }

    pub fn clear(&mut self) {
        self.t.clear();
    }

    pub fn len(&self) -> usize {
        self.t.len()
    }

    pub fn is_empty(&self) -> bool {
        self.t.is_empty()
    }

    pub fn add_task(&mut self, pc: Label) {
        //println!("Adding task with pc = {}", pc);
        if !self.t.contains(pc) {
            self.t.insert(pc);
        }
    }
}



pub struct ThompsonInterpreter {
    pub matches: Vec<MatchRecord>, 
    prog: Program,
}

impl ThompsonInterpreter {
    
    /// Make a new ThompsonInterpreter, with program `p` and no matches.
    pub fn new(p: Program) -> ThompsonInterpreter {
        ThompsonInterpreter {
            matches: vec![],
            prog: p,
        }
    }

    /// Return the best match at our current position 
    ///
    /// Where "best" means "longest". Ties are broken according to the 
    /// order of rules: earlier (lower-numbered) rules win.
    /// So clients should put the special cases first, 
    /// and default rules later on.
    fn best_match(&self) -> Option<MatchRecord> {
        if self.matches.is_empty() {
            return None;
        }
        let mut best = MatchRecord {len: 0, rule: 0};
        for m in &self.matches {
            if m > &best {
                best = m.clone();
            }
        }
        // NOTE: If no match compares better than {0,0}, we will end up 
        // returning that. This could happen if (1) a rule matched the 
        // empty string (BAD IDEA!), and (2) it was not rule #0.
        Some(best)
    }

    /// Execute tasks in clist
    ///
    /// Loop through clist. Epsilon transitions (Split) add new entries to clist,
    /// so this implements epsilon-closure. All other instructions add new 
    /// entries to nlist.
    /// So this will apply all character tests to the current character, and
    /// return when it is done.
    /// There is no direct notion of failure here. If nothing is added to nlist,
    /// then the whole procedure will terminate very soon. There is a global 
    /// notion of failure which can be checked then, namely were there any 
    /// matches. 
    fn advance(
        &mut self, 
        str_pos: usize, 
        ch: char, 
        clist: &mut TaskList, 
        nlist: &mut TaskList
    ) {
        //println!("advance: '{}'", ch);
        let mut i: usize = 0;
        loop {
            if i >= clist.len() {
                //println!("finished with clist, end of match advance");
                return; // really we want to break out of the outer loop here...
            }

            let pc = clist.t.at(i);
            i += 1;

            //println!("Executing instruction at line {}", pc);
            let inst = &self.prog[pc];
            match *inst {
                Char(ref data) => {
                    if data.ch == ch {
                        //println!("Matched '{}' at string pos {}", data.ch, str_pos);
                        //println!("Add task to nlist at {}", pc + 1);
                        nlist.add_task(data.goto);
                    } else if data.nocase {
                        if data.ch.to_lowercase().collect::<String>() == 
                           ch.to_lowercase().collect::<String>() {
                            //println!("i-Matched '{}' at string pos {}", data.ch, str_pos);
                            nlist.add_task(data.goto);
                        }
                    }
                    // otherwise the thread dies here
                }
                AnyChar(ref data) => {
                    nlist.add_task(data.goto);
                }
                CharClass(ref ccd) => {
                    if ccd.data.matches(ch) {
                        //println!("CharClass {} matches {} at {}", ccd.data, ch, str_pos);
                        nlist.add_task(ccd.goto);
                    } else if ccd.nocase {
                        if ccd.data.matches(ch.to_lowercase().next().unwrap()) {
                            //println!("CharClass {} i-matches {} at {}", ccd.data, ch, str_pos);
                            nlist.add_task(ccd.goto);
                        }
                    }
                }
                Match(ref data) => {
                    //println!("Match: {} [{}]", str_pos, data.rule_id);
                    self.matches.push(MatchRecord::new(str_pos, data.rule_id));
                }
                Split(l1, l2) => {
                    //println!("Task at {} added to clist", l1);
                    clist.add_task(l1);
                    //println!("Task at {} added to clist", l2);
                    clist.add_task(l2);
                }
            }
        }

    }



    /// Find a token starting at &text[begin..], if possible.
    ///
    /// Results are stored in self.matches, and so "failure" is indicated
    /// by an empty match list.
    ///
    /// Note that we only match patterns that are prefixes of text. 
    /// In effect, all patterns start with an implicit '^' anchor.
    fn all_matches_at(&mut self, text: &str) {

        let plen = self.prog.len();
        let mut clist = TaskList::new(plen);    // 'current' tasks
        let mut nlist = TaskList::new(plen);    // 'next' tasks 

        self.matches.clear();

        for start in &self.prog.starts {
            //println!(">> Adding entry point {} to clist", *start);
            clist.add_task(*start);
        }
        let mut pos = 0;
        let mut nxt = 0;
        let mut ch: char;
        while !clist.is_empty() {

            pos += nxt;

            match char_at(&text[pos..]) {
                None => { 
                    if pos == text.len() {
                        // At end of string. None is expected.
                        ch = 0 as char;
                    } else {
                        panic!("ERROR: Could not decode character at {}", pos);
                    }
                }
                Some((c, byte_len)) => {
                    nxt = byte_len;
                    ch = c;
                    //println!("pos: {}; nxt: {}; ch: '{}'", pos, nxt, ch);
                }
            }

            self.advance(pos, ch, &mut clist, &mut nlist);
            
            // rebind clist and nlist
            swap(&mut clist, &mut nlist);
            nlist.clear();
        }
    }


}

impl TokenRecognizer for ThompsonInterpreter {
    /// Find the best match for a prefix of `&text[pos..]`.
    fn next_token(&mut self, text:&str, pos: usize) -> Option<MatchRecord> {
        self.all_matches_at(&text[pos..]);
        self.best_match()
    }

}
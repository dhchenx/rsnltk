////////////////////////////////////////////////////////////////////////////
// reprog.rs

use std::ops::{Index, IndexMut};
use std::fmt;
use std::collections::HashMap;
use crate::native::nlpsvc::regex::reterm::CharClassData;

pub type Label = usize;

#[derive(Debug)]
pub enum Instruction {
    Char(CharInstData),
    AnyChar(AnyCharInst),
    CharClass(CharClassInst),
    Match(MatchInst),             // arg: rule#
    Split(Label, Label),
}


#[derive(Clone, Copy, Debug)]
pub struct CharInstData {
    pub ch: char,
    pub nocase: bool,
    pub goto: Label,
}

#[derive(Clone, Copy, Debug)]
pub struct AnyCharInst {
    pub goto: Label,
}

#[derive(Clone, Copy, Debug)]
pub struct MatchInst {
    pub rule_id: usize,
    //pub goto: Label,
}

#[derive(Debug)]
pub struct CharClassInst {
    pub data: CharClassData,
    pub nocase: bool,
    pub goto: Label,
}




impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instruction::*;
        match *self {
            Char(ref data) =>  write!(f, "char {} goto {} {}", data.ch, data.goto,
                if data.nocase { " [nocase]" } else { "" }),
            AnyChar(ref data) => write!(f, "any_char goto {}", data.goto),
            CharClass(ref cc) => write!(f, "{} goto {} {}", cc.data, cc.goto,
                if cc.nocase { " [nocase]" } else { "" }),
            Match(ref data) => write!(f, "match {}", data.rule_id),
            Split(l1, l2) => write!(f, "split {}, {}", l1, l2),
        }
    }
}




#[derive(Debug)]
pub struct Program {
    code: Vec<Instruction>,
    pub starts: Vec<usize>,         // entry points
}

impl Program {
    pub fn new() -> Program {
        Program {
            code: vec![],
            starts: vec![],
        }
    }
    pub fn len(&self) -> usize {
        self.code.len()
    }
    pub fn push(&mut self, instr: Instruction) {
        self.code.push(instr);
    }
    pub fn print(&self) {
        for (pos, inst) in self.code.iter().enumerate() {
            println!("{:03}: {}", pos, *inst);
        }
    }
    pub fn add_start(&mut self, start: usize) {
        self.starts.push(start);
    }
    pub fn ground_labels(&mut self, lblmap: &HashMap<Label,Label>) {
        use self::Instruction::*;
        let mut code_new = Vec::with_capacity(self.code.len());
        for inst in self.code.iter() {
            //let ref mut i: Instruction = *inst;
            match *inst {
                Char(ref data) => { 
                    code_new.push(Char(CharInstData {
                        ch: data.ch, 
                        nocase: data.nocase,
                        goto: lblmap[&data.goto],
                    }));
                }
                AnyChar(ref data) => {
                    code_new.push(AnyChar(AnyCharInst {
                        goto: lblmap[&data.goto],
                    }));
                }
                CharClass(ref ccdata) => {
                    code_new.push(CharClass(CharClassInst {
                        data: ccdata.data.clone(),
                        nocase: ccdata.nocase,
                        goto: lblmap[&ccdata.goto],
                    }));
                }
                Match(ref data) => {
                    code_new.push(Match(MatchInst {
                        rule_id: data.rule_id,
                        //goto: lblmap[&data.goto],
                    }));
                }
                Split(l1, l2) => {
                    let l1_new = lblmap[&l1];
                    let l2_new = lblmap[&l2];
                    code_new.push(Split(l1_new, l2_new));
                }
            }
        }
        self.code = code_new;
    }
}

impl Index<usize> for Program {
    type Output = Instruction;
    fn index(&self, index: usize) -> &Instruction {
        &self.code[index]
    }
}

impl IndexMut<usize> for Program {
    //type Output = Instruction;
    fn index_mut(&mut self, index: usize) -> &mut Instruction {
        &mut self.code[index]
    }
}


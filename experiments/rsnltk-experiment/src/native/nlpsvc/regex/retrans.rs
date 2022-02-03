use std::collections::HashMap;
use crate::native::nlpsvc::regex::reterm::{Term, CharClassData};
use crate::native::nlpsvc::regex::reprog::{Program, Label, Instruction};
use crate::native::nlpsvc::regex::reprog::{CharInstData, AnyCharInst, MatchInst, CharClassInst};
use crate::native::nlpsvc::regex::reprog::Instruction::*;
use crate::native::nlpsvc::regex::reterm::TermType::*;

pub struct RegexTranslator {
    pub prog: Program,
    next_label: usize,
    label_map: HashMap<Label, Label>,
}

impl RegexTranslator {
    pub fn new() -> RegexTranslator {
        RegexTranslator {
            prog:        Program::new(),
            next_label:  0,
            label_map:   HashMap::new(),
        }
    }

    pub fn get_program(&self) -> &Program {
        &self.prog
    }

    fn gen_label(&mut self) -> Label {
        let nxt = self.next_label;
        self.next_label += 1;
        nxt
    }

    /**
     * This method is meant to be called multiple times, so that a batch
     * of regular expressions can be matched in parallel, as if they were
     * all combined into a single disjunction.
     */
    pub fn compile(&mut self, regex: &Term, rule_nbr: usize) {
        let start = self.prog.len();
        self.prog.add_start(start);
        self.translate_root(regex, rule_nbr);
    }

    pub fn finish(&mut self) {
        self.prog.ground_labels(&self.label_map);
    }

    fn translate_root(&mut self, regex: &Term, rule_nbr: usize) {
        let l1 = self.gen_label();
        let l2 = self.gen_label();
        self.translate(regex, l1, l2);
        self.emit(Match(MatchInst {rule_id: rule_nbr, /*goto: l2*/}), l2);
    }

    fn translate(&mut self, regex: &Term, l0: Label, l: Label) {
        match regex.op {
            Alternation => self.trans_alt(regex, l0, l),
            Concatenation => self.trans_conc(regex, l0, l),
            Iteration => self.trans_iter(regex, l0, l),
            Optional => self.trans_opt(regex, l0, l),
            PositiveIteration => self.trans_pos(regex, l0, l),
            Atom(c, nocase) => self.trans_char(c, nocase, l0, l),
            CharClassTerm(ref ccd, nocase) => self.trans_chcls(ccd, nocase, l0, l),
            AnyCharTerm => self.trans_any_char(l0, l),
        }
    }

    fn emit(&mut self, instr: Instruction, at_line: Label) {
        self.prog.push(instr);
        let n = self.label_map.len();
        self.label_map.insert(at_line, n);
    }

    /*
        translate(a, L0) -->
            char a goto L0
    */

    /*
        translate(e1|e2, L0, L):
            L0: split L1, L2
            L1: translate(e1, L1, L)
            L2: translate(e2, L2, L)
    */
    fn trans_alt(&mut self, regex: &Term, l0: Label, l: Label) {
        let l1 = self.gen_label();
        let l2 = self.gen_label();
        self.emit(Split(l1, l2), l0);
        self.translate(&regex.subs[0], l1, l);
        self.translate(&regex.subs[1], l2, l);
    }

    /*
        translate(e1.e2, L0, L):
            L0: translate(e1, L0, L1)
            L1: translate(e2, L1, L)
    */
    fn trans_conc(&mut self, regex: &Term, l0: Label, l: Label) {
        let l1 = self.gen_label();
        self.translate(&regex.subs[0], l0, l1);
        self.translate(&regex.subs[1], l1, l);
    }

    /*
        translate(e*, L0, L):
            L0: split L1, L
            L1: translate(e, L1, L0)
    */
    fn trans_iter(&mut self, regex: &Term, l0: Label, l: Label) {
        let l1 = self.gen_label();
        self.emit(Split(l1, l), l0);
        self.translate(&regex.subs[0], l1, l0);
    }

    /*
        translate(e?, L0, L):
            L0: split L1, L
            L1: translate(e, L1, L)
    */
    fn trans_opt(&mut self, regex: &Term, l0: Label, l: Label) {
        let l1 = self.gen_label();
        self.emit(Split(l1, l), l0);
        self.translate(&regex.subs[0], l1, l);
    }

    /*
        translate(e+, L0, L):
            L0: translate(e, L0, L1)
            L1: split L0, L
    */
    fn trans_pos(&mut self, regex: &Term, l0: Label, l: Label) {
        let l1 = self.gen_label();
        self.translate(&regex.subs[0], l0, l1);
        self.emit(Split(l0, l), l1);
    }

    fn trans_char(&mut self, c: char, nocase: bool, l0: Label, l: Label) {
        self.emit(Char(CharInstData {ch: c, nocase: nocase, goto: l} ), l0);
    }

    fn trans_any_char(&mut self, l0: Label, l: Label) {
        self.emit(AnyChar(AnyCharInst {goto: l}), l0);
    }

    /*
        translate([es], L0, L:
            L0: charclass es goto L
    */
    fn trans_chcls(&mut self, 
                   clsdata: &CharClassData, nocase: bool, 
                   l0: Label, l: Label) {
        self.emit(CharClass(CharClassInst {
            data: clsdata.clone(),
            nocase: nocase,
            goto: l,
        }), l0);
    }

    pub fn print_prog(&self) {
        self.prog.print();
    }
}

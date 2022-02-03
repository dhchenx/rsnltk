extern crate indextree;

use std::collections::HashMap;

use crate::native::nlpsvc::*;


pub use node_label::NodeLabel;
pub use tree_sequence::TreeSequence;
pub use tree_sequence::TreeCursor;
pub use tree_sequence::CursorMemo;


#[cfg(test)]
mod tests {

    use crate::native::nlpsvc::annotated_document::*;

    fn print_label(cursor: &TreeCursor, doc: &AnnotatedDocument) {
        let label = cursor.get().unwrap();
        let span = label.get_span().unwrap();
        println!("({:>02}, {:>02}) [{}]", span.0, span.1,
                 &doc.get_text()[span.0..span.1]);
    }

    #[test]
    fn push_tokens_and_traverse() {
        // Fake tokenizer
        let mut doc = AnnotatedDocument::new("01 Hello!");
        let mut lbl0 = NodeLabel::new();
        lbl0.set_span(0, 2)
            .set_sym_val("toktype", "NUMBER");
        doc.get_trees_mut().push_back(lbl0);
        let mut lbl1 = NodeLabel::new();
        lbl1.set_span(3, 8)
            .set_sym_val("toktype", "WORD");
        doc.get_trees_mut().push_back(lbl1);
        let mut lbl2 = NodeLabel::new();
        lbl2.set_span(8, 9)
            .set_sym_val("toktype", "PUNCT");
        doc.get_trees_mut().push_back(lbl2);

        // Traverse (and print)
        let mut cursor = doc.get_trees().first();
        while cursor.is_valid() {
            print_label(&cursor, &doc);
            cursor.next();
        }
    }

    #[test]
    fn test_chunking() {
        let txt = "aa bb cc dd ee ff";
        let mut doc = AnnotatedDocument::new(txt);
        for (i, _) in txt.split_whitespace().enumerate() {
            let b = i * 3;
            let e = b + 2;
            let mut lbl = NodeLabel::new();
            lbl.set_span(b, e)
                .set_sym_val("toktype", "WORD");
            doc.get_trees_mut().push_back(lbl);
        }

        {
            println!("====================");
            let mut cursor = doc.get_trees().first();
            while cursor.is_valid() {
                print_label(&cursor, &doc);
                cursor.next();
            }
            println!("====================");
        }

        let (first_child, last_child) = fake_parse(&doc);
        let mut label = NodeLabel::new();
        label.set_sym_val("cat", "cc_ee");
        doc.get_trees_mut().chunk(label, first_child, last_child);
        doc.get_trees().print();
    }

    fn fake_parse(doc: &AnnotatedDocument) -> (CursorMemo, CursorMemo) {
        let mut cursor = doc.get_trees().first();   // reset cursor
        cursor.next();
        cursor.next();
        // cursor should now be sitting on [cc]
        print_label(&cursor, &doc);
        let first_child = cursor.to_memo();
        cursor.next();
        cursor.next();
        cursor.next();
        // cursor should now be sitting on [ff]
        let last_child = cursor.to_memo();
        (first_child, last_child)
    }

}



pub struct AnnotatedDocument {
    doc_string: String,
    tree_sequence: TreeSequence,
}

impl AnnotatedDocument {

    pub fn new(text: &str) -> AnnotatedDocument {
        AnnotatedDocument {
            doc_string: String::from(text),
            tree_sequence: TreeSequence::new(),
       }
    }
    pub fn get_text(&self) -> &str {
        &self.doc_string
    }
    pub fn get_trees_mut(&mut self) -> &mut TreeSequence {
        &mut self.tree_sequence
    }
    pub fn get_trees(&self) -> &TreeSequence {
        &self.tree_sequence
    }
}


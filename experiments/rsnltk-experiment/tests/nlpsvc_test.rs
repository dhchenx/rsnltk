#[cfg(test)]
mod tests {
    use rsnltk::native::nlpsvc::annotated_document::*;
    use rsnltk::native::nlpsvc::english_rules::EnglishTokenizer;
    use rsnltk::native::nlpsvc::regex_tokenizer::RegexTokenizer;

    // Tokenize the English text
    # [test]
    fn get_token_pos_list(){
        let text="A Rust library to support natural language processing!";
        let mut tokenizer = EnglishTokenizer::new();   // compile regex patterns
        let mut doc = AnnotatedDocument::new(text);
        tokenizer.apply_to(&mut doc);
        println!("Result is: ");
        let mut cursor = doc.get_trees().first();
        while cursor.is_valid() {
            print_label(&cursor, &doc);
            cursor.next();
        }
    }

    use rsnltk::native::nlpsvc::annotated_document::*;
    fn print_label(cursor: &TreeCursor, doc: &AnnotatedDocument) {
        let label = cursor.get().unwrap();
        let span = label.get_span().unwrap();
        println!("({:>02}, {:>02}) [{}]", span.0, span.1,
                 &doc.get_text()[span.0..span.1]);
    }

    // Manually set token information
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
        // split by whitespace and iterate
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


/////////////////////////////////////////////////////////
// CharDataIter and friends
//
// Probably CharDataIter could be replaced by a clever
// call to map() on the underlying char iterator...
//
pub static END_OF_STRING: char = '\0';

#[derive(Debug)]
pub struct CharData {
    pub ch: char,
    pub byte_offset: usize,
    pub char_offset: usize,
}

pub struct CharDataIter<'a> {
    char_stream: &'a mut dyn Iterator<Item = char>,
    byte_offset: usize,
    char_offset: usize,
    really_done: bool,
}

impl<'a> CharDataIter<'a> {
    pub fn new(chs: &'a mut dyn Iterator<Item = char>) -> Self {
        CharDataIter {
            char_stream: chs,
            byte_offset: 0,
            char_offset: 0,
            really_done: false,
        }
    }
}

impl<'a> Iterator for CharDataIter<'a> {
    type Item = CharData;

    fn next(&mut self) -> Option<Self::Item> {
        match self.char_stream.next() {
            Some(c) => {
                let result = CharData {
                    ch: c,
                    byte_offset: self.byte_offset,
                    char_offset: self.char_offset,
                };
                self.char_offset += 1;
                self.byte_offset += c.len_utf8();
                Some(result)
            },
            None => {
                if self.really_done {
                    None
                } else {
                    // Special <end-of-string> marker
                    self.really_done = true;
                    Some (
                        CharData {
                            ch: END_OF_STRING,  // should be ignored!
                            byte_offset: self.byte_offset,
                            char_offset: self.char_offset,
                        }
                    )
                }
            }
        }
    }
}
//
// CharDataIter
/////////////////////////////////////////////////////////
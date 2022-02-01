/////////////////////////////////////////////////////////
// TokenIter
//

use crate::native::chardata;

static IN_TOKEN: u8 = 1;
static BTWN_TOKS: u8 = 0;

/*  TODO: Probably there should not be a String member here.
    We should either borrow a string slice from the original text,
    or else leave it out, and provide some other facility for
    converting Tokens to Strings, given the underlying string.
    In a full fledged parser, the parsed-document representation
    would handle that.
 */
#[derive(Debug)]
pub struct Token {
    pub text: String,
    pub byte_offsets: (usize, usize),
    pub char_offsets: (usize, usize),
    pub token_offset: usize
}

impl Token {
    fn new() -> Token {
        Token {
            text: "".to_string(),
            byte_offsets: (0, 0),
            char_offsets: (0, 0),
            token_offset: 0,
        }
    }
}

pub struct TokenIter<'a> {
    chdat_stream: &'a mut chardata::CharDataIter<'a>,
    curr_tok_offset: usize,
    state: u8,
}

impl<'a> TokenIter<'a> {
    pub fn new(chdats: &'a mut chardata::CharDataIter<'a>) -> Self {
        TokenIter {
            chdat_stream: chdats,
            curr_tok_offset: 0,
            state: BTWN_TOKS,
        }
    }

    fn is_boundary_char(ch: char) -> bool {
        if ch == chardata::END_OF_STRING {
            true
        } else if ch.is_whitespace() {
            true
        } else {
            false
        }
    }
}

/*  Always start out BTWN_TOKS, and therefore always end in BTWN_TOKS.
    Start by skipping characters until state changes to IN_TOKEN.
    Then (1) set the token start offsets; (2) march the char data iter forward
    until state changes to BTWN_TOKS, then fix the end offsets of the token
    under construction. Update the current token offset.
    Leave the resulting Token as the return value of next().
    If the underlying chardata::CharDataIter yields END_OF_SENTENCE:
        IN_TOKEN --> ship the current token
        BTWN_TOKS --> return None
    In the first case, the next call to next() will immediately trigger
    the second case.
*/

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        assert_eq!(self.state, BTWN_TOKS);
        let mut curr_tok = Token::new();
        loop {
            match self.chdat_stream.next() {

                Some( chardata::CharData {ch, byte_offset, char_offset} ) => {

                    if TokenIter::is_boundary_char(ch) {
                        if self.state == IN_TOKEN {
                            // ship token
                            curr_tok.byte_offsets.1 = byte_offset;
                            curr_tok.char_offsets.1 = char_offset;
                            self.state = BTWN_TOKS;
                            self.curr_tok_offset += 1;
                            return Some(curr_tok);
                        }
                        // else do nothing -- skip boundary chars
                    } else {
                        if self.state == BTWN_TOKS {
                            // start token
                            curr_tok.token_offset = self.curr_tok_offset;
                            curr_tok.byte_offsets.0 = byte_offset;
                            curr_tok.char_offsets.0 = char_offset;
                            self.state = IN_TOKEN;
                        }
                        // Accumulate characters
                        curr_tok.text.push(ch);
                        curr_tok.byte_offsets.1 = byte_offset;
                        curr_tok.char_offsets.1 = char_offset;
                    }
                },

                None => {
                    // May need to ship a token here!
                    if self.state == IN_TOKEN {
                        self.state = BTWN_TOKS;
                        return Some(curr_tok);
                    }
                    return None;
                }
            }
        }
    }
}
//
// TokenIter
/////////////////////////////////////////////////////////
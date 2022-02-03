
/**
 * Return Some((ch, len)) if there is a character at the start of text,
 * None otherwise.
 */
pub fn char_at(text: &str) -> Option<(char, usize)> {
    static HI_BIT: u8 = 0b1000_0000;
    if text.is_empty() {
        return None; 
    }
    let leader: u8 = text.as_bytes()[0];
    let mut length = 1;
    if leader & HI_BIT == 0 {
        return Some((leader as char, length));
    }
    let mut bits: u32;
    if leader >= 0b1111_0000 {
        bits = (leader & 0b0000_0111) as u32;
        length = 4;
    } else if leader >= 0b1110_0000 {
        bits = (leader & 0b0000_1111) as u32;
        length = 3;
    } else if leader >= 0b1100_0000 {
        bits = (leader & 0b0001_1111) as u32;
        length = 2;
    } else {
        unreachable!();
    }

    if text.len() < length {
        panic!("UTF-8 cutoff error: String does not contain a whole character");
    }
    
    for i in 1..length {
        let byte: u8 = text.as_bytes()[i];
        bits = (bits << 6) | (byte & 0b0011_1111) as u32;
    }
    match ::std::char::from_u32(bits) {
        None => None,
        Some(ch) => Some((ch, length))
    }
}

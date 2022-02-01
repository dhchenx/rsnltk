#[cfg(test)]
mod tests {
    use crate::api::yn::*;
    # [test]
    fn test_yes(){
        let s="yes";
        println!("{:?}",yes(s));

        println!("{:?}",is_somewhat_yes("this has a y so it is the word"));

        println!("{:?}",is_kinda_yes("very much so"));
    }

}
extern crate yn;

pub fn yes(str:&str)->bool{
    return yn::yes(str);
}
pub fn is_somewhat_yes(str:&str)->bool{
    return yn::is_somewhat_yes(str);
}

pub fn is_kinda_yes(str:&str)->bool{
    yn::is_kinda_yes(str)
}




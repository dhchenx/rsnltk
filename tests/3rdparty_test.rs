#[cfg(test)]
mod tests {

    use rsnltk::wordnet::wordnet_similarity;
    use rsnltk::api::natural::*;
    use rsnltk::api::whatlang::*;
    use rsnltk::api::yn::*;

    # [test]
    fn test_distance(){
        println!("lev = {}",lev_dist("kitten", "sitting"));
        println!("winkler = {}",jw_dist("dixon", "dicksonx"));
    }

    # [test]
    fn test_whatlang(){
        let text = "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!";
        let ret=whatlang(text);
        println!("{:?}",ret);
    }

    # [test]
    fn test_yes(){
        let s="yes";
        println!("{:?}",yes(s));

        println!("{:?}",is_somewhat_yes("this has a y so it is the word"));

        println!("{:?}",is_kinda_yes("very much so"));
    }

}
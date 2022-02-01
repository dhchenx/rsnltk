#[cfg(test)]
mod tests{
    use rsnltk::wordnet::wordnet_similarity;
    #[test]
    fn test_wordnet_similarity(){
        let s1="dog.n.1";
        let s2="cat.n.2";
        let sims=wordnet_similarity(s1,s2);
        for sim in sims{
            println!("{:?}",sim);
        }
    }
}
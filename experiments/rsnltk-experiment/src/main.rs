
fn main(){
    println!("Hello, rsnltk!");


    use rsnltk::native::text::similar_with_english;
    let text="I like you!";
    println!("{}",similar_with_english(text))
    //let meaningful_words=get_segmentation();

    //println!("Result: {:?}",meaningful_words);

}
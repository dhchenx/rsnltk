
struct Info{
    pub s:String
}

impl Info{
    fn fn_a(&mut self){
        self.s+="1";
    }
}

fn main(){

    let mut foo=Info{s:"Hello".to_string()};
    println!("1: {}",foo.s);
    foo.fn_a();
    println!("2: {}",foo.s);
    foo.fn_a();
    println!("3: {}",foo.s);

}
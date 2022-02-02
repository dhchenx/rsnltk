

struct Info{
    s:i32
}

impl Info{
    fn new(a:i32)->Info{
        Info{
            s:a,
        }
    }
}

impl Clone for Info{
    fn clone(&self)->Info{
        Info{s:self.s}
    }
}

fn abc(a:Info)->Info{
    Info{s:a.s+1}
}

fn main(){
    let mut foo=Info::new(111);
    println!("1: {}", foo.s);
    abc(foo.clone());
    println!("2: {}",foo.s);
    abc(foo.clone());
    println!("3: {}",foo.s);
}
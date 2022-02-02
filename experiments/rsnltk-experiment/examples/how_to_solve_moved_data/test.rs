# https://www.cnblogs.com/dhcn/p/12152116.html

use std::rc::Rc;
use std::cell::RefCell;

struct Info{
    s:String
}

impl Info{
    fn new(a: &str)->Info{
        Info{
            s:a.to_string(),
        }
    }
}

fn abc(a:Rc<RefCell<Info>>){
    a.borrow_mut().s=="bbbb".to_string();
}


fn main(){
    let bar=Rc::new(RefCell::new(Info::new("abc")));
    println!("1: {}",bar.borrow().s);
    abc(bar.clone());
    println!("2: {}",bar.borrow().s);
    abc(bar.clone());
    println!("3: {}",bar.borrow().s);
}


#[cfg(test)]
mod tests {
    use crate::api::whatlang::*;
    # [test]
    fn test_whatlang(){
        let text = "Ĉu vi ne volas eklerni Esperanton? Bonvolu! Estas unu de la plej bonaj aferoj!";
        let ret=whatlang(text);
        println!("{:?}",ret);
    }

}

use std::collections::HashMap;


extern crate whatlang;
use whatlang::detect;
pub fn whatlang(str:&str) ->HashMap<String,String>{


    let info = detect(str).unwrap();
    let mut result:HashMap<String,String>=HashMap::new();
    result.insert(String::from("lang"),info.lang().to_string());
    result.insert(String::from("script"),info.script().to_string());
    result.insert(String::from("confidence"),info.confidence().to_string());
    result.insert(String::from("is_reliable"),info.is_reliable().to_string());

    result

}


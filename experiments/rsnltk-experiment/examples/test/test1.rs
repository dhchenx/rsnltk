use std::collections::HashMap;
use std::*;

fn main() {
    let things = vec!["Apple", "Banana", "Dog"];
    let animals = vec![];
    for thing in things {
        if thing == "Dog" {
            animals.push(thing);
        }
    }
    println!("{:?} ", animals);
}

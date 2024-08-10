mod basket;
mod stack;
mod container;

use basket::Basket;
use stack::Stack;
use container::Container;

fn add_string_to_container<T: Container<String>> (c: &mut T, s: String){
    c.put(s);
}

fn main() {
    let mut b1 = Basket::new(String::from("Banana"));
    let b2 = Basket::new(10);

    let st1 = Stack::new(vec![String::from("Hi"), String::from("Bye")]);
    let st2 = Stack::new(vec![1,2]);
    
    add_string_to_container(&mut b1, String::from("Added"));
}

use crate::ownships::Person;

pub mod ownships;

fn main() {
    let p = Person::new("Steven", 42);
    println!("Hello, world! {:?}", &p);
}

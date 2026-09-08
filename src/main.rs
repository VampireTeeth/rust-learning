use std::cell::RefCell;
use crate::ownships::{PeopleWithRef, Person};

pub mod ownships;
pub mod static_dynamic_dispatch;

fn main() {
    string_owning_struct();
    lifetime_showcase_not_live_long_enough();
    static_dynamic_dispatch::showcase1();
    static_dynamic_dispatch::showcase2();
    static_dynamic_dispatch::showcase3();
}

fn string_owning_struct() {
    let mut p = Person::new("Steven", 42);
    let mut pp = p.clone();
    println!("{:?}", &p);
    p.change_name("Ashly");
    println!("{:?}", &p);
    pp.change_name("Cloned Steven");
    println!("{:?}", &pp);
}

fn lifetime_showcase_not_live_long_enough() {
    let pr = RefCell::new(PeopleWithRef::default());
    {
        let name = String::from("Cody");
        println!("{:?}", &pr);
        *pr.borrow_mut() = PeopleWithRef::new(&name, 25);
        println!("{:?}", &pr);
    }
    // println!("{:?}", &pr); // this will cause error - name is not living long enough
}
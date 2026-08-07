#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
}

impl Person {

    pub fn new(name: &str, age: i32) -> Person {
        Self {
            name: String::from(name),
            age,
        }
    }

}
#[derive(Debug, Clone)]
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

    pub fn change_name(&mut self, name: &str) {
        self.name = String::from(name);
    }
}

#[derive(Debug)]
pub struct PeopleWithRef<'a> {
    name: &'a str,
    age: i32,
}

impl<'a> PeopleWithRef<'a> {
    pub fn new(name: &'a str, age: i32) -> PeopleWithRef<'a> {
        Self {
            name,
            age,
        }
    }
}
impl<'a> Default for PeopleWithRef<'a> {
    fn default() -> Self {
        Self {
            name: "",
            age: 0,
        }
    }
}
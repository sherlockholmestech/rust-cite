#[derive(Debug)]
pub enum Author {
    Individual(Person),
    Organisation(String),
}

impl Author {
    pub fn new_individual(first_name: &str, last_name: &str) -> Self {
        Author::Individual(Person::new(first_name, last_name))
    }
    pub fn new_organization(name: &str) -> Self {
        Author::Organisation(name.to_string())
    }
}

#[derive(Debug)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}

impl Person {
    pub fn new(first_name: &str, last_name: &str) -> Self {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }
}
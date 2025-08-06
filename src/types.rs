use chrono::{DateTime, Utc};


#[derive(Debug)]
pub enum SourceTypes {
    Website(Website)
}

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

#[derive(Debug)]
pub struct Website {
    pub author: Author,
    pub page_title: String,
    pub date_published: Option<DateTime<Utc>>,
    pub date_accessed: DateTime<Utc>,
    pub url: String,
    pub website_title: String,
    pub publisher: String,
}

impl Website {
    pub fn new(
        author: Author,
        page_title: &str,
        date_published: Option<DateTime<Utc>>,
        date_accessed: DateTime<Utc>,
        url: &str,
        website_title: &str,
        publisher: &str,
    ) -> Self {
        Website {
            author,
            page_title: page_title.to_string(),
            date_published: date_published.map(|d| d.to_owned()),
            date_accessed,
            url: url.to_string(),
            website_title: website_title.to_string(),
            publisher: publisher.to_string(),
        }
    }
}
use chrono::{DateTime, Utc};

struct Author {
    first_name: String,
    last_name: String,
}

struct Citation {
    type_: String,
    author: Author,
    title: String,
    date_published: Option<DateTime<Utc>>,
    date_original_published: Option<DateTime<Utc>>,
    date_accessed: DateTime<Utc>,
    edition: Option<String>,
    volume: Option<String>,
    issue: Option<String>,
    page_range: Option<String>,
    isbn: Option<String>,
    url: Option<String>,
    website_title: Option<String>,
    publisher: Option<String>,
    doi: Option<String>,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

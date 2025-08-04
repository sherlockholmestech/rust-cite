mod common_types;
mod website;

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::common_types;
    // Check if the `Author` enum can be used in the `Website` struct
    #[test]
    fn test_author_in_website() {
        let author = common_types::Author::new_individual("John", "Doe");
        let website = website::Website::new(
            author,
            "Test",
            None,
            chrono::Utc::now(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert_eq!(website.title, "Test");
        match website.author {
            common_types::Author::Individual(person) => {
                assert_eq!(person.first_name, "John");
                assert_eq!(person.last_name, "Doe");
            }
            _ => panic!("Expected an individual author"),
        }

    }
}
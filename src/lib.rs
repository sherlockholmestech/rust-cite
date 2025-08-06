pub mod types;
pub mod citation_styles;

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{self, website};
    // Check if the `Author` enum can be used in the `Website` struct
    #[test]
    fn website_chicago() {
        let author = types::Author::new_individual("John", "Doe");
        let date_accessed = chrono::Utc::now();
        let website = types::Website::new(
            author,
            "Example Page",
            Some(chrono::Utc::now()),
            date_accessed,
            "https://example.com",
            "Example Website",
            "Example Publisher",
        );

        let source = types::SourceTypes::Website(website);
        let footnote = citation_styles::chicago::Chicago::generate_bibiography(source);
        println!("{footnote}");
        assert!(footnote.contains("John Doe"));

    }
}
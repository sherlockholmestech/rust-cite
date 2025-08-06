use crate::types;

pub struct Chicago {
}

impl Chicago {
    pub fn generate_bibiography(source: types::SourceTypes ) -> String {
        match source {
            types::SourceTypes::Website(website) => {
                // Generate footnote for website
                let author = match website.author {
                    types::Author::Individual(person) => format!("{} {}", person.first_name, person.last_name),
                    types::Author::Organisation(org) => org,
                };
                match website.date_published {
                    Some(date) => {
                        let date_published = date.format("%B%e, %Y").to_string();
                        format!(
                            "{}. \"{}.\" {}. {}, {}. {}.",
                            author, website.page_title, website.website_title, website.publisher, date_published, website.url
                        )
                    },
                    None => {
                        // If no publication date is available
                        let date_accessed = website.date_accessed.format("%B%e, %Y").to_string();
                        format!(
                            "{}. \"{}\" {}. {}. Accessed {}. {}.",
                            author, website.page_title, website.website_title, website.publisher, date_accessed, website.url
                        )
                    }
                }
            }
        }
    }
}
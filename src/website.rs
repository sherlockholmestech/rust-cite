use chrono::{DateTime, Utc};
use crate::common_types::{Author};
pub struct Website {
    pub author: Author,
    pub title: String,
    pub date_published: Option<DateTime<Utc>>,
    pub date_accessed: DateTime<Utc>,
    pub edition: Option<String>,
    pub volume: Option<String>,
    pub issue: Option<String>,
    pub page_range: Option<String>,
    pub isbn: Option<String>,
    pub url: Option<String>,
    pub website_title: Option<String>,
    pub publisher: Option<String>,
    pub doi: Option<String>,
}

impl Website {
    pub fn new(
        author: Author,
        title: &str,
        date_published: Option<DateTime<Utc>>,
        date_accessed: DateTime<Utc>,
        edition: Option<&str>,
        volume: Option<&str>,
        issue: Option<&str>,
        page_range: Option<&str>,
        isbn: Option<&str>,
        url: Option<&str>,
        website_title: Option<&str>,
        publisher: Option<&str>,
        doi: Option<&str>,
    ) -> Self {
        Website {
            author,
            title: title.to_string(),
            date_published,
            date_accessed,
            edition: edition.map(|s| s.to_string()),
            volume: volume.map(|s| s.to_string()),
            issue: issue.map(|s| s.to_string()),
            page_range: page_range.map(|s| s.to_string()),
            isbn: isbn.map(|s| s.to_string()),
            url: url.map(|s| s.to_string()),
            website_title: website_title.map(|s| s.to_string()),
            publisher: publisher.map(|s| s.to_string()),
            doi: doi.map(|s| s.to_string()),
        }
    }
}
use clap::{Parser, Subcommand};
use rust_cite::{citation_styles::chicago::Chicago, types::{SourceTypes}};
use reqwest;
use scraper::{Html, Selector};
use chrono::Utc;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Website URL to generate a citation for
    website: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a Chicago-style citation for a website
    Chicago {
        /// Generate a footnote citation
        #[arg(short, long)]
        footnote: bool,

        /// Generate a shorthand citation
        #[arg(short, long)]
        shorthand: bool,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Chicago { footnote, shorthand }) => {
            let source = get_website_info(&cli.website).await;
            if *footnote {

            } else if *shorthand {

            } else {
                let bibiography = Chicago::generate_bibiography(source);
                println!("Generated Bibiography Entry: {}", bibiography);
            }
        }
        None => {}
    }

}

async fn get_website_info(website: &str) -> SourceTypes {
    println!("Fetching website: {}", website);

    // Fetch the website HTML
    let body = reqwest::get(website)
        .await
        .expect("Failed to fetch website")
        .text()
        .await
        .expect("Failed to read response body");
    println!("Fetched HTML body ({} bytes)", body.len());

    let document = Html::parse_document(&body);

    // Extract page title
    let title_selector = Selector::parse("title").unwrap();
    let page_title = document
        .select(&title_selector)
        .next()
        .map(|e| e.text().collect::<Vec<_>>().join(" "))
        .unwrap_or_else(|| "Untitled Page".to_string());
    println!("Extracted page title: {}", page_title);

    // Extract publisher (try meta[name=publisher] or fallback)
    let meta_publisher_selector = Selector::parse("meta[name='publisher']").unwrap();
    let publisher = document
        .select(&meta_publisher_selector)
        .next()
        .and_then(|e| e.value().attr("content"))
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Unknown Publisher".to_string());
    println!("Extracted publisher: {}", publisher);

    // Website title (use domain as fallback)
    let website_title = website.split('/').nth(2).unwrap_or("Unknown Website").to_string();
    println!("Website title/domain: {}", website_title);

    // Use default author (since most sites don't provide this)
    let author = rust_cite::types::Author::new_individual("Web", "User");
    println!("Using default author: Web User");

    // Use current date for accessed
    let date_accessed = Utc::now();
    println!("Date accessed: {}", date_accessed);

    // Try to extract publication date from meta tags
    let meta_date_selector = Selector::parse("meta[name='date'], meta[property='article:published_time']").unwrap();
    let date_published = document
        .select(&meta_date_selector)
        .next()
        .and_then(|e| e.value().attr("content"))
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc));
    match &date_published {
        Some(dt) => println!("Extracted publication date: {}", dt),
        None => println!("No publication date found"),
    }

    let website_struct = rust_cite::types::Website::new(
        author,
        &page_title,
        date_published,
        date_accessed,
        website,
        &website_title,
        &publisher,
    );

    println!("Constructed Website struct: {:?}", website_struct);

    SourceTypes::Website(website_struct)
}
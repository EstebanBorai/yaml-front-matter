//! # YAML Front Matter (YFM) Parser
//!
//! **yaml-front-matter** parses a valid YAML string into a `struct` which
//! implements the `DeserializeOwned` trait from serde.
//!
//! Consider the following YAML content on the top of your markdown file:
//!
//! ```ignore
//! ---
//! title: 'Parsing a Markdown file metadata into a struct'
//! description: 'This tutorial walks you through the practice of parsing markdown files for metadata'
//! tags: ['markdown', 'rust', 'files', 'parsing', 'metadata']
//! similar_posts:
//!   - 'Rendering markdown'
//!   - 'Using Rust to render markdown'
//! date: '2021-09-13T03:48:00'
//! favorite_numbers:
//!     - 3.14
//!     - 1970
//!     - 12345
//! ---
//! ```
//!
//! This crate takes care of extracting this header from your markdown file and
//! parse extracted data using `serde` and `serde_yaml`.
//!
//! ## Example
//!
//! ```rust
//! use serde::Deserialize;
//! use yaml_front_matter::YamlFrontMatter;
//!
//! const SIMPLE_MARKDOWN_YFM: &str = r#"
//! ---
//! title: 'Parsing a Markdown file metadata into a struct'
//! description: 'This tutorial walks you through the practice of parsing markdown files for metadata'
//! tags: ['markdown', 'rust', 'files', 'parsing', 'metadata']
//! similar_posts:
//!   - 'Rendering markdown'
//!   - 'Using Rust to render markdown'
//! date: '2021-09-13T03:48:00'
//! favorite_numbers:
//!     - 3.14
//!     - 1970
//!     - 12345
//! ---
//!
//!
//! # Parsing a **Markdown** file metadata into a `struct`
//!
//! > This tutorial walks you through the practice of parsing markdown files for metadata
//! "#;
//!
//! #[derive(Deserialize)]
//! struct Metadata {
//!     title: String,
//!     description: String,
//!     tags: Vec<String>,
//!     similar_posts: Vec<String>,
//!     date: String,
//!     favorite_numbers: Vec<f64>,
//! }
//!
//! let result = YamlFrontMatter::parse::<Metadata>(&SIMPLE_MARKDOWN_YFM).unwrap();
//!
//! let Metadata {
//!     title,
//!     description,
//!     tags,
//!     similar_posts,
//!     date,
//!     favorite_numbers,
//! } = result;
//!
//! assert_eq!(title, "Parsing a Markdown file metadata into a struct");
//! assert_eq!(
//!     description,
//!     "This tutorial walks you through the practice of parsing markdown files for metadata"
//! );
//! assert_eq!(
//!     tags,
//!     vec!["markdown", "rust", "files", "parsing", "metadata"]
//! );
//! assert_eq!(
//!     similar_posts,
//!     vec!["Rendering markdown", "Using Rust to render markdown"]
//! );
//! assert_eq!(date, "2021-09-13T03:48:00");
//! assert_eq!(favorite_numbers, vec![3.14, 1970., 12345.]);
//! ```
//!
use serde::de::DeserializeOwned;

/// YAML Front Matter (YFM) is an optional section of valid YAML that is
/// placed at the top of a page and is used for maintaining metadata for the
/// page and its contents.
pub struct YamlFrontMatter;

impl YamlFrontMatter {
    pub fn parse<T: DeserializeOwned>(markdown: &str) -> Result<T, Box<dyn std::error::Error>> {
        let yaml = YamlFrontMatter::extract(markdown)?;
        let result = serde_yaml::from_str::<T>(yaml.as_str())?;

        Ok(result)
    }

    fn extract(markdown: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut data = String::default();
        let mut sentinel = false;

        for line in markdown.lines() {
            if line.trim() == "---" {
                if sentinel {
                    break;
                }

                sentinel = true;
                continue;
            }

            if sentinel {
                data.push_str(line);
                data.push('\n');
            }
        }

        Ok(data)
    }
}

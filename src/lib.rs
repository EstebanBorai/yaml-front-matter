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
//! use yaml_front_matter::{Document, YamlFrontMatter};
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
//! let document: Document<Metadata> = YamlFrontMatter::parse::<Metadata>(&SIMPLE_MARKDOWN_YFM).unwrap();
//!
//! let Metadata {
//!     title,
//!     description,
//!     tags,
//!     similar_posts,
//!     date,
//!     favorite_numbers,
//! } = document.metadata;
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

/// A `Document` represents the Markdown file provided as input to
/// `YamlFrontMatter::parse` associated function.
///
/// The document holds two relevant fields:
///
/// - `metadata`: A generic type with the structure of the Markdown's
/// front matter header.
///
/// - `content`: The body of the Markdown without the front matter header
pub struct Document<T: DeserializeOwned> {
    /// A generic type with the structure of the Markdown's
    /// front matter header.
    pub metadata: T,
    /// The body of the Markdown without the front matter header
    pub content: String,
}

/// YAML Front Matter (YFM) is an optional section of valid YAML that is
/// placed at the top of a page and is used for maintaining metadata for the
/// page and its contents.
pub struct YamlFrontMatter;

impl YamlFrontMatter {
    pub fn parse<T: DeserializeOwned>(
        markdown: &str,
    ) -> Result<Document<T>, Box<dyn std::error::Error>> {
        let yaml = YamlFrontMatter::extract(markdown)?;
        let metadata = serde_yaml::from_str::<T>(yaml.0.as_str())?;

        Ok(Document {
            metadata,
            content: yaml.1,
        })
    }

    fn extract(markdown: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
        let mut front_matter = String::default();
        let mut sentinel = false;
        let mut front_matter_lines = 0;
        let lines = markdown.lines();

        for line in lines.clone() {
            front_matter_lines += 1;

            if line.trim() == "---" {
                if sentinel {
                    break;
                }

                sentinel = true;
                continue;
            }

            if sentinel {
                front_matter.push_str(line);
                front_matter.push('\n');
            }
        }

        Ok((
            front_matter,
            lines
                .skip(front_matter_lines)
                .collect::<Vec<&str>>()
                .join("\n"),
        ))
    }
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, __private::doc};

    const MARKDOWN: &'static str = r#"
---
title: "Installing The Rust Programming Language on Windows"
description: "A tutorial on installing the Rust Programming Language on Windows."
categories: [rust, tutorial, windows, install]
date: 2021-09-13T03:48:00
---

# Installing The Rust Programming Language on Windows

## Motivation

In the past days I´ve been using Unix based systems to do my software
development work, macOS and Ubuntu are both my main operative systems nowadays.

But Windows is getting closer as well, as I get more involved into systems
programming, I'm also getting into writing Rust crates which must be supported in
different platforms, such as macOS, Linux and Windows.

Currently I'm working on a crate called [local-ip-address](https://github.com/EstebanBorai/local-ip-address).

The main goal of this crate is to list system's network interfaces along
with related data such as interface name, interface family (AFINET or AFINET6 for instance),
IP address, subnet mask and any other relevant properties.

Given that every system has a particular way to gather network interfaces
details, I decided to install Windows in my PC as a dual-boot option along with Ubuntu.

This will give me first-class access to the popular Win32 API, which I'm using through [windows-rs](https://github.com/microsoft/windows-rs) crate.

After having Windows up and running, I'm also installing Rust on Windows and I'm documenting
it for future references.
"#;

    const FRONT_MATTER: &'static str = r#"title: "Installing The Rust Programming Language on Windows"
description: "A tutorial on installing the Rust Programming Language on Windows."
categories: [rust, tutorial, windows, install]
date: 2021-09-13T03:48:00
"#;

    const CONTENT: &'static str = r#"
# Installing The Rust Programming Language on Windows

## Motivation

In the past days I´ve been using Unix based systems to do my software
development work, macOS and Ubuntu are both my main operative systems nowadays.

But Windows is getting closer as well, as I get more involved into systems
programming, I'm also getting into writing Rust crates which must be supported in
different platforms, such as macOS, Linux and Windows.

Currently I'm working on a crate called [local-ip-address](https://github.com/EstebanBorai/local-ip-address).

The main goal of this crate is to list system's network interfaces along
with related data such as interface name, interface family (AFINET or AFINET6 for instance),
IP address, subnet mask and any other relevant properties.

Given that every system has a particular way to gather network interfaces
details, I decided to install Windows in my PC as a dual-boot option along with Ubuntu.

This will give me first-class access to the popular Win32 API, which I'm using through [windows-rs](https://github.com/microsoft/windows-rs) crate.

After having Windows up and running, I'm also installing Rust on Windows and I'm documenting
it for future references."#;

    #[derive(Deserialize)]
    struct Metadata {
        title: String,
        description: String,
        categories: Vec<String>,
        date: String,
    }

    #[test]
    fn retrieve_markdown_front_matter() {
        let (front_matter, _) = super::YamlFrontMatter::extract(MARKDOWN).unwrap();

        assert_eq!(front_matter, FRONT_MATTER);
    }

    #[test]
    fn retrieve_markdown_content() {
        let (_, content) = super::YamlFrontMatter::extract(MARKDOWN).unwrap();

        assert_eq!(content, CONTENT);
    }

    #[test]
    fn parses_markdown_into_document() {
        let document = super::YamlFrontMatter::parse::<Metadata>(MARKDOWN).unwrap();
        let metadata = document.metadata;

        assert_eq!(
            metadata.title,
            "Installing The Rust Programming Language on Windows"
        );
        assert_eq!(
            metadata.description,
            "A tutorial on installing the Rust Programming Language on Windows."
        );
        assert_eq!(
            metadata.categories,
            vec!["rust", "tutorial", "windows", "install"]
        );
        assert_eq!(metadata.date, "2021-09-13T03:48:00");
    }
}

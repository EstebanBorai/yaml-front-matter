<div>
  <h1 align="center">yaml-front-matter</h1>
  <h4 align="center">YAML Front Matter (YFM) parser for Markdown files</h4>
</div>

<div align="center">

  [![Crates.io](https://img.shields.io/crates/v/yaml-front-matter.svg)](https://crates.io/crates/yaml-front-matter)
  [![Documentation](https://docs.rs/yaml-front-matter/badge.svg)](https://docs.rs/yaml-front-matter)
  ![Build](https://github.com/EstebanBorai/yaml-front-matter/workflows/build/badge.svg)
  ![Clippy](https://github.com/EstebanBorai/yaml-front-matter/workflows/clippy/badge.svg)
  ![Fmt](https://github.com/EstebanBorai/yaml-front-matter/workflows/fmt/badge.svg)
  ![Release](https://github.com/EstebanBorai/yaml-front-matter/workflows/release/badge.svg)
  ![Tests](https://github.com/EstebanBorai/yaml-front-matter/workflows/tests/badge.svg)

</div>

 # YAML Front Matter (YFM) Parser

 **yaml-front-matter** parses a valid YAML string into a `struct` which
 implements the `DeserializeOwned` trait from serde.

 Consider the following YAML content on the top of your markdown file:

 ```yml
 ---
 title: 'Parsing a Markdown file metadata into a struct'
 description: 'This tutorial walks you through the practice of parsing markdown files for metadata'
 tags: ['markdown', 'rust', 'files', 'parsing', 'metadata']
 similar_posts:
   - 'Rendering markdown'
   - 'Using Rust to render markdown'
 date: '2021-09-13T03:48:00'
 favorite_numbers:
     - 3.14
     - 1970
     - 12345
 ---
 ```

 This crate takes care of extracting this header from your markdown file and
 parse extracted data using `serde` and `serde_yaml`.

 ## Example

 ```rust
 use serde::Deserialize;
 use yaml_front_matter::YamlFrontMatter;

 const SIMPLE_MARKDOWN_YFM: &str = r#"
 ---
 title: 'Parsing a Markdown file metadata into a struct'
 description: 'This tutorial walks you through the practice of parsing markdown files for metadata'
 tags: ['markdown', 'rust', 'files', 'parsing', 'metadata']
 similar_posts:
   - 'Rendering markdown'
   - 'Using Rust to render markdown'
 date: '2021-09-13T03:48:00'
 favorite_numbers:
     - 3.14
     - 1970
     - 12345
 ---


 # Parsing a **Markdown** file metadata into a `struct`

 > This tutorial walks you through the practice of parsing markdown files for metadata
 "#;

 #[derive(Deserialize)]
 struct Metadata {
     title: String,
     description: String,
     tags: Vec<String>,
     similar_posts: Vec<String>,
     date: String,
     favorite_numbers: Vec<f64>,
 }

 let result = YamlFrontMatter::parse::<Metadata>(&SIMPLE_MARKDOWN_YFM).unwrap();

 let Metadata {
     title,
     description,
     tags,
     similar_posts,
     date,
     favorite_numbers,
 } = result;

 assert_eq!(title, "Parsing a Markdown file metadata into a struct");
 assert_eq!(
     description,
     "This tutorial walks you through the practice of parsing markdown files for metadata"
 );
 assert_eq!(
     tags,
     vec!["markdown", "rust", "files", "parsing", "metadata"]
 );
 assert_eq!(
     similar_posts,
     vec!["Rendering markdown", "Using Rust to render markdown"]
 );
 assert_eq!(date, "2021-09-13T03:48:00");
 assert_eq!(favorite_numbers, vec![3.14, 1970., 12345.]);
 ```

use std::{
    collections::HashMap,
    error::Error,
    fs::{self},
    rc::Rc,
};

use chrono::DateTime;
use regex::Regex;
use serde::Serialize;
use serde_json::Value;

use crate::util::get_path_by_key;

pub struct DocumentOptions {
    pub with_content: bool,
    pub with_summary: bool,
}

#[derive(Debug, Serialize)]
pub struct Document {
    pub key: String,
    pub frontmatter: Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    #[serde(skip_serializing)]
    pub date: i64,
}

impl Document {
    pub fn from_key(
        key: &str,
        base_path: &str,
        options: &DocumentOptions,
    ) -> Result<Document, Box<dyn Error>> {
        // Read file and convert to LF.
        let data = fs::read_to_string(get_path_by_key(key, base_path)?)?.replace("\r\n", "\n");

        let re = Regex::new(r"(?ms)^---$\n(.*?)\n^---$\n*(.*)").unwrap();
        let caps = re.captures(&data).ok_or("invalid document structure")?;

        let frontmatter = caps.get(1).unwrap().as_str();
        let frontmatter: Value = serde_yaml::from_str(frontmatter)?;
        let content = caps.get(2).unwrap().as_str().to_string();

        let date = match frontmatter.get("date") {
            Some(d) => {
                DateTime::parse_from_rfc3339(d.as_str().ok_or("invalid frontmatter date type")?)?
                    .timestamp()
            }
            // Set to MAX to pin document.
            None => i64::MAX,
        };

        let mut doc_content: Option<String> = None;
        let mut doc_summary: Option<String> = None;

        if options.with_content {
            doc_content = Some(content.to_owned());
        }

        if options.with_summary {
            doc_summary = Some(extract_summary(&content).to_string());
        }

        Ok(Document {
            key: key.to_string(),
            frontmatter,
            date,
            content: doc_content,
            summary: doc_summary,
        })
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct DocumentCachePool {
    cache: HashMap<String, Rc<Document>>,
}

#[allow(unused)]
impl DocumentCachePool {
    pub fn new() -> DocumentCachePool {
        DocumentCachePool {
            cache: HashMap::new(),
        }
    }

    pub fn get_document(
        &mut self,
        key: &str,
        base_path: &str,
        options: &DocumentOptions,
    ) -> Result<Rc<Document>, Box<dyn Error>> {
        if self.cache.contains_key(key) {
            Ok(self.cache.get(key).unwrap().clone())
        } else {
            let doc = Rc::new(Document::from_key(key, base_path, options)?);
            self.cache.insert(key.to_string(), doc.clone());
            Ok(doc)
        }
    }
}

fn extract_summary(content: &str) -> &str {
    let re = Regex::new(r"(?s)^(.*?)<!--more-->").unwrap();
    let caps = re.captures(content);

    match caps {
        Some(caps) => caps.get(1).unwrap().as_str().trim_end(),
        None => content,
    }
}

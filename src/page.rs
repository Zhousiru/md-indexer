use std::{cmp::Reverse, rc::Rc, vec};

use crate::document::Document;

#[derive(Debug)]
pub struct Page {
    pub documents: Vec<Rc<Document>>,
}

impl Page {
    pub fn new() -> Page {
        Page {
            documents: Vec::new(),
        }
    }

    pub fn add_document(&mut self, document: Rc<Document>) {
        self.documents.push(document)
    }

    pub fn merge_page(&mut self, page: &Page) {
        self.documents.extend(page.documents.to_owned().into_iter())
    }

    pub fn generate_page_documents(&self, page_size: usize) -> Vec<Vec<Rc<Document>>> {
        let mut sorted = self.documents.to_owned();
        sorted.sort_by_key(|d| (Reverse(d.date), d.key.to_owned()));

        let mut page_documents: Vec<Vec<Rc<Document>>> = vec![Vec::new()];
        let mut current_count = 0;

        for document in sorted {
            if current_count >= page_size {
                current_count = 0;
                page_documents.push(Vec::new());
            }

            page_documents.last_mut().unwrap().push(document);

            current_count += 1;
        }

        page_documents
    }
}

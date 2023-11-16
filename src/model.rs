use std::rc::Rc;

use serde::Serialize;

use crate::document::Document;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SinglePageModel<'a> {
    pub documents: &'a Vec<Rc<Document>>,
    pub page_count: usize,
    pub map: &'a Vec<&'a str>,
}

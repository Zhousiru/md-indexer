use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
    rc::Rc,
};

use crate::{
    document::{Document, DocumentOptions},
    model::SinglePageModel,
    page::Page,
    util::{get_key, get_path_by_key},
};

pub fn generate_pages_from_path(
    key: &str,
    base_path: &str,
    options: &DocumentOptions,
) -> Result<HashMap<String, Page>, Box<dyn Error>> {
    let path = get_path_by_key(key, base_path)?;
    let dir = fs::read_dir(path).unwrap();

    let mut result: HashMap<String, Page> = HashMap::new();
    let mut current_page = Page::new();

    for item in dir {
        let item = item?;
        let file_type = item.file_type()?;
        let item_key = get_key(item.path().to_str().ok_or("invalid item path")?, base_path)?;

        if file_type.is_dir() {
            let sub_page_map = generate_pages_from_path(&item_key, base_path, options)?;

            let sub_page = sub_page_map
                .get(&item_key)
                .ok_or("failed to get sub page from map")?;
            current_page.merge_page(sub_page);

            result.extend(sub_page_map.into_iter());

            continue;
        }

        if !item_key.ends_with(".md") {
            // Skip non-Markdown files.
            continue;
        }

        let doc = Rc::new(Document::from_key(&item_key, base_path, options)?);

        current_page.add_document(doc);
    }

    result.insert(key.to_string(), current_page);

    Ok(result)
}

pub fn write_pages_to_path(
    pages: &HashMap<String, Page>,
    output_path: &str,
    page_size: usize,
) -> Result<(), Box<dyn Error>> {
    let mut page_map: Vec<&str> = Vec::new();
    for p in pages {
        page_map.push(p.0)
    }

    for (key, page) in pages {
        let page_base_path = get_path_by_key(key, output_path)?;
        fs::create_dir_all(&page_base_path)?;

        let paginated_documents = page.generate_page_documents(page_size);

        for (count, documents) in paginated_documents.iter().enumerate() {
            let page_path = page_base_path.join(format!("page-{}.json", count + 1));
            let file = File::create(page_path)?;

            serde_json::to_writer(
                file,
                &SinglePageModel {
                    documents,
                    page_count: paginated_documents.len(),
                    map: &page_map,
                },
            )?;
        }
    }

    Ok(())
}

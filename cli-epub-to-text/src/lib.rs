use epub::doc::EpubDoc;
use html2text;

pub fn epub_to_text(
  file_path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
  let mut epub = EpubDoc::new(file_path)?;

  let mut string_builder = vec![];
  for spine_item in epub.spine.clone() {
    let xhtml = epub.get_resource(&spine_item).unwrap().0;
    let text = html2text::from_read(&*xhtml, 110)?;
    string_builder.push(text);
  }

  Ok(string_builder.join("\n"))
}

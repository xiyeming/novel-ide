use crate::error::AppError;
use std::io::Write;
use zip::write::SimpleFileOptions;

pub fn export_epub(content: &str, output_path: &str, title: &str, author: &str) -> Result<(), AppError> {
    let file = std::fs::File::create(output_path)?;
    let mut zip = zip::ZipWriter::new(file);

    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    // 1. mimetype (uncompressed)
    let mimetype_options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored);
    zip.start_file("mimetype", mimetype_options)?;
    zip.write_all(b"application/epub+zip")?;

    // 2. META-INF/container.xml
    zip.start_file("META-INF/container.xml", options.clone())?;
    zip.write_all(CONTAINER_XML.as_bytes())?;

    // 3. OEBPS/content.opf
    zip.start_file("OEBPS/content.opf", options.clone())?;
    zip.write_all(content_opf(title, author).as_bytes())?;

    // 4. OEBPS/chapter.xhtml
    zip.start_file("OEBPS/chapter.xhtml", options.clone())?;
    zip.write_all(chapter_xhtml(title, content).as_bytes())?;

    zip.finish()?;
    Ok(())
}

fn content_opf(title: &str, author: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://www.idpf.org/2007/opf" unique-identifier="bookid" version="3.0">
  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
    <dc:title>{title}</dc:title>
    <dc:creator>{author}</dc:creator>
    <dc:identifier id="bookid">urn:uuid:{uuid}</dc:identifier>
    <dc:language>zh-CN</dc:language>
    <meta property="dcterms:modified">{modified}</meta>
  </metadata>
  <manifest>
    <item id="chapter" href="chapter.xhtml" media-type="application/xhtml+xml" properties="nav"/>
  </manifest>
  <spine>
    <itemref idref="chapter"/>
  </spine>
</package>"#,
        title = xml_escape(title),
        author = xml_escape(author),
        uuid = uuid::Uuid::new_v4(),
        modified = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ"),
    )
}

fn chapter_xhtml(title: &str, content: &str) -> String {
    let paragraphs: String = content
        .lines()
        .map(|line| format!("<p>{}</p>", xml_escape(line)))
        .collect::<Vec<_>>()
        .join("\n    ");

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
  <title>{title}</title>
</head>
<body>
  <h1>{title}</h1>
  {paragraphs}
</body>
</html>"#,
        title = xml_escape(title),
        paragraphs = paragraphs,
    )
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&apos;")
}

const CONTAINER_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<container xmlns="urn:oasis:names:tc:opendocument:xmlns:container" version="1.0">
  <rootfiles>
    <rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/>
  </rootfiles>
</container>"#;

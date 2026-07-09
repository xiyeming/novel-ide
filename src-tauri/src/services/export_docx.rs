use crate::error::AppError;
use std::fs::File;
use std::io::Write;

pub fn export_docx(content: &str, output_path: &str, title: &str) -> Result<(), AppError> {
    let docx_xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:r>
        <w:t>{}</w:t>
      </w:r>
    </w:p>
    <w:sectPr>
      <w:pgSz w:w="11906" w:h="16838"/>
    </w:sectPr>
  </w:body>
</w:document>"#,
        xml_escape(content)
    );

    let mut file = File::create(output_path)?;
    file.write_all(docx_xml.as_bytes())?;

    Ok(())
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&apos;")
}

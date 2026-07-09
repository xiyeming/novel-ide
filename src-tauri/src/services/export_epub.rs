use crate::error::AppError;
use std::fs::File;
use std::io::Write;

pub fn export_epub(content: &str, output_path: &str, title: &str, author: &str) -> Result<(), AppError> {
    let html_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
    <title>{}</title>
</head>
<body>
<h1>{}</h1>
<div>{}</div>
</body>
</html>"#,
        xml_escape(title),
        xml_escape(title),
        content.lines()
            .map(|line| format!("<p>{}</p>", xml_escape(line)))
            .collect::<Vec<_>>()
            .join("\n")
    );

    let mut file = File::create(output_path)?;
    file.write_all(html_content.as_bytes())?;

    Ok(())
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&apos;")
}

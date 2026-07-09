use crate::error::AppError;
use std::fs::File;
use std::io::Write;

fn escape_pdf_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '\\' => result.push_str("\\\\"),
            '(' => result.push_str("\\("),
            ')' => result.push_str("\\)"),
            _ => result.push(ch),
        }
    }
    result
}

fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        if paragraph.is_empty() {
            lines.push(String::new());
            continue;
        }
        let mut remaining = paragraph;
        while remaining.len() > max_chars {
            // Find a break point near max_chars
            let break_at = remaining[..max_chars]
                .rfind(' ')
                .unwrap_or(max_chars);
            lines.push(remaining[..break_at].to_string());
            remaining = &remaining[break_at..].trim_start();
        }
        if !remaining.is_empty() {
            lines.push(remaining.to_string());
        }
    }
    lines
}

pub fn export_pdf(content: &str, output_path: &str, title: &str) -> Result<(), AppError> {
    let escaped_title = escape_pdf_string(title);
    let wrapped_lines = wrap_text(content, 80);

    // Build the stream content (text operators)
    let mut stream_content = format!(
        "BT\n/F1 14 Tf\n50 750 Td\n({}) Tj\nET\n",
        escaped_title
    );
    // Add content lines below the title
    let mut y = 720.0;
    for line in &wrapped_lines {
        if y < 50.0 {
            break;
        }
        let escaped_line = escape_pdf_string(line);
        stream_content.push_str(&format!(
            "BT\n/F1 11 Tf\n50 {:.1} Td\n({}) Tj\nET\n",
            y, escaped_line
        ));
        y -= 14.0;
    }

    let stream_bytes = stream_content.as_bytes();
    let stream_len = stream_bytes.len();

    // Object 4 content stream with placeholder length
    let obj4_header = format!("4 0 obj\n<< /Length {} >>\nstream\n", stream_len);

    // Build the full PDF, tracking byte offsets
    let mut pdf = Vec::new();

    // Obj 1: Catalog
    let obj1_offset = pdf.len();
    pdf.extend_from_slice(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n");

    // Obj 2: Pages
    let obj2_offset = pdf.len();
    pdf.extend_from_slice(b"2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n");

    // Obj 3: Page
    let obj3_offset = pdf.len();
    pdf.extend_from_slice(b"3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>\nendobj\n");

    // Obj 4: Content stream
    let obj4_offset = pdf.len();
    pdf.extend_from_slice(obj4_header.as_bytes());
    pdf.extend_from_slice(stream_bytes);
    pdf.extend_from_slice(b"\nendstream\nendobj\n");

    // Obj 5: Font
    let obj5_offset = pdf.len();
    pdf.extend_from_slice(b"5 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>\nendobj\n");

    // xref
    let xref_offset = pdf.len();
    pdf.extend_from_slice(b"xref\n0 6\n");
    pdf.extend_from_slice(b"0000000000 65535 f \n");
    pdf.extend_from_slice(format!("{:010} 00000 n \n", obj1_offset).as_bytes());
    pdf.extend_from_slice(format!("{:010} 00000 n \n", obj2_offset).as_bytes());
    pdf.extend_from_slice(format!("{:010} 00000 n \n", obj3_offset).as_bytes());
    pdf.extend_from_slice(format!("{:010} 00000 n \n", obj4_offset).as_bytes());
    pdf.extend_from_slice(format!("{:010} 00000 n \n", obj5_offset).as_bytes());

    // trailer
    pdf.extend_from_slice(b"trailer\n<< /Size 6 /Root 1 0 R >>\nstartxref\n");
    pdf.extend_from_slice(format!("{}\n", xref_offset).as_bytes());
    pdf.extend_from_slice(b"%%EOF");

    let mut file = File::create(output_path)?;
    file.write_all(&pdf)?;

    Ok(())
}

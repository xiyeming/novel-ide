use crate::error::AppError;
use std::fs::File;
use std::io::Write;

pub fn export_pdf(content: &str, output_path: &str, title: &str) -> Result<(), AppError> {
    let pdf_content = format!(
        r#"%PDF-1.4
1 0 obj
<< /Type /Catalog /Pages 2 0 R >>
endobj
2 0 obj
<< /Type /Pages /Kids [3 0 R] /Count 1 >>
endobj
3 0 obj
<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>
endobj
4 0 obj
<< /Length 44 >>
stream
BT
/F1 12 Tf
100 700 Td
({}) Tj
ET
endstream
endobj
5 0 obj
<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>
endobj
xref
0 6
0000000000 65535 f 
0000000009 00000 n 
0000000058 00000 n 
0000000115 00000 n 
0000000266 00000 n 
0000000340 00000 n 
trailer
<< /Size 6 /Root 1 0 R >>
startxref
415
%%EOF"#,
        title.replace('\\', "\\\\").replace('(', "\\(").replace(')', "\\)")
    );

    let mut file = File::create(output_path)?;
    file.write_all(pdf_content.as_bytes())?;

    Ok(())
}

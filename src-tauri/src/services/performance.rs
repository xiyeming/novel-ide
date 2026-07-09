use crate::error::AppError;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct PerformanceService;

impl PerformanceService {
    pub fn new() -> Self {
        Self
    }

    pub fn count_lines(&self, file_path: &str) -> Result<usize, AppError> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let line_count = reader.lines().count();
        Ok(line_count)
    }

    pub fn read_file_chunk(
        &self,
        file_path: &str,
        start_line: usize,
        max_lines: usize,
    ) -> Result<String, AppError> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut content = String::new();
        let mut line_count = 0;

        for line in reader.lines() {
            if line_count >= start_line && line_count < start_line + max_lines {
                content.push_str(&line?);
                content.push('\n');
            }
            line_count += 1;

            if line_count >= start_line + max_lines {
                break;
            }
        }

        Ok(content)
    }

    pub fn get_file_size(&self, file_path: &str) -> Result<u64, AppError> {
        let metadata = std::fs::metadata(file_path)?;
        Ok(metadata.len())
    }

    pub fn optimize_content(&self, content: &str) -> String {
        content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

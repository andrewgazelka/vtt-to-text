use std::env;

use anyhow::Result;
use regex::Regex;

fn remove_tags(text: &str) -> String {
    let tags = [r"</c>", r"<c(\.color\w+)?>", r"<\d{2}:\d{2}:\d{2}\.\d{3}>"];

    let mut text = text.to_string();
    for pat in &tags {
        let re = Regex::new(pat).unwrap();
        text = re.replace_all(&text, "").to_string();
    }

    let re = Regex::new(r"(\d{2}:\d{2}):\d{2}\.\d{3} --> .* align:start position:0%").unwrap();
    text = re.replace_all(&text, "").to_string();

    let re = Regex::new(r"^\s+$").unwrap();
    text = re.replace_all(&text, "").to_string();

    text
}

fn remove_header(lines: &[String]) -> Vec<String> {
    let pos = lines
        .iter()
        .position(|x| x == "##" || x == "Language: en")
        .unwrap_or(0);
    lines[pos + 1..].to_vec()
}

fn merge_duplicates(lines: Vec<String>) -> Vec<String> {
    let mut last_timestamp = String::new();
    let mut last_cap = String::new();
    let mut result = vec![];

    for line in lines {
        if line.is_empty() {
            continue;
        }
        if Regex::new(r"^\d{2}:\d{2}$").unwrap().is_match(&line) {
            if line != last_timestamp {
                result.push(line.clone());
                last_timestamp = line;
            }
        } else if line != last_cap {
            result.push(line.clone());
            last_cap = line;
        }
    }

    result
}

fn merge_short_lines(lines: Vec<String>) -> Vec<String> {
    let mut buffer = String::new();
    let mut result = vec![];

    for line in lines {
        if line.is_empty() || Regex::new(r"^\d{2}:\d{2}$").unwrap().is_match(&line) {
            result.push(format!("\n{line}"));
            continue;
        }

        if (line.len() + buffer.len()) < 80 {
            buffer.push_str(&format!(" {line}"));
        } else {
            result.push(buffer.trim().to_string());
            buffer = line;
        }
    }

    result.push(buffer);
    result
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let vtt_file_name = &args[1];

    let text = std::fs::read_to_string(vtt_file_name)?;
    let text = remove_tags(&text);
    let lines: Vec<String> = text.lines().map(ToString::to_string).collect();
    let lines = remove_header(&lines);
    let lines = merge_duplicates(lines);
    let lines = merge_short_lines(lines);

    let result = lines.join(" ").replace("\n ", " ");

    println!("{}", result.trim());

    Ok(())
}

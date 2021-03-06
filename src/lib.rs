use std::path::Path;
use std::path::PathBuf;

pub type WebError = Box<dyn std::error::Error>;

pub fn get_destiny(from_url: &str, to_root: impl AsRef<Path>) -> PathBuf {
    let start = from_url.find("://").unwrap();
    let url = &from_url[start+3..];
    let mut result = to_root.as_ref().to_owned();
    for part in url.split("/") {
        let part = part
            .replace("<", "_")
            .replace(">", "_")
            .replace(":", "_")
            .replace("\"", "_")
            .replace("\\", "_")
            .replace("|", "_")
            .replace("?", "_")
            .replace("*", "_");
        result = result.join(part);
    }
    result
}

pub async fn make(from_url: &str, to_root: impl AsRef<Path>) -> Result<(), WebError> {
    let destiny = get_destiny(from_url, to_root);
    std::fs::create_dir_all(&destiny)?;
    let res = reqwest::get(from_url).await?;
    let doc = res.text().await?;
    let mut result = String::new();
    let mut on_bracket = false;
    let mut on_bracket_from: usize = 0;
    let mut on_bracket_opened_single_quote = false;
    let mut on_bracket_opened_double_quote = false;
    let mut on_bracket_tree: Vec<&str> = Vec::new();
    let mut on_content_from: usize = 0;
    for indexed_char in doc.char_indices() {
        let index = indexed_char.0;
        let actual = indexed_char.1;
        if on_bracket {
            if on_bracket_opened_single_quote {
                if actual == '\'' {
                    on_bracket_opened_single_quote = false;
                }
            } else if on_bracket_opened_double_quote {
                if actual == '"' {
                    on_bracket_opened_double_quote = false;
                }
            } else {
                if actual == '\'' {
                    on_bracket_opened_single_quote = true;
                } else if actual == '"' {
                    on_bracket_opened_double_quote = true;
                } else if actual == '>' {
                    on_bracket = false;
                    on_content_from = index + 1;
                    let bracket = &doc[on_bracket_from..on_content_from];
                    on_bracket_tree.push(bracket);
                    proc_bracket(bracket, &on_bracket_tree, &mut result);
                }
            }
        } else {
            if actual == '<' {
                if index > on_content_from {
                    let content = &doc[on_content_from..index];
                    proc_content(content, &mut result);
                }
                on_bracket = true;
                on_bracket_from = index;
                on_bracket_opened_single_quote = false;
                on_bracket_opened_double_quote = false;
            }
        }
    }
    std::fs::write(&destiny.join("contents.md"), result)?;
    Ok(())
}

fn proc_bracket(bracket: &str, _tree: &Vec<&str>, result: &mut String) {
    result.push_str("x----------------------------------------------------x\n");
    result.push_str("Bracket: ");
    result.push_str(bracket);
    result.push_str("\n");
}

fn proc_content(content: &str, result: &mut String) {
    result.push_str("x----------------------------------------------------x\n");
    result.push_str("Content: ");
    result.push_str(content);
    result.push_str("\n");
}




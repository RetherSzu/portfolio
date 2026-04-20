use gray_matter::engine::YAML;
use gray_matter::Matter;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct FrontMatter {
    id: usize,
    title: String,
    description: String,
    date: String,
}

fn main() {
    println!("cargo:rerun-if-changed=content/logs");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_logs.rs");

    let mut generated_code = String::from("pub const LOGS: &[LogD] = &[\n");
    let content_dir = Path::new("content/logs");

    let matter = Matter::<YAML>::new();

    if content_dir.exists() {
        for entry in fs::read_dir(content_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                let file_content = fs::read_to_string(&path).unwrap();

                match matter.parse::<FrontMatter>(&file_content) {
                    Ok(parsed_file) => {
                        if let Some(frontmatter) = parsed_file.data {
                            let markdown_content = parsed_file.content;

                            generated_code.push_str(&format!(
                                "    LogD {{ id: {:?}, title: {:?}, description: {:?}, date: {:?}, content: {:?} }},\n",
                                frontmatter.id.to_string(),
                                frontmatter.title,
                                frontmatter.description,
                                frontmatter.date,
                                markdown_content
                            ));
                            generated_code.push('\n');
                        }
                    }
                    Err(error) => {
                        println!(
                            "cargo:warning=Error when parsing {:?} : {:?}",
                            path, error
                        );
                    }
                }
            }
        }
    }

    generated_code.push_str("];\n");
    fs::write(&dest_path, generated_code).unwrap();
}

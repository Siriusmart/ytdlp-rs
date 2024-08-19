use std::{cell::OnceCell, cmp, env, fs, io::Write, process::Command};

#[derive(Debug)]
pub struct Section {
    label: String,
    options: Vec<Opt>,
}

impl ToString for Section {
    fn to_string(&self) -> String {
        format!(
            r#"/// {}
impl Builder {{
{}
}}"#,
            self.label,
            self.options
                .iter()
                .map(|opt| opt.to_string())
                .collect::<Vec<_>>()
                .join("\n\n")
        )
    }
}

#[derive(Debug)]
pub struct Opt {
    option: String,
    arg: Option<String>,
    desc: String,
}

impl ToString for Opt {
    fn to_string(&self) -> String {
        let arg_label = self.arg.as_ref().map(|arg| {
            arg.chars()
                .filter_map(|c| c.is_ascii_alphabetic().then(|| c.to_ascii_lowercase()))
                .collect::<String>()
        });

        let name = match self.option.replace("-", "_") {
            s if s == "continue".to_string() => "r#continue".to_string(),
            s => s,
        };

        format!(
            r#"    /// {}
    pub fn {}(&mut self{}) -> &mut Self {{
        self.command_mut().arg("--{}");{}
        self
    }}"#,
            self.desc,
            name,
            arg_label
                .as_ref()
                .map(|arg| format!(", {arg}: &str"))
                .unwrap_or(String::new()),
            self.option,
            arg_label
                .as_ref()
                .map(|arg| format!("\n        self.command_mut().arg({arg});"))
                .unwrap_or(String::new()),
        )
    }
}

impl Section {
    pub fn new(label: String) -> Self {
        Self {
            label,
            options: Vec::new(),
        }
    }

    pub fn push(&mut self, opt: Opt) {
        self.options.push(opt)
    }

    pub fn push_desc(&mut self, s: &str) {
        self.options.last_mut().unwrap().push_desc(s)
    }

    pub fn is_empty(&self) -> bool {
        self.options.is_empty()
    }

    pub fn push_section_desc(&mut self, s: &str) {
        self.label.push(' ');
        self.label.push_str(s)
    }
}

impl Opt {
    pub fn new(option: String, arg: Option<String>, desc: String) -> Self {
        Self { option, arg, desc }
    }

    pub fn push_desc(&mut self, s: &str) {
        if !self.desc.ends_with(" ") {
            self.desc.push(' ');
        }
        self.desc.push_str(s.trim())
    }
}

fn main() {
    let output = String::from_utf8(
        Command::new(env::args().skip(1).next().unwrap_or("yt-dlp".to_string()))
            .arg("--help")
            .output()
            .expect("failed to run yt-dlp")
            .stdout,
    )
    .expect("failed to decode command output");

    let mut sections = Vec::new();

    let mut current_section = None;
    let desc_indentation = OnceCell::new();

    for line in output.lines() {
        if line.starts_with("  ") && line.ends_with("Options:") {
            if let Some(section) = current_section {
                sections.push(section)
            }

            current_section = Some(Section::new(line[..line.len() - 1].trim().to_string()));
        } else if line.starts_with("    ") {
            let opt = if let Some(indent) = desc_indentation.get() {
                let mut index = cmp::min(*indent, line.len() - 1);
                if &line[index - 1..index] != " " {
                    index = line.len()
                }

                let spec = line[..index].trim().split(' ').collect::<Vec<_>>();
                let desc = line[index..].trim().to_string();

                // disable that command
                if spec[0] == "--alias" {
                    continue;
                }

                let label = spec
                    .iter()
                    .find_map(|s| s.starts_with("--").then(|| s[2..].to_string()));

                if label.is_none() {
                    if current_section.as_ref().unwrap().is_empty() {
                        current_section.as_mut().unwrap().push_section_desc(&desc);
                    } else {
                        current_section.as_mut().unwrap().push_desc(&desc);
                    }
                    continue;
                }

                let arg = if spec.last().unwrap().starts_with("--") {
                    None
                } else {
                    Some(spec.last().unwrap().to_string())
                };

                Opt::new(label.unwrap(), arg, desc)
            } else {
                let mut in_desc = false;
                let mut opt_label = String::new();
                let mut consecutive_dashes = 0;
                let mut desc = String::new();

                for (i, c) in line.chars().enumerate() {
                    if !in_desc {
                        if c == '-' {
                            consecutive_dashes += 1;
                            continue;
                        } else if consecutive_dashes < 2 {
                            consecutive_dashes = 0;
                        }
                    }

                    if consecutive_dashes >= 2 && !in_desc {
                        if c == ' ' {
                            in_desc = true;
                        } else {
                            opt_label.push(c)
                        }

                        continue;
                    }

                    if in_desc {
                        if desc.is_empty() && c == ' ' {
                            continue;
                        } else {
                            if desc.is_empty() {
                                desc_indentation.set(i).unwrap();
                            }

                            desc.push(c)
                        }
                    }
                }

                // the first command in help is -h
                Opt::new(opt_label, None, desc)
            };

            current_section.as_mut().expect("there are double indented before the options, the help message might have changed, please update it or open an issue on github").push(opt);
        }
    }

    let out = sections
        .iter()
        .map(|block| block.to_string())
        .collect::<Vec<_>>()
        .join("\n\n");

    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("../src/command/options.rs")
        .unwrap();

    file.write_all("use super::Builder;\n\n".as_bytes())
        .unwrap();
    file.write_all(out.as_bytes()).unwrap();
}

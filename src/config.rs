use std::collections::HashMap;
use std::path::{PathBuf, Path};
use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub aliases: HashMap<String, PathBuf>,
    pub editor: EditorData,
    pub sets: HashMap<String, Vec<PathBuf>>,
    pub settings: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EditorData {
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RepoData {
    pub repo: Data,
}

#[derive(Debug)]
pub struct Config {
    data: RepoData,
}

fn default_data() -> RepoData {
    return RepoData {
        repo: Data {
            aliases: HashMap::new(),
            editor: EditorData {
                command: None,
                args: None,
            },
            sets: HashMap::new(),
            settings: HashMap::new(),
        }
    };
}

#[derive(Debug)]
pub enum TargetType {
    Alias(PathBuf),
    Dir(PathBuf),
    Path(PathBuf),
    Set(Vec<PathBuf>),
}

impl Config {
    pub fn get_editor(&self) -> String {
        if let Some(command) = &self.data.repo.editor.command {
            return command.to_string();
        }

        let editor = std::env::var("EDITOR").expect("No $EDITOR set");
        return editor;
    }

    pub fn get_editor_args(&self, target: &TargetType) -> Vec<String> {
        let mut args: Vec<String> = vec![];
        if let Some(a) = &self.data.repo.editor.args {
            args = a.to_vec();
        }

        let target_path: &PathBuf = match target {
            TargetType::Alias(path) => {
                path
            },
            TargetType::Path(path) => {
                path
            },
            TargetType::Set(paths) => {
                // we only support one path for right now
                paths.first().expect("expected at least one path in set")
            },
            TargetType::Dir(path) => {
                path
            },
        };

        // Replace {{target}} with actual path
        let regex = Regex::new(r"(?<target>\{\{target\}\})").unwrap();
        return args.iter().map(|arg| {
            return regex.replace_all(arg, target_path.to_string_lossy()).to_string();
        }).collect();
    }

    fn get_path_for_alias(&self, alias: &String) -> Option<&PathBuf> {
        return self.data.repo.aliases.get(alias);
    }

    pub fn get_paths_for_target(&self, target: &String) -> Vec<PathBuf> {
        let t = self.get_target_type(&target);

        return match t {
            TargetType::Alias(path) => {
                vec![path]
            },
            TargetType::Path(path) => {
                vec![path]
            },
            TargetType::Set(paths) => {
                paths
            },
            TargetType::Dir(path) => {
                vec![path]
            },
        };
    }

    pub fn get_target_type(&self, target: &String) -> TargetType {
        // Alias
        if let Some(path) = self.get_path_for_alias(target) {
            return TargetType::Alias(path.to_path_buf()); 
        }

        // Path
        if let Some(_) = target.find("/") {
            return TargetType::Path(PathBuf::from(target));
        } 

        // Set
        if let Some(paths) = self.get_paths_for_set(target) {
            return TargetType::Set(paths.to_vec());
        }

        // Dir
        let default_root = "~/repos".to_string();
        let root = self.data.repo.settings.get("root").unwrap_or(&default_root);
        let path = Path::new(root).join(target);
        return TargetType::Dir(path);
    }

    pub fn get_paths_for_set(&self, target: &str) -> Option<&Vec<PathBuf>> {
        return self.data.repo.sets.get(target);
    }

    pub fn from_path_or_default(path: PathBuf) -> Self {
        if std::fs::metadata(&path).is_ok() {
            let contents = std::fs::read_to_string(&path);
            let contents = contents.unwrap_or("{\"config\":{}}".into());
            let data = serde_json::from_str(&contents);
            let data = data.unwrap_or(default_data());

            return Config {
                data,
            };
        }

        Config {
            data: default_data(),
        }
    }
}


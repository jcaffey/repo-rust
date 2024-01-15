use std::{collections::HashMap, str::FromStr};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub sets: HashMap<String, Vec<PathBuf>>
}

#[derive(Debug, Serialize, Deserialize)]
struct RepoData {
    pub repo: Data
}

#[derive(Debug)]
pub struct Config {
    data: RepoData,
}

fn default_data() -> RepoData {
    return RepoData {
        repo: Data {
            sets: HashMap::new(),
        }
    };
}

impl Config {
    pub fn paths_for_target(&self, target: &str) -> Vec<PathBuf> {
        let entry = self.data.repo.sets.get(target);
        let default: Vec<PathBuf> = vec![PathBuf::from(target)];
        return entry.unwrap_or(&default).to_vec();
    }

    pub fn from_path_or_default(path: PathBuf) -> Self {
        if std::fs::metadata(&path).is_ok() {
            let contents = std::fs::read_to_string(&path);
            let contents = contents.unwrap_or("{\"config\":{}}".into());
            let data = serde_json::from_str(&contents);
            println!("data: {:?}", data);
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


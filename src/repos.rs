use git2::Repository;
use std::{
    collections::{HashMap, VecDeque},
    fs,
    path::PathBuf,
};

pub(crate) fn get_repos(
    paths: Vec<PathBuf>,
    exclude: Vec<PathBuf>,
    full_path: Option<bool>,
) -> HashMap<String, Repository> {
    let mut repos = HashMap::new();
    let mut to_search = VecDeque::new();

    paths
        .iter()
        .for_each(|path| to_search.push_back(std::path::PathBuf::from(path)));

    // FIX: exclude full path or just file name?
    while let Some(file) = to_search.pop_front() {
        let file_name = file.file_name().unwrap().to_str().unwrap();
        if !exclude.contains(&file_name.into()) {
            if let Ok(repo) = git2::Repository::open(file.clone()) {
                let name = match full_path {
                    Some(true) => file.to_string_lossy().to_string(),
                    _ => file_name.to_string(),
                };
                repos.insert(name, repo);
            } else if file.is_dir() {
                to_search.extend(fs::read_dir(file).unwrap().map(|p| p.unwrap().path()));
            }
        }
    }

    repos
}


use git2::Repository;
use std::path::PathBuf;
use crate::config_file::GlobalConfig;

pub fn install(repo_url: &str, path: &PathBuf) -> Result<(), &'static str> {

    let mut config = GlobalConfig::load(path)?;

    // Get Repo name
    let repo_name = repo_url
        .trim_end_matches(".git")
        .split(&['/', ':'][..])
        .last()
        .unwrap();

    let into = path.join("texts").join(repo_name);

    // Attempt to clone the repository
    match Repository::clone(repo_url, into) {
        Err(_) => {return Err("Unable to download package")},
        _ => { },
    }
    // If cloned successfully
    config.priority.push(repo_name.to_string());
    config.store(path)?;

    Ok(())
}

pub fn remove(pkg_name: &str, path: &PathBuf) -> Result<(), &'static str> {
    let mut config = GlobalConfig::load(path)?;

    if !config.priority.contains(&pkg_name.to_string()) {
        return Err("Package not found");
    }
    config.priority.retain(|x| *x != pkg_name);
    config.store(path)?;

    match std::fs::remove_dir_all(path.join("texts").join(pkg_name)) {
        Err(_) => { return Err("Could not remove directory") },
        _ => {}
    }
    Ok(())
}

pub fn list(path: &PathBuf) -> Result<Vec<String>, &'static str> {
    let config = GlobalConfig::load(path)?;
    Ok(config.priority)
}


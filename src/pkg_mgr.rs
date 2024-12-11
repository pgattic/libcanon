use git2::CertificateCheckStatus;
use git2::RemoteCallbacks;
use std::path::PathBuf;
use crate::config_file::GlobalConfig;

/// Gets the available catalogue of downloadable packages.
/// For now, just returns a predefined list.
pub fn get_catalogue() -> Vec<(String, String)> {
    vec![
        ("ot-kjv-canon".to_string(), "https://github.com/pgattic/ot-kjv-canon.git".to_string()),
        ("nt-kjv-canon".to_string(), "https://github.com/pgattic/nt-kjv-canon.git".to_string()),
        ("bom-canon".to_string(), "https://github.com/pgattic/bom-canon.git".to_string()),
        ("dac-canon".to_string(), "https://github.com/pgattic/dac-canon.git".to_string()),
        ("pogp-canon".to_string(), "https://github.com/pgattic/pogp-canon.git".to_string()),
    ]
}

pub fn install(repo_url: &str, path: &PathBuf) -> Result<(), String> {

    let mut config = GlobalConfig::load(path)?;

    // Get Repo name
    let repo_name = repo_url
        .trim_end_matches(".git")
        .split(&['/', ':'][..])
        .last()
        .unwrap();

    let into = path.join(repo_name);

    // Attempt to clone the repository
    unsafe { let _ = git2::opts::set_verify_owner_validation(false); };

    // FIXME Clone the repo WITHOUT SSL CERT CHECKING!!!
    let mut callbacks = RemoteCallbacks::new();
    callbacks.certificate_check(|_, _| Ok(CertificateCheckStatus::CertificateOk));
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);
    match builder.clone(repo_url, &into) {
        Err(asdf) => {return Err(format!("{:?}", asdf))},
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

    match std::fs::remove_dir_all(path.join(pkg_name)) {
        Err(_) => { return Err("Could not remove directory") },
        _ => {}
    }
    Ok(())
}

pub fn list(path: &PathBuf) -> Result<Vec<String>, &'static str> {
    let config = GlobalConfig::load(path)?;
    Ok(config.priority)
}


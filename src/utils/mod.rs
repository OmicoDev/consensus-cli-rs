use git2::Repository;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub(crate) fn modify_gradle_properties<F>(modifier: F)
where
    F: FnOnce(&mut HashMap<String, String>),
{
    let content = fs::read_to_string(GRADLE_PROPERTIES_FILE_NAME).expect("Unable to read file");
    let mut properties = parse_gradle_properties();
    modifier(&mut properties);
    let new_content = modify_properties_content(properties, content);
    fs::write(GRADLE_PROPERTIES_FILE_NAME, new_content).expect("Unable to write file");
}

pub(crate) fn commit_gradle_properties(repo_path: PathBuf, message: &str, signed: bool) {
    let repo = Repository::open(repo_path).expect("Failed to open repository");
    let config = repo.config().expect("Failed to get repository config");
    Command::new("git")
        .args(["add", GRADLE_PROPERTIES_FILE_NAME])
        .spawn()
        .unwrap();
    let mut commit_command_args = Vec::from(["commit", "-m", message]);
    if signed | config.get_bool("commit.gpgSign").unwrap_or(false) {
        commit_command_args.push("-S");
    }
    Command::new("git")
        .args(commit_command_args)
        .spawn()
        .unwrap();
}

pub(crate) const GRADLE_PROPERTIES_FILE_NAME: &str = "gradle.properties";

pub(crate) const GRADLE_PROPERTY_PROJECT_VERSION: &str = "project.version";
pub(crate) const GRADLE_PROPERTY_PROJECT_VERSION_STAGE: &str = "project.versionStage";
pub(crate) const GRADLE_PROPERTY_PROJECT_VERSION_STAGE_NUMBER: &str = "project.versionStageNumber";

fn modify_properties_content(properties: HashMap<String, String>, content: String) -> String {
    let mut new_content = String::new();
    for line in content.lines() {
        if let Some((key_in_file, _)) = line.split_once('=') {
            let key = key_in_file.trim();
            let value = properties.get(key).unwrap();
            new_content.push_str(&format!("{}={}\n", key, value));
        } else {
            new_content.push_str(&format!("{}\n", line));
        }
    }
    new_content
}

fn parse_gradle_properties() -> HashMap<String, String> {
    let content = fs::read_to_string(GRADLE_PROPERTIES_FILE_NAME).expect("Unable to read file");
    let mut properties = HashMap::new();
    for line in content.lines() {
        // Skip comments
        if line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            properties.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    properties
}

#[test]
fn test_modify_properties_content() {
    use std::path::Path;
    let before_path = Path::new("test/gradle.properties.before");
    let before_content = fs::read_to_string(before_path).unwrap();
    let after_path = Path::new("test/gradle.properties.after");
    let after_content = fs::read_to_string(after_path).unwrap();
    let mut properties = HashMap::new();
    properties.insert(
        "project.group".to_string(),
        "me.omico.consensus".to_string(),
    );
    properties.insert(
        GRADLE_PROPERTY_PROJECT_VERSION.to_string(),
        "0.2.0".to_string(),
    );
    properties.insert(
        GRADLE_PROPERTY_PROJECT_VERSION_STAGE.to_string(),
        "alpha".to_string(),
    );
    properties.insert(
        GRADLE_PROPERTY_PROJECT_VERSION_STAGE_NUMBER.to_string(),
        "1".to_string(),
    );
    let content = modify_properties_content(properties, before_content);
    assert_eq!(content, after_content);
}

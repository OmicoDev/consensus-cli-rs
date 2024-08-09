use crate::utils::commit_gradle_properties;
use crate::utils::modify_gradle_properties;
use crate::utils::GRADLE_PROPERTY_PROJECT_VERSION;
use crate::utils::GRADLE_PROPERTY_PROJECT_VERSION_STAGE;
use crate::utils::GRADLE_PROPERTY_PROJECT_VERSION_STAGE_NUMBER;
use crate::version::VersionStage;
use std::path::PathBuf;

pub(crate) fn handle_prepare_command(version: String, signed: bool, project: PathBuf) {
    modify_gradle_properties(|properties| {
        properties.insert(GRADLE_PROPERTY_PROJECT_VERSION.to_string(), version);
        properties.insert(
            GRADLE_PROPERTY_PROJECT_VERSION_STAGE.to_string(),
            VersionStage::Snapshot.to_string(),
        );
        properties.insert(
            GRADLE_PROPERTY_PROJECT_VERSION_STAGE_NUMBER.to_string(),
            "0".to_string(),
        );
    });
    commit_gradle_properties(project, "Prepare for next version", signed);
}

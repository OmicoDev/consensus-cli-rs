use crate::utils::commit_gradle_properties;
use crate::utils::modify_gradle_properties;
use crate::utils::GRADLE_PROPERTY_PROJECT_VERSION;
use crate::utils::GRADLE_PROPERTY_PROJECT_VERSION_STAGE;
use crate::utils::GRADLE_PROPERTY_PROJECT_VERSION_STAGE_NUMBER;
use crate::version::VersionStage;
use std::path::PathBuf;

pub(crate) fn handle_release_command(
    stage: VersionStage,
    stage_number: u8,
    signed: bool,
    project: PathBuf,
) {
    if let Err(e) = validate_stage(stage, stage_number) {
        eprintln!("{}", e);
        return;
    }
    let mut version = String::new();
    modify_gradle_properties(|properties| {
        properties.insert(
            GRADLE_PROPERTY_PROJECT_VERSION_STAGE.to_string(),
            stage.to_string(),
        );
        properties.insert(
            GRADLE_PROPERTY_PROJECT_VERSION_STAGE_NUMBER.to_string(),
            stage_number.to_string(),
        );
        version.push_str(&properties[GRADLE_PROPERTY_PROJECT_VERSION]);
        if let VersionStage::Alpha | VersionStage::Beta | VersionStage::ReleaseCandidate = stage {
            version.push('-');
            version.push_str(stage.value());
            version.push_str(&format!("{:02}", stage_number));
        }
    });
    println!("Releasing {} ...", version);
    commit_gradle_properties(project, &format!("release: {}", version), signed);
}

fn validate_stage(stage: VersionStage, stage_number: u8) -> Result<(), String> {
    match stage {
        VersionStage::Snapshot => {
            return Err("Snapshot is not a valid stage for release. \
                    To prepare a new version, use the `prepare` command instead."
                .to_string())
        }
        VersionStage::Stable => {
            if stage_number != 0 {
                return Err(format!(
                    "{:?} stage number must be 0.\n\
                    Remove --stage-number or -n in your command.",
                    stage
                ));
            }
        }
        VersionStage::Alpha | VersionStage::Beta | VersionStage::ReleaseCandidate => {
            if !(1..=99).contains(&stage_number) {
                return Err(format!(
                    "{:?} stage number must be between 1 and 99.\n\
                    Use --stage-number or -n to specify the stage number.",
                    stage
                ));
            }
        }
    }
    Ok(())
}

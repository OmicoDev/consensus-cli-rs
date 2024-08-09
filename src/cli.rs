use crate::commands::prepare::handle_prepare_command;
use crate::commands::release::handle_release_command;
use crate::version::VersionStage;
use clap::arg;
use clap::value_parser;
use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

pub(crate) fn consensus_cli() {
    let args = ConsensusCli::parse();
    match args.command {
        Commands::Prepare {
            version,
            signed,
            project,
        } => handle_prepare_command(version, signed, project),
        Commands::Release {
            stage,
            stage_number,
            signed,
            project,
        } => handle_release_command(stage, stage_number, signed, project),
    }
}

#[derive(Debug, Parser)]
#[command(name = "consensus-cli", about = "CLI for Consensus")]
struct ConsensusCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Prepare a new version")]
    Prepare {
        #[arg(
            short,
            long,
            help = "Assign a new version",
            value_name = "VERSION",
            required = true
        )]
        version: String,
        #[arg(
            short,
            long,
            help = "Sign the commit",
            value_name = "SIGNED",
            default_value = "false"
        )]
        signed: bool,
        #[arg(
            short,
            long,
            help = "Project directory",
            value_name = "PROJECT",
            default_value = "."
        )]
        project: PathBuf,
    },
    #[command(about = "Release a new version")]
    Release {
        #[arg(
            short,
            long,
            help = "Stage name",
            value_name = "STAGE",
            value_enum,
            default_value = "stable"
        )]
        stage: VersionStage,
        #[arg(
            short = 'n',
            long,
            help = "Stage number",
            value_name = "STAGE_NUMBER",
            default_value = "0"
        )]
        #[arg(value_parser = value_parser!(u8).range(0..99))]
        stage_number: u8,
        #[arg(
            short,
            long,
            help = "Sign the commit",
            value_name = "SIGNED",
            default_value = "false"
        )]
        signed: bool,
        #[arg(
            short,
            long,
            help = "Project directory",
            value_name = "PROJECT",
            default_value = "."
        )]
        project: PathBuf,
    },
}

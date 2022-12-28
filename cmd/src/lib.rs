use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

pub mod error;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// specify the url where backup storage, eg, "s3://bucket/path/prefix"
    #[arg(short, long, global = true)]
    pub storage: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// backup a TiDB/TiKV cluster
    Backup {
        #[command(subcommand)]
        backup_type: Option<SubCommands>,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum SubCommands {
    /// backup a table
    Table {
        /// input table name
        #[arg(short, long = "table_name")]
        table_name: String,
    },
    /// backup a database
    DB {
        /// input table name
        #[arg(short, long = "db_name")]
        db_name: String,
    },
}

pub fn build_cli() -> Result<BackupInfo> {
    let cli = Cli::parse();
    if let Some(storage_path) = cli.storage {
        if let Some(cmd_flags) = cli.command {
            match cmd_flags {
                Commands::Backup { backup_type } => match backup_type {
                    Some(SubCommands::Table { table_name }) => Ok(BackupInfo::new(
                        BACKUP_TABLE.to_owned(),
                        storage_path,
                        table_name,
                    )),
                    Some(SubCommands::DB { db_name }) => todo!(),
                    None => todo!(),
                },
            }
        } else {
            Err(anyhow!("Missing attribute: {}", "456"))
        }
    } else {
        Err(anyhow!("Missing attribute: {}", "123"))
    }
}

// match backup_type {
//     SubCommands::Table { table_name } => Some(BackupInfo::new(
//         BACKUP_TABLE.to_owned(),
//         storage_path,
//         table_name,
//     )),
//     SubCommands::DB { db_name } => todo!(),
// },

#[derive(Debug, Clone)]
pub struct BackupInfo {
    pub backup_type: String,
    pub storage_path: String,
    pub table_name: String,
}

const BACKUP_TABLE: &str = "table";
// const BACKUP_DB: &str = "table";
impl BackupInfo {
    pub fn new(backup_type: String, storage_path: String, table_name: String) -> Self {
        return BackupInfo {
            backup_type,
            storage_path,
            table_name,
        };
    }
}

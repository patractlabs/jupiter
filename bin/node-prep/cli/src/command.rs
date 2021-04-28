use sc_cli::{ChainSpec, Role, RuntimeVersion, SubstrateCli};
use sc_service::PartialComponents;

use crate::chain_spec;
use crate::cli::{Cli, Subcommand};
use crate::service::{self, new_partial};

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Jupiter Node".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        env!("CARGO_PKG_DESCRIPTION").into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/patractlabs/patract".into()
    }

    fn copyright_start_year() -> i32 {
        2020
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
        Ok(match id {
            "jupiter-poa-dev" | "dev" => Box::new(chain_spec::jupiter::poa_development_config()?),
            "jupiter-poa-local" => Box::new(chain_spec::jupiter::poa_local_config()?),
            "jupiter-poa-staging" => Box::new(chain_spec::jupiter::poa_staging_config()?),
            "jupiter-poa" => Box::new(chain_spec::jupiter::poa_config()?),
            // TODO add jupiter spec
            "" | "jupiter" => return Err("Not implement jupiter spec know".to_string()),
            path => {
                let path = std::path::PathBuf::from(path);

                let starts_with = |prefix: &str| {
                    path.file_name()
                        .map(|f| f.to_str().map(|s| s.starts_with(&prefix)))
                        .flatten()
                        .unwrap_or(false)
                };
                if self.run.force_jupiter || starts_with("jupiter") {
                    Box::new(chain_spec::jupiter::ChainSpec::from_json_file(
                        std::path::PathBuf::from(path),
                    )?)
                } else {
                    return Err("Not implement jupiter spec know".to_string());
                }
            }
        })
    }

    fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        // todo add if condition to distinguish patract and jupiter
        &jupiter_runtime::VERSION
    }
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
    let cli = Cli::from_args();

    // set_default_ss58_version();

    match &cli.subcommand {
        None => {
            let runner = cli.create_runner(&cli.run.base)?;
            runner.run_node_until_exit(|config| async move {
                match config.role {
                    Role::Light => service::new_light(config),
                    _ => service::new_full(config),
                }
                .map_err(sc_cli::Error::Service)
            })
        }
        Some(Subcommand::Key(cmd)) => cmd.run(&cli),
        Some(Subcommand::Sign(cmd)) => cmd.run(),
        Some(Subcommand::Verify(cmd)) => cmd.run(),
        Some(Subcommand::Vanity(cmd)) => cmd.run(),
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = new_partial(&config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    ..
                } = new_partial(&config)?;
                Ok((cmd.run(client, config.database), task_manager))
            })
        }
        Some(Subcommand::ExportState(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    ..
                } = new_partial(&config)?;
                Ok((cmd.run(client, config.chain_spec), task_manager))
            })
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = new_partial(&config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            runner.sync_run(|config| cmd.run(config.database))
        }
        Some(Subcommand::Revert(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    backend,
                    ..
                } = new_partial(&config)?;
                Ok((cmd.run(client, backend), task_manager))
            })
        }
    }
}

// todo temp remove the ss58 setting
// fn set_default_ss58_version() {
//     use sp_core::crypto::Ss58AddressFormat;
//
//     let ss58_version = Ss58AddressFormat::Custom(26);
//
//     sp_core::crypto::set_default_ss58_version(ss58_version);
// }

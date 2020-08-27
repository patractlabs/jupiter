use sc_cli::{ChainSpec, Role, RuntimeVersion, SubstrateCli};
use sc_service::PartialComponents;

use crate::chain_spec;
use crate::cli::{Cli, Subcommand};
use crate::service;
use crate::service::new_partial;

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
        "https://github.com/patractlabs/jupiter".into()
    }

    fn copyright_start_year() -> i32 {
        2017
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
        Ok(match id {
            "dev" => Box::new(chain_spec::development_config()?),
            "" | "local" => Box::new(chain_spec::local_testnet_config()?),
            path => Box::new(chain_spec::ChainSpec::from_json_file(
                std::path::PathBuf::from(path),
            )?),
        })
    }

    fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        &jupiter_runtime::VERSION
    }
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
    let cli = Cli::from_args();

    match &cli.subcommand {
        None => {
            let runner = cli.create_runner(&cli.run)?;
            runner.run_node_until_exit(|config| match config.role {
                Role::Light => service::new_light(config),
                _ => service::new_full(config),
            })
        }
        Some(Subcommand::Key(cmd)) => cmd.run(),
        Some(Subcommand::Sign(cmd)) => cmd.run(),
        Some(Subcommand::Verify(cmd)) => cmd.run(),
        Some(Subcommand::Vanity(cmd)) => cmd.run(),
        Some(Subcommand::Base(subcommand)) => {
            let runner = cli.create_runner(subcommand)?;

            runner.run_subcommand(subcommand, |config| {
                let PartialComponents {
                    client,
                    backend,
                    task_manager,
                    import_queue,
                    ..
                } = new_partial(&config)?;
                Ok((client, backend, import_queue, task_manager))
            })
        }
    }
}

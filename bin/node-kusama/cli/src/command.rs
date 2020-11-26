// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use crate::chain_spec;
use crate::cli::{Cli, Subcommand};
use crate::service::{new_chain_ops, new_full, new_light};
use sc_cli::{ChainSpec, Result, Role, RuntimeVersion, SubstrateCli};

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Patract Labs Jupiter".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn executable_name() -> String {
        "jupiter-kusama".into()
    }

    fn description() -> String {
        env!("CARGO_PKG_DESCRIPTION").into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/patractlabs/jupiter/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2017
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        Ok(match id {
            "" | "dev" => Box::new(chain_spec::kusama_development_config()?),
            "local" => Box::new(chain_spec::kusama_local_testnet_config()?),
            "staging" => Box::new(chain_spec::kusama_staging_testnet_config()?),
            // TODO add in future
            // "" | _ => Box::new(chain_spec::kusama_config()?),
            _ => return Err("Not support yet".to_string()),
        })
    }

    fn native_runtime_version(_spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        &jupiter_kusama_runtime::VERSION
    }
}

fn set_default_ss58_version() {
    use sp_core::crypto::Ss58AddressFormat;

    sp_core::crypto::set_default_ss58_version(Ss58AddressFormat::KusamaAccount);
}

/// Parses polkadot specific CLI arguments and run the service.
pub fn run() -> Result<()> {
    let cli = Cli::from_args();

    match &cli.subcommand {
        None => {
            let runner = cli.create_runner(&cli.run.base)?;
            set_default_ss58_version();

            // let grandpa_pause = if cli.run.grandpa_pause.is_empty() {
            //     None
            // } else {
            //     Some((cli.run.grandpa_pause[0], cli.run.grandpa_pause[1]))
            // };

            runner.run_node_until_exit(|config| async move {
                let role = config.role.clone();

                match role {
                    Role::Light => new_light(config),
                    _ => new_full(config),
                }
            })
        }
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            set_default_ss58_version();

            runner.async_run(|mut config| {
                let (client, _, import_queue, task_manager) = new_chain_ops(&mut config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            set_default_ss58_version();

            runner.async_run(|mut config| {
                let (client, _, _, task_manager) = new_chain_ops(&mut config)?;
                Ok((cmd.run(client, config.database), task_manager))
            })
        }
        Some(Subcommand::ExportState(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            set_default_ss58_version();

            runner.async_run(|mut config| {
                let (client, _, _, task_manager) = new_chain_ops(&mut config)?;
                Ok((cmd.run(client, config.chain_spec), task_manager))
            })
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            set_default_ss58_version();

            runner.async_run(|mut config| {
                let (client, _, import_queue, task_manager) = new_chain_ops(&mut config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.database))
        }
        Some(Subcommand::Revert(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            set_default_ss58_version();

            runner.async_run(|mut config| {
                let (client, backend, _, task_manager) = new_chain_ops(&mut config)?;
                Ok((cmd.run(client, backend), task_manager))
            })
        }
        Some(Subcommand::Benchmark(cmd)) => {
            if cfg!(feature = "runtime-benchmarks") {
                let runner = cli.create_runner(cmd)?;
                set_default_ss58_version();

                runner.sync_run(|config| {
                    cmd.run::<jupiter_kusama_runtime::Block, crate::service::KusamaExecutor>(config)
                })
            } else {
                Err("Benchmarking wasn't enabled when building the node. \
				You can enable it with `--features runtime-benchmarks`.".into())
            }
        }
        Some(Subcommand::Key(cmd)) => cmd.run(),
    }
}

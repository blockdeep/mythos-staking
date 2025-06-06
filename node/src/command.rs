use std::path::PathBuf;

use cumulus_client_service::storage_proof_size::HostFunctions as ReclaimHostFunctions;
use cumulus_primitives_core::ParaId;
use frame_benchmarking_cli::{BenchmarkCmd, SUBSTRATE_REFERENCE_HARDWARE};
use log::info;
use runtime_common::Block;
use sc_cli::{
	ChainSpec, CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams,
	NetworkParams, Result, RpcEndpoint, SharedParams, SubstrateCli,
};
use sc_service::config::{BasePath, PrometheusConfig};
use sp_runtime::traits::AccountIdConversion;

use crate::{
	chain_spec::{self, GenericChainSpec},
	cli::{Cli, RelayChainCli, Subcommand},
	service::{new_partial, MainnetRuntimeExecutor, TestnetRuntimeExecutor},
};

/// Helper enum that is used for better distinction of different parachain/runtime configuration
/// (it is based/calculated on ChainSpec's 'chain_spec' attribute)
#[derive(Debug, PartialEq, Default)]
enum Runtime {
	/// This is the default runtime
	#[default]
	Default,
	Testnet,
	Mainnet,
}

trait RuntimeResolver {
	fn runtime(&self) -> Runtime;
}
/// Private helper that pattern matches on the input (which is expected to be a ChainSpec ID)
/// and returns the Runtime accordingly.
fn runtime(id: &str) -> Runtime {
	if id.starts_with("dev") {
		Runtime::Testnet
	} else if id.starts_with("main") {
		Runtime::Mainnet
	} else {
		log::warn!("No specific runtime was recognized for ChainSpec's Id: '{}', so Runtime::Default will be used", id);
		Runtime::default()
	}
}
/// Resolve runtime from ChainSpec ID
impl RuntimeResolver for dyn ChainSpec {
	fn runtime(&self) -> Runtime {
		runtime(self.id())
	}
}
/// Implementation, that can resolve [`Runtime`] from any json configuration file
impl RuntimeResolver for PathBuf {
	fn runtime(&self) -> Runtime {
		#[derive(Debug, serde::Deserialize)]
		struct EmptyChainSpecWithId {
			id: String,
		}

		let file = std::fs::File::open(self).expect("Failed to open file");
		let reader = std::io::BufReader::new(file);
		let chain_spec: EmptyChainSpecWithId = serde_json::from_reader(reader)
			.expect("Failed to read 'json' file with ChainSpec configuration");

		runtime(&chain_spec.id)
	}
}

fn load_spec(id: &str) -> std::result::Result<Box<dyn ChainSpec>, String> {
	Ok(match id {
		// Testnet - Muse
		"" | "dev" | "testnet-local" | "local-v" => {
			Box::new(chain_spec::testnet::development_config())
		},
		"muse" | "testnet" => Box::new(chain_spec::testnet::testnet_config()),
		// Mainnet - Mythos
		"main" | "mainnet-dev" | "mainnet-local-v" => {
			Box::new(chain_spec::mainnet::development_config())
		},
		"mythos" | "mainnet" => Box::new(GenericChainSpec::from_json_bytes(
			&include_bytes!("../../chainspecs/mythos-raw.json")[..],
		)?),
		path => {
			let path: PathBuf = path.into();
			match path.runtime() {
				Runtime::Testnet | Runtime::Default => {
					Box::new(GenericChainSpec::from_json_file(path)?)
				},
				Runtime::Mainnet => Box::new(GenericChainSpec::from_json_file(path)?),
			}
		},
	})
}

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Parachain Collator Template".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		format!(
			"Parachain Collator Template\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relay chain node.\n\n\
		{} <parachain-args> -- <relay-chain-args>",
			Self::executable_name()
		)
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/paritytech/cumulus/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2020
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		load_spec(id)
	}
}

impl SubstrateCli for RelayChainCli {
	fn impl_name() -> String {
		"Parachain Collator Template".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		format!(
			"Parachain Collator Template\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relay chain node.\n\n\
		{} <parachain-args> -- <relay-chain-args>",
			Self::executable_name()
		)
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/paritytech/cumulus/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2020
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		polkadot_cli::Cli::from_iter([RelayChainCli::executable_name()].iter()).load_spec(id)
	}
}

macro_rules! construct_async_run {
	(|$components:ident, $cli:ident, $cmd:ident, $config:ident| $( $code:tt )* ) => {{
		let runner = $cli.create_runner($cmd)?;
		match runner.config().chain_spec.runtime() {
			Runtime::Testnet | Runtime::Default => {
				runner.async_run(|$config| {
					let $components = new_partial::<testnet_runtime::RuntimeApi, _>(
						&$config,
						crate::service::build_import_queue::<testnet_runtime::RuntimeApi>,
					)?;
					let task_manager = $components.task_manager;
					{ $( $code )* }.map(|v| (v, task_manager))
				})
			}
			Runtime::Mainnet => {
				runner.async_run(|$config| {
					let $components = new_partial::<mainnet_runtime::RuntimeApi, _>(
						&$config,
						crate::service::build_import_queue::<mainnet_runtime::RuntimeApi>,
					)?;
					let task_manager = $components.task_manager;
					{ $( $code )* }.map(|v| (v, task_manager))
				})
			}
		}
	}}
}

macro_rules! construct_benchmark_partials {
	($config:expr, |$partials:ident| $code:expr) => {
		match $config.chain_spec.runtime() {
			Runtime::Testnet | Runtime::Default => {
				let $partials = new_partial::<testnet_runtime::RuntimeApi, _>(
					&$config,
					crate::service::build_import_queue::<_>,
				)?;
				$code
			},
			Runtime::Mainnet => {
				let $partials = new_partial::<mainnet_runtime::RuntimeApi, _>(
					&$config,
					crate::service::build_import_queue::<_>,
				)?;
				$code
			},
		}
	};
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, components.import_queue))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, config.database))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, config.chain_spec))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, components.import_queue))
			})
		},
		Some(Subcommand::Revert(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, components.backend, None))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			runner.sync_run(|config| {
				let polkadot_cli = RelayChainCli::new(
					&config,
					[RelayChainCli::executable_name()].iter().chain(cli.relay_chain_args.iter()),
				);

				let polkadot_config = SubstrateCli::create_configuration(
					&polkadot_cli,
					&polkadot_cli,
					config.tokio_handle.clone(),
				)
				.map_err(|err| format!("Relay chain argument error: {}", err))?;

				cmd.run(config, polkadot_config)
			})
		},
		Some(Subcommand::ExportGenesisHead(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| {
				construct_benchmark_partials!(config, |partials| {
					cmd.run(partials.client)
				})
			})
		},
		Some(Subcommand::ExportGenesisWasm(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|_config| {
				let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
				cmd.run(&*spec)
			})
		},
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			// Switch on the concrete benchmark sub-command-
			match cmd {
				BenchmarkCmd::Pallet(cmd) => {
					if cfg!(feature = "runtime-benchmarks") {
						runner.sync_run(|config| cmd.run_with_spec::<sp_runtime::traits::HashingFor<Block>, ReclaimHostFunctions>(Some(config.chain_spec)))
					} else {
						Err("Benchmarking wasn't enabled when building the node. \
					You can enable it with `--features runtime-benchmarks`."
							.into())
					}
				},

				BenchmarkCmd::Block(cmd) => runner.sync_run(|config| {
					construct_benchmark_partials!(config, |partials| cmd.run(partials.client))
				}),
				#[cfg(not(feature = "runtime-benchmarks"))]
				BenchmarkCmd::Storage(_) => Err(sc_cli::Error::Input(
					"Compile with --features=runtime-benchmarks \
						to enable storage benchmarks."
						.to_string(),
				)),
				#[cfg(feature = "runtime-benchmarks")]
				BenchmarkCmd::Storage(cmd) => runner.sync_run(|config| {
					construct_benchmark_partials!(config, |partials| {
						let db = partials.backend.expose_db();
						let storage = partials.backend.expose_storage();

						cmd.run(config, partials.client.clone(), db, storage)
					})
				}),
				BenchmarkCmd::Machine(cmd) => {
					runner.sync_run(|config| cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone()))
				},
				// NOTE: this allows the Client to leniently implement
				// new benchmark commands without requiring a companion MR.
				#[allow(unreachable_patterns)]
				_ => Err("Benchmarking sub-command unsupported".into()),
			}
		},
		Some(Subcommand::TryRuntime) => Err("The `try-runtime` subcommand has been migrated to a standalone CLI (https://github.com/paritytech/try-runtime-cli). It is no longer being maintained here and will be removed entirely some time after January 2024. Please remove this subcommand from your runtime and use the standalone CLI.".into()),
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		None => {
			let runner = cli.create_runner(&cli.run.normalize())?;
			let collator_options = cli.run.collator_options();

			runner.run_node_until_exit(|config| async move {
				let hwbench = (!cli.no_hardware_benchmarks)
					.then_some(config.database.path().map(|database_path| {
						let _ = std::fs::create_dir_all(database_path);
						sc_sysinfo::gather_hwbench(
							Some(database_path),
							&SUBSTRATE_REFERENCE_HARDWARE,
						)
					}))
					.flatten();

				let para_id = chain_spec::Extensions::try_get(&*config.chain_spec)
					.map(|e| e.para_id)
					.ok_or("Could not find parachain ID in chain-spec.")?;

				let polkadot_cli = RelayChainCli::new(
					&config,
					[RelayChainCli::executable_name()].iter().chain(cli.relay_chain_args.iter()),
				);

				let id = ParaId::from(para_id);

				let parachain_account =
					AccountIdConversion::<polkadot_primitives::AccountId>::into_account_truncating(
						&id,
					);

				let tokio_handle = config.tokio_handle.clone();
				let polkadot_config =
					SubstrateCli::create_configuration(&polkadot_cli, &polkadot_cli, tokio_handle)
						.map_err(|err| format!("Relay chain argument error: {}", err))?;

				info!("Parachain id: {:?}", id);
				info!("Parachain Account: {}", parachain_account);
				info!("Is collating: {}", if config.role.is_authority() { "yes" } else { "no" });

				match config.chain_spec.runtime() {
					Runtime::Default | Runtime::Testnet => {
						// If you want to support a custom SS58 prefix (that isn’t yet registered in the ss58-registry),
						// you are required to call this function with your desired prefix [Ss58AddressFormat::custom].
						// This will enable the node to decode ss58 addresses with this prefix.
						// This SS58 version/format is also only used by the node and not by the runtime.
						sp_core::crypto::set_default_ss58_version(
							testnet_runtime::SS58Prefix::get().into(),
						);
						crate::service::start_parachain_node::<
							testnet_runtime::RuntimeApi,
							TestnetRuntimeExecutor,
							sc_network::NetworkWorker<_, _>
						>(config, polkadot_config, collator_options, id, hwbench, cli.run.experimental_max_pov_percentage)
						.await
						.map(|r| r.0)
						.map_err(Into::into)
					},
					Runtime::Mainnet => {
						sp_core::crypto::set_default_ss58_version(
							mainnet_runtime::SS58Prefix::get().into(),
						);
						crate::service::start_parachain_node::<
							mainnet_runtime::RuntimeApi,
							MainnetRuntimeExecutor,
							sc_network::NetworkWorker<_, _>
						>(config, polkadot_config, collator_options, id, hwbench, cli.run.experimental_max_pov_percentage)
						.await
						.map(|r| r.0)
						.map_err(Into::into)
					},
				}
			})
		},
	}
}

impl DefaultConfigurationValues for RelayChainCli {
	fn p2p_listen_port() -> u16 {
		30334
	}

	fn rpc_listen_port() -> u16 {
		9945
	}

	fn prometheus_listen_port() -> u16 {
		9616
	}
}

impl CliConfiguration<Self> for RelayChainCli {
	fn shared_params(&self) -> &SharedParams {
		self.base.base.shared_params()
	}

	fn import_params(&self) -> Option<&ImportParams> {
		self.base.base.import_params()
	}

	fn network_params(&self) -> Option<&NetworkParams> {
		self.base.base.network_params()
	}

	fn keystore_params(&self) -> Option<&KeystoreParams> {
		self.base.base.keystore_params()
	}

	fn base_path(&self) -> Result<Option<BasePath>> {
		Ok(self
			.shared_params()
			.base_path()?
			.or_else(|| self.base_path.clone().map(Into::into)))
	}

	fn rpc_addr(&self, default_listen_port: u16) -> Result<Option<Vec<RpcEndpoint>>> {
		self.base.base.rpc_addr(default_listen_port)
	}

	fn prometheus_config(
		&self,
		default_listen_port: u16,
		chain_spec: &Box<dyn ChainSpec>,
	) -> Result<Option<PrometheusConfig>> {
		self.base.base.prometheus_config(default_listen_port, chain_spec)
	}

	fn init<F>(&self, _support_url: &String, _impl_version: &String, _logger_hook: F) -> Result<()>
	where
		F: FnOnce(&mut sc_cli::LoggerBuilder),
	{
		unreachable!("PolkadotCli is never initialized; qed");
	}

	fn chain_id(&self, is_dev: bool) -> Result<String> {
		let chain_id = self.base.base.chain_id(is_dev)?;

		Ok(if chain_id.is_empty() { self.chain_id.clone().unwrap_or_default() } else { chain_id })
	}

	fn role(&self, is_dev: bool) -> Result<sc_service::Role> {
		self.base.base.role(is_dev)
	}

	fn transaction_pool(&self, is_dev: bool) -> Result<sc_service::config::TransactionPoolOptions> {
		self.base.base.transaction_pool(is_dev)
	}

	fn trie_cache_maximum_size(&self) -> Result<Option<usize>> {
		self.base.base.trie_cache_maximum_size()
	}

	fn rpc_methods(&self) -> Result<sc_service::config::RpcMethods> {
		self.base.base.rpc_methods()
	}

	fn rpc_max_connections(&self) -> Result<u32> {
		self.base.base.rpc_max_connections()
	}

	fn rpc_cors(&self, is_dev: bool) -> Result<Option<Vec<String>>> {
		self.base.base.rpc_cors(is_dev)
	}

	fn default_heap_pages(&self) -> Result<Option<u64>> {
		self.base.base.default_heap_pages()
	}

	fn force_authoring(&self) -> Result<bool> {
		self.base.base.force_authoring()
	}

	fn disable_grandpa(&self) -> Result<bool> {
		self.base.base.disable_grandpa()
	}

	fn max_runtime_instances(&self) -> Result<Option<usize>> {
		self.base.base.max_runtime_instances()
	}

	fn announce_block(&self) -> Result<bool> {
		self.base.base.announce_block()
	}

	fn telemetry_endpoints(
		&self,
		chain_spec: &Box<dyn ChainSpec>,
	) -> Result<Option<sc_telemetry::TelemetryEndpoints>> {
		self.base.base.telemetry_endpoints(chain_spec)
	}

	fn node_name(&self) -> Result<String> {
		self.base.base.node_name()
	}
}

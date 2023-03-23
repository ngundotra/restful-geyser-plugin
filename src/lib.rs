use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, GeyserPluginError, ReplicaTransactionInfoVersions,
};

use tokio::runtime::{Builder, Runtime};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RestfulPluginError {
    #[error("Error in RestfulPlugin: {0}")]
    Custom(Box<dyn std::error::Error + Send + Sync>),
    #[error("Unable to get runtime: ({msg})")]
    SetupError { msg: String },
}

#[derive(Default, Debug)]
struct RestfulPlugin {
    runtime: Option<Runtime>,
}

impl RestfulPlugin {
    pub fn new() -> Self {
        RestfulPlugin { runtime: None }
    }

    fn get_runtime(&self) -> Result<&tokio::runtime::Runtime, GeyserPluginError> {
        if let Some(runtime) = &self.runtime {
            Ok(runtime)
        } else {
            Err(GeyserPluginError::Custom(Box::new(
                RestfulPluginError::SetupError {
                    msg: "No runtime contained in struct".to_string(),
                },
            )))
        }
    }
}

impl GeyserPlugin for RestfulPlugin {
    fn name(&self) -> &'static str {
        "RestfulPlugin"
    }

    fn on_load(
        &mut self,
        _config_file: &str,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        let runtime = Builder::new_multi_thread()
            .enable_all()
            .thread_name("plerkle-runtime-worker")
            .build()
            .map_err(|err| GeyserPluginError::ConfigFileReadError {
                msg: format!("Could not create tokio runtime: {:?}", err),
            })?;

        runtime.spawn(async move {
            println!("Inside the matrix");
        });

        self.runtime = Some(runtime);
        Ok(())
    }

    fn notify_transaction(
        &mut self,
        transaction_info: ReplicaTransactionInfoVersions,
        slot: u64,
    ) -> solana_geyser_plugin_interface::geyser_plugin_interface::Result<()> {
        println!("Received transaction");
        // let runtime = self.get_runtime()?;

        Ok(())
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = RestfulPlugin::new();
    let plugin: Box<dyn GeyserPlugin> = Box::new(plugin);
    Box::into_raw(plugin)
}

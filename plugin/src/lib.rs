mod logger;
mod plugin;
mod udp;

use crate::plugin::PluginError;
use tracing::info;
use xplm::plugin::{Plugin, PluginInfo};

struct XPlaneUdpBridgePlugin;

impl Plugin for XPlaneUdpBridgePlugin {
    type Error = PluginError;

    fn start() -> Result<Self, Self::Error> {
        logger::init();
        info!("{} plugin starting...", plugin::NAME);
        udp::start_udp_server(plugin::UDP_SERVER_PORT);
        info!("{} plugin started successfully", plugin::NAME);
        Ok(Self {})
    }

    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: String::from(plugin::NAME),
            signature: String::from(plugin::SIGN),
            description: String::from(plugin::DESC),
        }
    }
}

impl Drop for XPlaneUdpBridgePlugin {
    fn drop(&mut self) {
        info!("{} plugin dropping...", plugin::NAME);
        udp::stop_udp_server();
        info!("{} plugin dropped successfully", plugin::NAME);
    }
}

xplm::xplane_plugin!(XPlaneUdpBridgePlugin);

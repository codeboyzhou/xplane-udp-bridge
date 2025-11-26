mod error;
mod logger;
mod udp;

use crate::error::PluginError;
use tracing::info;
use xplm::plugin::{Plugin, PluginInfo};

struct XPlaneUdpBridgePlugin;

impl XPlaneUdpBridgePlugin {
    const UDP_SERVER_PORT: u16 = 49000;
    const NAME: &'static str = "XPlaneUdpBridge";
    const SIGN: &'static str = "https://github.com/codeboyzhou/xplane-udp-bridge";
    const DESC: &'static str = "A X-Plane plugin that bridges UDP communication with X-Plane";
}

impl Plugin for XPlaneUdpBridgePlugin {
    type Error = PluginError;

    fn start() -> Result<Self, Self::Error> {
        logger::init();
        info!("{} plugin starting...", Self::NAME);
        udp::server::start(Self::UDP_SERVER_PORT);
        info!("{} plugin started successfully", Self::NAME);
        Ok(Self {})
    }

    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: String::from(Self::NAME),
            signature: String::from(Self::SIGN),
            description: String::from(Self::DESC),
        }
    }
}

impl Drop for XPlaneUdpBridgePlugin {
    fn drop(&mut self) {
        info!("{} plugin dropping...", Self::NAME);
        udp::server::stop();
        info!("{} plugin dropped successfully", Self::NAME);
    }
}

xplm::xplane_plugin!(XPlaneUdpBridgePlugin);

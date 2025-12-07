use crate::dataref::DataRefReader;
use infra::{logger, udp};
use std::convert::Infallible;
use xplm::plugin::{Plugin, PluginInfo};

struct XPlaneUdpBridgePlugin;

impl XPlaneUdpBridgePlugin {
    const UDP_SERVER_PORT: u16 = 49000;

    const NAME: &'static str = "XPlaneUdpBridge";

    const LOG_FILE_NAME: &'static str = "XPlaneUdpBridgePlugin.log";

    const SIGNATURE: &'static str = "https://github.com/codeboyzhou/xplane-udp-bridge";

    const DESCRIPTION: &'static str = "This plugin connects X-Plane and external apps via UDP.";
}

impl Plugin for XPlaneUdpBridgePlugin {
    type Error = Infallible;

    fn start() -> Result<Self, Self::Error> {
        logger::init_file_logger(Self::LOG_FILE_NAME);
        udp::server::start(Self::UDP_SERVER_PORT);
        udp::server::register_request_handler(Box::new(DataRefReader::new()));
        Ok(Self {})
    }

    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: String::from(Self::NAME),
            signature: String::from(Self::SIGNATURE),
            description: String::from(Self::DESCRIPTION),
        }
    }
}

xplm::xplane_plugin!(XPlaneUdpBridgePlugin);

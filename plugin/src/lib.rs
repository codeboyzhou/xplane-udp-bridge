//! # X-Plane UDP Bridge Plugin
//!
//! This is the main entry point for the X-Plane UDP Bridge plugin.
//! It provides UDP communication capabilities between external applications
//! and X-Plane flight simulator.

mod error;
mod logger;
mod udp;

use crate::error::PluginError;
use crate::udp::server::UdpServer;
use tracing::info;
use xplm::plugin::{Plugin, PluginInfo};

/// The main plugin structure that implements the X-Plane plugin interface.
///
/// This plugin creates a UDP server that listens for external connections
/// and bridges communication with X-Plane's data and command systems.
struct XPlaneUdpBridgePlugin;

impl XPlaneUdpBridgePlugin {
    /// The default UDP port for the server to listen on.
    const UDP_SERVER_PORT: u16 = 49000;

    /// The name of the plugin.
    const NAME: &'static str = "XPlaneUdpBridge";

    /// The signature of the plugin.
    const SIGN: &'static str = "https://github.com/codeboyzhou/xplane-udp-bridge";

    /// A brief description of the plugin's functionality.
    const DESC: &'static str = "A X-Plane plugin that bridges UDP communication with X-Plane";
}

impl Plugin for XPlaneUdpBridgePlugin {
    /// The error type that can be returned by plugin operations.
    type Error = PluginError;

    /// Initializes and starts the plugin.
    ///
    /// This method is called by X-Plane when the plugin is loaded.
    /// It initializes the logger and starts the UDP server.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Self)` if the plugin started successfully,
    /// or `Err(Self::Error)` if initialization failed.
    ///
    /// # Examples
    ///
    /// ```
    /// // This method is called by X-Plane, not typically called directly
    /// let plugin = XPlaneUdpBridgePlugin::start()?;
    /// ```
    fn start() -> Result<Self, Self::Error> {
        logger::init_file_logger();
        info!("{} plugin starting...", Self::NAME);
        UdpServer::start(Self::UDP_SERVER_PORT);
        info!("{} plugin started successfully", Self::NAME);
        Ok(Self {})
    }

    /// Returns plugin information for display in X-Plane.
    ///
    /// This method provides metadata about the plugin that is displayed
    /// in X-Plane's plugin manager.
    ///
    /// # Returns
    ///
    /// Returns a `PluginInfo` struct containing the plugin's name,
    /// signature, and description.
    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: String::from(Self::NAME),
            signature: String::from(Self::SIGN),
            description: String::from(Self::DESC),
        }
    }
}

xplm::xplane_plugin!(XPlaneUdpBridgePlugin);

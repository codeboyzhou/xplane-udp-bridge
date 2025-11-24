use std::fmt::{Debug, Display, Formatter};

pub(crate) const UDP_SERVER_PORT: u16 = 49000;

pub(crate) const NAME: &str = "xplane-udp-bridge";

pub(crate) const SIGN: &str = "https://github.com/codeboyzhou/xplane-udp-bridge";

pub(crate) const DESC: &str = "This plugin allows you to communicate with X-Plane 12 via UDP.";

#[derive(Debug)]
pub(crate) struct PluginError {
    message: String,
}

impl Display for PluginError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} plugin error: {}", NAME, self.message)
    }
}

impl std::error::Error for PluginError {}

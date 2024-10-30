#[macro_use]
extern crate zed_extension_api;
use zed_extension_api::Extension;

struct PicklsExtension {}

impl Extension for PicklsExtension {
    fn new() -> Self {
        Self {}
    }
}

register_extension!(PicklsExtension);

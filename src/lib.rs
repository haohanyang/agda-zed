use zed_extension_api::{self as zed};

struct AgdaExtension;

impl zed::Extension for AgdaExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(AgdaExtension);

#[cfg(feature = "mocked")]
mod mocks {
    mod app;
    mod plugin;
    pub use self::{app::MockApp as App, plugin::MockPlugin, plugin::Plugin};
}

pub mod release {

    mod ordered_plugin;
    mod plugin_descriptor;

    pub use ordered_plugin::{app_dummy::AppDummy, OrderedPlugin, PlainDescriptor};
    pub use plugin_descriptor::{PluginDescriptor, PluginDescriptorCoercion};

    #[cfg(feature = "mocked")]
    pub use crate::mocks::*;
    use bevy::ecs::schedule::IntoSystemDescriptor;
    #[cfg(not(feature = "mocked"))]
    pub use bevy::prelude::{App, Plugin};
}

// unsafe impl Sync for TestPlugin {}
// unsafe impl Send for TestPlugin {}

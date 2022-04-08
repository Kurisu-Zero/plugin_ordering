#[cfg(feature = "mocked")]
pub mod mocks {
    mod app;
    mod plugin;
    pub use self::{app::MockApp as App, plugin::MockPlugin, plugin::Plugin};
}

pub mod release {

    mod ordered_plugin;
    mod plugin_descriptor;

    pub use ordered_plugin::{app_dummy::AppDummy, OrderedPlugin, PlainDescriptor};

    use bevy::ecs::schedule::IntoSystemDescriptor;
    pub use plugin_descriptor::{PluginDescriptor, PluginDescriptorCoercion};

    #[cfg(feature = "mocked")]
    pub use crate::mocks::*;
    #[cfg(not(feature = "mocked"))]
    pub use bevy::prelude::{App, Plugin};

    pub use bevy::ecs::schedule::SystemDescriptor;
}

// unsafe impl Sync for TestPlugin {}
// unsafe impl Send for TestPlugin {}

#[cfg(feature = "mocked")]
pub mod mocked {
    pub use crate::common::*;

    pub(crate) mod ordered_plugin;
    pub(crate) mod plugin_descriptor;

    mod app;
    mod plugin;
    pub use self::{app::MockApp as App, plugin::MockPlugin, plugin::Plugin};
    pub use __mock_MockApp::__add_system::Expectation as add_system_Expectation;
}

#[cfg(not(feature = "mocked"))]
pub mod release {
    pub use crate::common::*;

    pub(crate) mod ordered_plugin;
    pub(crate) mod plugin_descriptor;

    pub use bevy::prelude::{App, Plugin};
}

mod common {
    #[cfg(feature = "mocked")]
    use crate::mocked as root;
    #[cfg(not(feature = "mocked"))]
    use crate::release as root;

    pub(crate) use bevy::ecs::schedule::IntoSystemDescriptor;
    pub use root::ordered_plugin::{app_dummy::AppDummy, OrderedPlugin, PlainDescriptor};
    pub use root::plugin_descriptor::{PluginDescriptor, PluginDescriptorCoercion};
}

// unsafe impl Sync for TestPlugin {}
// unsafe impl Send for TestPlugin {}

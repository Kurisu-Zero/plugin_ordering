#[cfg(feature = "mocked")]
pub mod mocked {
    pub use crate::inner::*;

    mod app;
    mod plugin;
    pub use self::app::__mock_MockApp::__add_system::Expectation as add_system_Expectation;
    pub use self::{app::MockApp as App, plugin::MockPlugin, plugin::Plugin};
}

#[cfg(not(feature = "mocked"))]
pub mod release {
    pub use crate::inner::*;

    pub use bevy::prelude::{App, Plugin};
}

mod inner {

    #[cfg(feature = "mocked")]
    use crate::mocked::*;
    #[cfg(not(feature = "mocked"))]
    use crate::release::*;

    pub(crate) mod ordered_plugin;
    pub(crate) mod plugin_descriptor;

    pub(crate) use bevy::ecs::schedule::IntoSystemDescriptor;
    pub use ordered_plugin::{app_dummy::AppDummy, OrderedPlugin, PlainDescriptor};
    pub use plugin_descriptor::{PluginDescriptor, PluginDescriptorCoercion};
}

// unsafe impl Sync for TestPlugin {}
// unsafe impl Send for TestPlugin {}

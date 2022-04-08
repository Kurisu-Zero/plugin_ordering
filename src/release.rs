mod ordered_plugin;
mod plugin_descriptor;

pub use ordered_plugin::{app_dummy::AppDummy, OrderedPlugin, PlainDescriptor};

use bevy::ecs::schedule::IntoSystemDescriptor;
pub use bevy::prelude::{App, Plugin};
pub use plugin_descriptor::{PluginDescriptor, PluginDescriptorCoercion};

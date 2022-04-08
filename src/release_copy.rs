mod ordered_plugin;
mod plugin_descriptor;

pub use ordered_plugin::{app_dummy::AppDummy, OrderedPlugin, PlainDescriptor};

pub use crate::mocks::*;

pub use plugin_descriptor::{PluginDescriptor, PluginDescriptorCoercion};

use bevy::ecs::schedule::IntoSystemDescriptor;
pub use bevy::ecs::schedule::SystemDescriptor;

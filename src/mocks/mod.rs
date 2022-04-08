#[cfg(test)]
mod app;
#[cfg(test)]
mod plugin;

#[cfg(test)]
//pub mod expanded_app;
use core::any::Any;

use mockall::predicate::*;
use mockall::*;

#[cfg(test)]
pub use self::{app::MockApp as App, plugin::MockPlugin, plugin::Plugin};

// pub mod both {
//     #[cfg(test)]
//     pub use super::{app::MockApp, plugin::MockPlugin, plugin::Plugin as PluginDouble};
//     pub use bevy::prelude::{App, Plugin};
// }

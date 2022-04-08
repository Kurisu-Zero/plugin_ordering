pub mod app_dummy;
use super::*;

use std::any::Any;
pub trait OrderedPlugin
where
    Self: Any + Send + Sync,
{
    fn build_impl(&self, app: &mut AppDummy);
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

pub trait PlainDescriptor {
    fn as_is(self) -> PluginDescriptor;
}
impl<T: OrderedPlugin> PlainDescriptor for T {
    fn as_is(self) -> PluginDescriptor {
        PluginDescriptor {
            ordered_plugin: Box::new(self),
            labels: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
        }
    }
}

// unsafe impl Sync for OrderedPluginStruct {}
// unsafe impl Send for OrderedPluginStruct {}

impl Plugin for PluginDescriptor {
    fn build(&self, app: &mut App) {
        print!("Plugin is being build");
        let mut app_dummy = AppDummy::new(app, &self);
        app_dummy.build_impl();
    }

    fn name(&self) -> &str {
        self.ordered_plugin.name()
    }
}

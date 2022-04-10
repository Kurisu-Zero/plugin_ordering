pub mod app_dummy;
pub mod sized_label;

use super::*;

use std::any::Any;
pub trait OrderedPlugin
where
    Self: Any + Send + Sync,
{
    fn build(&self, app: &mut App);
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

pub trait DynClone
where
    Self: Sized,
{
    fn dyn_clone(self) -> Box<dyn OrderedPlugin>;
}

impl<T: OrderedPlugin> DynClone for T {
    fn dyn_clone(self) -> Box<dyn OrderedPlugin> {
        Box::new(self)
    }
}

struct EmptyOrderedPlugin;
impl OrderedPlugin for EmptyOrderedPlugin {
    fn build(&self, _: &mut App) {}
}

impl Plugin_internal for Box<dyn OrderedPlugin> {
    fn build(&self, app: &mut App_internal) {
        <dyn OrderedPlugin as Plugin_internal>::build(&**self, app);
    }
}

impl Plugin_internal for dyn OrderedPlugin {
    fn build(&self, app: &mut App_internal) {
        let pointer = self as *const dyn OrderedPlugin;
        let pointer_mut = pointer as *mut dyn OrderedPlugin;
        // #SAFETY: We have to make sure that the BOX will NOT be dropped!
        // #SAFETY: And we CANNOT use self in any way directly until it is cleaned up
        let mut desc = unsafe {
            PluginDescriptor {
                ordered_plugin: Box::from_raw(pointer_mut),
                labels: Vec::new(),
                before: Vec::new(),
                after: Vec::new(),
            }
        };
        println!("Plugin is being build from OrderedPlugin");
        let mut app_dummy = App::new(app, &desc);
        app_dummy.build_impl();
        // #SAFETY: here we extract the dirty Box and prevent it from being dropped
        let mut boxed_plugin: Box<dyn OrderedPlugin> = Box::new(EmptyOrderedPlugin);
        std::mem::swap(&mut boxed_plugin, &mut desc.ordered_plugin);
        Box::into_raw(boxed_plugin); // No more double free
    }
}

pub trait PlainDescriptor {
    fn as_is(self) -> PluginDescriptor;
}
impl<T: OrderedPlugin> PlainDescriptor for T {
    fn as_is(self) -> PluginDescriptor {
        println!("run as_is");
        PluginDescriptor {
            ordered_plugin: Box::new(self),
            labels: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
        }
    }
}

unsafe impl Sync for PluginDescriptor {}
unsafe impl Send for PluginDescriptor {}

impl Plugin_internal for PluginDescriptor {
    fn build(&self, app: &mut App_internal) {
        println!("Plugin is being build");
        let mut app_dummy = App::new(app, &self);
        app_dummy.build_impl();
    }

    fn name(&self) -> &str {
        self.ordered_plugin.name()
    }
}

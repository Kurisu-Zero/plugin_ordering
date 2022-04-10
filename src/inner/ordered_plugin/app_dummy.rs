use super::*;

use bevy::ecs::schedule::{ParallelSystemDescriptor, SystemDescriptor};
pub struct AppDummy<'a> {
    app: &'a mut App_internal,
    plugin_descriptor: &'a PluginDescriptor,
}

impl AppDummy<'_> {
    pub fn new<'a>(app: &'a mut App_internal, desc: &'a PluginDescriptor) -> AppDummy<'a> {
        AppDummy {
            app,
            plugin_descriptor: desc,
        }
    }

    pub fn build_impl(&mut self) {
        (*self.plugin_descriptor.ordered_plugin).build(self);
    }

    pub fn add_plugin<T>(&mut self, mut plugin: PluginDescriptor) -> &mut Self {
        //make sure the inner plugin also applies the modifications

        for label_function in &self.plugin_descriptor.labels {
            plugin.labels.push(label_function.dyn_clone());
        }
        for label_function in &self.plugin_descriptor.before {
            plugin.before.push(label_function.dyn_clone());
        }
        for label_function in &self.plugin_descriptor.after {
            plugin.after.push(label_function.dyn_clone());
        }

        plugin.build(self.app);
        self
    }

    fn label_parallel_system(
        &mut self,
        mut system_descriptor: ParallelSystemDescriptor,
    ) -> SystemDescriptor {
        for label_function in &self.plugin_descriptor.labels {
            system_descriptor = label_function.label()(system_descriptor);
        }
        for label_function in &self.plugin_descriptor.before {
            system_descriptor = label_function.before()(system_descriptor);
        }
        for label_function in &self.plugin_descriptor.after {
            system_descriptor = label_function.after()(system_descriptor);
        }
        SystemDescriptor::Parallel(system_descriptor)
    }

    fn add_system_impl<Params>(&mut self, system: impl IntoSystemDescriptor<Params>) -> &mut Self {
        println!("run app_dummy::add_system_impl");

        let system_descriptor = match system.into_descriptor() {
            bevy::ecs::schedule::SystemDescriptor::Parallel(p) => {
                Self::label_parallel_system(self, p)
            }
            bevy::ecs::schedule::SystemDescriptor::Exclusive(_e) => todo!(),
        };

        println!(
            "Adding system_descriptor: {}",
            Self::name(&system_descriptor)
        );
        self.app.add_system(system_descriptor);
        println!("Adding system_descriptor DONE");
        self
    }

    fn name<T>(_: &T) -> &'static str {
        std::any::type_name::<T>()
    }

    #[cfg(not(feature = "mocked"))]
    pub fn add_system<Params>(&mut self, system: impl IntoSystemDescriptor<Params>) -> &mut Self {
        Self::add_system_impl(self, system)
    }

    #[cfg(feature = "mocked")]
    pub fn add_system<T: 'static + IntoSystemDescriptor<Params>, Params: 'static>(
        &mut self,
        system: T,
    ) -> &mut Self {
        Self::add_system_impl(self, system)
    }
}

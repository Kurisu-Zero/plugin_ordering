use super::*;

use bevy::{
    ecs::schedule::{ParallelSystemDescriptor, SystemDescriptor},
    prelude::{ParallelSystemDescriptorCoercion, SystemLabel},
    reflect::TypeData,
};
pub struct AppDummy<'a> {
    app: &'a mut App,
    plugin_descriptor: &'a PluginDescriptor,
}

impl AppDummy<'_> {
    pub fn new<'a>(app: &'a mut App, desc: &'a PluginDescriptor) -> AppDummy<'a> {
        AppDummy {
            app,
            plugin_descriptor: desc,
        }
    }

    pub fn build_impl(&mut self) {
        self.plugin_descriptor.ordered_plugin.build_impl(self);
    }

    pub fn add_plugin<T>(&mut self, mut plugin: PluginDescriptor) -> &mut Self {
        //make sure the inner plugin also applies the modifications
        plugin
            .labels
            .append(&mut self.plugin_descriptor.labels.clone());
        plugin
            .after
            .append(&mut self.plugin_descriptor.after.clone());
        plugin
            .before
            .append(&mut self.plugin_descriptor.before.clone());

        plugin.build(self.app);
        self
    }

    fn f<T: SystemLabel + ?Sized>(
        sys: ParallelSystemDescriptor,
        label: Box<T>,
    ) -> ParallelSystemDescriptor {
        //   sys.label(*label)
        todo!();
    }

    fn label_parallel_system(
        &mut self,
        mut system_descriptor: ParallelSystemDescriptor,
    ) -> SystemDescriptor {
        let label = self.plugin_descriptor.labels[0].clone();
        let r = Self::f(system_descriptor, label);
        // for x in &self.plugin_descriptor.labels {
        //     //
        //     system_descriptor = system_descriptor.label(*x);
        // }
        SystemDescriptor::Parallel(r)
    }

    fn add_system_impl<Params>(&mut self, system: impl IntoSystemDescriptor<Params>) -> &mut Self {
        println!("run app_dummy::add_system_impl");

        let system_descriptor = match system.into_descriptor() {
            bevy::ecs::schedule::SystemDescriptor::Parallel(p) => {
                Self::label_parallel_system(self, p)
            }
            bevy::ecs::schedule::SystemDescriptor::Exclusive(e) => todo!(),
        };

        self.app.add_system(system_descriptor);
        self
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

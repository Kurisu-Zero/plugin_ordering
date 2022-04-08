use super::*;
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

    #[cfg(not(feature = "mocked"))]
    pub fn add_system<Params>(&mut self, system: impl IntoSystemDescriptor<Params>) -> &mut Self {
        println!("run app_dummy::release_add_system");
        self.app.add_system(system);
        self
    }

    #[cfg(feature = "mocked")]
    pub fn add_system<T: 'static + IntoSystemDescriptor<Params>, Params: 'static>(
        &mut self,
        system: T,
    ) -> &mut Self {
        println!("run app_dummy::mocked_add_system");
        self.app.add_system(system);
        self
    }
}

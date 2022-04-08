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

    pub fn add_plugin<T>(&mut self, plugin: T) -> &mut Self
    where
        T: Plugin,
    {
        // plugin.build(self);
        // self
        self
    }
    pub fn add_system<Params>(&mut self, system: impl IntoSystemDescriptor<Params>) -> &mut Self {
        //  self.add_system_to_stage(CoreStage::Update, system)
        self
    }
}

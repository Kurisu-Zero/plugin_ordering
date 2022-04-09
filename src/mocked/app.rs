use super::Plugin;
use mockall::*;

mock! {
    pub App {

    pub fn add_plugin<T>(&mut self, plugin: T) -> &mut Self
    where
        T: Plugin{
            // plugin.build(self);
            // self
        }

// mocks DO NOT EXECUTE CODE

   pub fn add_system<T:'static>(&mut self, system: T) -> &mut Self {
       // self.add_system_to_stage(CoreStage::Update, system)
    }

    }
}

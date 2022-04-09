#[cfg(feature = "mocked")]
pub mod mocked {
    pub use crate::inner::*;

    mod app;
    mod plugin;
    pub use self::app::__mock_MockApp::__add_system::Expectation as add_system_Expectation;
    pub use self::{app::MockApp as App, plugin::MockPlugin, plugin::Plugin};
}

#[cfg(not(feature = "mocked"))]
pub mod release {
    pub use crate::inner::*;

    pub use bevy::prelude::{App, Plugin};
}

mod inner {

    #[cfg(feature = "mocked")]
    use crate::mocked::*;
    #[cfg(not(feature = "mocked"))]
    use crate::release::*;

    pub(crate) mod ordered_plugin;
    pub(crate) mod plugin_descriptor;

    pub(crate) use bevy::ecs::schedule::IntoSystemDescriptor;
    // intended usage for doc-test
    pub mod test {
        pub use super::ordered_plugin::sized_label::{FunctionBuilder, LabelFunction};
    }
    pub use ordered_plugin::{app_dummy::AppDummy, OrderedPlugin, PlainDescriptor};
    pub use plugin_descriptor::{PluginDescriptor, PluginDescriptorCoercion};
}

#[cfg(doctest)]
fn example() {
    use crate::release::test::*;
    use bevy::prelude::ParallelSystemDescriptorCoercion;
    let sys = || ();
    let mut p = sys.after("");
    let label1 = "hi";
    let label2 = "some_label";
    let f1 = FunctionBuilder { label: label1 };
    let f2 = FunctionBuilder { label: label2 };
    let v: Vec<Box<dyn LabelFunction>> = vec![Box::new(f1), Box::new(f2)];
    for label_function in &v {
        p = label_function.label()(p);
    }
}

// unsafe impl Sync for TestPlugin {}
// unsafe impl Send for TestPlugin {}

// use bevy::prelude::Plugin;
// use release::OrderedPlugin;

// fn d<T: OrderedPlugin>(o: T) -> impl OrderedPlugin {
//     o
// }
// fn d2(o: impl OrderedPlugin) -> Box<dyn OrderedPlugin> {
//     Box::new(o)
// }

// fn test<Z: OrderedPlugin>(ordered_plugin: Z) {
//     //
//     let x = d2(ordered_plugin);
//     take_plug(x);

//     //
// }

// fn take_plug<T: Plugin>(_plug: T) {}

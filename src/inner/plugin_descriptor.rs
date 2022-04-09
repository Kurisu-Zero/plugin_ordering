use bevy::prelude::SystemLabel;

use super::{
    ordered_plugin::sized_label::{FunctionBuilder, LabelFunction},
    *,
};

// type BoxedSystemLabel = Box<dyn SystemLabel>;
type BoxedOrderedPlugin = Box<dyn OrderedPlugin>;
type BoxedLabelFunction = Box<dyn LabelFunction>;

// pub enum PluginDescriptor {
//     Parallel(PluginDescriptor),
//     //  Exclusive(ExclusivePluginDescriptor),
// }

pub trait IntoPluginDescriptor<Params> {
    fn into_descriptor(self) -> PluginDescriptor;
}

// pub struct PluginLabelMarker;

/////////////////////   PARALLEL    ///////////////////////

// impl IntoPluginDescriptor<()> for PluginDescriptor {
//     fn into_descriptor(self) -> PluginDescriptor {
//         PluginDescriptor::Parallel(self)
//     }
// }

impl<Params, S> IntoPluginDescriptor<Params> for S
where
    S: OrderedPlugin,
{
    fn into_descriptor(self) -> PluginDescriptor {
        new_parallel_descriptor(Box::new(self)).into_descriptor()
    }
}

impl IntoPluginDescriptor<()> for PluginDescriptor {
    fn into_descriptor(self) -> PluginDescriptor {
        self
    }
}

impl IntoPluginDescriptor<()> for BoxedOrderedPlugin {
    fn into_descriptor(self) -> PluginDescriptor {
        new_parallel_descriptor(self).into_descriptor()
    }
}

pub struct PluginDescriptor {
    pub(crate) ordered_plugin: BoxedOrderedPlugin,
    pub(crate) labels: Vec<BoxedLabelFunction>,
    pub(crate) before: Vec<BoxedLabelFunction>,
    pub(crate) after: Vec<BoxedLabelFunction>,
}

fn new_parallel_descriptor(plugin: BoxedOrderedPlugin) -> PluginDescriptor {
    PluginDescriptor {
        ordered_plugin: plugin,
        labels: Vec::new(),
        before: Vec::new(),
        after: Vec::new(),
    }
}

pub trait PluginDescriptorCoercion {
    /// Assigns a label to the system; there can be more than one, and it doesn't have to be unique.
    fn label<T: SystemLabel + Clone>(self, label: T) -> PluginDescriptor;

    /// Specifies that the system should run before systems with the given label.
    fn before<T: SystemLabel + Clone>(self, label: T) -> PluginDescriptor;
    /// Specifies that the system should run after systems with the given label.
    fn after<T: SystemLabel + Clone>(self, label: T) -> PluginDescriptor;
}

impl PluginDescriptorCoercion for PluginDescriptor {
    fn label<T: SystemLabel + Clone>(mut self, label: T) -> PluginDescriptor {
        self.labels.push(Box::new(FunctionBuilder { label }));
        self
    }

    fn before<T: SystemLabel + Clone>(mut self, label: T) -> PluginDescriptor {
        self.before.push(Box::new(FunctionBuilder { label }));
        self
    }

    fn after<T: SystemLabel + Clone>(mut self, label: T) -> PluginDescriptor {
        self.after.push(Box::new(FunctionBuilder { label }));
        self
    }
}

impl<S> PluginDescriptorCoercion for S
where
    S: OrderedPlugin,
{
    fn label<T: SystemLabel + Clone>(self, label: T) -> PluginDescriptor {
        new_parallel_descriptor(Box::new(self)).label(label)
    }

    fn before<T: SystemLabel + Clone>(self, label: T) -> PluginDescriptor {
        new_parallel_descriptor(Box::new(self)).before(label)
    }

    fn after<T: SystemLabel + Clone>(self, label: T) -> PluginDescriptor {
        new_parallel_descriptor(Box::new(self)).after(label)
    }
}

impl PluginDescriptorCoercion for BoxedOrderedPlugin {
    fn label<T: SystemLabel + Clone>(self, label: T) -> PluginDescriptor {
        new_parallel_descriptor(self).label(label)
    }

    fn before<T: SystemLabel + Clone>(self, label: T) -> PluginDescriptor {
        new_parallel_descriptor(self).before(label)
    }

    fn after<T: SystemLabel + Clone>(self, label: T) -> PluginDescriptor {
        new_parallel_descriptor(self).after(label)
    }
}

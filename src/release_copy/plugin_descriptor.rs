use bevy::prelude::SystemLabel;

pub use super::*;

type BoxedSystemLabel = Box<dyn SystemLabel>;
type BoxedOrderedPlugin = Box<dyn OrderedPlugin>;

// pub enum PluginDescriptor {
//     Parallel(PluginDescriptor),
//     //  Exclusive(ExclusivePluginDescriptor),
// }

pub trait IntoPluginDescriptor<Params> {
    fn into_descriptor(self) -> PluginDescriptor;
}

pub struct PluginLabelMarker;

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
    pub(crate) labels: Vec<BoxedSystemLabel>,
    pub(crate) before: Vec<BoxedSystemLabel>,
    pub(crate) after: Vec<BoxedSystemLabel>,
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
    fn label(self, label: impl SystemLabel) -> PluginDescriptor;

    /// Specifies that the system should run before systems with the given label.
    fn before(self, label: impl SystemLabel) -> PluginDescriptor;
    /// Specifies that the system should run after systems with the given label.
    fn after(self, label: impl SystemLabel) -> PluginDescriptor;
}

impl PluginDescriptorCoercion for PluginDescriptor {
    fn label(mut self, label: impl SystemLabel) -> PluginDescriptor {
        self.labels.push(Box::new(label));
        self
    }

    fn before(mut self, label: impl SystemLabel) -> PluginDescriptor {
        self.before.push(Box::new(label));
        self
    }

    fn after(mut self, label: impl SystemLabel) -> PluginDescriptor {
        self.after.push(Box::new(label));
        self
    }
}

impl<S> PluginDescriptorCoercion for S
where
    S: OrderedPlugin,
{
    fn label(self, label: impl SystemLabel) -> PluginDescriptor {
        new_parallel_descriptor(Box::new(self)).label(label)
    }

    fn before(self, label: impl SystemLabel) -> PluginDescriptor {
        new_parallel_descriptor(Box::new(self)).before(label)
    }

    fn after(self, label: impl SystemLabel) -> PluginDescriptor {
        new_parallel_descriptor(Box::new(self)).after(label)
    }
}

impl PluginDescriptorCoercion for BoxedOrderedPlugin {
    fn label(self, label: impl SystemLabel) -> PluginDescriptor {
        new_parallel_descriptor(self).label(label)
    }

    fn before(self, label: impl SystemLabel) -> PluginDescriptor {
        new_parallel_descriptor(self).before(label)
    }

    fn after(self, label: impl SystemLabel) -> PluginDescriptor {
        new_parallel_descriptor(self).after(label)
    }
}

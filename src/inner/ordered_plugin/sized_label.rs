use bevy::{
    ecs::schedule::ParallelSystemDescriptor,
    prelude::{ParallelSystemDescriptorCoercion, System, SystemLabel},
};

fn test_run<T: SystemLabel + Clone, U: SystemLabel + Clone>(
    label1: T,
    label2: U,
    mut p: ParallelSystemDescriptor,
) {
    let f1 = FunctionBuilder { label: label1 };

    let f2 = FunctionBuilder { label: label2 };

    let v: Vec<Box<dyn LabelFunction>> = vec![Box::new(f1), Box::new(f2)];
    for label_function in &v {
        p = label_function.label()(p);
    }
}

type ParaFunc = dyn Fn(ParallelSystemDescriptor) -> ParallelSystemDescriptor;

pub trait LabelFunction {
    fn label(&self) -> Box<ParaFunc>;
    fn before(&self) -> Box<ParaFunc>;
    fn after(&self) -> Box<ParaFunc>;
    fn dyn_clone(&self) -> Box<dyn LabelFunction>;
}

#[derive(Clone)]
pub struct FunctionBuilder<T: 'static + SystemLabel + Clone> {
    pub label: T,
}

impl<T: 'static + SystemLabel + Clone> LabelFunction for FunctionBuilder<T> {
    fn label(&self) -> Box<ParaFunc> {
        create_label_function(self.label.clone())
    }
    fn before(&self) -> Box<ParaFunc> {
        create_before_function(self.label.clone())
    }

    fn after(&self) -> Box<ParaFunc> {
        create_after_function(self.label.clone())
    }

    fn dyn_clone(&self) -> Box<dyn LabelFunction> {
        Box::new(self.clone())
    }
}

fn create_label_function<T: SystemLabel + Clone>(label: T) -> Box<ParaFunc> {
    Box::new(
        move |p: ParallelSystemDescriptor| -> ParallelSystemDescriptor {
            let x = label.clone();
            p.label(x)
        },
    )
}

fn create_before_function<T: SystemLabel + Clone>(label: T) -> Box<ParaFunc> {
    Box::new(
        move |p: ParallelSystemDescriptor| -> ParallelSystemDescriptor {
            let x = label.clone();
            p.after(x)
        },
    )
}

fn create_after_function<T: SystemLabel + Clone>(label: T) -> Box<ParaFunc> {
    Box::new(
        move |p: ParallelSystemDescriptor| -> ParallelSystemDescriptor {
            let x = label.clone();
            p.after(x)
        },
    )
}

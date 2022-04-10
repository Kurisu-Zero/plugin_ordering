#[cfg(feature = "mocked")]
pub mod mocked {
    pub use crate::inner::*;

    mod app;
    mod plugin;
    pub use self::app::__mock_MockApp::__add_system::Expectation as add_system_Expectation;
    pub use self::{
        app::MockApp as App_internal, plugin::MockPlugin, plugin::Plugin as Plugin_internal,
    };
}

#[cfg(not(feature = "mocked"))]
pub mod release {
    pub use crate::inner::*;

    pub use bevy::prelude::{App as App_internal, Plugin as Plugin_internal};
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
    pub use ordered_plugin::{app_dummy::AppDummy as App, OrderedPlugin, PlainDescriptor};
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

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct Data;
trait A {
    fn do_stuff(&self, data: &mut Data);

    fn mutate_me(&mut self);
}
struct DescriptorOfA {
    boxed_a: Box<dyn A>,
    //
    other_Data: Vec<String>,
}
fn do_stuff_function(data: &mut Data, a_descriptor: &DescriptorOfA) {
    // Do some known stuff to data, based on a_descriptor
}

fn example<T>(data: &mut Data, my_a: T)
where
    T: 'static + A,
{
    let my_boxed_a: Box<dyn A> = Box::new(my_a);
    let a_descriptor = DescriptorOfA {
        boxed_a: my_boxed_a,
        other_Data: Vec::new(),
    };
    do_stuff_function(data, &a_descriptor);
}

trait B {
    fn do_stuff(&self, data: &mut Data);
}

impl B for dyn A {
    fn do_stuff(&self, data: &mut Data) {
        // what we want to do:
        //
        // let my_boxed_a: Box<dyn A> = Box::new(self);
        // let a_descriptor = DescriptorOfA {
        //    boxed_a: my_boxed_a,
        //    other_Data: Vec::new(),
        // };
        // do_stuff_function(data, &a_descriptor);

        //  this does not work: Box::new(self) is type
        //  Box<&dyn A> not Box<dyn A>

        // let's try dereferencing self before Boxing it:
        // let my_boxed_a: Box<dyn A> = Box::new(*self);

        // also does not work, we cannot move the content
        // behind the self reference to the heap.
        // moving the content would produce bugs in other
        // parts of the program that also reference the self struct.

        // Well but this SHOULD work like the example above, right?
        // What could go wrong?

        // We have an immutable borrow of self. We do not own the memory,
        // it is freed somewhere else. Also some other parts of the
        // program might also hold immutable borrows of self.
        // for example in the Data argument, one of those could be stored.
        // and maybe that is accessed in the function
        // do_stuff_function(data, &a_descriptor);
        // So the compiler enforces that we do not do anything with
        // the self borrow that it cannot prove to be safe.

        // A Box owns it's content which is normally on the heap
        // so it frees the memory when it goes out of scope

        // There is another problem:
        // the self borrow is of type &dyn A, which is a fat pointer
        // that holds the adress of the method vtable and the adress of the beginning
        // of the concrete struct.
        // so even if we could bypass the borrow restriction,
        // we wouldn't be able to move self to the heap, because
        // we don't know how many bytes to move anyway

        // We don't want to move to heap anyway though.

        // with a bit of pointer trickery we can create the Box in place.
        // But if we arent careful the Box will try to free it's content
        // when it goes out of scope. Also the compiler cannot ensure
        // the immutability anymore. But mutating would be unsafe because
        // other threads might try to read the value at the same time. (data race)
        // with an unsafe block we can promise the compiler that we
        // will uphold the rules even though it cannot verify that.

        let const_pointer_to_a: *const dyn A = self as *const dyn A;
        let mut_pointer_to_a: *mut dyn A = const_pointer_to_a as *mut dyn A;

        // So let's assure the compiler that we in fact will not break the
        // rules:
        let my_boxed_a: Box<dyn A> = unsafe {
            // #SAFETY: We have to make sure that the BOX will NOT be dropped! (aka go out of scope)
            // #SAFETY: We have to make sure that the Box and it's content cannot
            //          escape this function in any way
            // #SAFETY: We cannot modify self through the box
            Box::from_raw(mut_pointer_to_a)
        };

        // now we could go on with:
        //
        // let a_descriptor = DescriptorOfA {
        //     boxed_a: my_boxed_a,
        //     other_Data: Vec::new(),
        // };
        // do_stuff_function(data, &a_descriptor);

        // this would compile now

        // if we let a_descriptor go out of scope here it will be cleaned up
        // and with it the box, which would free the memory, that we do not own.
        // That would be terrible.
        // but how to avoid that?
        // when a_descriptor goes out of scope it cannot hold that box anymore
        // so we need to make it mutable.

        let mut a_descriptor = DescriptorOfA {
            boxed_a: my_boxed_a,
            other_Data: Vec::new(),
        };
        do_stuff_function(data, &a_descriptor);

        // still compiles. But we have to be extra careful because now
        // a_descriptor.boxed_a.mutate_me(); would also compile.
        // calling do_stuff_function(data, &a_descriptor) is fine though,
        // because we pass an immutable borrow.

        // now we can switch out the held box with a dummy box that is
        // allowed to be freed, because we properly allocate and own it and its contents.

        let mut swap_box: Box<dyn A> = Box::new(EmptyA);
        std::mem::swap(&mut a_descriptor.boxed_a, &mut swap_box);

        // now the swap_box is the one we have to prevent of going out of scope:
        // we turning the box into a raw pointer by consuming it without dropping it's
        // content:
        let pointer_we_do_not_care_about: *mut dyn A = Box::into_raw(swap_box);
        // this does not need to be placed in an unsafe {} because the worst
        // that could happen here is that we do NOT free memory, but
        // the compiler does not guarantee the absence of memory leaks in your
        // program.

        // arbitrary complex cycles of owned Boxes that will never go out of scope
        // cannot be prevented at compile time for example

        // we done it!
    }
}

struct EmptyA;
impl A for EmptyA {
    fn do_stuff(&self, data: &mut Data) {
        unreachable!()
    }
    fn mutate_me(&mut self) {
        unreachable!()
    }
}

impl B for Box<dyn A> {
    fn do_stuff(&self, data: &mut Data) {
        // we want to delegate to the implementation of do_stuff of B for dyn A.
        // here self is of type &Box<dyn A>.  we need a double deref to get to dyn A
        // (**self).do_stuff(data);

        // sadly as A also has a function with the same name that one is called.

        // Let's upgrade our function call syntax HAHAHAHAHAHAHAHA
        <dyn A as B>::do_stuff(&**self, data);
    }
}

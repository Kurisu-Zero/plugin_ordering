use crate::release_copy::*;

pub struct DoNothingOrderedPlugin;
impl OrderedPlugin for DoNothingOrderedPlugin {
    fn build_impl(&self, app: &mut AppDummy) {}
}

pub struct Add2SystemsOrderedPlugin;
impl OrderedPlugin for Add2SystemsOrderedPlugin {
    fn build_impl(&self, app: &mut AppDummy) {
        app.add_system(test_system);
        app.add_system(test_system_2);
    }
}

fn test_system() {}
fn test_system_2() {}
struct Plug;
impl Plugin for Plug {
    fn build(&self, app: &mut App) {}
}

#[test]
fn do_nothing_plugin_does_nothing() {
    let mut mock_app = App::new();
    mock_app.expect_add_plugin::<PluginDescriptor>().never();
    mock_app.expect_add_system::<SystemDescriptor, ()>().never();

    DoNothingOrderedPlugin.as_is().build(&mut mock_app);
    DoNothingOrderedPlugin
        .label("test_label")
        .after("test_after")
        .before("test_before")
        .build(&mut mock_app);
}

#[test]
fn add_2_systems_plugin_does_add_2() {
    let mut mock_app = App::new();
    mock_app.expect_add_plugin::<PluginDescriptor>().never();
    mock_app
        .expect_add_system::<SystemDescriptor, ()>()
        .times(2);

    Add2SystemsOrderedPlugin.as_is().build(&mut mock_app);
}

// mock_app
//         .expect_add_system::<SystemDescriptor, ()>()
//         .times(2);

//     Add2SystemsOrderedPlugin
//         .label("test_label")
//         .after("test_after")
//         .before("test_before")
//         .build(&mut mock_app);

#[test]
fn test() {
    // let test_plugin = TestPlugin;
    // let plug = Plug;
    // plug.build(&mut mock_app);

    // let app = App::new().add_plugin(
    //     TestPlugin
    //         .label("test_label")
    //         .after("test_after")
    //         .before("test_before"),
    // );

    // todo!("Mock app and plugin")
}
#[test]
fn can_call_using_easy_syntax() {
    let mut app = App::new();
    app.expect_add_plugin::<PluginDescriptor>().times(2);

    app.add_plugin(
        DoNothingOrderedPlugin
            .label("test_label")
            .after("test_after")
            .before("test_before"),
    ); // mocked objects cannot return self :(
    app.add_plugin(DoNothingOrderedPlugin.as_is());
}

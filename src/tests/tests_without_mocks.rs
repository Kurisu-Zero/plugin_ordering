use crate::release::*;

pub struct TestPlugin;
impl OrderedPlugin for TestPlugin {
    fn build_impl(&self, app: &mut AppDummy) {
        println!("build_impl TestPlugin");
    }
}

fn test_system() {}
fn test_system_in_plugin() {}

#[test]
fn can_call_using_easy_syntax() {
    let mut app = App::new()
        .add_system(test_system)
        .add_plugin(
            TestPlugin
                .label("test_label")
                .after("test_after")
                .before("test_before"),
        )
        .add_plugin(TestPlugin.as_is());
}

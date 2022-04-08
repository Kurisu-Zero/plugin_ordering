use super::*;

use mockall::automock;

#[automock]
pub trait Plugin: Any + Send + Sync {
    /// Configures the [`App`] to which this plugin is added.
    fn build(&self, app: &mut App);
    /// Configures a name for the [`Plugin`]. Primarily for debugging.
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

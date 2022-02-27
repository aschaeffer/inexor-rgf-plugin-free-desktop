use std::sync::Arc;

use async_trait::async_trait;
use freedesktop_desktop_entry::{DesktopEntry, PathSource};
use inexor_rgf_core_model::EntityInstance;
use uuid::Uuid;

use crate::plugins::PluginContext;

pub const DESKTOP_ENTRY: &str = "desktop_entry";

// Ensure stable UUIDs for the desktop entries
pub static NAMESPACE_DESKTOP_ENTRY: Uuid = Uuid::from_u128(0x6ba7b8109dad11d150b400c04fd530c7);

#[async_trait]
pub trait DesktopEntryManager: Send + Sync {
    fn init(&self);

    fn shutdown(&self);

    fn set_context(&self, context: Arc<dyn PluginContext>);

    fn load_desktop_entries(&self);

    fn create_desktop_entry(&self, path_src: PathSource, desktop_entry: DesktopEntry) -> EntityInstance;
}

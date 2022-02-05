use std::fs;
use std::sync::{Arc, RwLock};

use crate::di::*;
use crate::model::EntityInstance;
use async_trait::async_trait;
use freedesktop_desktop_entry::{default_paths, DesktopEntry, Iter, PathSource};
use serde_json::json;
use uuid::Uuid;

use crate::api::{DesktopEntryManager, DESKTOP_ENTRY, NAMESPACE_DESKTOP_ENTRY};
use crate::builder::EntityInstanceBuilder;
use crate::plugins::PluginContext;
use crate::properties::DesktopEntryProperties;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<std::sync::Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    return PluginContextContainer(RwLock::new(None));
}

#[component]
pub struct DesktopEntryManagerImpl {
    context: PluginContextContainer,
}

impl DesktopEntryManagerImpl {}

#[async_trait]
#[provides]
impl DesktopEntryManager for DesktopEntryManagerImpl {
    fn init(&self) {
        self.load_desktop_entries();
    }

    fn shutdown(&self) {
        // TODO: self.remove_desktop_entries();
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) {
        self.context.0.write().unwrap().replace(context.clone());
    }

    fn load_desktop_entries(&self) {
        let reader = self.context.0.read().unwrap();
        let entity_instance_manager = reader.as_ref().unwrap().get_entity_instance_manager().clone();

        for (path_src, path) in Iter::new(default_paths()) {
            if let Ok(bytes) = fs::read_to_string(&path) {
                if let Ok(desktop_entry) = DesktopEntry::decode(&path, &bytes) {
                    let entity_instance = self.create_desktop_entry(path_src, desktop_entry);
                    let _entity_instance = entity_instance_manager.create(entity_instance);
                    // println!("{:?}: {}\n---\n{}", path_src, path.display(), entry);
                }
            }
        }
    }

    fn create_desktop_entry(&self, path_src: PathSource, desktop_entry: DesktopEntry) -> EntityInstance {
        let name = desktop_entry.name(None).map_or(String::new(), |s| String::from(s));
        let comment = desktop_entry.comment(None).map_or(String::new(), |s| String::from(s));
        let keywords = desktop_entry
            .comment(None)
            .map_or(String::new(), |s| String::from(s))
            .split(";")
            .into_iter()
            .map(|s| String::from(s))
            .collect::<Vec<_>>();
        // let groups = desktop_entry
        //     .groups
        //     .iter()
        //     .map(|(g, key_map)| (String::from(*g), key_map.into_iter().collect()))
        //     .collect::<HashMap<String, HashMap<String, _>>>();
        let categories = desktop_entry
            .categories()
            .map_or(String::new(), |s| String::from(s))
            .split(";")
            .into_iter()
            .filter(|s| *s != "")
            .map(|s| String::from(s))
            .collect::<Vec<_>>();
        EntityInstanceBuilder::new(DESKTOP_ENTRY)
            .id(Uuid::new_v5(&NAMESPACE_DESKTOP_ENTRY, desktop_entry.appid.as_bytes()))
            .property(DesktopEntryProperties::NAME.as_ref(), json!(name))
            .property(DesktopEntryProperties::APPID.as_ref(), json!(desktop_entry.appid))
            .property(DesktopEntryProperties::SOURCE.as_ref(), json!(format!("{:?}", path_src)))
            .property(DesktopEntryProperties::PATH.as_ref(), json!(desktop_entry.path.to_str().unwrap_or("")))
            .property(DesktopEntryProperties::CATEGORIES.as_ref(), json!(categories))
            .property(DesktopEntryProperties::GROUPS.as_ref(), json!([]))
            .property(DesktopEntryProperties::EXEC.as_ref(), json!(desktop_entry.exec().unwrap_or("")))
            .property(DesktopEntryProperties::ICON.as_ref(), json!(desktop_entry.icon().unwrap_or("")))
            .property(DesktopEntryProperties::TYPE.as_ref(), json!(desktop_entry.type_().unwrap_or("")))
            .property(DesktopEntryProperties::COMMENT.as_ref(), json!(comment))
            .property(DesktopEntryProperties::KEYWORDS.as_ref(), json!(keywords))
            .property(DesktopEntryProperties::MIME_TYPE.as_ref(), json!(desktop_entry.mime_type().unwrap_or("")))
            .property(DesktopEntryProperties::TERMINAL.as_ref(), json!(desktop_entry.terminal()))
            .property(DesktopEntryProperties::NO_DISPLAY.as_ref(), json!(desktop_entry.no_display()))
            .property(DesktopEntryProperties::STARTUP_NOTIFY.as_ref(), json!(desktop_entry.startup_notify()))
            .get()
    }
}

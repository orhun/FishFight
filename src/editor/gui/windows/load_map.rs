use macroquad::{
    experimental::collections::storage,
    prelude::*,
    ui::{hash, widgets, Ui},
};

use crate::gui::{GuiResources, ELEMENT_MARGIN, LIST_BOX_ENTRY_HEIGHT};

use crate::map::Map;

use super::{ButtonParams, EditorAction, EditorContext, Window, WindowParams};
use crate::Resources;

pub struct LoadMapWindow {
    params: WindowParams,
    index: Option<usize>,
}

impl LoadMapWindow {
    pub fn new() -> Self {
        let params = WindowParams {
            title: Some("Open Map".to_string()),
            size: vec2(350.0, 350.0),
            ..Default::default()
        };

        LoadMapWindow {
            params,
            index: None,
        }
    }
}

impl Window for LoadMapWindow {
    fn get_params(&self) -> &WindowParams {
        &self.params
    }

    fn draw(
        &mut self,
        ui: &mut Ui,
        size: Vec2,
        _map: &Map,
        _ctx: &EditorContext,
    ) -> Option<EditorAction> {
        let id = hash!("load_map_window");

        {
            let gui_resources = storage::get::<GuiResources>();
            ui.push_skin(&gui_resources.skins.list_box_no_bg);
        }

        let size = vec2(size.x, size.y - ELEMENT_MARGIN);
        widgets::Group::new(hash!(id, "list_box"), size)
            .position(Vec2::ZERO)
            .ui(ui, |ui| {
                let resources = storage::get::<Resources>();

                let entry_size = vec2(size.x, LIST_BOX_ENTRY_HEIGHT);

                for (i, map_resource) in resources.maps.iter().enumerate() {
                    let mut is_selected = false;
                    if let Some(index) = self.index {
                        is_selected = index == i;
                    }

                    if is_selected {
                        let gui_resources = storage::get::<GuiResources>();
                        ui.push_skin(&gui_resources.skins.list_box_selected);
                    }

                    let entry_position = vec2(0.0, i as f32 * entry_size.y);

                    let entry_btn = widgets::Button::new("")
                        .size(entry_size)
                        .position(entry_position);

                    if entry_btn.ui(ui) {
                        self.index = Some(i);
                    }

                    ui.label(entry_position, &map_resource.meta.path);

                    if is_selected {
                        ui.pop_skin();
                    }
                }
            });

        ui.pop_skin();

        None
    }

    fn get_buttons(&self, _map: &Map, _ctx: &EditorContext) -> Vec<ButtonParams> {
        let mut res = Vec::new();

        let mut open_action = None;
        let mut import_action = None;

        if let Some(index) = self.index {
            let open_batch = self.get_close_action().then(EditorAction::LoadMap(index));
            open_action = Some(open_batch);

            let import_batch = self
                .get_close_action()
                .then(EditorAction::OpenImportWindow(index));
            import_action = Some(import_batch);
        }

        res.push(ButtonParams {
            label: "Open",
            action: open_action,
            ..Default::default()
        });

        res.push(ButtonParams {
            label: "Import",
            action: import_action,
            ..Default::default()
        });

        res.push(ButtonParams {
            label: "Cancel",
            action: Some(self.get_close_action()),
            ..Default::default()
        });

        res
    }
}

impl Default for LoadMapWindow {
    fn default() -> Self {
        Self::new()
    }
}

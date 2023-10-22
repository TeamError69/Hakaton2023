use eframe::{App, CreationContext};
use egui::{CentralPanel, SidePanel, TopBottomPanel};
use tinyrand::{RandRange, StdRand};

use crate::structures::{view::View, ListWareHouse, WareHouse};

pub struct VisualsApp {
    wh_id: Option<u32>,
    fl_id: Option<u32>,
    l_panel: bool,
    panel: Box<dyn View>,
}

impl VisualsApp {
    pub fn new(_cc: &CreationContext) -> Self {
        let mut list = ListWareHouse { warehouses: vec![] };
        let mut rand = StdRand::default();
        for i in 0..30 {
            list.warehouses.push(WareHouse {
                id_me: i,
                active_tasks: rand.next_range(0..10),
                awaiting_fls: rand.next_range(0..8),
                forklifts: vec![],
            });
        }
        let wh = list.warehouses[4].clone();
        Self {
            wh_id: None,
            fl_id: None,
            l_panel: false,
            panel: Box::new(wh),
        }
    }
}

impl App for VisualsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("Top panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("left_menu").clicked() {
                        self.l_panel = !self.l_panel;
                    }
                });
            });

        SidePanel::left("Menu bar").show_animated(ctx, self.l_panel, |ui| {
            ui.heading("This is side panel");
            ui.label("Warehouse ID");
            integer_edit_field(ui, &mut self.wh_id);
            ui.label("Forklift ID");
            integer_edit_field(ui, &mut self.fl_id);
            ui.separator();
            if ui.button("SEARCH").clicked() {
                println!("This should call for selected Warehouse/Forklift");
            }
        });
        CentralPanel::default().show(ctx, |ui| {
            self.panel.view(ui);
        });
    }
}

fn integer_edit_field(ui: &mut egui::Ui, value: &mut Option<u32>) -> egui::Response {
    let mut tmp_val = match value {
        None => "".to_string(),
        Some(val) => {
            format!("{}", val)
        }
    };
    let res = ui.text_edit_singleline(&mut tmp_val);
    if let Ok(val) = tmp_val.parse::<u32>() {
        *value = Some(val);
        return res;
    }

    *value = None;
    res
}

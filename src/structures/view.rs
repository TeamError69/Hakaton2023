use egui::{Color32, Rect, ScrollArea, Ui};
use egui_plot::Plot;

use super::{ListWareHouse, WareHouse};

pub trait View {
    fn view(&self, ui: &mut Ui);
}

impl View for ListWareHouse {
    fn view(&self, ui: &mut Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.separator();
            for warehouse in &self.warehouses {
                ui.horizontal(|ui| {
                    ui.heading(format!("Warehouse #{}", warehouse.id_me));
                    ui.vertical(|ui| {
                        ui.label(format!("Active: {}", warehouse.active_tasks));
                        ui.label(format!("Awaiting: {}", warehouse.awaiting_fls));
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Show").clicked() {}
                    });
                });
                ui.separator();
            }
        });
    }
}

impl View for WareHouse {
    fn view(&self, ui: &mut Ui) {
        use egui_plot::*;
        ui.heading(format!("Warehouse #{}", self.id_me));
        ui.label(format!("Active: {}", self.active_tasks));
        ui.label(format!("Awaiting: {}", self.awaiting_fls));
        Plot::new("warehouse plot")
            .show_x(false)
            .show_y(false)
            .show_axes(false)
            .data_aspect(1.0)
            .show(ui, |ui| {
                for i in 0..3 {
                    let m = f64::from(i + 1);
                    ui.arrows(
                        Arrows::new(
                            vec![
                                [0.0, 5.0 * (m - 1.0)],
                                [0.0, 5.0 * m],
                                [-10.0, 5.0 * m],
                                [-10.0, 5.0 * m],
                                [-25.0, 5.0 * m],
                            ],
                            vec![
                                [0.0, 5.0 * m],
                                [-10.0, 5.0 * m],
                                [-25.0, 5.0 * m],
                                [-17.5, 5.0 * m - 2.0],
                                [-32.5, 5.0 * m - 2.0],
                            ],
                        )
                        .tip_length(15.0),
                    );
                }
            });
    }
}

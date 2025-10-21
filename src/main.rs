use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;

struct ImageViewer {
    current_texture: Option<egui::TextureHandle>,
    current_path: Option<PathBuf>,
    zoom_factor: f32,
    pan_offset: egui::Vec2,
}

impl Default for ImageViewer {
    fn default() -> Self {
        Self {
            current_texture: None,
            current_path: None,
            zoom_factor: 1.0,
            pan_offset: egui::Vec2::ZERO,
        }
    }
}

impl eframe::App for ImageViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 顶部菜单栏
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open Image").clicked() {
                        if let Some(path) = FileDialog::new()
                            .add_filter("Image files", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
                            .pick_file()
                        {
                            self.load_image(ctx, &path);
                        }
                        ui.close();
                    }
                    
                    if ui.button("Exit").clicked() {
                        std::process::exit(0);
                    }
                });
                
                ui.menu_button("View", |ui| {
                    if ui.button("Reset Zoom").clicked() {
                        self.zoom_factor = 1.0;
                        self.pan_offset = egui::Vec2::ZERO;
                    }
                    
                    if ui.button("Fit to Window").clicked() {
                        if let Some(texture) = &self.current_texture {
                            let available_size = ui.available_size();
                            let image_size = texture.size_vec2();
                            let scale_x = available_size.x / image_size.x;
                            let scale_y = available_size.y / image_size.y;
                            self.zoom_factor = scale_x.min(scale_y) * 0.9; // Leave some margin
                            self.pan_offset = egui::Vec2::ZERO;
                        }
                    }
                });
            });
        });

        // Toolbar
        egui::TopBottomPanel::bottom("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Zoom:");
                ui.add(egui::Slider::new(&mut self.zoom_factor, 0.1..=5.0));
                
                if ui.button("Reset").clicked() {
                    self.zoom_factor = 1.0;
                    self.pan_offset = egui::Vec2::ZERO;
                }
                
                ui.separator();
                
                if let Some(path) = &self.current_path {
                    ui.label(format!("Current file: {}", path.display()));
                } else {
                    ui.label("No image selected");
                }
            });
        });

        // Main display area
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    if let Some(texture) = &self.current_texture {
                        let available_size = ui.available_size();
                        let image_size = texture.size_vec2() * self.zoom_factor;
                        
                        // Calculate image position in scroll area
                        let image_rect = egui::Rect::from_center_size(
                            (available_size / 2.0 + self.pan_offset).to_pos2(),
                            image_size,
                        );
                        
                        // Handle mouse interactions
                        let response = ui.allocate_rect(image_rect, egui::Sense::click_and_drag());
                        
                        if response.dragged() {
                            self.pan_offset += response.drag_delta();
                        }
                        
                        if response.hovered() {
                            let scroll_delta = ui.input(|i| i.raw_scroll_delta.y);
                            if scroll_delta != 0.0 {
                                let zoom_speed = 0.1;
                                let old_zoom = self.zoom_factor;
                                self.zoom_factor *= 1.0 + scroll_delta * zoom_speed;
                                self.zoom_factor = self.zoom_factor.clamp(0.1, 10.0);
                                
                                // Keep mouse position unchanged during zoom
                                let mouse_pos = ui.input(|i| i.pointer.hover_pos().unwrap_or_default());
                                let zoom_ratio = self.zoom_factor / old_zoom;
                                self.pan_offset = mouse_pos - (mouse_pos - self.pan_offset) * zoom_ratio;
                            }
                        }
                        
                        // Draw image
                        ui.put(image_rect, egui::Image::new(texture));
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.label("Please select an image file");
                            ui.add_space(10.0);
                            if ui.button("Open Image").clicked() {
                                if let Some(path) = FileDialog::new()
                                    .add_filter("Image files", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
                                    .pick_file()
                                {
                                    self.load_image(ctx, &path);
                                }
                            }
                        });
                    }
                });
        });
    }
}

impl ImageViewer {
    fn load_image(&mut self, ctx: &egui::Context, path: &std::path::Path) {
        match image::open(path) {
            Ok(img) => {
                let rgba_image = img.to_rgba8();
                let size = [rgba_image.width() as usize, rgba_image.height() as usize];
                let pixels = rgba_image.into_raw();
                
                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
                self.current_texture = Some(ctx.load_texture("loaded_image", color_image, Default::default()));
                self.current_path = Some(path.to_path_buf());
                self.zoom_factor = 1.0;
                self.pan_offset = egui::Vec2::ZERO;
            }
            Err(e) => {
                eprintln!("Failed to load image: {}", e);
            }
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    
        eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(ImageViewer::default()))),
    )
}
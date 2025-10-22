use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;
use image::DynamicImage;
use std::fs;

struct ImageViewer {
    current_texture: Option<egui::TextureHandle>,
    current_path: Option<PathBuf>,
    zoom_factor: f32,
    pan_offset: egui::Vec2,
    original_image: Option<DynamicImage>,
    is_grayscale: bool,
    temp_path: Option<PathBuf>,
}

impl Default for ImageViewer {
    fn default() -> Self {
        Self {
            current_texture: None,
            current_path: None,
            zoom_factor: 1.0,
            pan_offset: egui::Vec2::ZERO,
            original_image: None,
            is_grayscale: false,
            temp_path: None,
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
                            .add_filter("Image files", &["png"])
                            .pick_file()
                        {
                            self.load_image(ctx, &path);
                        }
                        ui.close();
                    }
                    
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
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
            ui.horizontal(|ui|{
                if ui.button("Original").clicked() {
                    self.original(ctx);
                };

                if ui.button("Black & White").clicked() {
                    self.toggle_grayscale(ctx);
                };

                if ui.button("Fast Save").clicked() {
                    self.fast_save();
                };
            });
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
            // Draw checkerboard background for the entire area
            self.draw_checkerboard_background(ui);
            
            // Handle zoom with mouse wheel for the entire area
            if ui.input(|i| i.raw_scroll_delta.y != 0.0) {
                let scroll_delta = ui.input(|i| i.raw_scroll_delta.y);
                let zoom_speed = 0.02;
                let old_zoom = self.zoom_factor;
                let zoom_delta = scroll_delta * zoom_speed;
                self.zoom_factor *= 1.0 + zoom_delta;
                self.zoom_factor = self.zoom_factor.clamp(0.1, 5.0);
                
                // Keep the point under the mouse stationary during zoom
                if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                    let available_size = ui.available_size();
                    let center = available_size / 2.0;
                    let zoom_ratio = self.zoom_factor / old_zoom;
                    let mouse_offset = mouse_pos - center;
                    self.pan_offset = mouse_offset - (mouse_offset - self.pan_offset) * zoom_ratio;
                }
            }
            
            if let Some(texture) = &self.current_texture {
                let available_size = ui.available_size();
                let original_size = texture.size_vec2();
                let scaled_size = original_size * self.zoom_factor;
                
                // Calculate image position (centered with pan offset)
                let center = available_size / 2.0;
                let image_pos = center + self.pan_offset - scaled_size / 2.0;
                let image_rect = egui::Rect::from_min_size(image_pos.to_pos2(), scaled_size);
                
                // Handle mouse interactions for dragging
                let response = ui.allocate_rect(image_rect, egui::Sense::click_and_drag());
                
                if response.dragged() {
                    self.pan_offset += response.drag_delta();
                }
                
                // Draw image with scaling (no smoothing for pixel-perfect display)
                let image = egui::Image::new(texture)
                    .fit_to_exact_size(scaled_size)
                    .texture_options(egui::TextureOptions::NEAREST);
                ui.put(image_rect, image);
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Please select an image file");
                    ui.add_space(10.0);
                    if ui.button("Open Image").clicked() {
                        if let Some(path) = FileDialog::new()
                            .add_filter("Image files", &["png"])
                            .pick_file()
                        {
                            self.load_image(ctx, &path);
                        }
                    }
                });
            }
        });
    }
}

impl ImageViewer {
    fn load_image(&mut self, ctx: &egui::Context, path: &std::path::Path) {
        match image::open(path) {
            Ok(img) => {
                // Clean up any previous temp file
                if let Some(old_temp) = self.temp_path.take() {
                    let _ = fs::remove_file(old_temp);
                }

                // Store original image
                self.original_image = Some(img.clone());
                self.is_grayscale = false;
                self.current_path = Some(path.to_path_buf());
                self.zoom_factor = 1.0;
                self.pan_offset = egui::Vec2::ZERO;

                // Prepare initial texture
                self.update_texture_from_image(ctx, &img);

                // Create temp file alongside original (foo.png -> foo.egui_tmp.png)
                if let Some(original_path) = &self.current_path {
                    if let Some(stem) = original_path.file_stem().and_then(|s| s.to_str()) {
                        let mut temp_path = original_path.clone();
                        temp_path.set_file_name(format!("{}.egui_tmp.png", stem));
                        // Write current state into temp
                        let _ = img.save(&temp_path);
                        self.temp_path = Some(temp_path);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to load image: {}", e);
            }
        }
    }
    
    fn toggle_grayscale(&mut self, ctx: &egui::Context) {
        if let Some(original_img) = &self.original_image {
            let processed_img = if self.is_grayscale {
                // Convert back to color
                original_img.clone()
            } else {
                // Convert to grayscale using custom ITU-R BT.601 logic
                self.convert_to_grayscale_custom(original_img)
            };
            
            self.is_grayscale = !self.is_grayscale;
            self.update_texture_from_image(ctx, &processed_img);
            self.save_to_temp(&processed_img);
        }
    }

    fn original(&mut self, ctx: &egui::Context) {
        if let Some(original_img) = self.original_image.clone() {
            // Reset to original state
            self.is_grayscale = false;
            self.update_texture_from_image(ctx, &original_img);
            self.save_to_temp(&original_img);
        }
    }
    
    fn fast_save(&self) {
        if let (Some(current_path), Some(temp_path)) = (&self.current_path, &self.temp_path) {
            match fs::copy(temp_path, current_path) {
                Ok(_) => {
                    println!("Image saved successfully to: {}", current_path.display());
                }
                Err(e) => {
                    eprintln!("Failed to save image: {}", e);
                }
            }
        } else {
            eprintln!("No image loaded or temp file not available for saving");
        }
    }
    
    fn update_texture_from_image(&mut self, ctx: &egui::Context, img: &DynamicImage) {
        let rgba_image = img.to_rgba8();
        let size = [rgba_image.width() as usize, rgba_image.height() as usize];
        let pixels = rgba_image.into_raw();
        
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
        let texture_options = egui::TextureOptions {
            magnification: egui::TextureFilter::Nearest,
            minification: egui::TextureFilter::Nearest,
            ..Default::default()
        };
        self.current_texture = Some(ctx.load_texture("loaded_image", color_image, texture_options));
    }

    fn save_to_temp(&self, img: &DynamicImage) {
        if let Some(temp) = &self.temp_path {
            let _ = img.save(temp);
        }
    }

    fn convert_to_grayscale_custom(&self, img: &DynamicImage) -> DynamicImage {
        let rgba_image = img.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let mut pixels = rgba_image.into_raw();
        
        // Process pixels in RGBA format (4 bytes per pixel)
        for chunk in pixels.chunks_exact_mut(4) {
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            let a = chunk[3];
            
            // ITU-R BT.601 近似加权，整数计算避免浮点
            let luma = ((299 * r + 587 * g + 114 * b) / 1000) as u8;
            
            // 新的alpha处理逻辑：
            // - alpha为0的像素设为(0,0,0,0)
            // - alpha不为0的像素alpha设为255
            if a == 0 {
                chunk[0] = 0;
                chunk[1] = 0;
                chunk[2] = 0;
                chunk[3] = 0;
            } else {
                chunk[0] = luma;
                chunk[1] = luma;
                chunk[2] = luma;
                chunk[3] = 255;
            }
        }
        
        // Create new image from processed pixels
        DynamicImage::ImageRgba8(image::RgbaImage::from_raw(width, height, pixels).unwrap())
    }

    fn draw_checkerboard_background(&self, ui: &mut egui::Ui) {
        let checker_size = 16.0; // Size of each checker square
        let light_color = egui::Color32::from_gray(200);
        let dark_color = egui::Color32::from_gray(150);
        
        // Calculate the area to fill with checkerboard (entire available area)
        let available_size = ui.available_size();
        let checker_rect = egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            available_size
        );
        
        // Draw checkerboard pattern
        let painter = ui.painter();
        
        // Calculate how many checker squares we need to cover the area
        let cols = ((checker_rect.width() / checker_size) + 1.0).ceil() as i32;
        let rows = ((checker_rect.height() / checker_size) + 2.0).ceil() as i32;
        
        for row in 0..rows {
            for col in 0..cols {
                let x = checker_rect.min.x + col as f32 * checker_size;
                let y = checker_rect.min.y + row as f32 * checker_size;
                
                let square_rect = egui::Rect::from_min_size(
                    egui::Pos2::new(x, y),
                    egui::Vec2::new(checker_size, checker_size)
                );
                
                // Draw all squares in the available area
                let is_light = (row + col) % 2 == 0;
                let color = if is_light { light_color } else { dark_color };
                
                painter.rect_filled(square_rect, 0.0, color);
            }
        }
    }
}

impl Drop for ImageViewer {
    fn drop(&mut self) {
        if let Some(temp) = self.temp_path.take() {
            let _ = fs::remove_file(temp);
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
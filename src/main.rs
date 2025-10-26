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
    show_color_reflection: bool,
    // Color reflection related fields
    slider_amount_input: String,
    slider_amount: Option<usize>,
    slider_values: Vec<f32>,
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
            show_color_reflection: false,
            slider_amount_input: String::new(),
            slider_amount: None,
            slider_values: Vec::new(),
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

                if ui.button("color reflection").clicked() {
                    self.show_color_reflection = true;
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

        // Color Reflection Window
        if self.show_color_reflection {
            let mut show_window = self.show_color_reflection;
                egui::Window::new("Color Reflection")
                    .open(&mut show_window)
                    .default_size([800.0, 600.0])
                    .resizable(true)
                    .show(ctx, |ui| {
                        self.show_color_reflection_content(ui, ctx);
                    });
            self.show_color_reflection = show_window;
        }

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

    fn show_color_reflection_content(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("Color Reflection");
        ui.separator();
        
        // 第一行：滑块数量输入框和确认按钮
        ui.horizontal(|ui| {
            ui.label("Slider amount");
            
            // 输入框，限制输入为1-10的正整数
            let mut input_text = self.slider_amount_input.clone();
            let response = ui.add(
                egui::TextEdit::singleline(&mut input_text)
                    .hint_text("1-10")
                    .desired_width(60.0)
            );
            
            // 只允许输入数字
            if response.changed() {
                self.slider_amount_input = input_text
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>();
            }
            
            // 确认按钮
            if ui.button("Confirm Slider Amount").clicked() {
                if let Ok(amount) = self.slider_amount_input.parse::<usize>() {
                    if amount >= 1 && amount <= 10 {
                        self.slider_amount = Some(amount);
                        self.update_slider_values();
                    }
                }
            }
        });
        
        ui.add_space(10.0);
        
        // 第二行：单根滑动条上的多个滑块
        ui.label("Slider positions:");
        
        if let Some(amount) = self.slider_amount {
            // 显示滑块数量
            ui.label(format!("{} sliders on one track", amount));
            
            // 创建单根滑动条区域
            ui.allocate_ui_with_layout(
                egui::Vec2::new(ui.available_width(), 60.0),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    // 绘制滑动条背景
                    let rect = ui.available_rect_before_wrap();
                    let painter = ui.painter();
                    
                    // 绘制亮度渐变背景 (0-255)
                    let gradient_steps = rect.width() as usize;
                    for i in 0..gradient_steps {
                        let x = rect.min.x + i as f32;
                        let brightness = (i as f32 / gradient_steps as f32) * 255.0;
                        let gray_value = brightness as u8;
                        let color = egui::Color32::from_rgb(gray_value, gray_value, gray_value);
                        
                        let step_rect = egui::Rect::from_min_size(
                            egui::Pos2::new(x, rect.min.y),
                            egui::Vec2::new(1.0, rect.height())
                        );
                        painter.rect_filled(step_rect, 0.0, color);
                    }
                    
                    // 滑动条轨道边框
                    painter.rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::from_gray(120)), egui::StrokeKind::Outside);
                    
                    // 绘制滑块
                    for (i, &value) in self.slider_values.iter().enumerate() {
                        // 将0-255的值映射到滑动条位置
                        let normalized_value = value / 255.0;
                        let x = rect.min.x + normalized_value * rect.width();
                        let slider_rect = egui::Rect::from_center_size(
                            egui::Pos2::new(x, rect.center().y),
                            egui::Vec2::new(12.0, 24.0)
                        );
                        
                        // 滑块颜色（可以根据需要调整）
                        let color = egui::Color32::from_rgb(100, 150, 255);
                        painter.rect_filled(slider_rect, 2.0, color);
                        painter.rect_stroke(slider_rect, 0.0, egui::Stroke::new(1.0, egui::Color32::WHITE), egui::StrokeKind::Outside);
                        
                        // 滑块编号
                        painter.text(
                            egui::Pos2::new(x, rect.min.y - 5.0),
                            egui::Align2::CENTER_TOP,
                            format!("{}", i + 1),
                            egui::FontId::proportional(10.0),
                            egui::Color32::WHITE,
                        );
                        
                        // 显示当前值 (0-255)
                        painter.text(
                            egui::Pos2::new(x, rect.max.y + 5.0),
                            egui::Align2::CENTER_TOP,
                            format!("{:.0}", value),
                            egui::FontId::proportional(9.0),
                            egui::Color32::WHITE,
                        );
                    }
                    
                    // 处理鼠标交互
                    let response = ui.allocate_rect(rect, egui::Sense::click_and_drag());
                    
                    if response.dragged() {
                        if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                            let relative_x = (mouse_pos.x - rect.min.x) / rect.width();
                            let relative_x = relative_x.clamp(0.0, 1.0);
                            
                            // 将相对位置转换为0-255的值
                            let value = relative_x * 255.0;
                            
                            // 找到最近的滑块并移动它
                            if let Some(closest_idx) = self.find_closest_slider(value) {
                                self.slider_values[closest_idx] = value;
                            }
                        }
                    }
                    
                    if response.clicked() {
                        if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                            let relative_x = (mouse_pos.x - rect.min.x) / rect.width();
                            let relative_x = relative_x.clamp(0.0, 1.0);
                            
                            // 将相对位置转换为0-255的值
                            let value = relative_x * 255.0;
                            
                            // 点击时也移动最近的滑块
                            if let Some(closest_idx) = self.find_closest_slider(value) {
                                self.slider_values[closest_idx] = value;
                            }
                        }
                    }
                }
            );
        } else {
            ui.label("Please enter slider amount (1-10) and click confirm");
        }
        
        ui.add_space(20.0);
        
        // Confirm Reflection 按钮
        if ui.button("Confirm Reflection").clicked() {
            self.apply_color_reflection(ctx);
        }
        
        ui.add_space(10.0);
        
        if ui.button("Close Window").clicked() {
            self.show_color_reflection = false;
        }
    }
    
    fn update_slider_values(&mut self) {
        if let Some(amount) = self.slider_amount {
            self.slider_values.clear();
            for i in 0..amount {
                // 初始值平均分布，范围0-255
                let value = if amount == 1 {
                    127.0 // 只有一个滑块时放在中间
                } else {
                    (i as f32 / (amount - 1) as f32) * 255.0
                };
                self.slider_values.push(value);
            }
        }
    }
    
    fn find_closest_slider(&self, target_pos: f32) -> Option<usize> {
        if self.slider_values.is_empty() {
            return None;
        }
        
        let mut closest_idx = 0;
        let mut min_distance = (self.slider_values[0] - target_pos).abs();
        
        for (i, &value) in self.slider_values.iter().enumerate() {
            let distance = (value - target_pos).abs();
            if distance < min_distance {
                min_distance = distance;
                closest_idx = i;
            }
        }
        
        Some(closest_idx)
    }
    
    fn apply_color_reflection(&mut self, ctx: &egui::Context) {
        if let Some(original_img) = &self.original_image {
            if self.slider_values.is_empty() {
                eprintln!("No sliders configured for color reflection");
                return;
            }
            
            // 创建排序后的滑块值
            let mut sorted_values = self.slider_values.clone();
            sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
            // 转换为灰度图像
            let gray_img = original_img.grayscale();
            let rgba_image = gray_img.to_rgba8();
            let (width, height) = rgba_image.dimensions();
            let mut pixels = rgba_image.into_raw();
            
            // 处理每个像素
            for chunk in pixels.chunks_exact_mut(4) {
                let r = chunk[0] as f32;
                let g = chunk[1] as f32;
                let b = chunk[2] as f32;
                let a = chunk[3];
                
                // 计算灰度值 (使用ITU-R BT.601标准)
                let gray_value = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
                
                // 找到灰度值对应的区段
                let segment_value = self.get_segment_value(gray_value, &sorted_values);
                
                // 设置新的RGB值
                if a == 0 {
                    chunk[0] = 0;
                    chunk[1] = 0;
                    chunk[2] = 0;
                    chunk[3] = 0;
                } else {
                    chunk[0] = segment_value;
                    chunk[1] = segment_value;
                    chunk[2] = segment_value;
                    chunk[3] = 255;
                }
            }
            
            // 创建处理后的图像
            let processed_img = DynamicImage::ImageRgba8(
                image::RgbaImage::from_raw(width, height, pixels).unwrap()
            );
            
            // 更新显示
            self.update_texture_from_image(ctx, &processed_img);
            self.save_to_temp(&processed_img);
            
            println!("Color reflection applied successfully");
        } else {
            eprintln!("No image loaded for color reflection");
        }
    }
    
    fn get_segment_value(&self, gray_value: u8, sorted_values: &[f32]) -> u8 {
        let gray_f32 = gray_value as f32;
        
        // 如果只有一个滑块，直接返回该值
        if sorted_values.len() == 1 {
            return sorted_values[0] as u8;
        }
        
        // 找到灰度值所在的区段
        for i in 0..sorted_values.len() - 1 {
            let segment_start = sorted_values[i];
            let segment_end = sorted_values[i + 1];
            
            if gray_f32 >= segment_start && gray_f32 <= segment_end {
                // 计算区段的平均值
                let segment_avg = (segment_start + segment_end) / 2.0;
                return segment_avg as u8;
            }
        }
        
        // 处理边界情况
        if gray_f32 <= sorted_values[0] {
            // 小于第一个滑块的值，使用第一个区段的平均值
            if sorted_values.len() > 1 {
                let segment_avg = (0.0 + sorted_values[1]) / 2.0;
                return segment_avg as u8;
            } else {
                return sorted_values[0] as u8;
            }
        } else {
            // 大于最后一个滑块的值，使用最后一个区段的平均值
            let last_idx = sorted_values.len() - 1;
            if last_idx > 0 {
                let segment_avg = (sorted_values[last_idx - 1] + 255.0) / 2.0;
                return segment_avg as u8;
            } else {
                return sorted_values[last_idx] as u8;
            }
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

use egui;
use image::DynamicImage;
use std::path::PathBuf;
use std::fs;
use crate::utils::{ImageProcessor, UiUtils};
use crate::color_reflection_window::ColorReflectionWindow;

/// 主窗口的状态
pub struct MainWindow {
    pub current_texture: Option<egui::TextureHandle>,
    pub current_path: Option<PathBuf>,
    pub zoom_factor: f32,
    pub pan_offset: egui::Vec2,
    pub original_image: Option<DynamicImage>,
    pub is_grayscale: bool,
    pub temp_path: Option<PathBuf>,
    pub color_reflection_window: ColorReflectionWindow,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            current_texture: None,
            current_path: None,
            zoom_factor: 1.0,
            pan_offset: egui::Vec2::ZERO,
            original_image: None,
            is_grayscale: false,
            temp_path: None,
            color_reflection_window: ColorReflectionWindow::default(),
        }
    }
}

impl eframe::App for MainWindow {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.show(ctx, frame);
    }
}

impl MainWindow {
    /// 显示主窗口
    pub fn show(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.show_menu_bar(ctx, frame);
        self.show_toolbar(ctx);
        self.show_color_reflection_window(ctx);
        self.show_main_display(ctx);
    }

    /// 显示菜单栏
    fn show_menu_bar(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open Image").clicked() {
                        self.open_image_dialog(ctx);
                        ui.close();
                    }
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        ui.close();
                    }
                });
            });
        });
    }

    /// 显示工具栏
    fn show_toolbar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // 左侧按钮组
                ui.horizontal(|ui| {
                    if ui.button("Original").clicked() {
                        self.original(ctx);
                    }
                    if ui.button("Black & White").clicked() {
                        self.toggle_grayscale(ctx);
                    }
                    if ui.button("Fast Save").clicked() {
                        self.fast_save();
                    }
                    if ui.button("Color Reflection").clicked() {
                        self.color_reflection_window.show_window = true;
                    }
                    if ui.button("Clean").clicked() {
                        self.clean_image(ctx);
                    }
                });

                // 中间缩放信息
                ui.allocate_ui_with_layout(
                    egui::Vec2::new(ui.available_width(), ui.available_height()),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.centered_and_justified(|ui| {
                            ui.label(format!("Zoom: {:.1}%", self.zoom_factor * 100.0));
                        });
                    }
                );

                // 右侧空白区域（保持布局平衡）
                ui.horizontal(|ui| {
                    ui.add_space(ui.available_width());
                });
            });
        });
    }

    /// 显示Color Reflection窗口
    fn show_color_reflection_window(&mut self, ctx: &egui::Context) {
        self.color_reflection_window.show(
            ctx,
            &self.original_image,
            &mut self.current_texture,
            &self.temp_path,
        );
    }

    /// 显示主显示区域
    fn show_main_display(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            UiUtils::draw_checkerboard_background(ui);

            // 处理缩放
            if ui.input(|i| i.raw_scroll_delta.y != 0.0) {
                let zoom_delta = ui.input(|i| i.raw_scroll_delta.y) * 0.01;
                let old_zoom = self.zoom_factor;
                self.zoom_factor = (self.zoom_factor + zoom_delta).max(0.1).min(5.0);

                // 保持鼠标位置不变
                if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                    let zoom_ratio = self.zoom_factor / old_zoom;
                    // 计算鼠标相对于图像中心的位置
                    let image_center = egui::Pos2::new(
                        ui.available_size().x / 2.0 + self.pan_offset.x,
                        ui.available_size().y / 2.0 + self.pan_offset.y,
                    );
                    let mouse_offset = mouse_pos - image_center;
                    let new_offset = mouse_pos - mouse_offset * zoom_ratio;
                    self.pan_offset = egui::Vec2::new(
                        new_offset.x - ui.available_size().x / 2.0,
                        new_offset.y - ui.available_size().y / 2.0,
                    );
                }
            }

            if let Some(texture) = &self.current_texture {
                let texture_size = texture.size_vec2();
                let scaled_size = texture_size * self.zoom_factor;

                let available_size = ui.available_size();
                let image_pos = egui::Pos2::new(
                    (available_size.x - scaled_size.x) / 2.0 + self.pan_offset.x,
                    (available_size.y - scaled_size.y) / 2.0 + self.pan_offset.y,
                );

                let image_rect = egui::Rect::from_min_size(image_pos, scaled_size);
                let response = ui.allocate_rect(image_rect, egui::Sense::click_and_drag());

                // 处理拖拽
                if response.dragged() {
                    self.pan_offset += ui.input(|i| i.pointer.delta());
                }

                let image = egui::Image::new(texture)
                    .fit_to_exact_size(scaled_size)
                    .texture_options(egui::TextureOptions {
                        magnification: egui::TextureFilter::Nearest,
                        minification: egui::TextureFilter::Nearest,
                        ..Default::default()
                    });
                ui.put(image_rect, image);
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Please select an image file");
                });
            }
        });
    }

    /// 打开图片对话框
    fn open_image_dialog(&mut self, ctx: &egui::Context) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Image files", &["png", "jpg", "jpeg", "bmp", "gif", "tiff"])
            .pick_file()
        {
            self.load_image(ctx, &path);
        }
    }

    /// 加载图片
    fn load_image(&mut self, ctx: &egui::Context, path: &std::path::Path) {
        match image::open(path) {
            Ok(img) => {
                self.original_image = Some(img.clone());
                self.is_grayscale = false;

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
                self.current_path = Some(path.to_path_buf());
                self.zoom_factor = 1.0;
                self.pan_offset = egui::Vec2::ZERO;

                // 创建临时文件
                if let Some(original_path) = &self.current_path {
                    if let Some(stem) = original_path.file_stem().and_then(|s| s.to_str()) {
                        let mut temp_path = original_path.clone();
                        temp_path.set_file_name(format!("{}.egui_tmp.png", stem));
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

    /// 切换灰度
    fn toggle_grayscale(&mut self, ctx: &egui::Context) {
        if let Some(original_img) = &self.original_image {
            let processed_img = if self.is_grayscale {
                original_img.clone()
            } else {
                ImageProcessor::convert_to_grayscale_custom(original_img)
            };

            self.is_grayscale = !self.is_grayscale;
            self.current_texture = Some(ImageProcessor::update_texture_from_image(&processed_img, ctx));
            if let Some(temp_path) = &self.temp_path {
                let _ = ImageProcessor::save_to_temp(&processed_img, temp_path);
            }
        }
    }

    /// 恢复原始图片
    fn original(&mut self, ctx: &egui::Context) {
        if let Some(original_img) = self.original_image.clone() {
            self.is_grayscale = false;
            self.current_texture = Some(ImageProcessor::update_texture_from_image(&original_img, ctx));
            if let Some(temp_path) = &self.temp_path {
                let _ = ImageProcessor::save_to_temp(&original_img, temp_path);
            }
        }
    }

    /// 快速保存
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

    /// 清理图像
    fn clean_image(&mut self, ctx: &egui::Context) {
        if let Some(_current_texture) = &self.current_texture {
            // 从临时文件加载当前显示的图像
            if let Some(temp_path) = &self.temp_path {
                match image::open(temp_path) {
                    Ok(current_img) => {
                        let cleaned_img = ImageProcessor::clean_image(&current_img);
                        self.current_texture = Some(ImageProcessor::update_texture_from_image(&cleaned_img, ctx));
                        let _ = ImageProcessor::save_to_temp(&cleaned_img, temp_path);
                        println!("Image cleaned successfully");
                    }
                    Err(e) => {
                        eprintln!("Failed to load current image from temp file: {}", e);
                    }
                }
            } else {
                eprintln!("No temp file available for cleaning");
            }
        } else {
            eprintln!("No image loaded for cleaning");
        }
    }
}

impl Drop for MainWindow {
    fn drop(&mut self) {
        if let Some(temp) = self.temp_path.take() {
            let _ = fs::remove_file(temp);
        }
    }
}
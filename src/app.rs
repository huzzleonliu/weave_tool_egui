//! # 主应用模块
//! 
//! 包含主应用程序结构和核心逻辑

use eframe::egui;
use std::path::PathBuf;
use image::DynamicImage;

use crate::image_processor::ImageProcessor;
use crate::ui::UIComponents;

/// 图片查看器主应用
/// 
/// 负责管理应用程序状态、用户交互和界面渲染
pub struct ImageViewer {
    /// 当前显示的纹理
    current_texture: Option<egui::TextureHandle>,
    /// 当前图片路径
    current_path: Option<PathBuf>,
    /// 缩放因子
    zoom_factor: f32,
    /// 平移偏移量
    pan_offset: egui::Vec2,
    /// 图片处理器
    image_processor: ImageProcessor,
}

impl Default for ImageViewer {
    fn default() -> Self {
        Self {
            current_texture: None,
            current_path: None,
            zoom_factor: 1.0,
            pan_offset: egui::Vec2::ZERO,
            image_processor: ImageProcessor::new(),
        }
    }
}

impl eframe::App for ImageViewer {
    /// 更新应用程序状态和渲染界面
    /// 
    /// # 参数
    /// * `ctx` - egui上下文
    /// * `_frame` - 应用程序框架
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 绘制顶部菜单栏
        self.draw_menu_bar(ctx);
        
        // 绘制底部工具栏
        self.draw_toolbar(ctx);
        
        // 绘制主显示区域
        self.draw_main_area(ctx);
    }
}

impl ImageViewer {
    /// 绘制菜单栏
    fn draw_menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                // 文件菜单
                ui.menu_button("File", |ui| {
                    if ui.button("Open Image").clicked() {
                        self.open_image_dialog(ctx);
                        ui.close();
                    }
                    
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                
                // 视图菜单
                ui.menu_button("View", |ui| {
                    if ui.button("Reset Zoom").clicked() {
                        self.reset_zoom();
                    }
                    
                    if ui.button("Fit to Window").clicked() {
                        self.fit_to_window(ui);
                    }
                });
            });
        });
    }

    /// 绘制工具栏
    fn draw_toolbar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("toolbar").show(ctx, |ui| {
            // 第一行：功能按钮
            ui.horizontal(|ui| {
                if ui.button("Original").clicked() {
                    self.restore_original(ctx);
                }

                if ui.button("Black & White").clicked() {
                    self.toggle_grayscale(ctx);
                }

                if ui.button("Fast Save").clicked() {
                    self.fast_save();
                }
            });

            // 第二行：缩放控制和文件信息
            ui.horizontal(|ui| {
                ui.label("Zoom:");
                ui.add(egui::Slider::new(&mut self.zoom_factor, 0.1..=5.0));
                
                if ui.button("Reset").clicked() {
                    self.reset_zoom();
                }
                
                ui.separator();
                
                if let Some(path) = &self.current_path {
                    ui.label(format!("Current file: {}", path.display()));
                } else {
                    ui.label("No image selected");
                }
            });
        });
    }

    /// 绘制主显示区域
    fn draw_main_area(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // 绘制棋盘格背景
            self.draw_checkerboard_background(ui);
            
            // 处理鼠标滚轮缩放
            self.handle_mouse_zoom(ui);
            
            if let Some(texture) = self.current_texture.clone() {
                self.draw_image_with_texture(ui, &texture);
            } else {
                self.draw_empty_state(ui, ctx);
            }
        });
    }

    /// 处理鼠标滚轮缩放
    fn handle_mouse_zoom(&mut self, ui: &mut egui::Ui) {
        if ui.input(|i| i.raw_scroll_delta.y != 0.0) {
            let scroll_delta = ui.input(|i| i.raw_scroll_delta.y);
            let zoom_speed = 0.02;
            let old_zoom = self.zoom_factor;
            let zoom_delta = scroll_delta * zoom_speed;
            self.zoom_factor *= 1.0 + zoom_delta;
            self.zoom_factor = self.zoom_factor.clamp(0.1, 5.0);
            
            // 保持鼠标位置不变进行缩放
            if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                let available_size = ui.available_size();
                let center = available_size / 2.0;
                let zoom_ratio = self.zoom_factor / old_zoom;
                let mouse_offset = mouse_pos - center;
                self.pan_offset = mouse_offset - (mouse_offset - self.pan_offset) * zoom_ratio;
            }
        }
    }

    /// 绘制图片（使用纹理句柄）
    fn draw_image_with_texture(&mut self, ui: &mut egui::Ui, texture: &egui::TextureHandle) {
        let available_size = ui.available_size();
        let original_size = texture.size_vec2();
        let scaled_size = original_size * self.zoom_factor;
        
        // 计算图片位置（居中显示，考虑平移偏移）
        let center = available_size / 2.0;
        let image_pos = center + self.pan_offset - scaled_size / 2.0;
        let image_rect = egui::Rect::from_min_size(image_pos.to_pos2(), scaled_size);
        
        // 处理鼠标拖拽交互
        let response = ui.allocate_rect(image_rect, egui::Sense::click_and_drag());
        
        if response.dragged() {
            self.pan_offset += response.drag_delta();
        }
        
        // 绘制图片（使用最近邻过滤以保持像素完美显示）
        let image = egui::Image::new(texture)
            .fit_to_exact_size(scaled_size)
            .texture_options(egui::TextureOptions::NEAREST);
        ui.put(image_rect, image);
    }

    /// 打开图片选择对话框
    fn open_image_dialog(&mut self, ctx: &egui::Context) {
        if let Some(path) = UIComponents::show_file_dialog() {
            self.load_image(ctx, &path);
        }
    }

    /// 加载图片
    fn load_image(&mut self, ctx: &egui::Context, path: &std::path::Path) {
        match self.image_processor.load_image(path) {
            Ok(img) => {
                self.current_path = Some(path.to_path_buf());
                self.zoom_factor = 1.0;
                self.pan_offset = egui::Vec2::ZERO;
                self.update_texture_from_image(ctx, &img);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }

    /// 切换黑白模式
    fn toggle_grayscale(&mut self, ctx: &egui::Context) {
        if let Some(processed_img) = self.image_processor.toggle_grayscale() {
            self.update_texture_from_image(ctx, &processed_img);
        }
    }

    /// 恢复原始图片
    fn restore_original(&mut self, ctx: &egui::Context) {
        if let Some(original_img) = self.image_processor.restore_original() {
            self.update_texture_from_image(ctx, &original_img);
        }
    }

    /// 快速保存
    fn fast_save(&self) {
        if let Err(e) = self.image_processor.fast_save() {
            eprintln!("{}", e);
        }
    }

    /// 重置缩放
    fn reset_zoom(&mut self) {
        self.zoom_factor = 1.0;
        self.pan_offset = egui::Vec2::ZERO;
    }

    /// 适合窗口大小
    fn fit_to_window(&mut self, ui: &egui::Ui) {
        if let Some(texture) = &self.current_texture {
            let available_size = ui.available_size();
            let image_size = texture.size_vec2();
            let scale_x = available_size.x / image_size.x;
            let scale_y = available_size.y / image_size.y;
            self.zoom_factor = scale_x.min(scale_y) * 0.9; // 留一些边距
            self.pan_offset = egui::Vec2::ZERO;
        }
    }

    /// 从图片数据更新纹理
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

    /// 绘制棋盘格背景
    fn draw_checkerboard_background(&self, ui: &mut egui::Ui) {
        let checker_size = 16.0; // 每个棋盘格的大小
        let light_color = egui::Color32::from_gray(200);
        let dark_color = egui::Color32::from_gray(150);
        
        // 计算需要填充棋盘格的区域（整个可用区域）
        let available_size = ui.available_size();
        let checker_rect = egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            available_size
        );
        
        // 绘制棋盘格图案
        let painter = ui.painter();
        
        // 计算需要多少个棋盘格来覆盖整个区域
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
                
                // 绘制所有在可用区域内的方块
                let is_light = (row + col) % 2 == 0;
                let color = if is_light { light_color } else { dark_color };
                
                painter.rect_filled(square_rect, 0.0, color);
            }
        }
    }

    /// 绘制空状态界面（没有图片时）
    fn draw_empty_state(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.centered_and_justified(|ui| {
            ui.label("Please select an image file");
            ui.add_space(10.0);
            if ui.button("Open Image").clicked() {
                self.open_image_dialog(ctx);
            }
        });
    }
}

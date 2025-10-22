//! # 用户界面模块
//! 
//! 提供用户界面的各种组件和布局

use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;

/// UI组件集合
pub struct UIComponents;

impl UIComponents {
    /// 绘制菜单栏
    /// 
    /// # 参数
    /// * `ui` - egui UI上下文
    /// * `on_open_image` - 打开图片回调
    /// * `on_exit` - 退出回调
    /// * `on_reset_zoom` - 重置缩放回调
    /// * `on_fit_window` - 适合窗口回调
    pub fn draw_menu_bar(
        ui: &mut egui::Ui,
        on_open_image: impl FnOnce(),
        on_exit: impl FnOnce(),
        on_reset_zoom: impl FnOnce(),
        on_fit_window: impl FnOnce(),
    ) {
        egui::MenuBar::new().ui(ui, |ui| {
            // 文件菜单
            ui.menu_button("File", |ui| {
                    if ui.button("Open Image").clicked() {
                        on_open_image();
                        ui.close();
                    }
                
                if ui.button("Exit").clicked() {
                    on_exit();
                }
            });
            
            // 视图菜单
            ui.menu_button("View", |ui| {
                if ui.button("Reset Zoom").clicked() {
                    on_reset_zoom();
                }
                
                if ui.button("Fit to Window").clicked() {
                    on_fit_window();
                }
            });
        });
    }

    /// 绘制工具栏
    /// 
    /// # 参数
    /// * `ui` - egui UI上下文
    /// * `zoom_factor` - 当前缩放因子
    /// * `on_zoom_change` - 缩放变化回调
    /// * `on_reset` - 重置回调
    /// * `on_original` - 恢复原始图片回调
    /// * `on_grayscale` - 黑白转换回调
    /// * `on_fast_save` - 快速保存回调
    /// * `current_path` - 当前文件路径
    pub fn draw_toolbar(
        ui: &mut egui::Ui,
        zoom_factor: &mut f32,
        _on_zoom_change: impl FnOnce(f32),
        on_reset: impl FnOnce(),
        on_original: impl FnOnce(),
        on_grayscale: impl FnOnce(),
        on_fast_save: impl FnOnce(),
        current_path: Option<&PathBuf>,
    ) {
        // 第一行：功能按钮
        ui.horizontal(|ui| {
            if ui.button("Original").clicked() {
                on_original();
            }

            if ui.button("Black & White").clicked() {
                on_grayscale();
            }

            if ui.button("Fast Save").clicked() {
                on_fast_save();
            }
        });

        // 第二行：缩放控制和文件信息
        ui.horizontal(|ui| {
            ui.label("Zoom:");
            ui.add(egui::Slider::new(zoom_factor, 0.1..=5.0));
            
            if ui.button("Reset").clicked() {
                on_reset();
            }
            
            ui.separator();
            
            if let Some(path) = current_path {
                ui.label(format!("Current file: {}", path.display()));
            } else {
                ui.label("No image selected");
            }
        });
    }

    /// 绘制棋盘格背景
    /// 
    /// # 参数
    /// * `ui` - egui UI上下文
    pub fn draw_checkerboard_background(ui: &mut egui::Ui) {
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
    /// 
    /// # 参数
    /// * `ui` - egui UI上下文
    /// * `on_open_image` - 打开图片回调
    pub fn draw_empty_state(ui: &mut egui::Ui, on_open_image: impl FnOnce()) {
        ui.centered_and_justified(|ui| {
            ui.label("Please select an image file");
            ui.add_space(10.0);
            if ui.button("Open Image").clicked() {
                on_open_image();
            }
        });
    }

    /// 显示文件选择对话框
    /// 
    /// # 返回值
    /// 如果用户选择了文件，返回文件路径，否则返回None
    pub fn show_file_dialog() -> Option<PathBuf> {
        FileDialog::new()
            .add_filter("Image files", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
            .pick_file()
    }
}

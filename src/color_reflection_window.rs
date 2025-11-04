use egui;
use image::DynamicImage;
use std::path::PathBuf;
use crate::utils::{GrayscaleMode, ImageProcessor};

/// Color Reflection窗口的状态
pub struct ColorReflectionWindow {
    pub show_window: bool,
    pub slider_amount_input: String,
    pub slider_amount: Option<usize>,
    pub slider_values: Vec<f32>,
    pub reflection_mode: ReflectionMode,
    // 最近一次应用反射时的状态快照
    pub last_applied_slider_values: Option<Vec<f32>>,
    pub last_reflection_mode: Option<ReflectionMode>,
    pub has_applied_reflection: bool,
    pub message: Option<String>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ReflectionMode {
    Average,
    Partial,
}

impl Default for ColorReflectionWindow {
    fn default() -> Self {
        Self {
            show_window: false,
            slider_amount_input: String::new(),
            slider_amount: None,
            slider_values: Vec::new(),
            reflection_mode: ReflectionMode::Average,
            last_applied_slider_values: None,
            last_reflection_mode: None,
            has_applied_reflection: false,
            message: None,
        }
    }
}

impl ColorReflectionWindow {
    /// 显示Color Reflection窗口
    pub fn show(&mut self, ctx: &egui::Context, original_image: &Option<DynamicImage>, current_texture: &mut Option<egui::TextureHandle>, temp_path: &Option<PathBuf>, current_path: &Option<PathBuf>, grayscale_mode: &mut GrayscaleMode) {
        if self.show_window {
            let mut show_window = self.show_window;
            egui::Window::new("Color Reflection")
                .open(&mut show_window)
                .default_size([800.0, 600.0])
                .resizable(true)
                .show(ctx, |ui| {
                    self.show_content(ui, ctx, original_image, current_texture, temp_path, current_path, grayscale_mode);
                });
            self.show_window = show_window;

            // 简单提示窗口（克隆消息，避免借用冲突）
            if let Some(msg_owned) = self.message.clone() {
                let mut open = true;
                let mut clear_message = false;
                egui::Window::new("Info")
                    .open(&mut open)
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label(msg_owned);
                        if ui.button("OK").clicked() { clear_message = true; }
                    });
                if clear_message || !open { self.message = None; }
            }
        }
    }

    /// 显示窗口内容
    fn show_content(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        original_image: &Option<DynamicImage>,
        current_texture: &mut Option<egui::TextureHandle>,
        temp_path: &Option<PathBuf>,
        current_path: &Option<PathBuf>,
        grayscale_mode: &mut GrayscaleMode,
    ) {
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

            if ui.button("Load Anchors From PNG").clicked() {
                // 优先读取当前文件，如无则回退到临时文件
                let pick_path = current_path.as_ref().or(temp_path.as_ref());
                if let Some(p) = pick_path {
                    match crate::utils::ImageProcessor::read_png_text_value_from_path(p.as_path(), "anchors") {
                        Ok(Some(json)) => {
                            if let Some(vals) = Self::parse_anchors_from_json(&json) {
                                self.slider_amount = Some(vals.len());
                                self.slider_amount_input = vals.len().to_string();
                                self.slider_values = vals;
                                if let Some(g) = Self::parse_grayscale_from_json(&json) {
                                    *grayscale_mode = g;
                                }
                            } else {
                                self.message = Some("No slider anchors found in metadata".to_string());
                            }
                        }
                        Ok(None) => {
                            self.message = Some("No slider anchors found in metadata".to_string());
                        }
                        Err(e) => {
                            self.message = Some(format!("Failed to read anchors: {}", e));
                        }
                    }
                } else {
                    self.message = Some("No temp image available".to_string());
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
                    self.draw_slider_track(ui);
                }
            );
        } else {
            ui.label("Please enter slider amount (1-10) and click confirm");
        }

        ui.add_space(10.0);

        // 第三行：reflection mode选择
        ui.horizontal(|ui| {
            ui.label("Reflection mode:");
            ui.radio_value(&mut self.reflection_mode, ReflectionMode::Average, "Average");
            ui.radio_value(&mut self.reflection_mode, ReflectionMode::Partial, "Partial");
        });

        ui.add_space(20.0);

        // Confirm Reflection 按钮
        if ui.button("Confirm Reflection").clicked() {
            self.apply_color_reflection(ctx, original_image, current_texture, temp_path, grayscale_mode);
        }




    }

    /// 绘制滑动条轨道
    fn draw_slider_track(&mut self, ui: &mut egui::Ui) {
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

    /// 更新滑块值
    fn update_slider_values(&mut self) {
        if let Some(amount) = self.slider_amount {
            self.slider_values.clear();
            for i in 0..amount {
                // 将滑动条分成(amount + 2)份，取中间的端点
                // 例如：3个滑块 -> 分成5份 -> 取第2,3,4个端点
                let total_segments = amount + 2;
                let segment_size = 255.0 / total_segments as f32;
                let value = segment_size * (i + 1) as f32;
                self.slider_values.push(value);
            }
        }
    }

    /// 找到最近的滑块
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

    /// 应用颜色反射处理
    fn apply_color_reflection(
        &mut self,
        ctx: &egui::Context,
        original_image: &Option<DynamicImage>,
        current_texture: &mut Option<egui::TextureHandle>,
        temp_path: &Option<PathBuf>,
        grayscale_mode: &GrayscaleMode,
    ) {
        if let Some(original_img) = original_image {
            if self.slider_values.is_empty() {
                eprintln!("No sliders configured for color reflection");
                return;
            }

            // 根据选择的模式应用颜色反射处理
            let processed_img = match self.reflection_mode {
                ReflectionMode::Average => {
                    ImageProcessor::apply_color_reflection_with_mode(original_img, &self.slider_values, *grayscale_mode)
                }
                ReflectionMode::Partial => {
                    ImageProcessor::apply_color_reflection_partial_with_mode(original_img, &self.slider_values, *grayscale_mode)
                }
            };

            // 更新显示
            *current_texture = Some(ImageProcessor::update_texture_from_image(&processed_img, ctx));

            // 保存到临时文件
            if let Some(temp_path) = temp_path {
                if let Err(e) = ImageProcessor::save_to_temp(&processed_img, temp_path) {
                    eprintln!("Failed to save to temp file: {}", e);
                }
            }

            // 记录快照以便后续保存元数据
            self.snapshot_after_apply();

            println!("Color reflection applied successfully");
        } else {
            eprintln!("No image loaded for color reflection");
        }
    }
}

impl ColorReflectionWindow {
    /// 在反射应用成功后记录快照（供保存元数据使用）
    pub fn snapshot_after_apply(&mut self) {
        self.last_applied_slider_values = Some(self.slider_values.clone());
        self.last_reflection_mode = Some(self.reflection_mode.clone());
        self.has_applied_reflection = true;
    }

    /// 构造包含锚点与模式信息的JSON字符串
    pub fn build_anchors_metadata_json(&self, grayscale_mode: &GrayscaleMode) -> Option<String> {
        if !self.has_applied_reflection {
            return None;
        }
        let anchors = self
            .last_applied_slider_values
            .as_ref()
            .map(|v| {
                let values = v.iter().map(|x| format!("{:.0}", x)).collect::<Vec<_>>();
                format!("[{}]", values.join(","))
            })?;

        let reflection = match self.last_reflection_mode.as_ref()? {
            ReflectionMode::Average => "Average",
            ReflectionMode::Partial => "Partial",
        };
        let gray = match grayscale_mode {
            GrayscaleMode::Default => "Default",
            GrayscaleMode::Max => "Max",
            GrayscaleMode::Min => "Min",
        };

        // 简单JSON（避免引入serde依赖）
        let json = format!(
            "{{\"anchors\":{},\"reflectionMode\":\"{}\",\"grayscaleMode\":\"{}\"}}",
            anchors, reflection, gray
        );
        Some(json)
    }

    /// 解析JSON字符串中的 anchors 数组（不依赖serde）
    fn parse_anchors_from_json(json: &str) -> Option<Vec<f32>> {
        // 查找 "anchors":[...]
        let key = "\"anchors\"";
        let idx = json.find(key)?;
        let after = &json[idx + key.len()..];
        let lb = after.find('[')?;
        let after_lb = &after[lb + 1..];
        let rb = after_lb.find(']')?;
        let inside = &after_lb[..rb];
        let mut vals = Vec::new();
        for part in inside.split(',') {
            let t = part.trim();
            if t.is_empty() { continue; }
            if let Ok(v) = t.parse::<f32>() { vals.push(v); }
        }
        if vals.is_empty() { None } else { Some(vals) }
    }

    /// 解析 grayscaleMode 字段
    fn parse_grayscale_from_json(json: &str) -> Option<crate::utils::GrayscaleMode> {
        let key = "\"grayscaleMode\"";
        let idx = json.find(key)?;
        let after = &json[idx + key.len()..];
        let q1 = after.find('"')?; // 开始引号
        let rest = &after[q1 + 1..];
        let q2 = rest.find('"')?; // 结束引号
        let val = &rest[..q2];
        match val {
            "Default" => Some(crate::utils::GrayscaleMode::Default),
            "Max" => Some(crate::utils::GrayscaleMode::Max),
            "Min" => Some(crate::utils::GrayscaleMode::Min),
            _ => None,
        }
    }
}

# 了解项目结构和开发过程
_Exported on 11/4/2025 at 14:11:27 GMT+1 from Cursor (2.0.54)_

---

**User**

阅读整个项目，了解项目结构和聊天记录，了解开发过程

---

**Cursor**

我先快速查看关键文件（Cargo.toml、src 下主要模块、聊天记录）来梳理项目结构与开发脉络。接着我会给出简明的项目概览与要点。

我已完成对项目源码与聊天记录的快速通读，下面给出简明结构与演进概览。

### 项目结构与技术栈
- **名称**: `weave_tool_egui`
- **语言/框架**: Rust + `eframe/egui 0.33`
- **依赖**: `eframe`, `egui`, `egui_extras`, `image`, `rfd`
- **目录**
  - `src/main.rs`: 应用入口，创建窗口、运行 `MainWindow`
  - `src/main_window.rs`: 主界面与交互逻辑（菜单栏、工具栏、缩放/拖拽显示区、文件加载/保存、灰度与清理操作、窗口调用）
  - `src/color_reflection_window.rs`: “Color Reflection” 子窗口（滑块数量与位置、多模式反射处理）
  - `src/utils.rs`: 图像处理工具（灰度模式、反射算法、棋盘格背景）

示例（入口与主窗口挂载）：
```1:19:src/main.rs
mod main_window;
mod color_reflection_window;
mod utils;

use main_window::MainWindow;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(MainWindow::default()))),
    )
}
```

### 关键功能
- **图片加载/显示**: 支持多格式，纹理化显示，中心定位，拖拽平移，滚轮缩放（限制范围，保持鼠标点相对位置稳定）
- **灰度处理**: 三种灰度模式 `Default/Max/Min`，可应用到当前图像并保存到临时文件
- **快速保存**: 将临时结果覆盖原图路径
- **清理图像**: 去除“孤立像素”，用邻域最常见颜色替换
- **Color Reflection**:  
  - 可输入 1-10 个滑块数量，绘制在单轨道上，点击/拖动就近滑块
  - 模式选择：`Average` 与 `Partial`
  - 与灰度模式联动：先按灰度策略处理，再做分段/区间映射
  - 处理完成后更新纹理并写入临时文件

示例（主窗口状态/工具条/显示区）：
```8:18:src/main_window.rs
pub struct MainWindow {
    pub current_texture: Option<egui::TextureHandle>,
    pub current_path: Option<PathBuf>,
    pub zoom_factor: f32,
    pub pan_offset: egui::Vec2,
    pub original_image: Option<DynamicImage>,
    pub temp_path: Option<PathBuf>,
    pub color_reflection_window: ColorReflectionWindow,
    pub grayscale_mode: GrayscaleMode,
}
```

```145:156:src/main_window.rs
fn show_main_display(&mut self, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        UiUtils::draw_checkerboard_background(ui);

        // 处理缩放
        if ui.input(|i| i.raw_scroll_delta.y != 0.0) {
            let zoom_delta = ui.input(|i| i.raw_scroll_delta.y) * 0.01;
            let old_zoom = self.zoom_factor;
            self.zoom_factor = (self.zoom_factor + zoom_delta).max(0.1).min(5.0);
```

示例（Color Reflection 应用与灰度模式联动）：
```132:174:src/utils.rs
pub fn apply_color_reflection_with_mode(
    original_img: &DynamicImage,
    slider_values: &[f32],
    mode: GrayscaleMode,
) -> DynamicImage {
    let gray_img = match mode {
        GrayscaleMode::Default => Self::convert_to_grayscale_custom(original_img),
        GrayscaleMode::Max => Self::convert_to_grayscale_max(original_img),
        GrayscaleMode::Min => Self::convert_to_grayscale_min(original_img),
    };

    let mut sorted_values = slider_values.to_vec();
    sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // ... 分段映射并输出结果 ...
}
```

### 运行与使用
- `cargo run` 启动
- 菜单 `File → Open Image` 选择图片
- 底部工具栏：
  - `Original`: 恢复原图到显示与临时文件
  - `Black & White Mode` 下拉：选择灰度模式；`Black & White` 按钮应用
  - `Fast Save`: 将临时结果覆盖原文件
  - `Color Reflection`: 打开子窗口配置滑块/模式并执行
  - `Clean`: 去孤立像素
- 主显示区：
  - 滚轮缩放（0.1x–5x）
  - 按住拖拽平移
  - 棋盘格背景便于观察透明区域

### 开发历程（据聊天记录与现状归纳）
- 初版实现图片查看与缩放/拖拽
- 修复滚轮缩放导致“图片消失”的问题，完善保持鼠标点位置不变的缩放体验
- 增强：加入三种灰度模式、临时文件工作流与快速保存
- 新增：`Color Reflection` 子窗口，支持多滑块分段映射，`Average`/`Partial` 两种反射模式，并与灰度预处理联动
- 新增：清理图像功能（邻域众数替换孤立像素）

如果你希望，我可以：
- 演示一组小图测试用例（验证各模式效果）
- 将缩放居中/自适应窗口策略做成按钮
- 增加撤销/重做与历史栈
- 将 `Color Reflection` 的参数持久化到配置文件

我已完成阅读与梳理。你接下来想先改进哪一块？

```plaintext
mod main_window;
mod color_reflection_window;
mod utils;

use main_window::MainWindow;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(MainWindow::default()))),
    )
}
```

```plaintext
pub struct MainWindow {
    pub current_texture: Option<egui::TextureHandle>,
    pub current_path: Option<PathBuf>,
    pub zoom_factor: f32,
    pub pan_offset: egui::Vec2,
    pub original_image: Option<DynamicImage>,
    pub temp_path: Option<PathBuf>,
    pub color_reflection_window: ColorReflectionWindow,
    pub grayscale_mode: GrayscaleMode,
}
```

```plaintext
fn show_main_display(&mut self, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        UiUtils::draw_checkerboard_background(ui);

        // 处理缩放
        if ui.input(|i| i.raw_scroll_delta.y != 0.0) {
            let zoom_delta = ui.input(|i| i.raw_scroll_delta.y) * 0.01;
            let old_zoom = self.zoom_factor;
            self.zoom_factor = (self.zoom_factor + zoom_delta).max(0.1).min(5.0);
```

```plaintext
pub fn apply_color_reflection_with_mode(
    original_img: &DynamicImage,
    slider_values: &[f32],
    mode: GrayscaleMode,
) -> DynamicImage {
    let gray_img = match mode {
        GrayscaleMode::Default => Self::convert_to_grayscale_custom(original_img),
        GrayscaleMode::Max => Self::convert_to_grayscale_max(original_img),
        GrayscaleMode::Min => Self::convert_to_grayscale_min(original_img),
    };

    let mut sorted_values = slider_values.to_vec();
    sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // ... 分段映射并输出结果 ...
}
```

---

**User**

现在我想要在fast save下面加一个功能，如果图片经过了颜色映射的话，我希望能在保存图片的时候把颜色映射的每一个锚点的值以数组的形式存入到元数据中，可以实现吗？

---

**Cursor**

我将创建一个待办列表来规划实现：新增保存含锚点元数据的功能、在颜色反射应用时记录锚点与模式、实现PNG元数据写入、在工具栏添加按钮并调用保存逻辑。接着我会并行修改依赖与代码文件。

我将开始修改依赖与源码文件：在颜色反射时记录锚点数据，添加PNG文本元数据写入工具函数，并在工具栏新增“Save With Anchors”按钮调用保存逻辑。接下来我会应用这些编辑。

```plaintext
[package]
name = "weave_tool_egui"
version = "0.1.0"
edition = "2021"

[dependencies]
eframe = "0.33"
egui = "0.33"
egui_extras = "0.33"
image = "0.25"
rfd = "0.14"
png = "0.17"

```

```rust
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
}

#[derive(PartialEq)]
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
        }
    }
}

impl ColorReflectionWindow {
    /// 显示Color Reflection窗口
    pub fn show(&mut self, ctx: &egui::Context, original_image: &Option<DynamicImage>, current_texture: &mut Option<egui::TextureHandle>, temp_path: &Option<PathBuf>, grayscale_mode: &GrayscaleMode) {
        if self.show_window {
            let mut show_window = self.show_window;
            egui::Window::new("Color Reflection")
                .open(&mut show_window)
                .default_size([800.0, 600.0])
                .resizable(true)
                .show(ctx, |ui| {
                    self.show_content(ui, ctx, original_image, current_texture, temp_path, grayscale_mode);
                });
            self.show_window = show_window;
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
        grayscale_mode: &GrayscaleMode,
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

        ui.add_space(10.0);

        if ui.button("Close Window").clicked() {
            self.show_window = false;
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
        &self,
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
}

```

```rust
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
}

#[derive(PartialEq)]
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
        }
    }
}

impl ColorReflectionWindow {
    /// 显示Color Reflection窗口
    pub fn show(&mut self, ctx: &egui::Context, original_image: &Option<DynamicImage>, current_texture: &mut Option<egui::TextureHandle>, temp_path: &Option<PathBuf>, grayscale_mode: &GrayscaleMode) {
        if self.show_window {
            let mut show_window = self.show_window;
            egui::Window::new("Color Reflection")
                .open(&mut show_window)
                .default_size([800.0, 600.0])
                .resizable(true)
                .show(ctx, |ui| {
                    self.show_content(ui, ctx, original_image, current_texture, temp_path, grayscale_mode);
                });
            self.show_window = show_window;
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
        grayscale_mode: &GrayscaleMode,
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

        ui.add_space(10.0);

        if ui.button("Close Window").clicked() {
            self.show_window = false;
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
}

```

```rust
use image::DynamicImage;
use std::path::PathBuf;

/// 图像处理工具函数
pub struct ImageProcessor;

#[derive(Clone, Copy, PartialEq)]
pub enum GrayscaleMode {
    Default,
    Max,
    Min,
}

impl ImageProcessor {
    /// 更新纹理从图像
    pub fn update_texture_from_image(
        img: &DynamicImage,
        ctx: &egui::Context,
    ) -> egui::TextureHandle {
        let rgba_image = img.to_rgba8();
        let size = [rgba_image.width() as usize, rgba_image.height() as usize];
        let pixels = rgba_image.into_raw();

        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
        let texture_options = egui::TextureOptions {
            magnification: egui::TextureFilter::Nearest,
            minification: egui::TextureFilter::Nearest,
            ..Default::default()
        };
        ctx.load_texture("processed_image", color_image, texture_options)
    }

    /// 保存图像到临时文件
    pub fn save_to_temp(
        img: &DynamicImage,
        temp_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        img.save(temp_path)?;
        Ok(())
    }

    /// 自定义灰度转换 (ITU-R BT.601标准)
    pub fn convert_to_grayscale_custom(img: &DynamicImage) -> DynamicImage {
        let rgba_image = img.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let mut pixels = rgba_image.into_raw();

        for chunk in pixels.chunks_exact_mut(4) {
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            let a = chunk[3];

            let luma = ((299 * r + 587 * g + 114 * b) / 1000) as u8;

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

        DynamicImage::ImageRgba8(image::RgbaImage::from_raw(width, height, pixels).unwrap())
    }

    pub fn convert_to_grayscale_max(img: &DynamicImage) -> DynamicImage {
        let rgba_image = img.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let mut pixels = rgba_image.into_raw();

        for chunk in pixels.chunks_exact_mut(4) {
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            let a = chunk[3];

            let luma = (r.max(g).max(b)) as u8;

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

        DynamicImage::ImageRgba8(image::RgbaImage::from_raw(width, height, pixels).unwrap())
    }

    pub fn convert_to_grayscale_min(img: &DynamicImage) -> DynamicImage {
        let rgba_image = img.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let mut pixels = rgba_image.into_raw();

        for chunk in pixels.chunks_exact_mut(4) {
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            let a = chunk[3];

            let luma = (r.min(g).min(b)) as u8;

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

        DynamicImage::ImageRgba8(image::RgbaImage::from_raw(width, height, pixels).unwrap())
    }

    // 删除：旧的未使用 average 版本（避免 dead_code 警告）

    /// 应用颜色反射处理（根据灰度模式预处理）
    pub fn apply_color_reflection_with_mode(
        original_img: &DynamicImage,
        slider_values: &[f32],
        mode: GrayscaleMode,
    ) -> DynamicImage {
        let gray_img = match mode {
            GrayscaleMode::Default => Self::convert_to_grayscale_custom(original_img),
            GrayscaleMode::Max => Self::convert_to_grayscale_max(original_img),
            GrayscaleMode::Min => Self::convert_to_grayscale_min(original_img),
        };

        let mut sorted_values = slider_values.to_vec();
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let rgba_image = gray_img.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let mut pixels = rgba_image.into_raw();

        for chunk in pixels.chunks_exact_mut(4) {
            let r = chunk[0] as f32;
            let g = chunk[1] as f32;
            let b = chunk[2] as f32;
            let a = chunk[3];

            let gray_value = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
            let segment_value = Self::get_segment_value(gray_value, &sorted_values);

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

        DynamicImage::ImageRgba8(image::RgbaImage::from_raw(width, height, pixels).unwrap())
    }

    /// 应用颜色反射处理 Partial（根据灰度模式预处理）
    pub fn apply_color_reflection_partial_with_mode(
        original_img: &DynamicImage,
        slider_values: &[f32],
        mode: GrayscaleMode,
    ) -> DynamicImage {
        let gray_img = match mode {
            GrayscaleMode::Default => Self::convert_to_grayscale_custom(original_img),
            GrayscaleMode::Max => Self::convert_to_grayscale_max(original_img),
            GrayscaleMode::Min => Self::convert_to_grayscale_min(original_img),
        };

        let mut sorted_values = slider_values.to_vec();
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let rgba_image = gray_img.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let mut pixels = rgba_image.into_raw();

        let total_segments = sorted_values.len() + 2;
        let segment_size = 255.0 / total_segments as f32;
        let mut segment_colors = Vec::new();
        segment_colors.push(0.0);
        for i in 1..=sorted_values.len() {
            let segment_start = i as f32 * segment_size;
            let segment_end = (i + 1) as f32 * segment_size;
            let segment_avg = (segment_start + segment_end) / 2.0;
            segment_colors.push(segment_avg);
        }
        segment_colors.push(255.0);

        for chunk in pixels.chunks_exact_mut(4) {
            let r = chunk[0] as f32;
            let g = chunk[1] as f32;
            let b = chunk[2] as f32;
            let a = chunk[3];

            let gray_value = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
            let segment_value =
                Self::get_segment_value_partial(gray_value, &sorted_values, &segment_colors);

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

        DynamicImage::ImageRgba8(image::RgbaImage::from_raw(width, height, pixels).unwrap())
    }

    /// 获取区段值 - Partial模式
    fn get_segment_value_partial(
        gray_value: u8,
        sorted_values: &[f32],
        segment_colors: &[f32],
    ) -> u8 {
        let gray_f32 = gray_value as f32;

        // 如果只有一个滑块，分成两个区段：小于滑块值用黑色，大于等于滑块值用白色
        if sorted_values.len() == 1 {
            if gray_f32 < sorted_values[0] {
                return segment_colors[0] as u8; // 黑色
            } else {
                return segment_colors[2] as u8; // 白色
            }
        }

        // 找到灰度值所在的区段
        for i in 0..sorted_values.len() - 1 {
            let segment_start = sorted_values[i];
            let segment_end = sorted_values[i + 1];

            if gray_f32 >= segment_start && gray_f32 <= segment_end {
                // 返回对应的分段颜色
                return segment_colors[i + 1] as u8;
            }
        }

        // 处理边界情况
        if gray_f32 <= sorted_values[0] {
            // 小于第一个滑块的值，使用第一段颜色（纯黑色）
            return segment_colors[0] as u8;
        } else {
            // 大于最后一个滑块的值，使用最后一段颜色（纯白色）
            return segment_colors[segment_colors.len() - 1] as u8;
        }
    }

    /// 清理图像 - 移除孤立的像素
    pub fn clean_image(img: &DynamicImage) -> DynamicImage {
        let rgba_image = img.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let pixels = rgba_image.into_raw();

        // 创建结果像素数组
        let mut result_pixels = pixels.clone();

        // 遍历每个像素
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) as usize * 4;
                let a = pixels[idx + 3];

                // 只处理非透明像素
                if a != 0 {
                    let current_color = (pixels[idx], pixels[idx + 1], pixels[idx + 2]);

                    // 获取周围8个像素的颜色
                    let mut neighbor_colors = Vec::new();

                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue; // 跳过中心像素
                            }

                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;

                            // 检查边界
                            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                                let nidx = (ny * width as i32 + nx) as usize * 4;
                                let na = pixels[nidx + 3];

                                // 只考虑非透明像素
                                if na != 0 {
                                    let neighbor_color =
                                        (pixels[nidx], pixels[nidx + 1], pixels[nidx + 2]);
                                    neighbor_colors.push(neighbor_color);
                                }
                            }
                        }
                    }

                    // 如果当前像素颜色与所有邻居都不同，则替换它
                    if !neighbor_colors.is_empty() && !neighbor_colors.contains(&current_color) {
                        // 找到最常见的颜色
                        let mut color_counts = std::collections::HashMap::new();
                        for &color in &neighbor_colors {
                            *color_counts.entry(color).or_insert(0) += 1;
                        }

                        // 找到出现次数最多的颜色
                        let mut max_count = 0;
                        let mut most_common_color = neighbor_colors[0];

                        for (&color, &count) in &color_counts {
                            if count > max_count
                                || (count == max_count && color.0 > most_common_color.0)
                            {
                                max_count = count;
                                most_common_color = color;
                            }
                        }

                        // 替换当前像素颜色
                        result_pixels[idx] = most_common_color.0;
                        result_pixels[idx + 1] = most_common_color.1;
                        result_pixels[idx + 2] = most_common_color.2;
                        // alpha保持不变
                    }
                }
            }
        }

        DynamicImage::ImageRgba8(image::RgbaImage::from_raw(width, height, result_pixels).unwrap())
    }

    /// 获取区段值
    fn get_segment_value(gray_value: u8, sorted_values: &[f32]) -> u8 {
        let gray_f32 = gray_value as f32;

        // 如果只有一个滑块，分成两个区段：小于滑块值用第一段平均值，大于等于滑块值用第二段平均值
        if sorted_values.len() == 1 {
            if gray_f32 < sorted_values[0] {
                // 第一段：0到滑块值的平均值
                return ((0.0 + sorted_values[0]) / 2.0) as u8;
            } else {
                // 第二段：滑块值到255的平均值
                return ((sorted_values[0] + 255.0) / 2.0) as u8;
            }
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
}

/// UI工具函数
pub struct UiUtils;

impl UiUtils {
    /// 绘制棋盘格背景
    pub fn draw_checkerboard_background(ui: &mut egui::Ui) {
        let rect = ui.available_rect_before_wrap();
        let painter = ui.painter();

        let checker_size = 20.0;
        let cols = (rect.width() / checker_size).ceil() as i32;
        let rows = (rect.height() / checker_size).ceil() as i32;

        for row in 0..rows {
            for col in 0..cols {
                let x = rect.min.x + col as f32 * checker_size;
                let y = rect.min.y + row as f32 * checker_size;

                let checker_rect = egui::Rect::from_min_size(
                    egui::Pos2::new(x, y),
                    egui::Vec2::new(checker_size, checker_size),
                );

                let is_light = (row + col) % 2 == 0;
                let color = if is_light {
                    egui::Color32::from_gray(200)
                } else {
                    egui::Color32::from_gray(150)
                };

                painter.rect_filled(checker_rect, 0.0, color);
            }
        }
    }
}

```

```rust
use image::DynamicImage;
use std::path::PathBuf;

/// 图像处理工具函数
pub struct ImageProcessor;

#[derive(Clone, Copy, PartialEq)]
pub enum GrayscaleMode {
    Default,
    Max,
    Min,
}

impl ImageProcessor {
    /// 更新纹理从图像
    pub fn update_texture_from_image(
        img: &DynamicImage,
        ctx: &egui::Context,
    ) -> egui::TextureHandle {
        let rgba_image = img.to_rgba8();
        let size = [rgba_image.width() as usize, rgba_image.height() as usize];
        let pixels = rgba_image.into_raw();

        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
        let texture_options = egui::TextureOptions {
            magnification: egui::TextureFilter::Nearest,
            minification: egui::TextureFilter::Nearest,
            ..Default::default()
        };
        ctx.load_texture("processed_image", color_image, texture_options)
    }

    /// 保存图像到临时文件
    pub fn save_to_temp(
        img: &DynamicImage,
        temp_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        img.save(temp_path)?;
        Ok(())
    }

    /// 自定义灰度转换 (ITU-R BT.601标准)
    pub fn convert_to_grayscale_custom(img: &DynamicImage) -> DynamicImage {
        let rgba_image = img.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let mut pixels = rgba_image.into_raw();

        for chunk in pixels.chunks_exact_mut(4) {
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            let a = chunk[3];

            let luma = ((299 * r + 587 * g + 114 * b) / 1000) as u8;

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

        DynamicImage::ImageRgba8(image::RgbaImage::from_raw(width, height, pixels).unwrap())
    }

    pub fn convert_to_grayscale_max(img: &DynamicImage) -> DynamicImage {
        let rgba_image = img.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let mut pixels = rgba_image.into_raw();

        for chunk in pixels.chunks_exact_mut(4) {
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            let a = chunk[3];

            let luma = (r.max(g).max(b)) as u8;

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

        DynamicImage::ImageRgba8(image::RgbaImage::from_raw(width, height, pixels).unwrap())
    }

    pub fn convert_to_grayscale_min(img: &DynamicImage) -> DynamicImage {
        let rgba_image = img.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let mut pixels = rgba_image.into_raw();

        for chunk in pixels.chunks_exact_mut(4) {
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            let a = chunk[3];

            let luma = (r.min(g).min(b)) as u8;

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

        DynamicImage::ImageRgba8(image::RgbaImage::from_raw(width, height, pixels).unwrap())
    }

    // 删除：旧的未使用 average 版本（避免 dead_code 警告）

    /// 应用颜色反射处理（根据灰度模式预处理）
    pub fn apply_color_reflection_with_mode(
        original_img: &DynamicImage,
        slider_values: &[f32],
        mode: GrayscaleMode,
    ) -> DynamicImage {
        let gray_img = match mode {
            GrayscaleMode::Default => Self::convert_to_grayscale_custom(original_img),
            GrayscaleMode::Max => Self::convert_to_grayscale_max(original_img),
            GrayscaleMode::Min => Self::convert_to_grayscale_min(original_img),
        };

        let mut sorted_values = slider_values.to_vec();
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let rgba_image = gray_img.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let mut pixels = rgba_image.into_raw();

        for chunk in pixels.chunks_exact_mut(4) {
            let r = chunk[0] as f32;
            let g = chunk[1] as f32;
            let b = chunk[2] as f32;
            let a = chunk[3];

            let gray_value = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
            let segment_value = Self::get_segment_value(gray_value, &sorted_values);

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

        DynamicImage::ImageRgba8(image::RgbaImage::from_raw(width, height, pixels).unwrap())
    }

    /// 应用颜色反射处理 Partial（根据灰度模式预处理）
    pub fn apply_color_reflection_partial_with_mode(
        original_img: &DynamicImage,
        slider_values: &[f32],
        mode: GrayscaleMode,
    ) -> DynamicImage {
        let gray_img = match mode {
            GrayscaleMode::Default => Self::convert_to_grayscale_custom(original_img),
            GrayscaleMode::Max => Self::convert_to_grayscale_max(original_img),
            GrayscaleMode::Min => Self::convert_to_grayscale_min(original_img),
        };

        let mut sorted_values = slider_values.to_vec();
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let rgba_image = gray_img.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let mut pixels = rgba_image.into_raw();

        let total_segments = sorted_values.len() + 2;
        let segment_size = 255.0 / total_segments as f32;
        let mut segment_colors = Vec::new();
        segment_colors.push(0.0);
        for i in 1..=sorted_values.len() {
            let segment_start = i as f32 * segment_size;
            let segment_end = (i + 1) as f32 * segment_size;
            let segment_avg = (segment_start + segment_end) / 2.0;
            segment_colors.push(segment_avg);
        }
        segment_colors.push(255.0);

        for chunk in pixels.chunks_exact_mut(4) {
            let r = chunk[0] as f32;
            let g = chunk[1] as f32;
            let b = chunk[2] as f32;
            let a = chunk[3];

            let gray_value = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
            let segment_value =
                Self::get_segment_value_partial(gray_value, &sorted_values, &segment_colors);

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

        DynamicImage::ImageRgba8(image::RgbaImage::from_raw(width, height, pixels).unwrap())
    }

    /// 获取区段值 - Partial模式
    fn get_segment_value_partial(
        gray_value: u8,
        sorted_values: &[f32],
        segment_colors: &[f32],
    ) -> u8 {
        let gray_f32 = gray_value as f32;

        // 如果只有一个滑块，分成两个区段：小于滑块值用黑色，大于等于滑块值用白色
        if sorted_values.len() == 1 {
            if gray_f32 < sorted_values[0] {
                return segment_colors[0] as u8; // 黑色
            } else {
                return segment_colors[2] as u8; // 白色
            }
        }

        // 找到灰度值所在的区段
        for i in 0..sorted_values.len() - 1 {
            let segment_start = sorted_values[i];
            let segment_end = sorted_values[i + 1];

            if gray_f32 >= segment_start && gray_f32 <= segment_end {
                // 返回对应的分段颜色
                return segment_colors[i + 1] as u8;
            }
        }

        // 处理边界情况
        if gray_f32 <= sorted_values[0] {
            // 小于第一个滑块的值，使用第一段颜色（纯黑色）
            return segment_colors[0] as u8;
        } else {
            // 大于最后一个滑块的值，使用最后一段颜色（纯白色）
            return segment_colors[segment_colors.len() - 1] as u8;
        }
    }

    /// 清理图像 - 移除孤立的像素
    pub fn clean_image(img: &DynamicImage) -> DynamicImage {
        let rgba_image = img.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let pixels = rgba_image.into_raw();

        // 创建结果像素数组
        let mut result_pixels = pixels.clone();

        // 遍历每个像素
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) as usize * 4;
                let a = pixels[idx + 3];

                // 只处理非透明像素
                if a != 0 {
                    let current_color = (pixels[idx], pixels[idx + 1], pixels[idx + 2]);

                    // 获取周围8个像素的颜色
                    let mut neighbor_colors = Vec::new();

                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue; // 跳过中心像素
                            }

                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;

                            // 检查边界
                            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                                let nidx = (ny * width as i32 + nx) as usize * 4;
                                let na = pixels[nidx + 3];

                                // 只考虑非透明像素
                                if na != 0 {
                                    let neighbor_color =
                                        (pixels[nidx], pixels[nidx + 1], pixels[nidx + 2]);
                                    neighbor_colors.push(neighbor_color);
                                }
                            }
                        }
                    }

                    // 如果当前像素颜色与所有邻居都不同，则替换它
                    if !neighbor_colors.is_empty() && !neighbor_colors.contains(&current_color) {
                        // 找到最常见的颜色
                        let mut color_counts = std::collections::HashMap::new();
                        for &color in &neighbor_colors {
                            *color_counts.entry(color).or_insert(0) += 1;
                        }

                        // 找到出现次数最多的颜色
                        let mut max_count = 0;
                        let mut most_common_color = neighbor_colors[0];

                        for (&color, &count) in &color_counts {
                            if count > max_count
                                || (count == max_count && color.0 > most_common_color.0)
                            {
                                max_count = count;
                                most_common_color = color;
                            }
                        }

                        // 替换当前像素颜色
                        result_pixels[idx] = most_common_color.0;
                        result_pixels[idx + 1] = most_common_color.1;
                        result_pixels[idx + 2] = most_common_color.2;
                        // alpha保持不变
                    }
                }
            }
        }

        DynamicImage::ImageRgba8(image::RgbaImage::from_raw(width, height, result_pixels).unwrap())
    }

    /// 获取区段值
    fn get_segment_value(gray_value: u8, sorted_values: &[f32]) -> u8 {
        let gray_f32 = gray_value as f32;

        // 如果只有一个滑块，分成两个区段：小于滑块值用第一段平均值，大于等于滑块值用第二段平均值
        if sorted_values.len() == 1 {
            if gray_f32 < sorted_values[0] {
                // 第一段：0到滑块值的平均值
                return ((0.0 + sorted_values[0]) / 2.0) as u8;
            } else {
                // 第二段：滑块值到255的平均值
                return ((sorted_values[0] + 255.0) / 2.0) as u8;
            }
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

    /// 将PNG写入包含文本元数据（tEXt）
    pub fn write_png_with_text_from_path(
        temp_path: &std::path::Path,
        out_path: &std::path::Path,
        key: &str,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let img = image::open(temp_path)?;
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        let pixels = rgba.into_raw();

        let file = std::fs::File::create(out_path)?;
        let writer = std::io::BufWriter::new(file);
        let mut encoder = png::Encoder::new(writer, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        // 写入文本元数据（键值）
        encoder.add_text_chunk(key.to_string(), value.to_string())?;
        let mut png_writer = encoder.write_header()?;
        png_writer.write_image_data(&pixels)?;
        Ok(())
    }
}

/// UI工具函数
pub struct UiUtils;

impl UiUtils {
    /// 绘制棋盘格背景
    pub fn draw_checkerboard_background(ui: &mut egui::Ui) {
        let rect = ui.available_rect_before_wrap();
        let painter = ui.painter();

        let checker_size = 20.0;
        let cols = (rect.width() / checker_size).ceil() as i32;
        let rows = (rect.height() / checker_size).ceil() as i32;

        for row in 0..rows {
            for col in 0..cols {
                let x = rect.min.x + col as f32 * checker_size;
                let y = rect.min.y + row as f32 * checker_size;

                let checker_rect = egui::Rect::from_min_size(
                    egui::Pos2::new(x, y),
                    egui::Vec2::new(checker_size, checker_size),
                );

                let is_light = (row + col) % 2 == 0;
                let color = if is_light {
                    egui::Color32::from_gray(200)
                } else {
                    egui::Color32::from_gray(150)
                };

                painter.rect_filled(checker_rect, 0.0, color);
            }
        }
    }
}

```

```rust
use crate::color_reflection_window::ColorReflectionWindow;
use crate::utils::{GrayscaleMode, ImageProcessor, UiUtils};
use egui;
use image::DynamicImage;
use std::fs;
use std::path::PathBuf;

/// 主窗口的状态
pub struct MainWindow {
    pub current_texture: Option<egui::TextureHandle>,
    pub current_path: Option<PathBuf>,
    pub zoom_factor: f32,
    pub pan_offset: egui::Vec2,
    pub original_image: Option<DynamicImage>,
    pub temp_path: Option<PathBuf>,
    pub color_reflection_window: ColorReflectionWindow,
    pub grayscale_mode: GrayscaleMode,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            current_texture: None,
            current_path: None,
            zoom_factor: 1.0,
            pan_offset: egui::Vec2::ZERO,
            original_image: None,
            temp_path: None,
            color_reflection_window: ColorReflectionWindow::default(),
            grayscale_mode: GrayscaleMode::Default,
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
                    ui.label("Black & White Mode:");
                    egui::ComboBox::from_id_salt("bw_mode")
                        .selected_text(match self.grayscale_mode {
                            GrayscaleMode::Default => "default",
                            GrayscaleMode::Max => "max",
                            GrayscaleMode::Min => "min",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.grayscale_mode,
                                GrayscaleMode::Default,
                                "default",
                            );
                            ui.selectable_value(
                                &mut self.grayscale_mode,
                                GrayscaleMode::Max,
                                "max",
                            );
                            ui.selectable_value(
                                &mut self.grayscale_mode,
                                GrayscaleMode::Min,
                                "min",
                            );
                        });
                    if ui.button("Black & White").clicked() {
                        self.apply_grayscale_current_mode(ctx);
                    }
                    if ui.button("Fast Save").clicked() {
                        self.fast_save();
                    }
                    if ui.button("Save With Anchors").clicked() {
                        self.save_with_anchors();
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
                    },
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
            &self.grayscale_mode,
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

                let rgba_image = img.to_rgba8();
                let size = [rgba_image.width() as usize, rgba_image.height() as usize];
                let pixels = rgba_image.into_raw();

                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
                let texture_options = egui::TextureOptions {
                    magnification: egui::TextureFilter::Nearest,
                    minification: egui::TextureFilter::Nearest,
                    ..Default::default()
                };
                self.current_texture =
                    Some(ctx.load_texture("loaded_image", color_image, texture_options));
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

    /// 应用当前模式的灰度（非切换，直接应用）
    fn apply_grayscale_current_mode(&mut self, ctx: &egui::Context) {
        if let Some(original_img) = &self.original_image {
            let processed_img = match self.grayscale_mode {
                GrayscaleMode::Default => {
                    ImageProcessor::convert_to_grayscale_custom(original_img)
                }
                GrayscaleMode::Max => ImageProcessor::convert_to_grayscale_max(original_img),
                GrayscaleMode::Min => ImageProcessor::convert_to_grayscale_min(original_img),
            };

            self.current_texture = Some(ImageProcessor::update_texture_from_image(
                &processed_img,
                ctx,
            ));
            if let Some(temp_path) = &self.temp_path {
                let _ = ImageProcessor::save_to_temp(&processed_img, temp_path);
            }
        }
    }

    /// 恢复原始图片
    fn original(&mut self, ctx: &egui::Context) {
        if let Some(original_img) = self.original_image.clone() {
            self.current_texture = Some(ImageProcessor::update_texture_from_image(
                &original_img,
                ctx,
            ));
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

    /// 保存并将颜色映射的锚点写入PNG文本元数据
    fn save_with_anchors(&self) {
        if let (Some(current_path), Some(temp_path)) = (&self.current_path, &self.temp_path) {
            if let Some(json) = self
                .color_reflection_window
                .build_anchors_metadata_json(&self.grayscale_mode)
            {
                match crate::utils::ImageProcessor::write_png_with_text_from_path(
                    temp_path.as_path(),
                    current_path.as_path(),
                    "anchors",
                    &json,
                ) {
                    Ok(_) => println!(
                        "Image with anchors metadata saved to: {}",
                        current_path.display()
                    ),
                    Err(e) => eprintln!("Failed to save image with anchors metadata: {}", e),
                }
            } else {
                eprintln!("No color reflection anchors available to write into metadata");
            }
        } else {
            eprintln!("No image loaded or temp file not available for saving with anchors");
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
                        self.current_texture =
                            Some(ImageProcessor::update_texture_from_image(&cleaned_img, ctx));
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

```

我已开始实现：在颜色反射应用时记录锚点、添加PNG元数据写入工具、以及新增保存按钮。现在我会总结具体改动与如何使用。

- 我已完成所有任务，并添加了“Save With Anchors”保存能力。

### 已实现的能力
- 新增按钮：在主窗口工具栏 `Fast Save` 旁新增 `Save With Anchors`。当图片做过颜色反射后，点击即可将锚点数组与模式信息写入 PNG 文本元数据后保存到当前文件。
- 记录上下文：每次点击 Color Reflection 的 “Confirm Reflection” 后，会快照记录当时的滑块锚点、反射模式；保存时用它们生成 JSON。
- PNG 元数据写入：采用 PNG `tEXt` 文本块，键为 `anchors`，值为 JSON，例如：
  {"anchors":[10,64,128,200],"reflectionMode":"Average","grayscaleMode":"Max"}

### 关键代码位置
- `Cargo.toml`：新增依赖
```6:12:Cargo.toml
[dependencies]
eframe = "0.33"
egui = "0.33"
egui_extras = "0.33"
image = "0.25"
rfd = "0.14"
png = "0.17"
```

- 颜色反射窗口：记录最近一次应用反射的快照，并构建 JSON
```6:19:src/color_reflection_window.rs
pub struct ColorReflectionWindow {
    pub show_window: bool,
    pub slider_amount_input: String,
    pub slider_amount: Option<usize>,
    pub slider_values: Vec<f32>,
    pub reflection_mode: ReflectionMode,
    pub last_applied_slider_values: Option<Vec<f32>>,
    pub last_reflection_mode: Option<ReflectionMode>,
    pub has_applied_reflection: bool,
}
```

```266:306:src/color_reflection_window.rs
fn apply_color_reflection(
    &mut self,
    ctx: &egui::Context,
    original_image: &Option<DynamicImage>,
    current_texture: &mut Option<egui::TextureHandle>,
    temp_path: &Option<PathBuf>,
    grayscale_mode: &GrayscaleMode,
) {
    // ... 处理并保存临时文件 ...
    self.snapshot_after_apply();
}
```

```307:340:src/color_reflection_window.rs
impl ColorReflectionWindow {
    pub fn snapshot_after_apply(&mut self) { /* 记录锚点与模式 */ }

    pub fn build_anchors_metadata_json(&self, grayscale_mode: &GrayscaleMode) -> Option<String> {
        // 返回包含 anchors、reflectionMode、grayscaleMode 的JSON字符串
    }
}
```

- PNG文本元数据写入工具函数（写入 tEXt）
```14:36:src/utils.rs
impl ImageProcessor {
    pub fn write_png_with_text_from_path(
        temp_path: &std::path::Path,
        out_path: &std::path::Path,
        key: &str,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 使用 png::Encoder 写入 RGBA8 与文本块
    }
}
```

- 主窗口工具栏新增按钮与保存逻辑
```100:113:src/main_window.rs
if ui.button("Fast Save").clicked() {
    self.fast_save();
}
if ui.button("Save With Anchors").clicked() {
    self.save_with_anchors();
}
```

```289:318:src/main_window.rs
fn save_with_anchors(&self) {
    if let (Some(current_path), Some(temp_path)) = (&self.current_path, &self.temp_path) {
        if let Some(json) = self.color_reflection_window.build_anchors_metadata_json(&self.grayscale_mode) {
            match crate::utils::ImageProcessor::write_png_with_text_from_path(
                temp_path.as_path(),
                current_path.as_path(),
                "anchors",
                &json,
            ) {
                Ok(_) => println!("Image with anchors metadata saved to: {}", current_path.display()),
                Err(e) => eprintln!("Failed to save image with anchors metadata: {}", e),
            }
        } else {
            eprintln!("No color reflection anchors available to write into metadata");
        }
    } else {
        eprintln!("No image loaded or temp file not available for saving with anchors");
    }
}
```

### 使用说明
- 正常流程：打开图片 → 打开 `Color Reflection` → 设置滑块/选择模式 → 点击 `Confirm Reflection` → 点击 `Save With Anchors` 即可将锚点与模式写入保存的 PNG 元数据。
- 注意：当前保存实现使用 PNG 文本元数据。由于工作流里临时文件为 `.egui_tmp.png`，保存时会以 PNG 编码输出到当前文件路径（即便原始扩展名可能不是 `.png`）。这与现有 `Fast Save` 行为一致；如需按原始格式写回并携带元数据，需要针对不同格式实现对应元数据写入路径。

需要我把锚点元数据的键名、JSON结构或写入位置（改为 iTXt/自定义键）做成可配置吗？如果你更希望保留原始文件格式，我也可以扩展为仅在原始文件为 PNG 时写入文本块，其他格式给出提示或采用旁挂 `.json`。

```toml
[dependencies]
eframe = "0.33"
egui = "0.33"
egui_extras = "0.33"
image = "0.25"
rfd = "0.14"
png = "0.17"
```

```plaintext
pub struct ColorReflectionWindow {
    pub show_window: bool,
    pub slider_amount_input: String,
    pub slider_amount: Option<usize>,
    pub slider_values: Vec<f32>,
    pub reflection_mode: ReflectionMode,
    pub last_applied_slider_values: Option<Vec<f32>>,
    pub last_reflection_mode: Option<ReflectionMode>,
    pub has_applied_reflection: bool,
}
```

```plaintext
fn apply_color_reflection(
    &mut self,
    ctx: &egui::Context,
    original_image: &Option<DynamicImage>,
    current_texture: &mut Option<egui::TextureHandle>,
    temp_path: &Option<PathBuf>,
    grayscale_mode: &GrayscaleMode,
) {
    // ... 处理并保存临时文件 ...
    self.snapshot_after_apply();
}
```

```plaintext
impl ColorReflectionWindow {
    pub fn snapshot_after_apply(&mut self) { /* 记录锚点与模式 */ }

    pub fn build_anchors_metadata_json(&self, grayscale_mode: &GrayscaleMode) -> Option<String> {
        // 返回包含 anchors、reflectionMode、grayscaleMode 的JSON字符串
    }
}
```

```plaintext
impl ImageProcessor {
    pub fn write_png_with_text_from_path(
        temp_path: &std::path::Path,
        out_path: &std::path::Path,
        key: &str,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 使用 png::Encoder 写入 RGBA8 与文本块
    }
}
```

```plaintext
if ui.button("Fast Save").clicked() {
    self.fast_save();
}
if ui.button("Save With Anchors").clicked() {
    self.save_with_anchors();
}
```

```plaintext
fn save_with_anchors(&self) {
    if let (Some(current_path), Some(temp_path)) = (&self.current_path, &self.temp_path) {
        if let Some(json) = self.color_reflection_window.build_anchors_metadata_json(&self.grayscale_mode) {
            match crate::utils::ImageProcessor::write_png_with_text_from_path(
                temp_path.as_path(),
                current_path.as_path(),
                "anchors",
                &json,
            ) {
                Ok(_) => println!("Image with anchors metadata saved to: {}", current_path.display()),
                Err(e) => eprintln!("Failed to save image with anchors metadata: {}", e),
            }
        } else {
            eprintln!("No color reflection anchors available to write into metadata");
        }
    } else {
        eprintln!("No image loaded or temp file not available for saving with anchors");
    }
}
```

---

**User**

检查编译，报错了

---

**Cursor**

我先修复可能的编译错误：为 `ReflectionMode` 增加 `Clone, Copy`，以满足此前对其 `clone()` 的使用。随后我会再次检查静态诊断。

```rust
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
        }
    }
}

impl ColorReflectionWindow {
    /// 显示Color Reflection窗口
    pub fn show(&mut self, ctx: &egui::Context, original_image: &Option<DynamicImage>, current_texture: &mut Option<egui::TextureHandle>, temp_path: &Option<PathBuf>, grayscale_mode: &GrayscaleMode) {
        if self.show_window {
            let mut show_window = self.show_window;
            egui::Window::new("Color Reflection")
                .open(&mut show_window)
                .default_size([800.0, 600.0])
                .resizable(true)
                .show(ctx, |ui| {
                    self.show_content(ui, ctx, original_image, current_texture, temp_path, grayscale_mode);
                });
            self.show_window = show_window;
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
        grayscale_mode: &GrayscaleMode,
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

        ui.add_space(10.0);

        if ui.button("Close Window").clicked() {
            self.show_window = false;
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
}

```

我已修复一个可能的编译点：为 `ReflectionMode` 增加了 `Clone, Copy`，以满足我们在快照记录中对其执行 `clone()` 的用法。静态检查未发现新的报错。

- 若仍有编译错误，请把编译器的具体错误信息贴出来（错误代码与报错行），我会立即对症修复。
- 当前实现要点：点击 “Confirm Reflection” 后会快照锚点与模式；点击 “Save With Anchors” 使用 PNG `tEXt` 元数据写入 `anchors` 键的 JSON 到目标文件。

---

**User**

约束导入的图片格式，要求只能是png，在reflection窗口中的最上面一排加入一个按钮，解析图片的锚点信息，解析后的滑块信息直接在滑动条上复现，如果没有滑块信息，那么就跳出窗口提示没有获取到滑块信息

---

**Cursor**

我将更新导入过滤为仅 PNG，新增读取 PNG 文本元数据的函数，并在 Color Reflection 窗口添加“加载锚点”按钮与提示窗口。随后我会检查静态诊断。

```rust
use crate::color_reflection_window::ColorReflectionWindow;
use crate::utils::{GrayscaleMode, ImageProcessor, UiUtils};
use egui;
use image::DynamicImage;
use std::fs;
use std::path::PathBuf;

/// 主窗口的状态
pub struct MainWindow {
    pub current_texture: Option<egui::TextureHandle>,
    pub current_path: Option<PathBuf>,
    pub zoom_factor: f32,
    pub pan_offset: egui::Vec2,
    pub original_image: Option<DynamicImage>,
    pub temp_path: Option<PathBuf>,
    pub color_reflection_window: ColorReflectionWindow,
    pub grayscale_mode: GrayscaleMode,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            current_texture: None,
            current_path: None,
            zoom_factor: 1.0,
            pan_offset: egui::Vec2::ZERO,
            original_image: None,
            temp_path: None,
            color_reflection_window: ColorReflectionWindow::default(),
            grayscale_mode: GrayscaleMode::Default,
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
                    ui.label("Black & White Mode:");
                    egui::ComboBox::from_id_salt("bw_mode")
                        .selected_text(match self.grayscale_mode {
                            GrayscaleMode::Default => "default",
                            GrayscaleMode::Max => "max",
                            GrayscaleMode::Min => "min",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.grayscale_mode,
                                GrayscaleMode::Default,
                                "default",
                            );
                            ui.selectable_value(
                                &mut self.grayscale_mode,
                                GrayscaleMode::Max,
                                "max",
                            );
                            ui.selectable_value(
                                &mut self.grayscale_mode,
                                GrayscaleMode::Min,
                                "min",
                            );
                        });
                    if ui.button("Black & White").clicked() {
                        self.apply_grayscale_current_mode(ctx);
                    }
                    if ui.button("Fast Save").clicked() {
                        self.fast_save();
                    }
                    if ui.button("Save With Anchors").clicked() {
                        self.save_with_anchors();
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
                    },
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
            &self.grayscale_mode,
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
            .add_filter("PNG images", &["png"]) 
            .pick_file()
        {
            // 进一步校验扩展名
            if path.extension().and_then(|s| s.to_str()).map(|s| s.eq_ignore_ascii_case("png")).unwrap_or(false) {
                self.load_image(ctx, &path);
            } else {
                eprintln!("Only PNG format is supported");
            }
        }
    }

    /// 加载图片
    fn load_image(&mut self, ctx: &egui::Context, path: &std::path::Path) {
        match image::open(path) {
            Ok(img) => {
                self.original_image = Some(img.clone());

                let rgba_image = img.to_rgba8();
                let size = [rgba_image.width() as usize, rgba_image.height() as usize];
                let pixels = rgba_image.into_raw();

                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
                let texture_options = egui::TextureOptions {
                    magnification: egui::TextureFilter::Nearest,
                    minification: egui::TextureFilter::Nearest,
                    ..Default::default()
                };
                self.current_texture =
                    Some(ctx.load_texture("loaded_image", color_image, texture_options));
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

    /// 应用当前模式的灰度（非切换，直接应用）
    fn apply_grayscale_current_mode(&mut self, ctx: &egui::Context) {
        if let Some(original_img) = &self.original_image {
            let processed_img = match self.grayscale_mode {
                GrayscaleMode::Default => {
                    ImageProcessor::convert_to_grayscale_custom(original_img)
                }
                GrayscaleMode::Max => ImageProcessor::convert_to_grayscale_max(original_img),
                GrayscaleMode::Min => ImageProcessor::convert_to_grayscale_min(original_img),
            };

            self.current_texture = Some(ImageProcessor::update_texture_from_image(
                &processed_img,
                ctx,
            ));
            if let Some(temp_path) = &self.temp_path {
                let _ = ImageProcessor::save_to_temp(&processed_img, temp_path);
            }
        }
    }

    /// 恢复原始图片
    fn original(&mut self, ctx: &egui::Context) {
        if let Some(original_img) = self.original_image.clone() {
            self.current_texture = Some(ImageProcessor::update_texture_from_image(
                &original_img,
                ctx,
            ));
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

    /// 保存并将颜色映射的锚点写入PNG文本元数据
    fn save_with_anchors(&self) {
        if let (Some(current_path), Some(temp_path)) = (&self.current_path, &self.temp_path) {
            if let Some(json) = self
                .color_reflection_window
                .build_anchors_metadata_json(&self.grayscale_mode)
            {
                match crate::utils::ImageProcessor::write_png_with_text_from_path(
                    temp_path.as_path(),
                    current_path.as_path(),
                    "anchors",
                    &json,
                ) {
                    Ok(_) => println!(
                        "Image with anchors metadata saved to: {}",
                        current_path.display()
                    ),
                    Err(e) => eprintln!("Failed to save image with anchors metadata: {}", e),
                }
            } else {
                eprintln!("No color reflection anchors available to write into metadata");
            }
        } else {
            eprintln!("No image loaded or temp file not available for saving with anchors");
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
                        self.current_texture =
                            Some(ImageProcessor::update_texture_from_image(&cleaned_img, ctx));
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

```

```rust
use image::DynamicImage;
use std::path::PathBuf;

/// 图像处理工具函数
pub struct ImageProcessor;

#[derive(Clone, Copy, PartialEq)]
pub enum GrayscaleMode {
    Default,
    Max,
    Min,
}

impl ImageProcessor {
    /// 更新纹理从图像
    pub fn update_texture_from_image(
        img: &DynamicImage,
        ctx: &egui::Context,
    ) -> egui::TextureHandle {
        let rgba_image = img.to_rgba8();
        let size = [rgba_image.width() as usize, rgba_image.height() as usize];
        let pixels = rgba_image.into_raw();

        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
        let texture_options = egui::TextureOptions {
            magnification: egui::TextureFilter::Nearest,
            minification: egui::TextureFilter::Nearest,
            ..Default::default()
        };
        ctx.load_texture("processed_image", color_image, texture_options)
    }

    /// 保存图像到临时文件
    pub fn save_to_temp(
        img: &DynamicImage,
        temp_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        img.save(temp_path)?;
        Ok(())
    }

    /// 自定义灰度转换 (ITU-R BT.601标准)
    pub fn convert_to_grayscale_custom(img: &DynamicImage) -> DynamicImage {
        let rgba_image = img.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let mut pixels = rgba_image.into_raw();

        for chunk in pixels.chunks_exact_mut(4) {
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            let a = chunk[3];

            let luma = ((299 * r + 587 * g + 114 * b) / 1000) as u8;

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

        DynamicImage::ImageRgba8(image::RgbaImage::from_raw(width, height, pixels).unwrap())
    }

    pub fn convert_to_grayscale_max(img: &DynamicImage) -> DynamicImage {
        let rgba_image = img.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let mut pixels = rgba_image.into_raw();

        for chunk in pixels.chunks_exact_mut(4) {
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            let a = chunk[3];

            let luma = (r.max(g).max(b)) as u8;

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

        DynamicImage::ImageRgba8(image::RgbaImage::from_raw(width, height, pixels).unwrap())
    }

    pub fn convert_to_grayscale_min(img: &DynamicImage) -> DynamicImage {
        let rgba_image = img.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let mut pixels = rgba_image.into_raw();

        for chunk in pixels.chunks_exact_mut(4) {
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            let a = chunk[3];

            let luma = (r.min(g).min(b)) as u8;

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

        DynamicImage::ImageRgba8(image::RgbaImage::from_raw(width, height, pixels).unwrap())
    }

    // 删除：旧的未使用 average 版本（避免 dead_code 警告）

    /// 应用颜色反射处理（根据灰度模式预处理）
    pub fn apply_color_reflection_with_mode(
        original_img: &DynamicImage,
        slider_values: &[f32],
        mode: GrayscaleMode,
    ) -> DynamicImage {
        let gray_img = match mode {
            GrayscaleMode::Default => Self::convert_to_grayscale_custom(original_img),
            GrayscaleMode::Max => Self::convert_to_grayscale_max(original_img),
            GrayscaleMode::Min => Self::convert_to_grayscale_min(original_img),
        };

        let mut sorted_values = slider_values.to_vec();
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let rgba_image = gray_img.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let mut pixels = rgba_image.into_raw();

        for chunk in pixels.chunks_exact_mut(4) {
            let r = chunk[0] as f32;
            let g = chunk[1] as f32;
            let b = chunk[2] as f32;
            let a = chunk[3];

            let gray_value = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
            let segment_value = Self::get_segment_value(gray_value, &sorted_values);

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

        DynamicImage::ImageRgba8(image::RgbaImage::from_raw(width, height, pixels).unwrap())
    }

    /// 应用颜色反射处理 Partial（根据灰度模式预处理）
    pub fn apply_color_reflection_partial_with_mode(
        original_img: &DynamicImage,
        slider_values: &[f32],
        mode: GrayscaleMode,
    ) -> DynamicImage {
        let gray_img = match mode {
            GrayscaleMode::Default => Self::convert_to_grayscale_custom(original_img),
            GrayscaleMode::Max => Self::convert_to_grayscale_max(original_img),
            GrayscaleMode::Min => Self::convert_to_grayscale_min(original_img),
        };

        let mut sorted_values = slider_values.to_vec();
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let rgba_image = gray_img.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let mut pixels = rgba_image.into_raw();

        let total_segments = sorted_values.len() + 2;
        let segment_size = 255.0 / total_segments as f32;
        let mut segment_colors = Vec::new();
        segment_colors.push(0.0);
        for i in 1..=sorted_values.len() {
            let segment_start = i as f32 * segment_size;
            let segment_end = (i + 1) as f32 * segment_size;
            let segment_avg = (segment_start + segment_end) / 2.0;
            segment_colors.push(segment_avg);
        }
        segment_colors.push(255.0);

        for chunk in pixels.chunks_exact_mut(4) {
            let r = chunk[0] as f32;
            let g = chunk[1] as f32;
            let b = chunk[2] as f32;
            let a = chunk[3];

            let gray_value = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
            let segment_value =
                Self::get_segment_value_partial(gray_value, &sorted_values, &segment_colors);

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

        DynamicImage::ImageRgba8(image::RgbaImage::from_raw(width, height, pixels).unwrap())
    }

    /// 获取区段值 - Partial模式
    fn get_segment_value_partial(
        gray_value: u8,
        sorted_values: &[f32],
        segment_colors: &[f32],
    ) -> u8 {
        let gray_f32 = gray_value as f32;

        // 如果只有一个滑块，分成两个区段：小于滑块值用黑色，大于等于滑块值用白色
        if sorted_values.len() == 1 {
            if gray_f32 < sorted_values[0] {
                return segment_colors[0] as u8; // 黑色
            } else {
                return segment_colors[2] as u8; // 白色
            }
        }

        // 找到灰度值所在的区段
        for i in 0..sorted_values.len() - 1 {
            let segment_start = sorted_values[i];
            let segment_end = sorted_values[i + 1];

            if gray_f32 >= segment_start && gray_f32 <= segment_end {
                // 返回对应的分段颜色
                return segment_colors[i + 1] as u8;
            }
        }

        // 处理边界情况
        if gray_f32 <= sorted_values[0] {
            // 小于第一个滑块的值，使用第一段颜色（纯黑色）
            return segment_colors[0] as u8;
        } else {
            // 大于最后一个滑块的值，使用最后一段颜色（纯白色）
            return segment_colors[segment_colors.len() - 1] as u8;
        }
    }

    /// 清理图像 - 移除孤立的像素
    pub fn clean_image(img: &DynamicImage) -> DynamicImage {
        let rgba_image = img.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let pixels = rgba_image.into_raw();

        // 创建结果像素数组
        let mut result_pixels = pixels.clone();

        // 遍历每个像素
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) as usize * 4;
                let a = pixels[idx + 3];

                // 只处理非透明像素
                if a != 0 {
                    let current_color = (pixels[idx], pixels[idx + 1], pixels[idx + 2]);

                    // 获取周围8个像素的颜色
                    let mut neighbor_colors = Vec::new();

                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue; // 跳过中心像素
                            }

                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;

                            // 检查边界
                            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                                let nidx = (ny * width as i32 + nx) as usize * 4;
                                let na = pixels[nidx + 3];

                                // 只考虑非透明像素
                                if na != 0 {
                                    let neighbor_color =
                                        (pixels[nidx], pixels[nidx + 1], pixels[nidx + 2]);
                                    neighbor_colors.push(neighbor_color);
                                }
                            }
                        }
                    }

                    // 如果当前像素颜色与所有邻居都不同，则替换它
                    if !neighbor_colors.is_empty() && !neighbor_colors.contains(&current_color) {
                        // 找到最常见的颜色
                        let mut color_counts = std::collections::HashMap::new();
                        for &color in &neighbor_colors {
                            *color_counts.entry(color).or_insert(0) += 1;
                        }

                        // 找到出现次数最多的颜色
                        let mut max_count = 0;
                        let mut most_common_color = neighbor_colors[0];

                        for (&color, &count) in &color_counts {
                            if count > max_count
                                || (count == max_count && color.0 > most_common_color.0)
                            {
                                max_count = count;
                                most_common_color = color;
                            }
                        }

                        // 替换当前像素颜色
                        result_pixels[idx] = most_common_color.0;
                        result_pixels[idx + 1] = most_common_color.1;
                        result_pixels[idx + 2] = most_common_color.2;
                        // alpha保持不变
                    }
                }
            }
        }

        DynamicImage::ImageRgba8(image::RgbaImage::from_raw(width, height, result_pixels).unwrap())
    }

    /// 获取区段值
    fn get_segment_value(gray_value: u8, sorted_values: &[f32]) -> u8 {
        let gray_f32 = gray_value as f32;

        // 如果只有一个滑块，分成两个区段：小于滑块值用第一段平均值，大于等于滑块值用第二段平均值
        if sorted_values.len() == 1 {
            if gray_f32 < sorted_values[0] {
                // 第一段：0到滑块值的平均值
                return ((0.0 + sorted_values[0]) / 2.0) as u8;
            } else {
                // 第二段：滑块值到255的平均值
                return ((sorted_values[0] + 255.0) / 2.0) as u8;
            }
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

    /// 将PNG写入包含文本元数据（tEXt）
    pub fn write_png_with_text_from_path(
        temp_path: &std::path::Path,
        out_path: &std::path::Path,
        key: &str,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let img = image::open(temp_path)?;
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        let pixels = rgba.into_raw();

        let file = std::fs::File::create(out_path)?;
        let writer = std::io::BufWriter::new(file);
        let mut encoder = png::Encoder::new(writer, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        // 写入文本元数据（键值）
        encoder.add_text_chunk(key.to_string(), value.to_string())?;
        let mut png_writer = encoder.write_header()?;
        png_writer.write_image_data(&pixels)?;
        Ok(())
    }

    /// 从PNG文件读取指定tEXt键的值（Latin-1文本），若无则返回None
    pub fn read_png_text_value_from_path(
        path: &std::path::Path,
        key: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        use std::io::Read;
        let mut f = std::fs::File::open(path)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;

        // PNG签名
        let signature: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
        if buf.len() < 8 || &buf[0..8] != signature {
            return Ok(None);
        }

        let mut idx = 8;
        while idx + 12 <= buf.len() {
            let length = u32::from_be_bytes([buf[idx], buf[idx + 1], buf[idx + 2], buf[idx + 3]]) as usize;
            let ctype = &buf[idx + 4..idx + 8];
            let data_start = idx + 8;
            let data_end = data_start + length;
            if data_end + 4 > buf.len() { break; }

            if ctype == b"tEXt" {
                // 文本数据：keyword(1-79) + 0x00 + text
                let data = &buf[data_start..data_end];
                if let Some(pos) = data.iter().position(|&b| b == 0) {
                    let keyword = &data[..pos];
                    let text_bytes = &data[pos + 1..];
                    if keyword == key.as_bytes() {
                        // Latin-1转String
                        let s: String = text_bytes.iter().map(|&b| b as char).collect();
                        return Ok(Some(s));
                    }
                }
            }

            // 跳到下一个chunk（包含CRC）
            idx = data_end + 4;
            if ctype == b"IEND" { break; }
        }
        Ok(None)
    }
}

/// UI工具函数
pub struct UiUtils;

impl UiUtils {
    /// 绘制棋盘格背景
    pub fn draw_checkerboard_background(ui: &mut egui::Ui) {
        let rect = ui.available_rect_before_wrap();
        let painter = ui.painter();

        let checker_size = 20.0;
        let cols = (rect.width() / checker_size).ceil() as i32;
        let rows = (rect.height() / checker_size).ceil() as i32;

        for row in 0..rows {
            for col in 0..cols {
                let x = rect.min.x + col as f32 * checker_size;
                let y = rect.min.y + row as f32 * checker_size;

                let checker_rect = egui::Rect::from_min_size(
                    egui::Pos2::new(x, y),
                    egui::Vec2::new(checker_size, checker_size),
                );

                let is_light = (row + col) % 2 == 0;
                let color = if is_light {
                    egui::Color32::from_gray(200)
                } else {
                    egui::Color32::from_gray(150)
                };

                painter.rect_filled(checker_rect, 0.0, color);
            }
        }
    }
}

```

```rust
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
    pub fn show(&mut self, ctx: &egui::Context, original_image: &Option<DynamicImage>, current_texture: &mut Option<egui::TextureHandle>, temp_path: &Option<PathBuf>, grayscale_mode: &GrayscaleMode) {
        if self.show_window {
            let mut show_window = self.show_window;
            egui::Window::new("Color Reflection")
                .open(&mut show_window)
                .default_size([800.0, 600.0])
                .resizable(true)
                .show(ctx, |ui| {
                    self.show_content(ui, ctx, original_image, current_texture, temp_path, grayscale_mode);
                });
            self.show_window = show_window;

            // 简单提示窗口
            if let Some(msg) = &self.message {
                let mut open = true;
                egui::Window::new("Info")
                    .open(&mut open)
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label(msg);
                        if ui.button("OK").clicked() { self.message = None; }
                    });
                if !open { self.message = None; }
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
        grayscale_mode: &GrayscaleMode,
    ) {
        ui.heading("Color Reflection");
        ui.separator();

        // 第一行：滑块数量输入框和确认按钮
        ui.horizontal(|ui| {
            if ui.button("Load Anchors From PNG").clicked() {
                if let Some(p) = temp_path {
                    match crate::utils::ImageProcessor::read_png_text_value_from_path(p.as_path(), "anchors") {
                        Ok(Some(json)) => {
                            if let Some(vals) = Self::parse_anchors_from_json(&json) {
                                self.slider_amount = Some(vals.len());
                                self.slider_amount_input = vals.len().to_string();
                                self.slider_values = vals;
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

        ui.add_space(10.0);

        if ui.button("Close Window").clicked() {
            self.show_window = false;
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
}

```

我已实现你的需求并完成静态检查。

- 已限制导入格式为 PNG：文件对话框只允许选择 PNG，并在选择后再次校验扩展名，非 PNG 将拒绝加载。
- 在 Color Reflection 窗口顶部新增“Load Anchors From PNG”按钮：
  - 会从当前临时 PNG 文件元数据的 tEXt 中读取 `anchors` 键的 JSON；
  - 成功解析后，直接设置滑块数量与位置并在滑动条上复现；
  - 若未读取到或格式不对，会弹出提示小窗口显示原因。

如何使用
- 打开 PNG 图片 → 进入 Color Reflection → 点击 “Load Anchors From PNG” 即可恢复锚点；若保存时用了 “Save With Anchors”，读取将复现之前的锚点。

如果你还有期望同时恢复反射模式与灰度模式，也可以告诉我，我可以一并解析并设置对应的单选与下拉。

---

**User**

反射模式不需要，但是灰度模式需要，帮我一并添加进去

---

**Cursor**

我将更新反射窗口的函数签名以允许修改主窗口的灰度模式，并在“Load Anchors From PNG”按钮处理逻辑里解析并设置 `grayscale_mode`。随后进行静态检查。

```rust
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
    pub fn show(&mut self, ctx: &egui::Context, original_image: &Option<DynamicImage>, current_texture: &mut Option<egui::TextureHandle>, temp_path: &Option<PathBuf>, grayscale_mode: &mut GrayscaleMode) {
        if self.show_window {
            let mut show_window = self.show_window;
            egui::Window::new("Color Reflection")
                .open(&mut show_window)
                .default_size([800.0, 600.0])
                .resizable(true)
                .show(ctx, |ui| {
                    self.show_content(ui, ctx, original_image, current_texture, temp_path, grayscale_mode);
                });
            self.show_window = show_window;

            // 简单提示窗口
            if let Some(msg) = &self.message {
                let mut open = true;
                egui::Window::new("Info")
                    .open(&mut open)
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label(msg);
                        if ui.button("OK").clicked() { self.message = None; }
                    });
                if !open { self.message = None; }
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
        grayscale_mode: &mut GrayscaleMode,
    ) {
        ui.heading("Color Reflection");
        ui.separator();

        // 第一行：滑块数量输入框和确认按钮
        ui.horizontal(|ui| {
            if ui.button("Load Anchors From PNG").clicked() {
                if let Some(p) = temp_path {
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

        ui.add_space(10.0);

        if ui.button("Close Window").clicked() {
            self.show_window = false;
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

```

```rust
use crate::color_reflection_window::ColorReflectionWindow;
use crate::utils::{GrayscaleMode, ImageProcessor, UiUtils};
use egui;
use image::DynamicImage;
use std::fs;
use std::path::PathBuf;

/// 主窗口的状态
pub struct MainWindow {
    pub current_texture: Option<egui::TextureHandle>,
    pub current_path: Option<PathBuf>,
    pub zoom_factor: f32,
    pub pan_offset: egui::Vec2,
    pub original_image: Option<DynamicImage>,
    pub temp_path: Option<PathBuf>,
    pub color_reflection_window: ColorReflectionWindow,
    pub grayscale_mode: GrayscaleMode,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            current_texture: None,
            current_path: None,
            zoom_factor: 1.0,
            pan_offset: egui::Vec2::ZERO,
            original_image: None,
            temp_path: None,
            color_reflection_window: ColorReflectionWindow::default(),
            grayscale_mode: GrayscaleMode::Default,
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
                    ui.label("Black & White Mode:");
                    egui::ComboBox::from_id_salt("bw_mode")
                        .selected_text(match self.grayscale_mode {
                            GrayscaleMode::Default => "default",
                            GrayscaleMode::Max => "max",
                            GrayscaleMode::Min => "min",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.grayscale_mode,
                                GrayscaleMode::Default,
                                "default",
                            );
                            ui.selectable_value(
                                &mut self.grayscale_mode,
                                GrayscaleMode::Max,
                                "max",
                            );
                            ui.selectable_value(
                                &mut self.grayscale_mode,
                                GrayscaleMode::Min,
                                "min",
                            );
                        });
                    if ui.button("Black & White").clicked() {
                        self.apply_grayscale_current_mode(ctx);
                    }
                    if ui.button("Fast Save").clicked() {
                        self.fast_save();
                    }
                    if ui.button("Save With Anchors").clicked() {
                        self.save_with_anchors();
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
                    },
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
            &mut self.grayscale_mode,
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
            .add_filter("PNG images", &["png"]) 
            .pick_file()
        {
            // 进一步校验扩展名
            if path.extension().and_then(|s| s.to_str()).map(|s| s.eq_ignore_ascii_case("png")).unwrap_or(false) {
                self.load_image(ctx, &path);
            } else {
                eprintln!("Only PNG format is supported");
            }
        }
    }

    /// 加载图片
    fn load_image(&mut self, ctx: &egui::Context, path: &std::path::Path) {
        match image::open(path) {
            Ok(img) => {
                self.original_image = Some(img.clone());

                let rgba_image = img.to_rgba8();
                let size = [rgba_image.width() as usize, rgba_image.height() as usize];
                let pixels = rgba_image.into_raw();

                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
                let texture_options = egui::TextureOptions {
                    magnification: egui::TextureFilter::Nearest,
                    minification: egui::TextureFilter::Nearest,
                    ..Default::default()
                };
                self.current_texture =
                    Some(ctx.load_texture("loaded_image", color_image, texture_options));
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

    /// 应用当前模式的灰度（非切换，直接应用）
    fn apply_grayscale_current_mode(&mut self, ctx: &egui::Context) {
        if let Some(original_img) = &self.original_image {
            let processed_img = match self.grayscale_mode {
                GrayscaleMode::Default => {
                    ImageProcessor::convert_to_grayscale_custom(original_img)
                }
                GrayscaleMode::Max => ImageProcessor::convert_to_grayscale_max(original_img),
                GrayscaleMode::Min => ImageProcessor::convert_to_grayscale_min(original_img),
            };

            self.current_texture = Some(ImageProcessor::update_texture_from_image(
                &processed_img,
                ctx,
            ));
            if let Some(temp_path) = &self.temp_path {
                let _ = ImageProcessor::save_to_temp(&processed_img, temp_path);
            }
        }
    }

    /// 恢复原始图片
    fn original(&mut self, ctx: &egui::Context) {
        if let Some(original_img) = self.original_image.clone() {
            self.current_texture = Some(ImageProcessor::update_texture_from_image(
                &original_img,
                ctx,
            ));
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

    /// 保存并将颜色映射的锚点写入PNG文本元数据
    fn save_with_anchors(&self) {
        if let (Some(current_path), Some(temp_path)) = (&self.current_path, &self.temp_path) {
            if let Some(json) = self
                .color_reflection_window
                .build_anchors_metadata_json(&self.grayscale_mode)
            {
                match crate::utils::ImageProcessor::write_png_with_text_from_path(
                    temp_path.as_path(),
                    current_path.as_path(),
                    "anchors",
                    &json,
                ) {
                    Ok(_) => println!(
                        "Image with anchors metadata saved to: {}",
                        current_path.display()
                    ),
                    Err(e) => eprintln!("Failed to save image with anchors metadata: {}", e),
                }
            } else {
                eprintln!("No color reflection anchors available to write into metadata");
            }
        } else {
            eprintln!("No image loaded or temp file not available for saving with anchors");
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
                        self.current_texture =
                            Some(ImageProcessor::update_texture_from_image(&cleaned_img, ctx));
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

```

我已把灰度模式一起恢复进来：

- 改为可写入灰度模式：`ColorReflectionWindow::show`/`show_content` 接收 `&mut GrayscaleMode`，这样在加载锚点时可直接更新主窗口的 `grayscale_mode`。
- 按钮“Load Anchors From PNG”在解析到 `anchors` 后，会继续解析 `grayscaleMode`，若存在则同步设置当前灰度模式为 `Default/Max/Min`。
- 若没有解析到 `anchors`，依旧会弹出提示窗口。

你可以现在用含有之前“Save With Anchors”保存过的 PNG 测试：点击“Load Anchors From PNG”后，滑块位置会复现，灰度模式也会自动切换。

---

**User**

检查编译


---

**Cursor**

我将修复借用冲突：克隆 `self.message` 到本地变量，在窗口闭包中使用本地副本，并通过标记在闭包外清除 `self.message`，以避免同时不可变借用与可变借用冲突。接着检查诊断。

```rust
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
    pub fn show(&mut self, ctx: &egui::Context, original_image: &Option<DynamicImage>, current_texture: &mut Option<egui::TextureHandle>, temp_path: &Option<PathBuf>, grayscale_mode: &mut GrayscaleMode) {
        if self.show_window {
            let mut show_window = self.show_window;
            egui::Window::new("Color Reflection")
                .open(&mut show_window)
                .default_size([800.0, 600.0])
                .resizable(true)
                .show(ctx, |ui| {
                    self.show_content(ui, ctx, original_image, current_texture, temp_path, grayscale_mode);
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
        grayscale_mode: &mut GrayscaleMode,
    ) {
        ui.heading("Color Reflection");
        ui.separator();

        // 第一行：滑块数量输入框和确认按钮
        ui.horizontal(|ui| {
            if ui.button("Load Anchors From PNG").clicked() {
                if let Some(p) = temp_path {
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

        ui.add_space(10.0);

        if ui.button("Close Window").clicked() {
            self.show_window = false;
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

```

我已修复 `color_reflection_window.rs` 的借用冲突：提示窗口现在克隆 `self.message`，在闭包内只读使用，并通过标记在闭包外清除，从而避免同时不可变借用与可变借用导致的 E0500。静态检查通过。

- 可以重新 `cargo run` 测试。
- 如果还有其它编译错误，请贴出报错信息。

---

**User**

前面的改动中，元数据的写入好像有问题

---

**Cursor**

我将更新 Color Reflection 窗口以优先从当前文件路径读取元数据（若无则回退到临时文件），并在“Save With Anchors”保存时同时把带元数据的PNG写回当前路径与临时文件，确保“Load Anchors From PNG”立即可用。现在开始应用这些编辑。

```rust
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

```

```rust
use crate::color_reflection_window::ColorReflectionWindow;
use crate::utils::{GrayscaleMode, ImageProcessor, UiUtils};
use egui;
use image::DynamicImage;
use std::fs;
use std::path::PathBuf;

/// 主窗口的状态
pub struct MainWindow {
    pub current_texture: Option<egui::TextureHandle>,
    pub current_path: Option<PathBuf>,
    pub zoom_factor: f32,
    pub pan_offset: egui::Vec2,
    pub original_image: Option<DynamicImage>,
    pub temp_path: Option<PathBuf>,
    pub color_reflection_window: ColorReflectionWindow,
    pub grayscale_mode: GrayscaleMode,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            current_texture: None,
            current_path: None,
            zoom_factor: 1.0,
            pan_offset: egui::Vec2::ZERO,
            original_image: None,
            temp_path: None,
            color_reflection_window: ColorReflectionWindow::default(),
            grayscale_mode: GrayscaleMode::Default,
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
                    ui.label("Black & White Mode:");
                    egui::ComboBox::from_id_salt("bw_mode")
                        .selected_text(match self.grayscale_mode {
                            GrayscaleMode::Default => "default",
                            GrayscaleMode::Max => "max",
                            GrayscaleMode::Min => "min",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.grayscale_mode,
                                GrayscaleMode::Default,
                                "default",
                            );
                            ui.selectable_value(
                                &mut self.grayscale_mode,
                                GrayscaleMode::Max,
                                "max",
                            );
                            ui.selectable_value(
                                &mut self.grayscale_mode,
                                GrayscaleMode::Min,
                                "min",
                            );
                        });
                    if ui.button("Black & White").clicked() {
                        self.apply_grayscale_current_mode(ctx);
                    }
                    if ui.button("Fast Save").clicked() {
                        self.fast_save();
                    }
                    if ui.button("Save With Anchors").clicked() {
                        self.save_with_anchors();
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
                    },
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
            &self.current_path,
            &mut self.grayscale_mode,
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
            .add_filter("PNG images", &["png"]) 
            .pick_file()
        {
            // 进一步校验扩展名
            if path.extension().and_then(|s| s.to_str()).map(|s| s.eq_ignore_ascii_case("png")).unwrap_or(false) {
                self.load_image(ctx, &path);
            } else {
                eprintln!("Only PNG format is supported");
            }
        }
    }

    /// 加载图片
    fn load_image(&mut self, ctx: &egui::Context, path: &std::path::Path) {
        match image::open(path) {
            Ok(img) => {
                self.original_image = Some(img.clone());

                let rgba_image = img.to_rgba8();
                let size = [rgba_image.width() as usize, rgba_image.height() as usize];
                let pixels = rgba_image.into_raw();

                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
                let texture_options = egui::TextureOptions {
                    magnification: egui::TextureFilter::Nearest,
                    minification: egui::TextureFilter::Nearest,
                    ..Default::default()
                };
                self.current_texture =
                    Some(ctx.load_texture("loaded_image", color_image, texture_options));
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

    /// 应用当前模式的灰度（非切换，直接应用）
    fn apply_grayscale_current_mode(&mut self, ctx: &egui::Context) {
        if let Some(original_img) = &self.original_image {
            let processed_img = match self.grayscale_mode {
                GrayscaleMode::Default => {
                    ImageProcessor::convert_to_grayscale_custom(original_img)
                }
                GrayscaleMode::Max => ImageProcessor::convert_to_grayscale_max(original_img),
                GrayscaleMode::Min => ImageProcessor::convert_to_grayscale_min(original_img),
            };

            self.current_texture = Some(ImageProcessor::update_texture_from_image(
                &processed_img,
                ctx,
            ));
            if let Some(temp_path) = &self.temp_path {
                let _ = ImageProcessor::save_to_temp(&processed_img, temp_path);
            }
        }
    }

    /// 恢复原始图片
    fn original(&mut self, ctx: &egui::Context) {
        if let Some(original_img) = self.original_image.clone() {
            self.current_texture = Some(ImageProcessor::update_texture_from_image(
                &original_img,
                ctx,
            ));
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

    /// 保存并将颜色映射的锚点写入PNG文本元数据
    fn save_with_anchors(&self) {
        if let (Some(current_path), Some(temp_path)) = (&self.current_path, &self.temp_path) {
            if let Some(json) = self
                .color_reflection_window
                .build_anchors_metadata_json(&self.grayscale_mode)
            {
                match crate::utils::ImageProcessor::write_png_with_text_from_path(
                    temp_path.as_path(),
                    current_path.as_path(),
                    "anchors",
                    &json,
                ) {
                    Ok(_) => println!(
                        "Image with anchors metadata saved to: {}",
                        current_path.display()
                    ),
                    Err(e) => eprintln!("Failed to save image with anchors metadata: {}", e),
                }
            } else {
                eprintln!("No color reflection anchors available to write into metadata");
            }
        } else {
            eprintln!("No image loaded or temp file not available for saving with anchors");
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
                        self.current_texture =
                            Some(ImageProcessor::update_texture_from_image(&cleaned_img, ctx));
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

```

```rust
use crate::color_reflection_window::ColorReflectionWindow;
use crate::utils::{GrayscaleMode, ImageProcessor, UiUtils};
use egui;
use image::DynamicImage;
use std::fs;
use std::path::PathBuf;

/// 主窗口的状态
pub struct MainWindow {
    pub current_texture: Option<egui::TextureHandle>,
    pub current_path: Option<PathBuf>,
    pub zoom_factor: f32,
    pub pan_offset: egui::Vec2,
    pub original_image: Option<DynamicImage>,
    pub temp_path: Option<PathBuf>,
    pub color_reflection_window: ColorReflectionWindow,
    pub grayscale_mode: GrayscaleMode,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            current_texture: None,
            current_path: None,
            zoom_factor: 1.0,
            pan_offset: egui::Vec2::ZERO,
            original_image: None,
            temp_path: None,
            color_reflection_window: ColorReflectionWindow::default(),
            grayscale_mode: GrayscaleMode::Default,
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
                    ui.label("Black & White Mode:");
                    egui::ComboBox::from_id_salt("bw_mode")
                        .selected_text(match self.grayscale_mode {
                            GrayscaleMode::Default => "default",
                            GrayscaleMode::Max => "max",
                            GrayscaleMode::Min => "min",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.grayscale_mode,
                                GrayscaleMode::Default,
                                "default",
                            );
                            ui.selectable_value(
                                &mut self.grayscale_mode,
                                GrayscaleMode::Max,
                                "max",
                            );
                            ui.selectable_value(
                                &mut self.grayscale_mode,
                                GrayscaleMode::Min,
                                "min",
                            );
                        });
                    if ui.button("Black & White").clicked() {
                        self.apply_grayscale_current_mode(ctx);
                    }
                    if ui.button("Fast Save").clicked() {
                        self.fast_save();
                    }
                    if ui.button("Save With Anchors").clicked() {
                        self.save_with_anchors();
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
                    },
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
            &self.current_path,
            &mut self.grayscale_mode,
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
            .add_filter("PNG images", &["png"]) 
            .pick_file()
        {
            // 进一步校验扩展名
            if path.extension().and_then(|s| s.to_str()).map(|s| s.eq_ignore_ascii_case("png")).unwrap_or(false) {
                self.load_image(ctx, &path);
            } else {
                eprintln!("Only PNG format is supported");
            }
        }
    }

    /// 加载图片
    fn load_image(&mut self, ctx: &egui::Context, path: &std::path::Path) {
        match image::open(path) {
            Ok(img) => {
                self.original_image = Some(img.clone());

                let rgba_image = img.to_rgba8();
                let size = [rgba_image.width() as usize, rgba_image.height() as usize];
                let pixels = rgba_image.into_raw();

                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
                let texture_options = egui::TextureOptions {
                    magnification: egui::TextureFilter::Nearest,
                    minification: egui::TextureFilter::Nearest,
                    ..Default::default()
                };
                self.current_texture =
                    Some(ctx.load_texture("loaded_image", color_image, texture_options));
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

    /// 应用当前模式的灰度（非切换，直接应用）
    fn apply_grayscale_current_mode(&mut self, ctx: &egui::Context) {
        if let Some(original_img) = &self.original_image {
            let processed_img = match self.grayscale_mode {
                GrayscaleMode::Default => {
                    ImageProcessor::convert_to_grayscale_custom(original_img)
                }
                GrayscaleMode::Max => ImageProcessor::convert_to_grayscale_max(original_img),
                GrayscaleMode::Min => ImageProcessor::convert_to_grayscale_min(original_img),
            };

            self.current_texture = Some(ImageProcessor::update_texture_from_image(
                &processed_img,
                ctx,
            ));
            if let Some(temp_path) = &self.temp_path {
                let _ = ImageProcessor::save_to_temp(&processed_img, temp_path);
            }
        }
    }

    /// 恢复原始图片
    fn original(&mut self, ctx: &egui::Context) {
        if let Some(original_img) = self.original_image.clone() {
            self.current_texture = Some(ImageProcessor::update_texture_from_image(
                &original_img,
                ctx,
            ));
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

    /// 保存并将颜色映射的锚点写入PNG文本元数据
    fn save_with_anchors(&self) {
        if let (Some(current_path), Some(temp_path)) = (&self.current_path, &self.temp_path) {
            if let Some(json) = self
                .color_reflection_window
                .build_anchors_metadata_json(&self.grayscale_mode)
            {
                match crate::utils::ImageProcessor::write_png_with_text_from_path(
                    temp_path.as_path(),
                    current_path.as_path(),
                    "anchors",
                    &json,
                ) {
                    Ok(_) => println!(
                        "Image with anchors metadata saved to: {}",
                        current_path.display()
                    ),
                    Err(e) => eprintln!("Failed to save image with anchors metadata: {}", e),
                }

                // 同步更新临时文件也带有元数据，便于会话内读取
                let _ = crate::utils::ImageProcessor::write_png_with_text_from_path(
                    temp_path.as_path(),
                    temp_path.as_path(),
                    "anchors",
                    &json,
                );
            } else {
                eprintln!("No color reflection anchors available to write into metadata");
            }
        } else {
            eprintln!("No image loaded or temp file not available for saving with anchors");
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
                        self.current_texture =
                            Some(ImageProcessor::update_texture_from_image(&cleaned_img, ctx));
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

```

我定位到问题关键：读取锚点时是从临时文件读取，但“Save With Anchors”只把元数据写到了当前文件，导致会话内“Load Anchors From PNG”读不到。已做两点修复：

- 读取来源调整：`Load Anchors From PNG` 现在优先从 `current_path` 读取，若无再回退 `temp_path`。
- 保存时同步写临时文件：在写入 `current_path` 的同时，把带 `anchors` 元数据的 PNG 同步写回 `temp_path`，保证会话内可立即读取。

你可以重新运行：
- 进行一次 Color Reflection → 点击 “Save With Anchors”
- 打开 Color Reflection → 点击 “Load Anchors From PNG”
  - 滑块应复现；若 JSON 中包含 `grayscaleMode`，灰度模式会同步设置

如果仍有异常，请告诉我出现问题的具体文件路径与操作顺序。

---

**User**

这次似乎是可以了但是我在windows的文件属性对话框里为什么找不到相关的元数据？

---

**Cursor**


Windows 文件属性对话框默认不显示 PNG 的 `tEXt` 文本块；这些块是有效的 PNG 元数据，但资源管理器不展示。我们先用代码验证写入是否成功，再确认是否需要改用 Windows 可见的格式。


Windows 文件属性对话框默认不显示 PNG 的 `tEXt` 文本块；这些块是有效的 PNG 元数据，但 Windows 资源管理器不展示。我们添加一个验证功能，在应用内显示保存后的元数据读取结果，便于确认是否成功写入。


在保存后添加验证步骤：读取刚写入的元数据并显示，同时在 Color Reflection 窗口添加一个按钮显示当前文件的元数据内容。


```rust
use crate::color_reflection_window::ColorReflectionWindow;
use crate::utils::{GrayscaleMode, ImageProcessor, UiUtils};
use egui;
use image::DynamicImage;
use std::fs;
use std::path::PathBuf;

/// 主窗口的状态
pub struct MainWindow {
    pub current_texture: Option<egui::TextureHandle>,
    pub current_path: Option<PathBuf>,
    pub zoom_factor: f32,
    pub pan_offset: egui::Vec2,
    pub original_image: Option<DynamicImage>,
    pub temp_path: Option<PathBuf>,
    pub color_reflection_window: ColorReflectionWindow,
    pub grayscale_mode: GrayscaleMode,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            current_texture: None,
            current_path: None,
            zoom_factor: 1.0,
            pan_offset: egui::Vec2::ZERO,
            original_image: None,
            temp_path: None,
            color_reflection_window: ColorReflectionWindow::default(),
            grayscale_mode: GrayscaleMode::Default,
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
                    ui.label("Black & White Mode:");
                    egui::ComboBox::from_id_salt("bw_mode")
                        .selected_text(match self.grayscale_mode {
                            GrayscaleMode::Default => "default",
                            GrayscaleMode::Max => "max",
                            GrayscaleMode::Min => "min",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.grayscale_mode,
                                GrayscaleMode::Default,
                                "default",
                            );
                            ui.selectable_value(
                                &mut self.grayscale_mode,
                                GrayscaleMode::Max,
                                "max",
                            );
                            ui.selectable_value(
                                &mut self.grayscale_mode,
                                GrayscaleMode::Min,
                                "min",
                            );
                        });
                    if ui.button("Black & White").clicked() {
                        self.apply_grayscale_current_mode(ctx);
                    }
                    if ui.button("Fast Save").clicked() {
                        self.fast_save();
                    }
                    if ui.button("Save With Anchors").clicked() {
                        self.save_with_anchors();
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
                    },
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
            &self.current_path,
            &mut self.grayscale_mode,
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
            .add_filter("PNG images", &["png"]) 
            .pick_file()
        {
            // 进一步校验扩展名
            if path.extension().and_then(|s| s.to_str()).map(|s| s.eq_ignore_ascii_case("png")).unwrap_or(false) {
                self.load_image(ctx, &path);
            } else {
                eprintln!("Only PNG format is supported");
            }
        }
    }

    /// 加载图片
    fn load_image(&mut self, ctx: &egui::Context, path: &std::path::Path) {
        match image::open(path) {
            Ok(img) => {
                self.original_image = Some(img.clone());

                let rgba_image = img.to_rgba8();
                let size = [rgba_image.width() as usize, rgba_image.height() as usize];
                let pixels = rgba_image.into_raw();

                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
                let texture_options = egui::TextureOptions {
                    magnification: egui::TextureFilter::Nearest,
                    minification: egui::TextureFilter::Nearest,
                    ..Default::default()
                };
                self.current_texture =
                    Some(ctx.load_texture("loaded_image", color_image, texture_options));
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

    /// 应用当前模式的灰度（非切换，直接应用）
    fn apply_grayscale_current_mode(&mut self, ctx: &egui::Context) {
        if let Some(original_img) = &self.original_image {
            let processed_img = match self.grayscale_mode {
                GrayscaleMode::Default => {
                    ImageProcessor::convert_to_grayscale_custom(original_img)
                }
                GrayscaleMode::Max => ImageProcessor::convert_to_grayscale_max(original_img),
                GrayscaleMode::Min => ImageProcessor::convert_to_grayscale_min(original_img),
            };

            self.current_texture = Some(ImageProcessor::update_texture_from_image(
                &processed_img,
                ctx,
            ));
            if let Some(temp_path) = &self.temp_path {
                let _ = ImageProcessor::save_to_temp(&processed_img, temp_path);
            }
        }
    }

    /// 恢复原始图片
    fn original(&mut self, ctx: &egui::Context) {
        if let Some(original_img) = self.original_image.clone() {
            self.current_texture = Some(ImageProcessor::update_texture_from_image(
                &original_img,
                ctx,
            ));
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

    /// 保存并将颜色映射的锚点写入PNG文本元数据
    fn save_with_anchors(&self) {
        if let (Some(current_path), Some(temp_path)) = (&self.current_path, &self.temp_path) {
            if let Some(json) = self
                .color_reflection_window
                .build_anchors_metadata_json(&self.grayscale_mode)
            {
                match crate::utils::ImageProcessor::write_png_with_text_from_path(
                    temp_path.as_path(),
                    current_path.as_path(),
                    "anchors",
                    &json,
                ) {
                    Ok(_) => {
                        println!(
                            "Image with anchors metadata saved to: {}",
                            current_path.display()
                        );
                        // 验证：立即读取并打印元数据，证明写入成功
                        if let Ok(Some(verified)) = crate::utils::ImageProcessor::read_png_text_value_from_path(
                            current_path.as_path(),
                            "anchors",
                        ) {
                            println!("Metadata verified: {}", verified);
                        } else {
                            println!("Warning: Could not verify metadata after save");
                        }
                    }
                    Err(e) => eprintln!("Failed to save image with anchors metadata: {}", e),
                }

                // 同步更新临时文件也带有元数据，便于会话内读取
                let _ = crate::utils::ImageProcessor::write_png_with_text_from_path(
                    temp_path.as_path(),
                    temp_path.as_path(),
                    "anchors",
                    &json,
                );
            } else {
                eprintln!("No color reflection anchors available to write into metadata");
            }
        } else {
            eprintln!("No image loaded or temp file not available for saving with anchors");
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
                        self.current_texture =
                            Some(ImageProcessor::update_texture_from_image(&cleaned_img, ctx));
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

```

在 Color Reflection 窗口添加一个按钮，用于显示并验证当前文件的元数据：


在 Color Reflection 窗口添加一个“Verify Metadata”按钮，用于显示当前文件的元数据内容：


```rust
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

            if ui.button("Verify Metadata").clicked() {
                // 验证并显示元数据内容
                let pick_path = current_path.as_ref().or(temp_path.as_ref());
                if let Some(p) = pick_path {
                    match crate::utils::ImageProcessor::read_png_text_value_from_path(p.as_path(), "anchors") {
                        Ok(Some(json)) => {
                            self.message = Some(format!("Metadata found: {}", json));
                        }
                        Ok(None) => {
                            self.message = Some("No metadata found in PNG file".to_string());
                        }
                        Err(e) => {
                            self.message = Some(format!("Error reading metadata: {}", e));
                        }
                    }
                } else {
                    self.message = Some("No image file available".to_string());
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

```

已添加验证功能。说明如下：

## 为什么 Windows 属性对话框不显示 PNG 元数据

PNG 的 `tEXt` 文本块是有效的 PNG 元数据，但 Windows 资源管理器的属性对话框默认不显示这些块。它通常只显示 EXIF、XMP 等更常见的元数据格式。

## 如何验证元数据已成功写入

我添加了两种验证方式：

### 1. 控制台验证（自动）
点击 "Save With Anchors" 后，控制台会打印：
- `Image with anchors metadata saved to: ...`
- `Metadata verified: {"anchors":[10,64,128],"reflectionMode":"Average","grayscaleMode":"Max"}`

如果看到 "Metadata verified" 消息，说明元数据已成功写入。

### 2. 应用内验证按钮
在 Color Reflection 窗口中，点击 "Verify Metadata" 按钮，会弹出一个信息窗口显示：
- 如果找到元数据：显示完整的 JSON 内容
- 如果没有找到：提示 "No metadata found in PNG file"

## 其他验证工具

如果想用外部工具验证，可以使用：
- **ExifTool**：`exiftool -a image.png` 会显示所有元数据，包括 tEXt 块
- **pngcheck**：`pngcheck -v image.png` 会列出所有 PNG chunks
- **在线工具**：一些在线 PNG 查看器可以显示元数据

总结：元数据已正确写入 PNG 文件，只是 Windows 属性对话框不显示。你可以使用应用内的 "Verify Metadata" 按钮或查看控制台输出来确认。

---

**User**

删掉Verify Metadata相关的代码。我不需要这个功能

---

**Cursor**

我将删除 Color Reflection 窗口中“Verify Metadata”按钮及其逻辑，不影响其他功能。随后检查静态诊断。

```rust
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

```

已移除 Color Reflection 窗口中的“Verify Metadata”按钮及相关代码，其他功能不受影响。静态检查通过。

- 仍保留“Load Anchors From PNG”及提示弹窗。
- “Save With Anchors”后端的写入与控制台校验未改动；如果也要去掉控制台校验提示，可说一声我一起删除。

---

**User**

现在有一个小问题，当前一张图片处理完之后，打开新的图片的时候，前一张图片的临时文件没有被删除，帮我解决这个问题

---

**Cursor**

我将更新在成功加载新图片时删除上一张图片的临时文件，避免残留。随后我会做一次静态检查。

```rust
use crate::color_reflection_window::ColorReflectionWindow;
use crate::utils::{GrayscaleMode, ImageProcessor, UiUtils};
use egui;
use image::DynamicImage;
use std::fs;
use std::path::PathBuf;

/// 主窗口的状态
pub struct MainWindow {
    pub current_texture: Option<egui::TextureHandle>,
    pub current_path: Option<PathBuf>,
    pub zoom_factor: f32,
    pub pan_offset: egui::Vec2,
    pub original_image: Option<DynamicImage>,
    pub temp_path: Option<PathBuf>,
    pub color_reflection_window: ColorReflectionWindow,
    pub grayscale_mode: GrayscaleMode,
}

impl Default for MainWindow {
    fn default() -> Self {
        Self {
            current_texture: None,
            current_path: None,
            zoom_factor: 1.0,
            pan_offset: egui::Vec2::ZERO,
            original_image: None,
            temp_path: None,
            color_reflection_window: ColorReflectionWindow::default(),
            grayscale_mode: GrayscaleMode::Default,
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
                    ui.label("Black & White Mode:");
                    egui::ComboBox::from_id_salt("bw_mode")
                        .selected_text(match self.grayscale_mode {
                            GrayscaleMode::Default => "default",
                            GrayscaleMode::Max => "max",
                            GrayscaleMode::Min => "min",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.grayscale_mode,
                                GrayscaleMode::Default,
                                "default",
                            );
                            ui.selectable_value(
                                &mut self.grayscale_mode,
                                GrayscaleMode::Max,
                                "max",
                            );
                            ui.selectable_value(
                                &mut self.grayscale_mode,
                                GrayscaleMode::Min,
                                "min",
                            );
                        });
                    if ui.button("Black & White").clicked() {
                        self.apply_grayscale_current_mode(ctx);
                    }
                    if ui.button("Fast Save").clicked() {
                        self.fast_save();
                    }
                    if ui.button("Save With Anchors").clicked() {
                        self.save_with_anchors();
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
                    },
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
            &self.current_path,
            &mut self.grayscale_mode,
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
            .add_filter("PNG images", &["png"]) 
            .pick_file()
        {
            // 进一步校验扩展名
            if path.extension().and_then(|s| s.to_str()).map(|s| s.eq_ignore_ascii_case("png")).unwrap_or(false) {
                self.load_image(ctx, &path);
            } else {
                eprintln!("Only PNG format is supported");
            }
        }
    }

    /// 加载图片
    fn load_image(&mut self, ctx: &egui::Context, path: &std::path::Path) {
        match image::open(path) {
            Ok(img) => {
                // 清理上一张图片的临时文件
                if let Some(old_temp) = self.temp_path.take() {
                    let _ = fs::remove_file(old_temp);
                }

                self.original_image = Some(img.clone());

                let rgba_image = img.to_rgba8();
                let size = [rgba_image.width() as usize, rgba_image.height() as usize];
                let pixels = rgba_image.into_raw();

                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
                let texture_options = egui::TextureOptions {
                    magnification: egui::TextureFilter::Nearest,
                    minification: egui::TextureFilter::Nearest,
                    ..Default::default()
                };
                self.current_texture =
                    Some(ctx.load_texture("loaded_image", color_image, texture_options));
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

    /// 应用当前模式的灰度（非切换，直接应用）
    fn apply_grayscale_current_mode(&mut self, ctx: &egui::Context) {
        if let Some(original_img) = &self.original_image {
            let processed_img = match self.grayscale_mode {
                GrayscaleMode::Default => {
                    ImageProcessor::convert_to_grayscale_custom(original_img)
                }
                GrayscaleMode::Max => ImageProcessor::convert_to_grayscale_max(original_img),
                GrayscaleMode::Min => ImageProcessor::convert_to_grayscale_min(original_img),
            };

            self.current_texture = Some(ImageProcessor::update_texture_from_image(
                &processed_img,
                ctx,
            ));
            if let Some(temp_path) = &self.temp_path {
                let _ = ImageProcessor::save_to_temp(&processed_img, temp_path);
            }
        }
    }

    /// 恢复原始图片
    fn original(&mut self, ctx: &egui::Context) {
        if let Some(original_img) = self.original_image.clone() {
            self.current_texture = Some(ImageProcessor::update_texture_from_image(
                &original_img,
                ctx,
            ));
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

    /// 保存并将颜色映射的锚点写入PNG文本元数据
    fn save_with_anchors(&self) {
        if let (Some(current_path), Some(temp_path)) = (&self.current_path, &self.temp_path) {
            if let Some(json) = self
                .color_reflection_window
                .build_anchors_metadata_json(&self.grayscale_mode)
            {
                match crate::utils::ImageProcessor::write_png_with_text_from_path(
                    temp_path.as_path(),
                    current_path.as_path(),
                    "anchors",
                    &json,
                ) {
                    Ok(_) => {
                        println!(
                            "Image with anchors metadata saved to: {}",
                            current_path.display()
                        );
                        // 验证：立即读取并打印元数据，证明写入成功
                        if let Ok(Some(verified)) = crate::utils::ImageProcessor::read_png_text_value_from_path(
                            current_path.as_path(),
                            "anchors",
                        ) {
                            println!("Metadata verified: {}", verified);
                        } else {
                            println!("Warning: Could not verify metadata after save");
                        }
                    }
                    Err(e) => eprintln!("Failed to save image with anchors metadata: {}", e),
                }

                // 同步更新临时文件也带有元数据，便于会话内读取
                let _ = crate::utils::ImageProcessor::write_png_with_text_from_path(
                    temp_path.as_path(),
                    temp_path.as_path(),
                    "anchors",
                    &json,
                );
            } else {
                eprintln!("No color reflection anchors available to write into metadata");
            }
        } else {
            eprintln!("No image loaded or temp file not available for saving with anchors");
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
                        self.current_texture =
                            Some(ImageProcessor::update_texture_from_image(&cleaned_img, ctx));
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

```

已修复：当成功加载新图片时，会先删除上一张图片的临时文件（如果存在），避免残留。你现在可多次打开图片验证，旧的 `.egui_tmp.png` 会被及时清理。

- 改动位置：`MainWindow::load_image` 成功分支最前添加删除逻辑，避免在加载失败时误删当前临时文件。


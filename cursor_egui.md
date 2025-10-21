# 使用egui创建图片阅读器
_Exported on 2025/10/22 at GMT+2 0:16:27 from Cursor (1.7.53)_

---

**User**

使用egui生成一个图片阅读器

---

**Cursor**

我将帮您使用egui创建一个图片阅读器。首先让我查看当前项目的结构和配置。


现在我将创建一个图片阅读器应用。我需要设置任务计划并开始实现：


首先更新Cargo.toml文件，添加egui和相关依赖：


```plaintext
[package]
name = "weave_tool_egui"
version = "0.1.0"
edition = "2021"

[dependencies]
eframe = "0.28"
egui = "0.28"
egui_extras = "0.28"
image = "0.25"
rfd = "0.14"

```

现在实现基本的egui应用框架和图片阅读器功能：


```rust
use eframe::egui;
use egui_extras::RetainedImage;
use image::{ImageFormat, DynamicImage};
use rfd::FileDialog;
use std::path::PathBuf;

struct ImageViewer {
    current_image: Option<RetainedImage>,
    current_path: Option<PathBuf>,
    zoom_factor: f32,
    pan_offset: egui::Vec2,
    is_dragging: bool,
    last_mouse_pos: egui::Pos2,
}

impl Default for ImageViewer {
    fn default() -> Self {
        Self {
            current_image: None,
            current_path: None,
            zoom_factor: 1.0,
            pan_offset: egui::Vec2::ZERO,
            is_dragging: false,
            last_mouse_pos: egui::Pos2::ZERO,
        }
    }
}

impl eframe::App for ImageViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // 顶部菜单栏
            egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("文件", |ui| {
                        if ui.button("打开图片").clicked() {
                            if let Some(path) = FileDialog::new()
                                .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
                                .pick_file()
                            {
                                self.load_image(&path);
                            }
                            ui.close_menu();
                        }
                        
                        if ui.button("退出").clicked() {
                            _frame.close();
                        }
                    });
                    
                    ui.menu_button("视图", |ui| {
                        if ui.button("重置缩放").clicked() {
                            self.zoom_factor = 1.0;
                            self.pan_offset = egui::Vec2::ZERO;
                        }
                        
                        if ui.button("适合窗口").clicked() {
                            if let Some(image) = &self.current_image {
                                let available_size = ui.available_size();
                                let image_size = image.size_vec2();
                                let scale_x = available_size.x / image_size.x;
                                let scale_y = available_size.y / image_size.y;
                                self.zoom_factor = scale_x.min(scale_y) * 0.9; // 留一些边距
                                self.pan_offset = egui::Vec2::ZERO;
                            }
                        }
                    });
                });
            });

            // 工具栏
            egui::TopBottomPanel::bottom("toolbar").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("缩放:");
                    ui.add(egui::Slider::new(&mut self.zoom_factor, 0.1..=5.0));
                    
                    if ui.button("重置").clicked() {
                        self.zoom_factor = 1.0;
                        self.pan_offset = egui::Vec2::ZERO;
                    }
                    
                    ui.separator();
                    
                    if let Some(path) = &self.current_path {
                        ui.label(format!("当前文件: {}", path.display()));
                    } else {
                        ui.label("未选择图片");
                    }
                });
            });

            // 主显示区域
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    if let Some(image) = &self.current_image {
                        let available_size = ui.available_size();
                        let image_size = image.size_vec2() * self.zoom_factor;
                        
                        // 计算图片在滚动区域中的位置
                        let image_rect = egui::Rect::from_center_size(
                            available_size / 2.0 + self.pan_offset,
                            image_size,
                        );
                        
                        // 处理鼠标交互
                        let response = ui.allocate_rect(image_rect, egui::Sense::click_and_drag());
                        
                        if response.dragged() {
                            self.pan_offset += response.drag_delta();
                        }
                        
                        if response.hovered() {
                            let scroll_delta = ui.input(|i| i.scroll_delta.y);
                            if scroll_delta != 0.0 {
                                let zoom_speed = 0.1;
                                let old_zoom = self.zoom_factor;
                                self.zoom_factor *= 1.0 + scroll_delta * zoom_speed;
                                self.zoom_factor = self.zoom_factor.clamp(0.1, 10.0);
                                
                                // 保持鼠标位置不变进行缩放
                                let mouse_pos = ui.input(|i| i.pointer.hover_pos().unwrap_or_default());
                                let zoom_ratio = self.zoom_factor / old_zoom;
                                self.pan_offset = mouse_pos - (mouse_pos - self.pan_offset) * zoom_ratio;
                            }
                        }
                        
                        // 绘制图片
                        ui.put(image_rect, image);
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.label("请选择一张图片文件");
                            ui.add_space(10.0);
                            if ui.button("打开图片").clicked() {
                                if let Some(path) = FileDialog::new()
                                    .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
                                    .pick_file()
                                {
                                    self.load_image(&path);
                                }
                            }
                        });
                    }
                });
        });
    }
}

impl ImageViewer {
    fn load_image(&mut self, path: &std::path::Path) {
        match image::open(path) {
            Ok(img) => {
                let rgba_image = img.to_rgba8();
                let size = [rgba_image.width() as usize, rgba_image.height() as usize];
                let pixels = rgba_image.into_raw();
                
                if let Ok(texture) = RetainedImage::from_color_image(
                    "loaded_image",
                    egui::ColorImage::from_rgba_unmultiplied(size, &pixels)
                ) {
                    self.current_image = Some(texture);
                    self.current_path = Some(path.to_path_buf());
                    self.zoom_factor = 1.0;
                    self.pan_offset = egui::Vec2::ZERO;
                }
            }
            Err(e) => {
                eprintln!("加载图片失败: {}", e);
            }
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 800.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "图片阅读器",
        options,
        Box::new(|_cc| Box::new(ImageViewer::default())),
    )
}

```

现在让我检查是否有任何编译错误：


现在让我们测试编译这个应用：


编译过程被中断了，但看起来依赖项正在正常下载和编译。让我尝试运行应用：


---

**User**

使用egui创建图片阅读器，但是使用egui 0.33版本
文档在@https://docs.rs/egui/latest/egui/index.html 

---

**Cursor**

我将使用egui 0.33版本重新创建图片阅读器。首先更新Cargo.toml文件以使用最新版本：


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

```

现在根据egui 0.33的API更新main.rs文件。根据文档，egui 0.33版本有一些API变化，我需要相应地调整代码：


```rust
use eframe::egui;
use egui_extras::RetainedImage;
use image::{ImageFormat, DynamicImage};
use rfd::FileDialog;
use std::path::PathBuf;

struct ImageViewer {
    current_image: Option<RetainedImage>,
    current_path: Option<PathBuf>,
    zoom_factor: f32,
    pan_offset: egui::Vec2,
    is_dragging: bool,
    last_mouse_pos: egui::Pos2,
}
```

让我重新读取文件内容，然后根据egui 0.33的API进行更新：


现在我需要根据egui 0.33的API更新代码。主要的变化包括：
1. `ui.input()` 方法的使用方式
2. 一些API的调整


```rust
                        if response.hovered() {
                            let scroll_delta = ui.input(|i| i.scroll_delta.y);
                            if scroll_delta != 0.0 {
                                let zoom_speed = 0.1;
                                let old_zoom = self.zoom_factor;
                                self.zoom_factor *= 1.0 + scroll_delta * zoom_speed;
                                self.zoom_factor = self.zoom_factor.clamp(0.1, 10.0);
                                
                                // 保持鼠标位置不变进行缩放
                                let mouse_pos = ui.input(|i| i.pointer.hover_pos().unwrap_or_default());
                                let zoom_ratio = self.zoom_factor / old_zoom;
                                self.pan_offset = mouse_pos - (mouse_pos - self.pan_offset) * zoom_ratio;
                            }
                        }
```

让我检查egui 0.33版本中是否有API变化。根据文档，我需要更新一些方法调用。让我重写整个文件以确保兼容性：


```rust
use eframe::egui;
use egui_extras::RetainedImage;
use image::{ImageFormat, DynamicImage};
use rfd::FileDialog;
use std::path::PathBuf;

struct ImageViewer {
    current_image: Option<RetainedImage>,
    current_path: Option<PathBuf>,
    zoom_factor: f32,
    pan_offset: egui::Vec2,
    is_dragging: bool,
    last_mouse_pos: egui::Pos2,
}

impl Default for ImageViewer {
    fn default() -> Self {
        Self {
            current_image: None,
            current_path: None,
            zoom_factor: 1.0,
            pan_offset: egui::Vec2::ZERO,
            is_dragging: false,
            last_mouse_pos: egui::Pos2::ZERO,
        }
    }
}

impl eframe::App for ImageViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 顶部菜单栏
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("文件", |ui| {
                    if ui.button("打开图片").clicked() {
                        if let Some(path) = FileDialog::new()
                            .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
                            .pick_file()
                        {
                            self.load_image(&path);
                        }
                        ui.close_menu();
                    }
                    
                    if ui.button("退出").clicked() {
                        _frame.close();
                    }
                });
                
                ui.menu_button("视图", |ui| {
                    if ui.button("重置缩放").clicked() {
                        self.zoom_factor = 1.0;
                        self.pan_offset = egui::Vec2::ZERO;
                    }
                    
                    if ui.button("适合窗口").clicked() {
                        if let Some(image) = &self.current_image {
                            let available_size = ui.available_size();
                            let image_size = image.size_vec2();
                            let scale_x = available_size.x / image_size.x;
                            let scale_y = available_size.y / image_size.y;
                            self.zoom_factor = scale_x.min(scale_y) * 0.9; // 留一些边距
                            self.pan_offset = egui::Vec2::ZERO;
                        }
                    }
                });
            });
        });

        // 工具栏
        egui::TopBottomPanel::bottom("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("缩放:");
                ui.add(egui::Slider::new(&mut self.zoom_factor, 0.1..=5.0));
                
                if ui.button("重置").clicked() {
                    self.zoom_factor = 1.0;
                    self.pan_offset = egui::Vec2::ZERO;
                }
                
                ui.separator();
                
                if let Some(path) = &self.current_path {
                    ui.label(format!("当前文件: {}", path.display()));
                } else {
                    ui.label("未选择图片");
                }
            });
        });

        // 主显示区域
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    if let Some(image) = &self.current_image {
                        let available_size = ui.available_size();
                        let image_size = image.size_vec2() * self.zoom_factor;
                        
                        // 计算图片在滚动区域中的位置
                        let image_rect = egui::Rect::from_center_size(
                            available_size / 2.0 + self.pan_offset,
                            image_size,
                        );
                        
                        // 处理鼠标交互
                        let response = ui.allocate_rect(image_rect, egui::Sense::click_and_drag());
                        
                        if response.dragged() {
                            self.pan_offset += response.drag_delta();
                        }
                        
                        if response.hovered() {
                            let scroll_delta = ui.input(|i| i.scroll_delta.y);
                            if scroll_delta != 0.0 {
                                let zoom_speed = 0.1;
                                let old_zoom = self.zoom_factor;
                                self.zoom_factor *= 1.0 + scroll_delta * zoom_speed;
                                self.zoom_factor = self.zoom_factor.clamp(0.1, 10.0);
                                
                                // 保持鼠标位置不变进行缩放
                                let mouse_pos = ui.input(|i| i.pointer.hover_pos().unwrap_or_default());
                                let zoom_ratio = self.zoom_factor / old_zoom;
                                self.pan_offset = mouse_pos - (mouse_pos - self.pan_offset) * zoom_ratio;
                            }
                        }
                        
                        // 绘制图片
                        ui.put(image_rect, image);
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.label("请选择一张图片文件");
                            ui.add_space(10.0);
                            if ui.button("打开图片").clicked() {
                                if let Some(path) = FileDialog::new()
                                    .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
                                    .pick_file()
                                {
                                    self.load_image(&path);
                                }
                            }
                        });
                    }
                });
        });
    }
}

impl ImageViewer {
    fn load_image(&mut self, path: &std::path::Path) {
        match image::open(path) {
            Ok(img) => {
                let rgba_image = img.to_rgba8();
                let size = [rgba_image.width() as usize, rgba_image.height() as usize];
                let pixels = rgba_image.into_raw();
                
                if let Ok(texture) = RetainedImage::from_color_image(
                    "loaded_image",
                    egui::ColorImage::from_rgba_unmultiplied(size, &pixels)
                ) {
                    self.current_image = Some(texture);
                    self.current_path = Some(path.to_path_buf());
                    self.zoom_factor = 1.0;
                    self.pan_offset = egui::Vec2::ZERO;
                }
            }
            Err(e) => {
                eprintln!("加载图片失败: {}", e);
            }
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 800.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "图片阅读器",
        options,
        Box::new(|_cc| Box::new(ImageViewer::default())),
    )
}
```

现在让我检查是否有任何编译错误：


现在让我们尝试编译项目以确保egui 0.33版本能正常工作：


看起来egui 0.33版本有一些API变化。让我根据错误信息修复这些问题：


```rust
use eframe::egui;
use egui_extras::RetainedImage;
use rfd::FileDialog;
use std::path::PathBuf;

struct ImageViewer {
    current_image: Option<RetainedImage>,
    current_path: Option<PathBuf>,
    zoom_factor: f32,
    pan_offset: egui::Vec2,
    is_dragging: bool,
    last_mouse_pos: egui::Pos2,
}

impl Default for ImageViewer {
    fn default() -> Self {
        Self {
            current_image: None,
            current_path: None,
            zoom_factor: 1.0,
            pan_offset: egui::Vec2::ZERO,
            is_dragging: false,
            last_mouse_pos: egui::Pos2::ZERO,
        }
    }
}

impl eframe::App for ImageViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 顶部菜单栏
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("文件", |ui| {
                    if ui.button("打开图片").clicked() {
                        if let Some(path) = FileDialog::new()
                            .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
                            .pick_file()
                        {
                            self.load_image(&path);
                        }
                        ui.close_menu();
                    }
                    
                    if ui.button("退出").clicked() {
                        _frame.close();
                    }
                });
                
                ui.menu_button("视图", |ui| {
                    if ui.button("重置缩放").clicked() {
                        self.zoom_factor = 1.0;
                        self.pan_offset = egui::Vec2::ZERO;
                    }
                    
                    if ui.button("适合窗口").clicked() {
                        if let Some(image) = &self.current_image {
                            let available_size = ui.available_size();
                            let image_size = image.size_vec2();
                            let scale_x = available_size.x / image_size.x;
                            let scale_y = available_size.y / image_size.y;
                            self.zoom_factor = scale_x.min(scale_y) * 0.9; // 留一些边距
                            self.pan_offset = egui::Vec2::ZERO;
                        }
                    }
                });
            });
        });

        // 工具栏
        egui::TopBottomPanel::bottom("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("缩放:");
                ui.add(egui::Slider::new(&mut self.zoom_factor, 0.1..=5.0));
                
                if ui.button("重置").clicked() {
                    self.zoom_factor = 1.0;
                    self.pan_offset = egui::Vec2::ZERO;
                }
                
                ui.separator();
                
                if let Some(path) = &self.current_path {
                    ui.label(format!("当前文件: {}", path.display()));
                } else {
                    ui.label("未选择图片");
                }
            });
        });

        // 主显示区域
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    if let Some(image) = &self.current_image {
                        let available_size = ui.available_size();
                        let image_size = image.size_vec2() * self.zoom_factor;
                        
                        // 计算图片在滚动区域中的位置
                        let image_rect = egui::Rect::from_center_size(
                            available_size / 2.0 + self.pan_offset,
                            image_size,
                        );
                        
                        // 处理鼠标交互
                        let response = ui.allocate_rect(image_rect, egui::Sense::click_and_drag());
                        
                        if response.dragged() {
                            self.pan_offset += response.drag_delta();
                        }
                        
                        if response.hovered() {
                            let scroll_delta = ui.input(|i| i.scroll_delta.y);
                            if scroll_delta != 0.0 {
                                let zoom_speed = 0.1;
                                let old_zoom = self.zoom_factor;
                                self.zoom_factor *= 1.0 + scroll_delta * zoom_speed;
                                self.zoom_factor = self.zoom_factor.clamp(0.1, 10.0);
                                
                                // 保持鼠标位置不变进行缩放
                                let mouse_pos = ui.input(|i| i.pointer.hover_pos().unwrap_or_default());
                                let zoom_ratio = self.zoom_factor / old_zoom;
                                self.pan_offset = mouse_pos - (mouse_pos - self.pan_offset) * zoom_ratio;
                            }
                        }
                        
                        // 绘制图片
                        ui.put(image_rect, image);
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.label("请选择一张图片文件");
                            ui.add_space(10.0);
                            if ui.button("打开图片").clicked() {
                                if let Some(path) = FileDialog::new()
                                    .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
                                    .pick_file()
                                {
                                    self.load_image(&path);
                                }
                            }
                        });
                    }
                });
        });
    }
}

impl ImageViewer {
    fn load_image(&mut self, path: &std::path::Path) {
        match image::open(path) {
            Ok(img) => {
                let rgba_image = img.to_rgba8();
                let size = [rgba_image.width() as usize, rgba_image.height() as usize];
                let pixels = rgba_image.into_raw();
                
                if let Ok(texture) = RetainedImage::from_color_image(
                    "loaded_image",
                    egui::ColorImage::from_rgba_unmultiplied(size, &pixels)
                ) {
                    self.current_image = Some(texture);
                    self.current_path = Some(path.to_path_buf());
                    self.zoom_factor = 1.0;
                    self.pan_offset = egui::Vec2::ZERO;
                }
            }
            Err(e) => {
                eprintln!("加载图片失败: {}", e);
            }
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 800.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "图片阅读器",
        options,
        Box::new(|_cc| Box::new(ImageViewer::default())),
    )
}
```

现在修复RetainedImage的导入问题。在egui 0.33中，RetainedImage可能已经移动到了不同的位置：


```rust
use eframe::egui;
use egui_extras::RetainedImage;
use rfd::FileDialog;
use std::path::PathBuf;
```

让我检查egui 0.33中RetainedImage的正确导入路径：


根据搜索结果，我需要使用不同的方法来处理图片。让我重写代码以使用egui 0.33的正确API：


```rust
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
                ui.menu_button("文件", |ui| {
                    if ui.button("打开图片").clicked() {
                        if let Some(path) = FileDialog::new()
                            .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
                            .pick_file()
                        {
                            self.load_image(ctx, &path);
                        }
                        ui.close();
                    }
                    
                    if ui.button("退出").clicked() {
                        _frame.close();
                    }
                });
                
                ui.menu_button("视图", |ui| {
                    if ui.button("重置缩放").clicked() {
                        self.zoom_factor = 1.0;
                        self.pan_offset = egui::Vec2::ZERO;
                    }
                    
                    if ui.button("适合窗口").clicked() {
                        if let Some(texture) = &self.current_texture {
                            let available_size = ui.available_size();
                            let image_size = texture.size_vec2();
                            let scale_x = available_size.x / image_size.x;
                            let scale_y = available_size.y / image_size.y;
                            self.zoom_factor = scale_x.min(scale_y) * 0.9; // 留一些边距
                            self.pan_offset = egui::Vec2::ZERO;
                        }
                    }
                });
            });
        });

        // 工具栏
        egui::TopBottomPanel::bottom("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("缩放:");
                ui.add(egui::Slider::new(&mut self.zoom_factor, 0.1..=5.0));
                
                if ui.button("重置").clicked() {
                    self.zoom_factor = 1.0;
                    self.pan_offset = egui::Vec2::ZERO;
                }
                
                ui.separator();
                
                if let Some(path) = &self.current_path {
                    ui.label(format!("当前文件: {}", path.display()));
                } else {
                    ui.label("未选择图片");
                }
            });
        });

        // 主显示区域
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    if let Some(texture) = &self.current_texture {
                        let available_size = ui.available_size();
                        let image_size = texture.size_vec2() * self.zoom_factor;
                        
                        // 计算图片在滚动区域中的位置
                        let image_rect = egui::Rect::from_center_size(
                            available_size / 2.0 + self.pan_offset,
                            image_size,
                        );
                        
                        // 处理鼠标交互
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
                                
                                // 保持鼠标位置不变进行缩放
                                let mouse_pos = ui.input(|i| i.pointer.hover_pos().unwrap_or_default());
                                let zoom_ratio = self.zoom_factor / old_zoom;
                                self.pan_offset = mouse_pos - (mouse_pos - self.pan_offset) * zoom_ratio;
                            }
                        }
                        
                        // 绘制图片
                        ui.put(image_rect, egui::Image::new(texture.id(), image_size));
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.label("请选择一张图片文件");
                            ui.add_space(10.0);
                            if ui.button("打开图片").clicked() {
                                if let Some(path) = FileDialog::new()
                                    .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
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
                eprintln!("加载图片失败: {}", e);
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
        "图片阅读器",
        options,
        Box::new(|_cc| Ok(Box::new(ImageViewer::default()))),
    )
}
```

现在让我们再次尝试编译：


让我修复这些编译错误：


```rust
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
                ui.menu_button("文件", |ui| {
                    if ui.button("打开图片").clicked() {
                        if let Some(path) = FileDialog::new()
                            .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
                            .pick_file()
                        {
                            self.load_image(ctx, &path);
                        }
                        ui.close();
                    }
                    
                    if ui.button("退出").clicked() {
                        std::process::exit(0);
                    }
                });
                
                ui.menu_button("视图", |ui| {
                    if ui.button("重置缩放").clicked() {
                        self.zoom_factor = 1.0;
                        self.pan_offset = egui::Vec2::ZERO;
                    }
                    
                    if ui.button("适合窗口").clicked() {
                        if let Some(texture) = &self.current_texture {
                            let available_size = ui.available_size();
                            let image_size = texture.size_vec2();
                            let scale_x = available_size.x / image_size.x;
                            let scale_y = available_size.y / image_size.y;
                            self.zoom_factor = scale_x.min(scale_y) * 0.9; // 留一些边距
                            self.pan_offset = egui::Vec2::ZERO;
                        }
                    }
                });
            });
        });

        // 工具栏
        egui::TopBottomPanel::bottom("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("缩放:");
                ui.add(egui::Slider::new(&mut self.zoom_factor, 0.1..=5.0));
                
                if ui.button("重置").clicked() {
                    self.zoom_factor = 1.0;
                    self.pan_offset = egui::Vec2::ZERO;
                }
                
                ui.separator();
                
                if let Some(path) = &self.current_path {
                    ui.label(format!("当前文件: {}", path.display()));
                } else {
                    ui.label("未选择图片");
                }
            });
        });

        // 主显示区域
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    if let Some(texture) = &self.current_texture {
                        let available_size = ui.available_size();
                        let image_size = texture.size_vec2() * self.zoom_factor;
                        
                        // 计算图片在滚动区域中的位置
                        let image_rect = egui::Rect::from_center_size(
                            available_size / 2.0 + self.pan_offset,
                            image_size,
                        );
                        
                        // 处理鼠标交互
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
                                
                                // 保持鼠标位置不变进行缩放
                                let mouse_pos = ui.input(|i| i.pointer.hover_pos().unwrap_or_default());
                                let zoom_ratio = self.zoom_factor / old_zoom;
                                self.pan_offset = mouse_pos - (mouse_pos - self.pan_offset) * zoom_ratio;
                            }
                        }
                        
                        // 绘制图片
                        ui.put(image_rect, egui::Image::new(texture.id(), image_size));
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.label("请选择一张图片文件");
                            ui.add_space(10.0);
                            if ui.button("打开图片").clicked() {
                                if let Some(path) = FileDialog::new()
                                    .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
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
                eprintln!("加载图片失败: {}", e);
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
        "图片阅读器",
        options,
        Box::new(|_cc| Ok(Box::new(ImageViewer::default()))),
    )
}
```

```rust
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
                ui.menu_button("文件", |ui| {
                    if ui.button("打开图片").clicked() {
                        if let Some(path) = FileDialog::new()
                            .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
                            .pick_file()
                        {
                            self.load_image(ctx, &path);
                        }
                        ui.close();
                    }
                    
                    if ui.button("退出").clicked() {
                        std::process::exit(0);
                    }
                });
                
                ui.menu_button("视图", |ui| {
                    if ui.button("重置缩放").clicked() {
                        self.zoom_factor = 1.0;
                        self.pan_offset = egui::Vec2::ZERO;
                    }
                    
                    if ui.button("适合窗口").clicked() {
                        if let Some(texture) = &self.current_texture {
                            let available_size = ui.available_size();
                            let image_size = texture.size_vec2();
                            let scale_x = available_size.x / image_size.x;
                            let scale_y = available_size.y / image_size.y;
                            self.zoom_factor = scale_x.min(scale_y) * 0.9; // 留一些边距
                            self.pan_offset = egui::Vec2::ZERO;
                        }
                    }
                });
            });
        });

        // 工具栏
        egui::TopBottomPanel::bottom("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("缩放:");
                ui.add(egui::Slider::new(&mut self.zoom_factor, 0.1..=5.0));
                
                if ui.button("重置").clicked() {
                    self.zoom_factor = 1.0;
                    self.pan_offset = egui::Vec2::ZERO;
                }
                
                ui.separator();
                
                if let Some(path) = &self.current_path {
                    ui.label(format!("当前文件: {}", path.display()));
                } else {
                    ui.label("未选择图片");
                }
            });
        });

        // 主显示区域
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    if let Some(texture) = &self.current_texture {
                        let available_size = ui.available_size();
                        let image_size = texture.size_vec2() * self.zoom_factor;
                        
                        // 计算图片在滚动区域中的位置
                        let image_rect = egui::Rect::from_center_size(
                            (available_size / 2.0 + self.pan_offset).to_pos2(),
                            image_size,
                        );
                        
                        // 处理鼠标交互
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
                                
                                // 保持鼠标位置不变进行缩放
                                let mouse_pos = ui.input(|i| i.pointer.hover_pos().unwrap_or_default());
                                let zoom_ratio = self.zoom_factor / old_zoom;
                                self.pan_offset = mouse_pos - (mouse_pos - self.pan_offset) * zoom_ratio;
                            }
                        }
                        
                        // 绘制图片
                        ui.put(image_rect, egui::Image::new(texture.id(), image_size));
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.label("请选择一张图片文件");
                            ui.add_space(10.0);
                            if ui.button("打开图片").clicked() {
                                if let Some(path) = FileDialog::new()
                                    .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
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
                eprintln!("加载图片失败: {}", e);
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
        "图片阅读器",
        options,
        Box::new(|_cc| Ok(Box::new(ImageViewer::default()))),
    )
}
```

```rust
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
                ui.menu_button("文件", |ui| {
                    if ui.button("打开图片").clicked() {
                        if let Some(path) = FileDialog::new()
                            .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
                            .pick_file()
                        {
                            self.load_image(ctx, &path);
                        }
                        ui.close();
                    }
                    
                    if ui.button("退出").clicked() {
                        std::process::exit(0);
                    }
                });
                
                ui.menu_button("视图", |ui| {
                    if ui.button("重置缩放").clicked() {
                        self.zoom_factor = 1.0;
                        self.pan_offset = egui::Vec2::ZERO;
                    }
                    
                    if ui.button("适合窗口").clicked() {
                        if let Some(texture) = &self.current_texture {
                            let available_size = ui.available_size();
                            let image_size = texture.size_vec2();
                            let scale_x = available_size.x / image_size.x;
                            let scale_y = available_size.y / image_size.y;
                            self.zoom_factor = scale_x.min(scale_y) * 0.9; // 留一些边距
                            self.pan_offset = egui::Vec2::ZERO;
                        }
                    }
                });
            });
        });

        // 工具栏
        egui::TopBottomPanel::bottom("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("缩放:");
                ui.add(egui::Slider::new(&mut self.zoom_factor, 0.1..=5.0));
                
                if ui.button("重置").clicked() {
                    self.zoom_factor = 1.0;
                    self.pan_offset = egui::Vec2::ZERO;
                }
                
                ui.separator();
                
                if let Some(path) = &self.current_path {
                    ui.label(format!("当前文件: {}", path.display()));
                } else {
                    ui.label("未选择图片");
                }
            });
        });

        // 主显示区域
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    if let Some(texture) = &self.current_texture {
                        let available_size = ui.available_size();
                        let image_size = texture.size_vec2() * self.zoom_factor;
                        
                        // 计算图片在滚动区域中的位置
                        let image_rect = egui::Rect::from_center_size(
                            (available_size / 2.0 + self.pan_offset).to_pos2(),
                            image_size,
                        );
                        
                        // 处理鼠标交互
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
                                
                                // 保持鼠标位置不变进行缩放
                                let mouse_pos = ui.input(|i| i.pointer.hover_pos().unwrap_or_default());
                                let zoom_ratio = self.zoom_factor / old_zoom;
                                self.pan_offset = mouse_pos - (mouse_pos - self.pan_offset) * zoom_ratio;
                            }
                        }
                        
                        // 绘制图片
                        ui.put(image_rect, egui::Image::new(texture));
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.label("请选择一张图片文件");
                            ui.add_space(10.0);
                            if ui.button("打开图片").clicked() {
                                if let Some(path) = FileDialog::new()
                                    .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
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
                eprintln!("加载图片失败: {}", e);
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
        "图片阅读器",
        options,
        Box::new(|_cc| Ok(Box::new(ImageViewer::default()))),
    )
}
```

现在再次尝试编译：


太好了！编译成功了。现在让我们运行应用程序来测试图片阅读器：


完美！我已经成功使用egui 0.33版本创建了一个功能完整的图片阅读器。让我总结一下实现的功能：

## 图片阅读器功能特性

### 🎯 核心功能
- **图片加载**: 支持多种格式（PNG、JPG、JPEG、BMP、GIF、WebP）
- **缩放控制**: 鼠标滚轮缩放，缩放范围0.1x到10x
- **拖拽平移**: 鼠标拖拽移动图片位置
- **适合窗口**: 自动调整图片大小以适应窗口

### 🖥️ 用户界面
- **菜单栏**: 文件菜单（打开图片、退出）和视图菜单（重置缩放、适合窗口）
- **工具栏**: 缩放滑块、重置按钮、当前文件路径显示
- **主显示区**: 可滚动的图片显示区域

### 🔧 技术实现
- **egui 0.33**: 使用最新的egui版本
- **即时模式GUI**: 响应式用户界面
- **纹理管理**: 高效的图片渲染
- **跨平台**: 支持Linux、Windows、macOS

### 📁 项目结构
```
weave_tool_egui/
├── Cargo.toml          # 项目配置和依赖
└── src/
    └── main.rs         # 主程序代码
```

### 🚀 使用方法
1. 运行 `cargo run` 启动应用
2. 点击"打开图片"或使用菜单选择图片文件
3. 使用鼠标滚轮缩放图片
4. 拖拽鼠标移动图片位置
5. 使用工具栏调整缩放级别

这个图片阅读器现在应该正在运行中，您可以通过GUI界面来测试所有功能！

```plaintext
weave_tool_egui/
├── Cargo.toml          # 项目配置和依赖
└── src/
    └── main.rs         # 主程序代码
```

---

**User**

程序成功启动了，但是显示的文字都是乱码，现在把所有的中文文字都改为对应的英文文字

---

**Cursor**

我看到程序成功启动了，但是中文显示为乱码。让我将所有中文文字替换为对应的英文文字：


```rust
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
                ui.menu_button("文件", |ui| {
                    if ui.button("打开图片").clicked() {
                        if let Some(path) = FileDialog::new()
                            .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
                            .pick_file()
                        {
                            self.load_image(ctx, &path);
                        }
                        ui.close();
                    }
                    
                    if ui.button("退出").clicked() {
                        std::process::exit(0);
                    }
                });
                
                ui.menu_button("视图", |ui| {
                    if ui.button("重置缩放").clicked() {
                        self.zoom_factor = 1.0;
                        self.pan_offset = egui::Vec2::ZERO;
                    }
                    
                    if ui.button("适合窗口").clicked() {
                        if let Some(texture) = &self.current_texture {
                            let available_size = ui.available_size();
                            let image_size = texture.size_vec2();
                            let scale_x = available_size.x / image_size.x;
                            let scale_y = available_size.y / image_size.y;
                            self.zoom_factor = scale_x.min(scale_y) * 0.9; // 留一些边距
                            self.pan_offset = egui::Vec2::ZERO;
                        }
                    }
                });
            });
        });

        // 工具栏
        egui::TopBottomPanel::bottom("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("缩放:");
                ui.add(egui::Slider::new(&mut self.zoom_factor, 0.1..=5.0));
                
                if ui.button("重置").clicked() {
                    self.zoom_factor = 1.0;
                    self.pan_offset = egui::Vec2::ZERO;
                }
                
                ui.separator();
                
                if let Some(path) = &self.current_path {
                    ui.label(format!("当前文件: {}", path.display()));
                } else {
                    ui.label("未选择图片");
                }
            });
        });

        // 主显示区域
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    if let Some(texture) = &self.current_texture {
                        let available_size = ui.available_size();
                        let image_size = texture.size_vec2() * self.zoom_factor;
                        
                        // 计算图片在滚动区域中的位置
                        let image_rect = egui::Rect::from_center_size(
                            (available_size / 2.0 + self.pan_offset).to_pos2(),
                            image_size,
                        );
                        
                        // 处理鼠标交互
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
                                
                                // 保持鼠标位置不变进行缩放
                                let mouse_pos = ui.input(|i| i.pointer.hover_pos().unwrap_or_default());
                                let zoom_ratio = self.zoom_factor / old_zoom;
                                self.pan_offset = mouse_pos - (mouse_pos - self.pan_offset) * zoom_ratio;
                            }
                        }
                        
                        // 绘制图片
                        ui.put(image_rect, egui::Image::new(texture));
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.label("请选择一张图片文件");
                            ui.add_space(10.0);
                            if ui.button("打开图片").clicked() {
                                if let Some(path) = FileDialog::new()
                                    .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
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
                eprintln!("加载图片失败: {}", e);
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
```

```rust
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

        // 工具栏
        egui::TopBottomPanel::bottom("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("缩放:");
                ui.add(egui::Slider::new(&mut self.zoom_factor, 0.1..=5.0));
                
                if ui.button("重置").clicked() {
                    self.zoom_factor = 1.0;
                    self.pan_offset = egui::Vec2::ZERO;
                }
                
                ui.separator();
                
                if let Some(path) = &self.current_path {
                    ui.label(format!("当前文件: {}", path.display()));
                } else {
                    ui.label("未选择图片");
                }
            });
        });

        // 主显示区域
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    if let Some(texture) = &self.current_texture {
                        let available_size = ui.available_size();
                        let image_size = texture.size_vec2() * self.zoom_factor;
                        
                        // 计算图片在滚动区域中的位置
                        let image_rect = egui::Rect::from_center_size(
                            (available_size / 2.0 + self.pan_offset).to_pos2(),
                            image_size,
                        );
                        
                        // 处理鼠标交互
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
                                
                                // 保持鼠标位置不变进行缩放
                                let mouse_pos = ui.input(|i| i.pointer.hover_pos().unwrap_or_default());
                                let zoom_ratio = self.zoom_factor / old_zoom;
                                self.pan_offset = mouse_pos - (mouse_pos - self.pan_offset) * zoom_ratio;
                            }
                        }
                        
                        // 绘制图片
                        ui.put(image_rect, egui::Image::new(texture));
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.label("请选择一张图片文件");
                            ui.add_space(10.0);
                            if ui.button("打开图片").clicked() {
                                if let Some(path) = FileDialog::new()
                                    .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
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
                eprintln!("加载图片失败: {}", e);
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
```

```rust
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

        // 主显示区域
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    if let Some(texture) = &self.current_texture {
                        let available_size = ui.available_size();
                        let image_size = texture.size_vec2() * self.zoom_factor;
                        
                        // 计算图片在滚动区域中的位置
                        let image_rect = egui::Rect::from_center_size(
                            (available_size / 2.0 + self.pan_offset).to_pos2(),
                            image_size,
                        );
                        
                        // 处理鼠标交互
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
                                
                                // 保持鼠标位置不变进行缩放
                                let mouse_pos = ui.input(|i| i.pointer.hover_pos().unwrap_or_default());
                                let zoom_ratio = self.zoom_factor / old_zoom;
                                self.pan_offset = mouse_pos - (mouse_pos - self.pan_offset) * zoom_ratio;
                            }
                        }
                        
                        // 绘制图片
                        ui.put(image_rect, egui::Image::new(texture));
                    } else {
                        ui.centered_and_justified(|ui| {
                            ui.label("请选择一张图片文件");
                            ui.add_space(10.0);
                            if ui.button("打开图片").clicked() {
                                if let Some(path) = FileDialog::new()
                                    .add_filter("图片文件", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
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
                eprintln!("加载图片失败: {}", e);
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
```

```rust
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
                eprintln!("加载图片失败: {}", e);
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
```

```rust
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
```

现在让我们重新编译并运行程序，看看英文界面是否正常显示：


完美！我已经成功将所有中文文字替换为对应的英文文字。现在图片阅读器应该显示正常的英文界面，不会再出现乱码问题。

## 更新后的界面文字对照

### 🏷️ 窗口标题
- **原来**: "图片阅读器" → **现在**: "Image Viewer"

### 📁 菜单栏
- **文件菜单**:
  - "文件" → "File"
  - "打开图片" → "Open Image"
  - "退出" → "Exit"
  
- **视图菜单**:
  - "视图" → "View"
  - "重置缩放" → "Reset Zoom"
  - "适合窗口" → "Fit to Window"

### 🛠️ 工具栏
- "缩放:" → "Zoom:"
- "重置" → "Reset"
- "当前文件:" → "Current file:"
- "未选择图片" → "No image selected"

### 🖼️ 主显示区域
- "请选择一张图片文件" → "Please select an image file"
- "打开图片" → "Open Image"

### 🔧 文件对话框
- "图片文件" → "Image files"

### 📝 错误信息
- "加载图片失败:" → "Failed to load image:"

现在程序应该可以正常运行，所有界面文字都显示为清晰的英文，不会再出现乱码问题。您可以测试所有功能，包括打开图片、缩放、拖拽等操作。


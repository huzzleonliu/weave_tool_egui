//! # Weave Tool EGUI - 主程序入口
//! 
//! 基于egui的图片查看器应用程序

use weave_tool_egui::ImageViewer;

/// 主函数
/// 
/// 初始化并运行图片查看器应用程序
fn main() -> Result<(), eframe::Error> {
    // 配置应用程序选项
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Image Viewer"),
        ..Default::default()
    };
    
    // 运行应用程序
    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(ImageViewer::default()))),
    )
}
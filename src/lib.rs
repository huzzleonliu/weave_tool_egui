//! # Weave Tool EGUI
//! 
//! 一个基于egui的图片查看器应用程序，提供图片查看、缩放、平移、黑白转换等功能。

pub mod app;
pub mod image_processor;
pub mod ui;

// 重新导出主要类型，方便外部使用
pub use app::ImageViewer;
pub use image_processor::ImageProcessor;

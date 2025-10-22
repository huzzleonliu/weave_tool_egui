//! # 图片处理模块
//! 
//! 提供图片加载、转换、保存等功能

use image::DynamicImage;
use std::path::Path;
use std::fs;

/// 图片处理器
/// 
/// 负责图片的加载、转换、保存等操作
pub struct ImageProcessor {
    /// 原始图片数据
    original_image: Option<DynamicImage>,
    /// 当前图片路径
    current_path: Option<std::path::PathBuf>,
    /// 临时文件路径
    temp_path: Option<std::path::PathBuf>,
    /// 是否为黑白模式
    is_grayscale: bool,
}

impl ImageProcessor {
    /// 创建新的图片处理器
    pub fn new() -> Self {
        Self {
            original_image: None,
            current_path: None,
            temp_path: None,
            is_grayscale: false,
        }
    }

    /// 加载图片文件
    /// 
    /// # 参数
    /// * `path` - 图片文件路径
    /// 
    /// # 返回值
    /// 成功时返回处理后的图片，失败时返回错误信息
    pub fn load_image(&mut self, path: &Path) -> Result<DynamicImage, String> {
        match image::open(path) {
            Ok(img) => {
                // 清理之前的临时文件
                if let Some(old_temp) = self.temp_path.take() {
                    let _ = fs::remove_file(old_temp);
                }

                // 保存原始图片
                self.original_image = Some(img.clone());
                self.is_grayscale = false;
                self.current_path = Some(path.to_path_buf());

                // 创建临时文件
                self.create_temp_file(&img)?;

                Ok(img)
            }
            Err(e) => Err(format!("加载图片失败: {}", e)),
        }
    }

    /// 切换黑白模式
    /// 
    /// # 返回值
    /// 返回处理后的图片
    pub fn toggle_grayscale(&mut self) -> Option<DynamicImage> {
        if let Some(original_img) = &self.original_image {
            let processed_img = if self.is_grayscale {
                // 恢复彩色模式
                original_img.clone()
            } else {
                // 转换为黑白模式
                self.convert_to_grayscale_custom(original_img)
            };
            
            self.is_grayscale = !self.is_grayscale;
            self.save_to_temp(&processed_img);
            Some(processed_img)
        } else {
            None
        }
    }

    /// 恢复原始图片
    /// 
    /// # 返回值
    /// 返回原始图片
    pub fn restore_original(&mut self) -> Option<DynamicImage> {
        if let Some(original_img) = self.original_image.clone() {
            self.is_grayscale = false;
            self.save_to_temp(&original_img);
            Some(original_img)
        } else {
            None
        }
    }

    /// 快速保存当前图片到原文件
    /// 
    /// # 返回值
    /// 成功返回Ok，失败返回错误信息
    pub fn fast_save(&self) -> Result<(), String> {
        if let (Some(current_path), Some(temp_path)) = (&self.current_path, &self.temp_path) {
            match fs::copy(temp_path, current_path) {
                Ok(_) => {
                    println!("图片保存成功: {}", current_path.display());
                    Ok(())
                }
                Err(e) => Err(format!("保存图片失败: {}", e)),
            }
        } else {
            Err("没有加载图片或临时文件不可用".to_string())
        }
    }

    /// 获取当前图片路径
    pub fn current_path(&self) -> Option<&std::path::PathBuf> {
        self.current_path.as_ref()
    }

    /// 获取当前是否为黑白模式
    pub fn is_grayscale(&self) -> bool {
        self.is_grayscale
    }

    /// 创建临时文件
    fn create_temp_file(&mut self, img: &DynamicImage) -> Result<(), String> {
        if let Some(original_path) = &self.current_path {
            if let Some(stem) = original_path.file_stem().and_then(|s| s.to_str()) {
                let mut temp_path = original_path.clone();
                temp_path.set_file_name(format!("{}.egui_tmp.png", stem));
                
                // 保存当前状态到临时文件
                img.save(&temp_path)
                    .map_err(|e| format!("创建临时文件失败: {}", e))?;
                
                self.temp_path = Some(temp_path);
            }
        }
        Ok(())
    }

    /// 保存图片到临时文件
    fn save_to_temp(&self, img: &DynamicImage) {
        if let Some(temp) = &self.temp_path {
            let _ = img.save(temp);
        }
    }

    /// 自定义黑白转换算法
    /// 
    /// 使用ITU-R BT.601标准进行亮度计算
    fn convert_to_grayscale_custom(&self, img: &DynamicImage) -> DynamicImage {
        let rgba_image = img.to_rgba8();
        let (width, height) = rgba_image.dimensions();
        let mut pixels = rgba_image.into_raw();
        
        // 处理每个像素的RGBA数据（每像素4字节）
        for chunk in pixels.chunks_exact_mut(4) {
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            let a = chunk[3];
            
            // ITU-R BT.601 近似加权，使用整数计算避免浮点运算
            let luma = ((299 * r + 587 * g + 114 * b) / 1000) as u8;
            
            // Alpha通道处理逻辑：
            // - alpha为0的像素设为(0,0,0,0)（完全透明）
            // - alpha不为0的像素alpha设为255（完全不透明）
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
        
        // 从处理后的像素数据创建新图片
        DynamicImage::ImageRgba8(image::RgbaImage::from_raw(width, height, pixels).unwrap())
    }
}

impl Drop for ImageProcessor {
    /// 析构函数，清理临时文件
    fn drop(&mut self) {
        if let Some(temp) = self.temp_path.take() {
            let _ = fs::remove_file(temp);
        }
    }
}

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

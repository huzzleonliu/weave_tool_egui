# Weave Tool EGUI

一个基于egui的现代化图片查看器应用程序，提供直观的用户界面和强大的图片处理功能。

## ✨ 功能特性

### 🖼️ 图片查看
- **多格式支持**: PNG、JPG、JPEG、BMP、GIF、WebP
- **高质量渲染**: 像素完美显示，支持最近邻过滤
- **棋盘格背景**: 透明图片的完美显示效果

### 🔍 缩放和平移
- **鼠标滚轮缩放**: 0.1x 到 5.0x 缩放范围
- **智能缩放**: 保持鼠标位置不变进行缩放
- **拖拽平移**: 鼠标拖拽移动图片位置
- **适合窗口**: 一键调整图片大小适应窗口

### 🎨 图片处理
- **黑白转换**: 使用ITU-R BT.601标准的专业黑白转换
- **原始恢复**: 一键恢复原始彩色图片
- **快速保存**: 保存处理后的图片到原文件

### 🖥️ 用户界面
- **现代化设计**: 基于egui的即时模式GUI
- **直观操作**: 菜单栏、工具栏、快捷键支持
- **跨平台**: 支持Linux、Windows、macOS

## 🚀 快速开始

### 系统要求
- Rust 1.70+
- 支持OpenGL的显卡

### 安装和运行

1. **克隆项目**
   ```bash
   git clone <repository-url>
   cd weave_tool_egui
   ```

2. **编译项目**
   ```bash
   cargo build --release
   ```

3. **运行应用**
   ```bash
   cargo run
   ```

### 开发模式
```bash
# 检查代码
cargo check

# 运行开发版本
cargo run

# 运行发布版本
cargo run --release
```

## 📖 使用指南

### 基本操作

1. **打开图片**
   - 点击菜单栏 "File" → "Open Image"
   - 或点击主界面的 "Open Image" 按钮
   - 支持的文件格式：PNG、JPG、JPEG、BMP、GIF、WebP

2. **缩放图片**
   - 使用鼠标滚轮进行缩放
   - 使用工具栏的缩放滑块
   - 点击 "Reset" 按钮重置缩放

3. **移动图片**
   - 按住鼠标左键拖拽移动图片位置
   - 使用 "Fit to Window" 自动调整大小

4. **图片处理**
   - 点击 "Black & White" 转换为黑白
   - 点击 "Original" 恢复原始颜色
   - 点击 "Fast Save" 保存修改

### 快捷键

- **Ctrl+O**: 打开图片
- **Ctrl+Q**: 退出程序
- **鼠标滚轮**: 缩放
- **鼠标拖拽**: 平移

## 🏗️ 项目结构

```
weave_tool_egui/
├── Cargo.toml          # 项目配置和依赖
├── README.md           # 项目说明文档
├── LICENSE             # 许可证文件
└── src/
    ├── lib.rs          # 库入口，模块声明
    ├── main.rs         # 程序入口点
    ├── app.rs          # 主应用程序结构
    ├── image_processor.rs  # 图片处理模块
    └── ui.rs           # 用户界面组件
```

### 模块说明

- **`lib.rs`**: 库入口，声明所有模块和公共API
- **`main.rs`**: 程序入口，初始化并运行应用程序
- **`app.rs`**: 主应用程序结构，管理状态和用户交互
- **`image_processor.rs`**: 图片处理功能，包括加载、转换、保存
- **`ui.rs`**: UI组件集合，提供可重用的界面元素

## 🔧 技术实现

### 核心技术栈
- **egui 0.33**: 即时模式GUI框架
- **eframe**: 跨平台应用程序框架
- **image**: Rust图片处理库
- **rfd**: 跨平台文件对话框

### 架构设计
- **模块化设计**: 清晰的模块分离，便于维护和扩展
- **错误处理**: 完善的错误处理和用户反馈
- **内存管理**: 自动清理临时文件，防止内存泄漏
- **性能优化**: 高效的图片渲染和用户交互

### 图片处理算法
- **ITU-R BT.601标准**: 专业的黑白转换算法
- **Alpha通道处理**: 正确处理透明图片
- **像素完美显示**: 使用最近邻过滤保持图片清晰度

## 🛠️ 开发指南

### 添加新功能

1. **图片滤镜**
   - 在 `image_processor.rs` 中添加新的处理方法
   - 在 `app.rs` 中添加对应的UI控制

2. **新的UI组件**
   - 在 `ui.rs` 中添加可重用的组件
   - 遵循现有的设计模式

3. **文件格式支持**
   - 更新 `image_processor.rs` 中的加载逻辑
   - 更新文件对话框的过滤器

### 代码规范
- 使用详细的中文注释
- 遵循Rust命名约定
- 保持模块间的清晰边界
- 编写完整的文档注释

## 📝 许可证

本项目采用 MIT 许可证。详情请参阅 [LICENSE](LICENSE) 文件。

## 🤝 贡献

欢迎提交Issue和Pull Request！

1. Fork 本项目
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📞 联系方式

如有问题或建议，请通过以下方式联系：

- 提交 Issue: [GitHub Issues](https://github.com/yourusername/weave_tool_egui/issues)
- 邮箱: your.email@example.com

---

**Weave Tool EGUI** - 让图片查看变得简单而强大！ 🎨✨

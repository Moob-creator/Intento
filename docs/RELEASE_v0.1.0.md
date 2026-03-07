# Intento v0.1.0 - Release Build (2026-02-21)

## 📦 安装包信息

### DMG 安装镜像（推荐）
- **文件**: `Intento_0.1.0_universal.dmg`
- **大小**: 17 MB
- **架构**: Universal Binary (Intel + Apple Silicon)
- **位置**: `src-tauri/target/universal-apple-darwin/release/bundle/dmg/`

### .app 应用包
- **文件**: `Intento.app`
- **大小**: 46 MB
- **位置**: `src-tauri/target/universal-apple-darwin/release/bundle/macos/`

## 🚀 安装方法

### 方法1：使用 DMG（推荐）
1. 双击 `Intento_0.1.0_universal.dmg` 打开安装镜像
2. 将 Intento 图标拖拽到 Applications 文件夹
3. 在 Launchpad 或 Applications 文件夹中找到并启动 Intento

### 方法2：直接使用 .app
1. 复制 `Intento.app` 到 Applications 文件夹
2. 双击运行

### 首次启动
- macOS 可能提示"无法打开，因为它来自身份不明的开发者"
- 解决方法：右键点击应用 → 选择"打开"

## ✨ 本版本特性

### 核心功能
- ✅ 智能任务管理（AI 文本解析）
- ✅ 图片识别创建任务
- ✅ 日历视图
- ✅ 任务过滤和搜索
- ✅ 桌面通知
- ✅ 自动总结（日/周/月）

### 本次构建更新
1. **Bundle ID 分离** ✨
   - Debug 版本: `com.intento.app.debug`
   - Release 版本: `com.intento.app`
   - 开发版和正式版完全独立，可同时运行

2. **日期解析优化** 🔧
   - 修复"X号"日期解析逻辑
   - 智能处理月份天数
   - 提供详细的日期上下文给 AI

3. **UI 修复** 🎨
   - 日历视图边框显示正确
   - 文本选择控制优化
   - 过期任务标签准确显示

## 📁 数据存储位置

```
~/Library/Application Support/com.intento.app/intento.db
```

## 🔧 系统要求

- macOS 10.13+ (支持 Intel & Apple Silicon)
- 网络连接（AI 功能需要）

---

**构建时间**: 2026-02-21 22:02  
**版本**: v0.1.0  
**架构**: Universal Binary

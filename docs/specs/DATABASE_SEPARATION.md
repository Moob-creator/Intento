# Database Separation - Debug vs Release

## Overview

从 2026-02-21 开始，应用的开发版本(debug)和正式版本(release)使用**完全独立的应用标识符(Bundle ID)**和**独立的数据目录**，实现彻底的数据隔离。

## Bundle ID 区分

| 模式 | Bundle ID | 说明 |
|------|-----------|------|
| **Debug** | `com.intento.app.debug` | 开发/测试版本 |
| **Release** | `com.intento.app` | 正式发布版本 |

不同的 Bundle ID 意味着：
- ✅ macOS 将它们视为**完全独立的应用**
- ✅ 可以**同时安装和运行**两个版本
- ✅ **各自独立的应用数据目录**
- ✅ **各自独立的系统权限和设置**

## 数据库文件位置

### macOS
```
~/Library/Application Support/
├── com.intento.app.debug/      # Debug 版本目录
│   └── intento.db              # Debug 数据库
└── com.intento.app/            # Release 版本目录
    └── intento.db              # Release 数据库
```

### Windows
```
%APPDATA%\
├── com.intento.app.debug\      # Debug 版本目录
│   └── intento.db
└── com.intento.app\            # Release 版本目录
    └── intento.db
```

### Linux
```
~/.config/
├── com.intento.app.debug/      # Debug 版本目录
│   └── intento.db
└── com.intento.app/            # Release 版本目录
    └── intento.db
```

## 使用方式

### 开发模式 (Debug)
```bash
npm run tauri:dev
```
- Bundle ID: `com.intento.app.debug`
- 数据目录: `~/Library/Application Support/com.intento.app.debug/`
- 数据库: `intento.db`
- 可以随意测试、创建测试数据
- 不会影响正式版本的用户数据

### 正式版本 (Release)
```bash
npm run tauri:build
# 然后运行打包后的应用
```
- Bundle ID: `com.intento.app`
- 数据目录: `~/Library/Application Support/com.intento.app/`
- 数据库: `intento.db`
- 存储真实的用户数据
- 与开发版本完全隔离

## 验证方法

运行测试脚本：
```bash
./test_db_separation.sh
```

或手动检查：
```bash
# macOS - 查看两个应用的目录
ls -lh ~/Library/Application\ Support/com.intento.app.debug/
ls -lh ~/Library/Application\ Support/com.intento.app/

# 查看数据库内容
sqlite3 ~/Library/Application\ Support/com.intento.app.debug/intento.db .tables
sqlite3 ~/Library/Application\ Support/com.intento.app/intento.db .tables
```

## 技术实现

修改位置：`src-tauri/src/main.rs:64-92`

```rust
// Debug and Release use different app identifiers for complete isolation
#[cfg(debug_assertions)]
let app_identifier = "com.intento.app.debug";

#[cfg(not(debug_assertions))]
let app_identifier = "com.intento.app";

// Get base data directory and append our custom identifier
let base_data_dir = app.path().app_data_dir()
    .expect("Failed to get app data directory");

// Replace the default identifier path with our custom one
let app_data_dir = base_data_dir
    .parent()
    .expect("Failed to get parent directory")
    .join(app_identifier);
```

## 迁移说明

如果你之前在开发模式下创建了数据，想要迁移到正式版本：

```bash
# macOS
cd ~/Library/Application\ Support/
cp -r com.intento.app.debug com.intento.app
```

或者只迁移数据库：
```bash
cp ~/Library/Application\ Support/com.intento.app.debug/intento.db \
   ~/Library/Application\ Support/com.intento.app/intento.db
```

## 优势对比

### 之前的方案（同目录不同文件名）
```
~/Library/Application Support/com.intento.app/
├── intento-dev.db    # Debug
└── intento.db        # Release
```
❌ 共享应用目录
❌ 共享系统权限
⚠️ 文件名容易混淆

### 当前方案（不同 Bundle ID）
```
~/Library/Application Support/
├── com.intento.app.debug/intento.db    # Debug
└── com.intento.app/intento.db          # Release
```
✅ **完全独立的应用**
✅ **独立的系统权限**
✅ **更清晰的隔离**
✅ **可以同时运行**

## 注意事项

1. ✅ 两个版本是完全独立的应用
2. ✅ 可以同时安装和运行
3. ✅ 各自拥有独立的数据和设置
4. ⚠️ 测试时注意使用正确的版本
5. ⚠️ 迁移数据前建议先备份
6. 💡 Debug 版本的应用名称在 Dock 和 Finder 中仍显示为 "Intento"

## 相关文件

- 实现代码: `src-tauri/src/main.rs:64-92`
- 测试脚本: `test_db_separation.sh`
- 项目文档: `CLAUDE.md`

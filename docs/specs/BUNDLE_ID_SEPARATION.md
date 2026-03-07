# Bundle ID Separation - Implementation Summary

## 改进对比

### 之前的方案 ❌
```
~/Library/Application Support/com.intento.app/
├── intento-dev.db    # Debug 数据库
└── intento.db        # Release 数据库
```

**问题：**
- ❌ 共享同一个应用目录
- ❌ 共享同一个 Bundle ID
- ❌ 共享系统权限和设置
- ⚠️ 文件名容易混淆
- ⚠️ 不能真正隔离应用状态

### 当前方案 ✅
```
~/Library/Application Support/
├── com.intento.app.debug/     # Debug 应用完整目录
│   └── intento.db
└── com.intento.app/           # Release 应用完整目录
    └── intento.db
```

**优势：**
- ✅ 完全独立的 Bundle ID
- ✅ 完全独立的应用目录
- ✅ 完全独立的系统权限
- ✅ 可以同时安装和运行
- ✅ 清晰的版本隔离
- ✅ 符合 macOS 应用开发最佳实践

## 实现细节

### 1. Bundle ID 配置

**Debug 模式：**
```rust
#[cfg(debug_assertions)]
let app_identifier = "com.intento.app.debug";
```

**Release 模式：**
```rust
#[cfg(not(debug_assertions))]
let app_identifier = "com.intento.app";
```

### 2. 目录路径构建

```rust
// 获取基础数据目录
let base_data_dir = app.path().app_data_dir()
    .expect("Failed to get app data directory");

// 替换为自定义的 Bundle ID 目录
let app_data_dir = base_data_dir
    .parent()
    .expect("Failed to get parent directory")
    .join(app_identifier);

// 创建目录并设置数据库路径
std::fs::create_dir_all(&app_data_dir)
    .expect("Failed to create app data directory");
let db_path = app_data_dir.join("intento.db");
```

### 3. 日志输出

启动时会打印：
```
📦 App identifier: com.intento.app.debug (或 com.intento.app)
📦 App data directory: /Users/.../Library/Application Support/com.intento.app.debug
📦 Database path: /Users/.../Library/Application Support/com.intento.app.debug/intento.db
```

## 使用场景

### 开发调试
```bash
npm run tauri:dev
```
- 使用 `com.intento.app.debug`
- 独立的开发数据
- 可以尽情测试不影响正式版

### 正式发布
```bash
npm run tauri:build
./src-tauri/target/release/bundle/macos/Intento.app
```
- 使用 `com.intento.app`
- 真实用户数据
- 完全独立运行

### 同时运行
```bash
# Terminal 1: 开发版本
npm run tauri:dev

# Terminal 2: 打开正式版本
open src-tauri/target/release/bundle/macos/Intento.app
```
- ✅ 两个版本可以同时运行
- ✅ 各自独立的窗口和状态
- ✅ 完全不会互相干扰

## 测试验证

运行测试脚本：
```bash
./test_db_separation.sh
```

输出示例：
```
🔍 Checking database file separation...

📁 Expected database locations:
  Debug mode:   ~/Library/Application Support/com.intento.app.debug/intento.db
  Release mode: ~/Library/Application Support/com.intento.app/intento.db

✅ Debug app directory exists
✅ Development database found
-rw-r--r-- 1 user staff 123K Feb 21 20:30 intento.db

✅ Release app directory exists
✅ Production database found
-rw-r--r-- 1 user staff 456K Feb 21 19:15 intento.db
```

## 文件变更记录

| 文件 | 变更内容 |
|------|---------|
| `src-tauri/src/main.rs` | 添加 Bundle ID 动态设置逻辑 |
| `CLAUDE.md` | 更新数据库架构说明 |
| `docs/DATABASE_SEPARATION.md` | 完整的实现文档 |
| `test_db_separation.sh` | 更新测试脚本 |
| `docs/BUNDLE_ID_SEPARATION.md` | 本实现总结文档 |

## 技术要点

1. **编译时条件判断**: 使用 `#[cfg(debug_assertions)]` 在编译时确定 Bundle ID
2. **路径操作**: 通过 `parent()` + `join()` 替换默认的应用标识符路径
3. **目录创建**: 使用 `create_dir_all` 确保父目录存在
4. **日志记录**: 打印关键路径信息便于调试

## 最佳实践

✅ **开发时**
- 使用 `npm run tauri:dev` 启动开发版本
- 可以清空 debug 数据库重新测试
- 不用担心影响正式版本数据

✅ **发布前**
- 使用 `npm run tauri:build` 构建正式版本
- 在正式版本中进行最终测试
- 确认数据迁移和升级流程

✅ **数据管理**
- Debug 数据可以随时删除
- Release 数据需要谨慎备份
- 版本升级时注意数据库迁移

## 参考资料

- Tauri 文档: https://tauri.app/v1/guides/features/data-dir
- macOS Bundle ID 规范: https://developer.apple.com/documentation/bundleresources/information_property_list/cfbundleidentifier
- SQLite 最佳实践: https://www.sqlite.org/bestpractice.html

# Intento v0.1.0 发布总结

## ✅ 打包完成

**构建时间:** 2026-02-19
**总耗时:** ~3 分钟

### 📦 生成的文件

```
src-tauri/target/release/bundle/
├── macos/
│   └── Intento.app (22 MB) - macOS 应用包
└── dmg/
    └── Intento_0.1.0_aarch64.dmg (8.5 MB) - 分发安装包
```

---

## 🚀 快速测试

```bash
# 运行应用
open src-tauri/target/release/bundle/macos/Intento.app

# 或打开 DMG
open src-tauri/target/release/bundle/dmg/Intento_0.1.0_aarch64.dmg
```

---

## 📤 分发准备

### 1. 复制安装包到发布目录

```bash
mkdir -p releases/v0.1.0
cp src-tauri/target/release/bundle/dmg/Intento_0.1.0_aarch64.dmg releases/v0.1.0/
```

### 2. 生成 SHA256 校验和

```bash
cd releases/v0.1.0
shasum -a 256 Intento_0.1.0_aarch64.dmg > Intento_0.1.0_aarch64.dmg.sha256
```

### 3. 压缩 DMG（可选）

```bash
zip Intento_0.1.0_aarch64.dmg.zip Intento_0.1.0_aarch64.dmg
```

---

## 🌐 发布到 GitHub

### 创建 Release

```bash
# 创建 tag
git tag -a v0.1.0 -m "Release v0.1.0 - MVP"
git push origin v0.1.0

# 在 GitHub 创建 Release
# 1. 访问 https://github.com/your-username/Intento/releases/new
# 2. 选择 tag: v0.1.0
# 3. 标题: Intento v0.1.0 - MVP Release
# 4. 描述: 复制 docs/RELEASE_NOTES.md 内容
# 5. 上传: Intento_0.1.0_aarch64.dmg
# 6. 发布
```

---

## 📋 测试清单

使用安装包测试以下功能：

- [ ] 应用可以正常启动
- [ ] Command Palette (⌘K) 工作正常
- [ ] 创建任务（手动/AI/图片）
- [ ] 编辑和删除任务
- [ ] 日历视图显示正常
- [ ] 通知权限请求
- [ ] 设置面板可访问
- [ ] 总结功能正常
- [ ] 统计面板显示数据

---

## ⚠️ 已知问题

1. **Bundle ID 警告**
   - 当前: com.intento.app
   - 建议: com.intento.todo
   - 修复: 编辑 src-tauri/tauri.conf.json

2. **代码警告**
   - 21 个未使用代码警告
   - 不影响功能
   - 可运行: `cd src-tauri && cargo fix`

3. **首次打开警告**
   - macOS 会提示"无法验证开发者"
   - 解决: 右键点击 → 打开

---

## 📄 相关文档

- 发布说明: `docs/RELEASE_NOTES.md`
- 更新日志: `docs/CHANGELOG.md`
- 用户指南: `docs/user-guide/README.md`
- 项目路线图: `docs/ROADMAP.md`

---

## 🎯 下一步

1. **测试安装包** - 在干净的系统上测试
2. **修复警告** - 清理未使用的代码
3. **准备发布** - 创建 GitHub Release
4. **收集反馈** - 邀请用户试用

---

**发布成功！** 🎉

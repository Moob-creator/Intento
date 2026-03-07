# Intento v0.1.0 Release Notes

**发布日期:** 2026-02-19
**版本类型:** MVP (Minimum Viable Product)

---

## 📦 下载

### macOS (Apple Silicon)

- **DMG 安装包**: `Intento_0.1.0_aarch64.dmg` (8.5 MB)
  - 适用于: M1/M2/M3 芯片的 Mac
  - 安装方式: 双击打开，拖动到应用程序文件夹

### 系统要求

- macOS 11.0 (Big Sur) 或更高版本
- Apple Silicon (M1/M2/M3) 处理器
- 至少 50 MB 可用磁盘空间

---

## ✨ 功能特性

### 🎯 核心功能

- **⌨️ Command Palette** - ⌘K 打开通用命令中心
- **✍️ AI 文本解析** - 自然语言快速创建任务
- **📸 图片识别** - 从截图提取任务信息
- **📅 日历视图** - 月视图可视化任务分布
- **🔔 智能提醒** - 可配置的截止日期提醒
- **📊 自动总结** - 每日/周/月自动生成工作总结
- **🔍 高级筛选** - 多维度任务筛选
- **📈 统计面板** - 任务完成率分析

### 🎨 设计特色

- 温暖柔和的配色方案
- 流畅的动画过渡
- 全键盘快捷键支持
- 极简直观的界面

---

## 🚀 快速开始

### 安装步骤

1. 下载 `Intento_0.1.0_aarch64.dmg`
2. 双击打开 DMG 文件
3. 拖动 Intento 到"应用程序"文件夹
4. 从启动台或应用程序文件夹打开 Intento

### 首次使用

#### 1. 配置 AI API Key

打开设置 (⌘,)，配置以下任一 API Key：

- **OpenAI** (推荐 gpt-4o)
- **Anthropic Claude**
- **Moonshot Kimi**

#### 2. 允许通知权限

首次使用时，系统会请求通知权限：
- 点击"允许"以接收任务提醒
- 如误点拒绝，可在"系统设置 > 通知"中开启

#### 3. 创建第一个任务

**方式 1: AI 文本输入**
```
按 ⌘/ → 输入"明天下午3点前完成季度报告" → 确认
```

**方式 2: 图片识别**
```
按 ⌘/ → 粘贴截图 → AI 识别 → 确认
```

**方式 3: 手动创建**
```
按 ⌘N → 填写表单 → 保存
```

---

## ⌨️ 快捷键

| 快捷键 | 功能 |
|--------|------|
| `⌘K` | 打开 Command Palette |
| `⌘N` | 新建任务 |
| `⌘/` | AI 添加任务 |
| `⌘R` | 查看总结 |
| `⌘,` | 打开设置 |
| `ESC` | 关闭面板 |

---

## 📝 已知限制

### v0.1.0 MVP 版本限制

- ⚠️ 仅支持 Apple Silicon Mac（Intel 版本计划中）
- ⚠️ 暂无深色模式（计划在 v0.2.0）
- ⚠️ 上下文理解功能未完全实现
- ⚠️ 缺少季度和年度总结

### 安装注意事项

**首次打开可能提示"无法验证开发者":**

这是因为应用未经过 Apple 公证。解决方法：
1. 右键点击应用 → 选择"打开"
2. 在弹出对话框中点击"打开"
3. 或在"系统设置 > 隐私与安全性"中允许

**Bundle Identifier 警告:**
- 当前使用 `com.intento.app`
- 计划在下一版本改为 `com.intento.todo`

---

## 🐛 问题反馈

遇到问题？请通过以下方式反馈：

1. **GitHub Issues**: [创建 Issue](https://github.com/your-username/Intento/issues)
2. **功能建议**: [Discussions](https://github.com/your-username/Intento/discussions)
3. **邮件**: your-email@example.com

---

## 🗺️ 路线图

### v0.2.0 (计划 1-2 周后)

- [ ] 上下文缓存机制
- [ ] 季度/年度总结
- [ ] 日历周视图
- [ ] 操作撤销 (Undo)

### v1.0.0 (计划 2-3 个月后)

- [ ] 深色模式
- [ ] Intel Mac 支持
- [ ] Windows 版本
- [ ] 多语言支持

详见: [产品路线图](https://github.com/your-username/Intento/blob/master/docs/ROADMAP.md)

---

## 📚 文档

- **用户指南**: [docs/user-guide/README.md](../user-guide/README.md)
- **功能详解**:
  - [图片识别](../user-guide/features/image-parsing.md)
  - [通知设置](../user-guide/features/notifications.md)
- **开发文档**: [docs/specs/](../specs/)

---

## 🙏 致谢

### 技术栈

- [Tauri](https://tauri.app) - 跨平台桌面框架
- [React](https://react.dev) - UI 构建库
- [Rust](https://www.rust-lang.org) - 系统编程语言
- [ADK-Rust](https://github.com/adk-rust) - AI 开发套件

### 灵感来源

- [Raycast](https://raycast.com) - Command Palette 设计
- [Linear](https://linear.app) - 简洁的任务管理
- [Things](https://culturedcode.com/things/) - 优雅的交互

---

## 📄 许可证

MIT License - 详见 [LICENSE](../../LICENSE) 文件

---

## 📞 联系方式

- **作者**: @wangshuo
- **项目**: [GitHub](https://github.com/your-username/Intento)

---

**感谢使用 Intento！** 🎉

如果觉得有用，请给项目一个 ⭐️ Star！

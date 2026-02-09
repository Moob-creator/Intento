# Intento 前端界面参考设计

**说明：** 此文档记录了前端界面的初步设计参考，供后续开发时参考使用。

## 界面结构

项目包含以下主要界面设计：

### 1. 主页 (Home Page)
- **路径：** `specs/front-reference-pages/home_page/`
- **文件：**
  - `screen.png` - 界面截图
  - `code.html` - HTML 代码参考

### 2. 任务页面 (Tasks Page)
- **路径：** `specs/front-reference-pages/tasks_page/`
- **文件：**
  - `screen.png` - 界面截图
  - `code.html` - HTML 代码参考
- **功能：** 任务列表展示、任务状态管理（To Do / Doing / Done）

### 3. 任务确认对话框 (Task Confirm Dialog)
- **路径：** `specs/front-reference-pages/task_confirm_dialog/`
- **文件：**
  - `screen.png` - 界面截图
  - `code.html` - HTML 代码参考
- **功能：** AI 解析结果确认界面，用户可以修改或确认添加任务

### 4. 总结中心 (Summary Center Page)
- **路径：** `specs/front-reference-pages/summary_center_page/`
- **文件：**
  - `screen.png` - 界面截图
  - `code.html` - HTML 代码参考
- **功能：** 展示每日、月度、季度、年度总结的时间轴界面

### 5. 设置页面 (Settings Page)
- **路径：** `specs/front-reference-pages/settings_page/`
- **文件：**
  - `screen.png` - 界面截图
  - `code.html` - HTML 代码参考
- **功能：** 用户设置配置，包括 AI 提供商、API Key、提醒时间等

## 技术栈建议

### 前端框架
- **React 19.2.4 + TypeScript**
- **Vite 7.3.1** (构建工具)
- **Zustand** (状态管理)
- **shadcn/ui 3.8.4** (UI 组件库，推荐)
- **Tailwind CSS** (样式方案)

### UI 组件使用建议
根据设计参考，主要需要的 shadcn/ui 组件：
- `button` - 各种操作按钮
- `card` - 任务卡片、总结卡片
- `dialog` - 任务确认对话框
- `input` - 输入框
- `textarea` - 多行文本输入
- `checkbox` - 任务状态切换
- `select` - 下拉选择
- `calendar` - 日期选择器
- `dropdown-menu` - 下拉菜单

## 开发注意事项

1. **设计风格：** 界面采用现代简洁风格，参考 shadcn/ui 的 Slate 主题色
2. **响应式：** 需要考虑桌面应用的不同窗口大小
3. **用户体验：**
   - 任务确认流程要简洁流畅
   - 总结中心的时间轴要清晰易读
   - 设置页面要简单明了
4. **性能：** 任务列表需要考虑虚拟滚动（大量任务时）

## 后续开发计划

1. **Phase 2** (基础任务管理) 将参考 `tasks_page` 和 `task_confirm_dialog`
2. **Phase 5** (自动总结功能) 将参考 `summary_center_page`
3. **Phase 6** (优化与发布) 将参考 `settings_page`

## 参考资源

- 所有设计稿位于：`specs/front-reference-pages/`
- 每个页面都包含截图和 HTML 代码参考
- 建议在实际开发前仔细查看截图理解交互流程

---

**创建时间：** 2026-02-09
**状态：** 参考设计，待开发实现

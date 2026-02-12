# Intento 测试体系

本文档描述 Intento 项目的测试架构和测试策略。

## 测试层次

```
┌─────────────────────────────────────┐
│      E2E 测试（端到端测试）            │
│   测试完整的用户工作流                 │
└─────────────────────────────────────┘
              ↑
┌─────────────────────────────────────┐
│      集成测试（Integration Tests）    │
│   测试前端与后端的交互                 │
└─────────────────────────────────────┘
              ↑
┌──────────────────┬──────────────────┐
│   前端单元测试      │   后端单元测试     │
│  React Components │  Rust Functions  │
└──────────────────┴──────────────────┘
```

## 1. 后端测试（Rust）

### 1.1 单元测试

**位置**: `src-tauri/src/*/tests.rs` 或 `#[cfg(test)]` 模块

**已有测试**:
- ✅ 数据库测试：`src-tauri/src/db/mod.rs`
  - `test_database_creation`
  - `test_task_crud`
- ✅ AI 客户端测试：`src-tauri/src/ai/tests.rs`
  - 15 个单元测试已通过

**运行命令**:
```bash
cd src-tauri
cargo test
```

**需要添加的测试**:
- [ ] Task 模型验证测试
- [ ] 时间解析测试
- [ ] 错误处理测试

### 1.2 集成测试

**位置**: `src-tauri/tests/`

**测试内容**:
- Tauri Commands 集成
- AI API 真实调用
- 数据库完整工作流

**示例结构**:
```rust
// src-tauri/tests/integration_test.rs
#[tokio::test]
async fn test_full_task_workflow() {
    // 1. 解析文本
    // 2. 创建任务
    // 3. 查询任务
    // 4. 更新任务
    // 5. 删除任务
}
```

### 1.3 性能测试

**工具**: `criterion` crate

**位置**: `src-tauri/benches/`

**测试内容**:
- 数据库查询性能
- AI 解析响应时间
- 大量任务列表渲染

## 2. 前端测试（React + TypeScript）

### 2.1 单元测试

**工具**: Vitest + React Testing Library

**安装依赖**:
```bash
npm install -D vitest @testing-library/react @testing-library/jest-dom @testing-library/user-event jsdom
```

**配置**: `vitest.config.ts`

**测试文件**: `src/**/*.test.tsx`

**测试内容**:
- 组件渲染
- 用户交互
- 状态管理（Zustand store）
- 工具函数

**示例**:
```typescript
// src/components/TaskCard.test.tsx
import { render, screen } from '@testing-library/react';
import { TaskCard } from './TaskCard';

test('renders task card with title', () => {
  const task = {
    id: 1,
    title: 'Test Task',
    status: 'todo',
    // ...
  };

  render(<TaskCard task={task} />);
  expect(screen.getByText('Test Task')).toBeInTheDocument();
});
```

**运行命令**:
```bash
npm run test        # 运行所有测试
npm run test:watch  # 监视模式
npm run test:ui     # UI 模式
```

### 2.2 集成测试

**工具**: Vitest + MSW (Mock Service Worker)

**测试内容**:
- Tauri invoke 调用
- 完整的用户交互流程
- 错误处理

**示例**:
```typescript
// src/__tests__/task-creation.test.tsx
test('user can create task via AI input', async () => {
  const user = userEvent.setup();

  render(<App />);

  // 点击 AI Add Task 按钮
  await user.click(screen.getByText('AI Add Task'));

  // 输入任务描述
  await user.type(screen.getByPlaceholderText(/describe/i),
    '明天下午3点开会');

  // 提交
  await user.click(screen.getByRole('button', { name: /send/i }));

  // 验证确认对话框出现
  expect(await screen.findByText(/confirm/i)).toBeInTheDocument();
});
```

## 3. E2E 测试（端到端）

### 3.1 工具选择

**推荐**: Playwright

**安装**:
```bash
npm install -D @playwright/test
npx playwright install
```

**配置**: `playwright.config.ts`

### 3.2 测试场景

**位置**: `e2e/`

**场景列表**:
1. **任务创建流程**
   - 用户打开应用
   - 点击 AI Add Task
   - 输入任务描述
   - 确认并创建
   - 验证任务出现在列表

2. **任务管理流程**
   - 筛选任务（To Do / Doing / Done）
   - 搜索任务
   - 编辑任务
   - 删除任务

3. **AI 解析流程**
   - 测试各种自然语言输入
   - 验证解析结果准确性

**示例**:
```typescript
// e2e/task-creation.spec.ts
import { test, expect } from '@playwright/test';

test('create task via AI input', async ({ page }) => {
  await page.goto('http://localhost:1420');

  // 点击 AI Add Task 按钮
  await page.click('text=AI Add Task');

  // 输入任务描述
  await page.fill('textarea', '明天下午3点开会讨论项目进度');

  // 提交
  await page.click('button:has-text("发送")');

  // 等待确认对话框
  await expect(page.locator('text=确认任务')).toBeVisible();

  // 确认创建
  await page.click('button:has-text("Add Task")');

  // 验证任务出现在列表
  await expect(page.locator('text=开会讨论项目进度')).toBeVisible();
});
```

**运行**:
```bash
npx playwright test
npx playwright test --ui  # UI 模式
```

## 4. 测试覆盖率

### 4.1 后端覆盖率

**工具**: `tarpaulin`

**安装**:
```bash
cargo install cargo-tarpaulin
```

**运行**:
```bash
cd src-tauri
cargo tarpaulin --out Html
```

**目标**: > 80% 覆盖率

### 4.2 前端覆盖率

**工具**: Vitest 内置

**配置**: `vitest.config.ts`
```typescript
export default defineConfig({
  test: {
    coverage: {
      provider: 'v8',
      reporter: ['text', 'html'],
      exclude: ['node_modules/', 'dist/'],
    },
  },
});
```

**运行**:
```bash
npm run test:coverage
```

**目标**: > 80% 覆盖率

## 5. CI/CD 集成

### 5.1 GitHub Actions

**配置**: `.github/workflows/test.yml`

```yaml
name: Tests

on: [push, pull_request]

jobs:
  rust-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run Rust tests
        run: cd src-tauri && cargo test

  frontend-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: 18
      - run: npm ci
      - run: npm run test
      - run: npm run test:coverage

  e2e-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
      - run: npm ci
      - run: npx playwright install --with-deps
      - run: npm run build
      - run: npx playwright test
```

### 5.2 测试报告

- **覆盖率报告**: 上传到 Codecov
- **E2E 视频**: 失败时自动录制
- **性能基准**: 保存历史数据

## 6. 测试最佳实践

### 6.1 命名规范

```
test_<function_name>_<scenario>_<expected_result>

示例:
- test_create_task_with_valid_data_succeeds
- test_parse_text_input_with_invalid_format_fails
```

### 6.2 AAA 模式

```typescript
test('example', () => {
  // Arrange - 准备测试数据
  const task = createMockTask();

  // Act - 执行操作
  const result = processTask(task);

  // Assert - 验证结果
  expect(result).toBe(expected);
});
```

### 6.3 Mock 策略

- **后端**: 使用 Mock 替代真实 AI API（单元测试）
- **前端**: 使用 MSW 拦截 Tauri invoke 调用
- **E2E**: 使用真实 API（测试环境）

## 7. 快速开始

### 初始化测试环境

```bash
# 1. 安装前端测试依赖
npm install -D vitest @testing-library/react @testing-library/jest-dom \
  @testing-library/user-event jsdom @vitest/ui

# 2. 安装 E2E 测试工具
npm install -D @playwright/test
npx playwright install

# 3. 安装后端覆盖率工具
cargo install cargo-tarpaulin
```

### 运行所有测试

```bash
# 后端测试
cd src-tauri && cargo test

# 前端测试
npm run test

# E2E 测试
npm run build
npx playwright test

# 查看覆盖率
npm run test:coverage
cd src-tauri && cargo tarpaulin --out Html
```

## 8. 测试清单

### Phase 1: 基础测试（当前）
- [x] 数据库 CRUD 测试
- [x] AI 客户端单元测试
- [ ] Tauri Commands 单元测试
- [ ] 前端组件单元测试

### Phase 2: 集成测试
- [ ] 前后端集成测试
- [ ] AI 真实 API 集成测试
- [ ] 完整工作流测试

### Phase 3: E2E 测试
- [ ] 任务创建流程
- [ ] 任务管理流程
- [ ] AI 解析流程
- [ ] 错误处理流程

### Phase 4: 性能测试
- [ ] 数据库查询性能
- [ ] UI 渲染性能
- [ ] AI 响应时间

## 9. 参考资源

- **Rust 测试**: https://doc.rust-lang.org/book/ch11-00-testing.html
- **Vitest**: https://vitest.dev/
- **React Testing Library**: https://testing-library.com/react
- **Playwright**: https://playwright.dev/
- **Tauri 测试**: https://tauri.app/v1/guides/testing/

---

**维护者**: 请在添加新功能时同步添加相应的测试。

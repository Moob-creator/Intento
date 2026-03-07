# Intento 自动化测试策略

## 测试金字塔架构

```
           /\
          /  \    E2E Tests (端到端测试)
         /----\   - 完整用户流程验证
        /      \  - 关键业务场景
       /--------\
      /          \ Integration Tests (集成测试)
     /            \ - Tauri 命令测试
    /--------------\ - AI 集成测试
   /                \ - 数据库集成测试
  /------------------\
 /                    \ Unit Tests (单元测试)
/______________________\ - Rust 业务逻辑
                          - React 组件测试
```

## 1. 单元测试 (Unit Tests)

### 1.1 Rust 后端单元测试

**测试范围:**
- 数据库 CRUD 操作
- AI 提示词生成
- 日期时间计算
- 数据模型序列化/反序列化

**工具:**
- `cargo test` - Rust 内置测试框架
- `tempfile` - 临时数据库测试
- `mockall` - Mock 对象（可选）

**示例测试文件结构:**
```
src-tauri/
├── src/
│   ├── db/
│   │   ├── mod.rs
│   │   └── tests.rs          # 数据库单元测试
│   ├── ai/
│   │   ├── prompts.rs
│   │   └── tests.rs          # AI 模块单元测试
│   └── summary/
│       ├── period.rs
│       └── tests.rs          # 时间计算单元测试
```

**运行命令:**
```bash
cd src-tauri
cargo test --lib                    # 运行所有单元测试
cargo test --lib db::tests          # 只测试数据库模块
cargo test --lib -- --nocapture     # 显示 println! 输出
```

### 1.2 React 组件单元测试

**测试范围:**
- UI 组件渲染
- 用户交互逻辑
- Zustand store 状态管理
- 工具函数

**工具:**
- `vitest` - 现代化测试框架
- `@testing-library/react` - React 组件测试
- `@testing-library/user-event` - 用户交互模拟
- `happy-dom` - 轻量级 DOM 环境

**配置 (package.json):**
```json
{
  "scripts": {
    "test": "vitest",
    "test:ui": "vitest --ui",
    "test:coverage": "vitest --coverage"
  },
  "devDependencies": {
    "vitest": "^2.0.0",
    "@testing-library/react": "^16.0.0",
    "@testing-library/user-event": "^14.5.0",
    "@testing-library/jest-dom": "^6.4.0",
    "@vitest/ui": "^2.0.0",
    "happy-dom": "^15.0.0",
    "jsdom": "^25.0.0"
  }
}
```

**示例测试文件:**
```typescript
// src/__tests__/components/TaskCard.test.tsx
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { TaskCard } from '@/components/TaskCard';

describe('TaskCard', () => {
  it('renders task title', () => {
    const task = { id: 1, title: '测试任务', status: 'todo' };
    render(<TaskCard task={task} />);
    expect(screen.getByText('测试任务')).toBeInTheDocument();
  });
});
```

## 2. 集成测试 (Integration Tests)

### 2.1 Rust Backend 集成测试

**测试范围:**
- Tauri 命令完整流程
- 数据库迁移
- AI API 真实调用 (带 `#[ignore]` 标记)
- 后台调度任务

**现有测试文件:**
```
src-tauri/tests/
├── test_integration.rs           # AI 图像解析集成测试
├── test_phase2_integration.rs    # Phase 2 完整流程测试
├── test_real_image.rs            # 真实图像测试
└── test_improved_ocr.rs          # OCR 改进测试
```

**运行命令:**
```bash
# 运行快速集成测试（不调用 AI API）
cargo test --test test_integration

# 运行完整测试（包括 AI API 调用）
cargo test --test test_integration -- --ignored --nocapture

# 运行特定测试
cargo test --test test_phase2_integration test_phase2_full_workflow
```

### 2.2 Frontend-Backend 集成测试

**测试范围:**
- Tauri `invoke()` 命令调用
- 前后端数据流转
- 错误处理

**工具:**
- `@tauri-apps/api` 的 Mock 版本
- `msw` (Mock Service Worker) - Mock AI API

**示例:**
```typescript
// src/__tests__/integration/taskStore.test.ts
import { renderHook, waitFor } from '@testing-library/react';
import { useTaskStore } from '@/store/taskStore';
import { mockIPC } from '@tauri-apps/api/mocks';

describe('Task Store Integration', () => {
  beforeEach(() => {
    mockIPC((cmd, args) => {
      if (cmd === 'list_tasks') {
        return [{ id: 1, title: '测试', status: 'todo' }];
      }
    });
  });

  it('loads tasks from backend', async () => {
    const { result } = renderHook(() => useTaskStore());
    await waitFor(() => {
      expect(result.current.tasks).toHaveLength(1);
    });
  });
});
```

## 3. 端到端测试 (E2E Tests)

### 3.1 完整用户流程测试

**测试范围:**
- 完整的用户操作流程
- 跨窗口交互
- 系统通知
- 真实数据持久化

**工具:**
- `Playwright` - 推荐用于 Tauri E2E 测试
- `tauri-driver` + WebDriver - 官方推荐

**关键测试场景:**
1. 新建任务流程
2. AI 文本解析流程
3. AI 图像识别流程
4. 任务状态更新
5. 摘要生成
6. 通知触发

### 3.2 Playwright 配置

**安装:**
```bash
npm install -D @playwright/test
npx playwright install
```

**配置文件 (playwright.config.ts):**
```typescript
import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './e2e',
  timeout: 30000,
  use: {
    headless: false,
    viewport: { width: 1280, height: 720 },
  },
  projects: [
    {
      name: 'dev',
      use: {
        // 连接到 tauri:dev 启动的应用
        baseURL: 'http://localhost:1420',
      },
    },
    {
      name: 'production',
      use: {
        // 测试打包后的应用
        executablePath: './src-tauri/target/release/bundle/macos/Intento.app',
      },
    },
  ],
});
```

**示例 E2E 测试:**
```typescript
// e2e/task-creation.spec.ts
import { test, expect } from '@playwright/test';

test('create task with text input', async ({ page }) => {
  await page.goto('/');

  // 点击新建任务
  await page.click('button:has-text("新建任务")');

  // 输入任务标题
  await page.fill('input[name="title"]', '完成测试报告');

  // 设置优先级
  await page.selectOption('select[name="priority"]', 'high');

  // 保存
  await page.click('button:has-text("保存")');

  // 验证任务出现在列表中
  await expect(page.locator('text=完成测试报告')).toBeVisible();
});

test('parse text with AI', async ({ page }) => {
  await page.goto('/');

  // 输入自然语言
  await page.fill('textarea', '明天下午3点前完成季度报告，优先级高');

  // 触发 AI 解析
  await page.keyboard.press('Control+Enter');

  // 等待解析结果
  await page.waitForSelector('text=解析成功');

  // 验证确认对话框
  await expect(page.locator('text=完成季度报告')).toBeVisible();

  // 确认创建
  await page.click('button:has-text("确认")');

  // 验证任务创建成功
  await expect(page.locator('.task-list').locator('text=季度报告')).toBeVisible();
});
```

## 4. 测试数据管理

### 4.1 测试数据库

**策略:**
- 每个测试使用独立的临时数据库
- 测试结束后自动清理
- 使用 fixture 预设测试数据

**Rust 实现:**
```rust
use tempfile::TempDir;

pub fn create_test_db() -> (Database, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = Database::new(db_path.to_str().unwrap()).unwrap();
    (db, temp_dir)
}

#[test]
fn test_create_task() {
    let (db, _temp_dir) = create_test_db();
    let task_id = db.create_task(/* ... */).unwrap();
    assert!(task_id > 0);
    // temp_dir 在函数结束时自动删除
}
```

### 4.2 测试图像

**准备测试图像:**
```
src-tauri/test-assets/
├── valid/
│   ├── task-list-clear.jpg      # 清晰的任务列表
│   ├── task-list-handwritten.jpg # 手写任务
│   └── task-with-deadline.png   # 包含截止日期
├── invalid/
│   ├── no-text.jpg              # 无文字图像
│   └── corrupted.jpg            # 损坏图像
└── edge-cases/
    ├── multilingual.jpg         # 多语言混合
    └── small-text.jpg           # 小号字体
```

## 5. CI/CD 自动化

### 5.1 GitHub Actions 配置

```yaml
# .github/workflows/test.yml
name: Run Tests

on:
  push:
    branches: [master, develop]
  pull_request:
    branches: [master, develop]

jobs:
  rust-tests:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Cache Cargo
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            src-tauri/target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}

      - name: Run unit tests
        working-directory: src-tauri
        run: cargo test --lib

      - name: Run integration tests (no API)
        working-directory: src-tauri
        run: cargo test --tests

  frontend-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'

      - name: Install dependencies
        run: npm ci

      - name: Run unit tests
        run: npm test

      - name: Generate coverage
        run: npm run test:coverage

      - name: Upload coverage
        uses: codecov/codecov-action@v4
        with:
          files: ./coverage/coverage-final.json

  e2e-tests:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Install dependencies
        run: |
          npm ci
          rustup target add aarch64-apple-darwin

      - name: Build Tauri app
        run: npm run tauri:build

      - name: Install Playwright
        run: npx playwright install --with-deps

      - name: Run E2E tests
        run: npm run test:e2e

      - name: Upload test results
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: playwright-report
          path: playwright-report/
```

## 6. 测试覆盖率目标

| 层级 | 目标覆盖率 | 当前状态 |
|------|-----------|---------|
| Rust 单元测试 | 80% | 待实施 |
| Rust 集成测试 | 60% | 部分完成 |
| React 组件测试 | 70% | 待实施 |
| E2E 关键流程 | 100% | 待实施 |

## 7. 快速开始指南

### 7.1 运行所有测试

```bash
# 1. Rust 单元测试
cd src-tauri && cargo test --lib

# 2. Rust 集成测试（快速）
cargo test --tests

# 3. Frontend 单元测试
npm test

# 4. E2E 测试（需要先构建应用）
npm run tauri:dev &  # 在后台启动应用
npm run test:e2e
```

### 7.2 测试开发工作流

**开发新功能时:**
1. 先写单元测试（TDD）
2. 实现功能代码
3. 运行单元测试确保通过
4. 添加集成测试
5. 手动测试 UI
6. 提交前运行完整测试套件

**修复 Bug 时:**
1. 先写复现 Bug 的测试用例
2. 确认测试失败
3. 修复代码
4. 确认测试通过
5. 添加回归测试

## 8. 性能测试

### 8.1 数据库性能

```rust
#[test]
fn bench_create_1000_tasks() {
    let (db, _temp_dir) = create_test_db();
    let start = std::time::Instant::now();

    for i in 0..1000 {
        db.create_task(&format!("Task {}", i), /* ... */).unwrap();
    }

    let duration = start.elapsed();
    println!("Created 1000 tasks in {:?}", duration);
    assert!(duration.as_millis() < 5000); // 应在 5 秒内完成
}
```

### 8.2 AI API 响应时间

```rust
#[tokio::test]
#[ignore]
async fn bench_ai_parse_text() {
    let client = AiClient::new_default().unwrap();
    let start = std::time::Instant::now();

    let result = client.parse_text("明天完成报告").await.unwrap();

    let duration = start.elapsed();
    println!("AI parsing took {:?}", duration);
    assert!(duration.as_secs() < 10); // 应在 10 秒内响应
}
```

## 9. 测试最佳实践

### 9.1 编写好的测试

✅ **DO:**
- 每个测试只验证一个功能点
- 使用清晰的测试名称描述期望行为
- 使用 AAA 模式: Arrange, Act, Assert
- 测试应该是独立的，可以并行运行
- 使用 fixture 和 helper 函数减少重复代码

❌ **DON'T:**
- 测试依赖外部服务（除非是集成测试）
- 测试之间有顺序依赖
- 使用 sleep() 等待异步操作（使用 waitFor）
- 测试实现细节而非行为
- 忽略测试失败

### 9.2 Mock vs Real API

**Mock AI API (单元/集成测试):**
```rust
// 使用 mock 避免消耗 API credits
pub fn mock_ai_response() -> ParsedTask {
    ParsedTask {
        title: "测试任务".to_string(),
        // ...
    }
}
```

**Real AI API (E2E 测试):**
```rust
#[tokio::test]
#[ignore] // 手动运行，避免 CI 消耗配额
async fn test_real_ai_parsing() {
    dotenv::dotenv().ok();
    let client = AiClient::new_default().unwrap();
    // 调用真实 API
}
```

## 10. 测试维护

### 10.1 定期审查

- 每月检查测试覆盖率
- 移除过时的测试
- 更新测试数据
- 重构重复的测试代码

### 10.2 测试失败处理

**本地失败:**
1. 检查是否是环境问题（数据库、API key）
2. 查看详细错误日志
3. 单独运行失败的测试
4. 使用 debugger 定位问题

**CI 失败:**
1. 查看 CI 日志
2. 本地复现问题
3. 修复后重新提交
4. 考虑是否需要更新 CI 配置

## 11. 下一步行动

### Phase 1: 基础测试框架 (1-2 天)
- [x] Rust 集成测试（已存在）
- [ ] 配置 Vitest
- [ ] 配置 Playwright
- [ ] 添加 5 个核心组件单元测试
- [ ] 添加 3 个 E2E 关键流程测试

### Phase 2: 扩展测试覆盖 (3-5 天)
- [ ] 数据库模块单元测试
- [ ] AI 模块单元测试
- [ ] Summary 模块单元测试
- [ ] 所有 React 组件单元测试
- [ ] 完整的 E2E 测试套件

### Phase 3: CI/CD 集成 (1-2 天)
- [ ] GitHub Actions 配置
- [ ] 覆盖率报告
- [ ] 自动化测试通知
- [ ] 性能基准测试

---

**文档版本:** 1.0
**更新日期:** 2026-02-21
**负责人:** Development Team

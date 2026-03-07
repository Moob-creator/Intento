# Intento 自动化测试完整指南

## 测试已配置完成 ✅

你的项目现在已经有了完整的自动化测试基础设施。

## 📋 快速开始

### 1. 立即可用 - Rust 测试（无需额外配置）

```bash
# 运行所有测试（最简单）
./test.sh all

# 查看帮助
./test.sh --help

# 运行特定类型测试
./test.sh rust          # Rust 单元测试
./test.sh rust-int      # Rust 集成测试
./test.sh rust-all      # 所有 Rust 测试

# 带详细输出
./test.sh -v rust
```

### 2. Rust 测试详细用法

**数据库单元测试 (新增):**
```bash
cd src-tauri
cargo test --lib db::tests          # 只运行数据库测试
cargo test --lib db::tests::test_create_task  # 运行单个测试
cargo test --lib -- --nocapture     # 显示 println! 输出
```

**已有的集成测试:**
```bash
# 快速测试（不调用 AI API）
cargo test --tests

# AI 真实调用测试（会消耗配额）
./test.sh rust-ai
# 或
cargo test --test test_integration -- --ignored --nocapture
```

### 3. 前端测试（需要先配置）

**安装依赖:**
```bash
npm install -D vitest @testing-library/react @testing-library/user-event \
  @testing-library/jest-dom happy-dom @vitest/ui
```

**运行测试:**
```bash
./test.sh frontend
# 或
npm test
```

### 4. E2E 测试（需要先配置）

**安装依赖:**
```bash
npm install -D @playwright/test
npx playwright install chromium
```

**运行测试:**
```bash
# 1. 启动应用
npm run tauri:dev

# 2. 在另一个终端运行测试
./test.sh e2e
# 或
npx playwright test
```

## 📊 当前测试覆盖

### Rust Backend

✅ **集成测试（已存在）:**
- `test_integration.rs` - AI 图像解析流程
- `test_phase2_integration.rs` - 多操作解析
- `test_real_image.rs` - 真实图像测试
- `test_improved_ocr.rs` - OCR 测试

✅ **单元测试（新增）:**
- `src/db/tests.rs` - 数据库 CRUD 测试
  - ✅ 创建任务
  - ✅ 列出任务
  - ✅ 更新任务
  - ✅ 软删除
  - ✅ 标签处理
  - ✅ 状态筛选
  - ✅ 优先级
  - ✅ 截止日期
  - ✅ 搜索功能

### Frontend（待配置）

📝 准备好的测试框架:
- Vitest 配置文件示例
- React Testing Library 设置
- 组件测试模板

### E2E（待配置）

📝 准备好的测试框架:
- Playwright 配置文件
- 关键流程测试模板

## 🎯 测试示例

### 运行新添加的数据库测试

```bash
cd src-tauri

# 运行所有数据库测试
cargo test --lib db::tests

# 运行特定测试
cargo test --lib db::tests::test_create_task
cargo test --lib db::tests::test_soft_delete
cargo test --lib db::tests::test_multiple_operations

# 查看测试列表
cargo test --lib db::tests -- --list
```

**预期输出:**
```
running 12 tests
test db::tests::test_create_task ... ok
test db::tests::test_list_tasks ... ok
test db::tests::test_update_task ... ok
test db::tests::test_soft_delete ... ok
test db::tests::test_task_with_tags ... ok
test db::tests::test_filter_by_status ... ok
test db::tests::test_priority_values ... ok
test db::tests::test_deadline_handling ... ok
test db::tests::test_multiple_operations ... ok
test db::tests::test_empty_database ... ok
test db::tests::test_search_functionality ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured
```

### 运行集成测试

```bash
cd src-tauri

# 快速集成测试（不调用 AI）
cargo test --tests

# 包含 AI API 调用的测试
cargo test --test test_integration test_full_workflow_with_real_image -- --ignored
```

## 📝 手动测试清单

如果你想快速手动验证功能，可以使用这个清单：

```bash
# 复制手动测试清单到桌面
cp docs/testing/manual_test_checklist.md ~/Desktop/
```

然后打开文件，按照清单逐项测试应用功能。

## 🔧 配置前端测试（可选）

### Step 1: 创建 Vitest 配置

```bash
cat > vitest.config.ts << 'EOF'
import { defineConfig } from 'vitest/config';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  test: {
    environment: 'happy-dom',
    setupFiles: ['./src/test/setup.ts'],
    globals: true,
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
});
EOF
```

### Step 2: 创建测试 setup 文件

```bash
mkdir -p src/test
cat > src/test/setup.ts << 'EOF'
import '@testing-library/jest-dom';
import { cleanup } from '@testing-library/react';
import { afterEach } from 'vitest';

afterEach(() => {
  cleanup();
});

// Mock Tauri API
global.window.__TAURI_INTERNALS__ = {
  invoke: async (cmd: string, args?: any) => {
    console.log(`Mock invoke: ${cmd}`, args);
    return null;
  },
};
EOF
```

### Step 3: 更新 package.json

```json
{
  "scripts": {
    "test": "vitest",
    "test:ui": "vitest --ui",
    "test:coverage": "vitest --coverage"
  }
}
```

### Step 4: 创建示例测试

参考 `docs/testing/QUICK_START.md` 中的组件测试示例。

## 🚀 配置 E2E 测试（可选）

### Step 1: 创建 Playwright 配置

```bash
cat > playwright.config.ts << 'EOF'
import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './e2e',
  timeout: 30000,
  use: {
    headless: false,
    viewport: { width: 1280, height: 720 },
    screenshot: 'only-on-failure',
  },
});
EOF
```

### Step 2: 创建测试目录和示例

```bash
mkdir -p e2e
```

参考 `docs/testing/QUICK_START.md` 中的 E2E 测试示例。

## 📚 文档资源

### 完整文档
- **测试策略:** `docs/testing/TESTING_STRATEGY.md`
  - 完整的测试架构和最佳实践
  - 测试金字塔
  - CI/CD 配置

- **快速开始:** `docs/testing/QUICK_START.md`
  - 详细的配置步骤
  - 测试示例代码
  - 常见问题解答

### 快速参考

```bash
# Rust 测试
./test.sh rust              # 单元测试
./test.sh rust-int          # 集成测试
./test.sh rust-ai           # AI 测试（消耗配额）
./test.sh rust-all          # 所有 Rust 测试

# 前端测试（配置后）
./test.sh frontend
npm test
npm run test:ui

# E2E 测试（配置后）
./test.sh e2e
npx playwright test

# 查看统计
./test.sh stats
```

## 🎓 测试开发工作流

### 日常开发

1. 写代码前先写测试（TDD）
2. 实现功能
3. 运行测试确保通过
4. 提交前运行 `./test.sh all`

### 修复 Bug

1. 写能复现 Bug 的测试用例
2. 确认测试失败
3. 修复代码
4. 确认测试通过

## ⚡ 性能测试

```bash
cd src-tauri

# 测试大量任务创建性能
cargo test --lib db::tests::test_multiple_operations -- --nocapture

# 自定义性能测试
cargo test --release --lib -- --nocapture
```

## 🐛 调试测试

### Rust 测试

```bash
# 运行单个测试并显示输出
cargo test --lib db::tests::test_create_task -- --nocapture

# 使用 rust-lldb 调试
rust-lldb target/debug/deps/intento-<hash>
```

### 前端测试

```bash
# 使用 UI 模式调试
npm run test:ui

# 使用 VSCode 调试
# 在测试文件中设置断点，然后按 F5
```

### E2E 测试

```bash
# Debug 模式运行（会暂停在每个操作）
npx playwright test --debug

# 查看测试报告
npx playwright show-report
```

## 🔄 CI/CD 集成（后续）

当你准备好时，可以参考 `docs/testing/TESTING_STRATEGY.md` 第 5 节配置 GitHub Actions。

## ❓ 常见问题

### Q: 测试运行很慢？

```bash
# 只运行特定测试
cargo test --lib db::tests::test_create_task

# 使用 release 模式
cargo test --release
```

### Q: AI 测试消耗太多配额？

```bash
# 大部分时间只运行快速测试
./test.sh all

# 只在需要时手动运行 AI 测试
./test.sh rust-ai
```

### Q: 如何添加新测试？

**Rust 单元测试:**
1. 在相应模块创建 `tests.rs`
2. 在 `mod.rs` 中添加 `#[cfg(test)] mod tests;`
3. 编写测试函数（`#[test]` 标注）

**Rust 集成测试:**
1. 在 `src-tauri/tests/` 创建新文件
2. 编写测试函数
3. 使用 `#[ignore]` 标记需要手动运行的测试

**前端测试:**
1. 在 `src/__tests__/` 创建测试文件
2. 使用 `describe` 和 `it` 组织测试
3. 运行 `npm test` 自动检测新测试

## 📞 获取帮助

- 查看完整测试策略: `docs/testing/TESTING_STRATEGY.md`
- 查看快速开始指南: `docs/testing/QUICK_START.md`
- 运行 `./test.sh --help` 查看测试脚本帮助

---

**祝测试愉快！** 🎉

有任何问题随时查阅文档或寻求帮助。

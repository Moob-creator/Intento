# 快速开始 - Intento 自动化测试

## 🚀 立即可用的测试方法

你的项目已经有一些测试基础，下面是立即可以使用的测试方法。

## 1. Rust Backend 测试（已可用）✅

### 运行现有集成测试

```bash
cd src-tauri

# 运行所有快速测试（不调用真实 AI API）
cargo test --tests

# 查看详细输出
cargo test --tests -- --nocapture

# 运行特定测试文件
cargo test --test test_phase2_integration

# 运行单个测试
cargo test --test test_phase2_integration test_operation_execution_simulation
```

### 运行 AI API 真实调用测试（消耗配额）

```bash
# 确保 .env 文件配置了 API keys
# OPENAI_API_KEY=sk-xxx
# ANTHROPIC_API_KEY=sk-ant-xxx

# 运行被 ignore 的测试（会调用真实 AI API）
cargo test --test test_integration -- --ignored --nocapture

# 运行图像解析测试
cargo test --test test_real_image -- --ignored --nocapture
```

### 现有测试文件说明

| 文件 | 用途 | 是否调用 API |
|------|------|-------------|
| `test_integration.rs` | AI 图像解析完整流程 | ✅ 需要 |
| `test_phase2_integration.rs` | 多操作解析和模拟执行 | ⚠️ 部分需要 |
| `test_real_image.rs` | 真实图像识别测试 | ✅ 需要 |
| `test_improved_ocr.rs` | OCR 改进测试 | ✅ 需要 |

## 2. 手动端到端测试（最快验证方法）✅

如果你只是想快速验证功能，可以手动测试关键流程。

### 测试清单模板

创建测试检查清单：

```markdown
# Intento 手动测试清单

测试日期: 2026-02-21
测试人: [你的名字]
版本: v0.1.0

## 基础功能测试

### 1. 任务创建
- [ ] 点击"新建任务"按钮
- [ ] 输入标题："测试任务 1"
- [ ] 设置优先级为"高"
- [ ] 设置截止日期为明天
- [ ] 添加标签 "测试"
- [ ] 点击保存
- [ ] 验证任务出现在列表中

### 2. AI 文本解析
- [ ] 在输入框输入："明天下午3点前完成季度报告，优先级高"
- [ ] 按 Cmd+Enter 或点击"解析"
- [ ] 验证解析结果正确
  - 标题：完成季度报告
  - 截止时间：明天 15:00
  - 优先级：高
- [ ] 点击"确认"创建任务
- [ ] 验证任务成功创建

### 3. AI 图像解析
- [ ] 复制包含任务列表的图片
- [ ] 在应用中粘贴（Cmd+V）
- [ ] 验证识别出的任务
- [ ] 勾选需要创建的任务
- [ ] 点击"确认"
- [ ] 验证任务成功创建

### 4. 任务状态更新
- [ ] 选中一个任务
- [ ] 点击"标记为进行中"
- [ ] 验证状态变为 "doing"
- [ ] 点击"标记为完成"
- [ ] 验证状态变为 "done"

### 5. 任务编辑
- [ ] 双击任务打开详情
- [ ] 修改标题
- [ ] 修改描述
- [ ] 修改优先级
- [ ] 保存更改
- [ ] 验证更改成功

### 6. 任务删除
- [ ] 右键点击任务
- [ ] 选择"删除"
- [ ] 确认删除
- [ ] 验证任务从列表消失

### 7. 摘要生成
- [ ] 点击"摘要"选项卡
- [ ] 选择"生成日摘要"
- [ ] 等待生成完成
- [ ] 验证摘要内容包含今天的任务
- [ ] 测试"生成周摘要"
- [ ] 测试"生成月摘要"

### 8. 通知系统
- [ ] 打开设置
- [ ] 启用"截止日期提醒"
- [ ] 创建一个 5 分钟后截止的任务
- [ ] 等待通知出现
- [ ] 验证通知内容正确

### 9. 设置同步
- [ ] 修改 AI 提供商为 Claude
- [ ] 保存设置
- [ ] 重启应用
- [ ] 验证设置保持

### 10. 数据持久化
- [ ] 创建 3 个任务
- [ ] 关闭应用
- [ ] 重新打开应用
- [ ] 验证 3 个任务仍然存在

## 边界情况测试

### 11. 空输入
- [ ] 尝试创建空标题任务
- [ ] 验证错误提示

### 12. 超长输入
- [ ] 输入 1000 字的标题
- [ ] 验证是否正常处理

### 13. 特殊字符
- [ ] 任务标题包含 emoji 😀
- [ ] 任务标题包含中英文混合
- [ ] 验证显示正常

### 14. 网络错误
- [ ] 断开网络
- [ ] 尝试 AI 解析
- [ ] 验证错误提示友好

## 性能测试

### 15. 大量任务
- [ ] 创建 100 个任务
- [ ] 测试列表滚动流畅度
- [ ] 测试搜索响应速度
- [ ] 测试筛选速度

## 测试结果

通过: ____ / 15
失败: ____ / 15
备注:
```

### 保存此清单

```bash
# 复制模板到桌面
cat > ~/Desktop/intento_manual_test.md << 'EOF'
[粘贴上面的清单内容]
EOF
```

## 3. 快速设置 E2E 自动化测试

### Step 1: 安装 Playwright（5 分钟）

```bash
# 在项目根目录
npm install -D @playwright/test
npx playwright install chromium
```

### Step 2: 创建配置文件

```bash
# 创建 Playwright 配置
cat > playwright.config.ts << 'EOF'
import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './e2e',
  timeout: 30000,
  retries: 1,
  use: {
    headless: false,
    viewport: { width: 1280, height: 720 },
    screenshot: 'only-on-failure',
    video: 'retain-on-failure',
  },
});
EOF
```

### Step 3: 创建第一个 E2E 测试

```bash
mkdir -p e2e
cat > e2e/basic-flow.spec.ts << 'EOF'
import { test, expect } from '@playwright/test';

test.describe('Basic Task Management', () => {
  test.beforeEach(async ({ page }) => {
    // 启动应用（需要先运行 npm run tauri:dev）
    await page.goto('http://localhost:1420');
    await page.waitForLoadState('networkidle');
  });

  test('create new task', async ({ page }) => {
    // 点击新建任务按钮
    await page.click('button:has-text("新建")');

    // 填写任务信息
    await page.fill('input[placeholder*="标题"]', '测试任务');
    await page.fill('textarea[placeholder*="描述"]', '这是一个测试任务');

    // 保存
    await page.click('button:has-text("保存")');

    // 验证任务出现在列表中
    await expect(page.locator('text=测试任务')).toBeVisible({ timeout: 5000 });
  });

  test('AI text parsing', async ({ page }) => {
    // 输入自然语言
    await page.fill('textarea[placeholder*="输入"]', '明天下午3点前完成报告');

    // 触发解析（假设是 Ctrl+Enter）
    await page.press('textarea', 'Control+Enter');

    // 等待解析完成
    await page.waitForSelector('text=解析完成', { timeout: 10000 });

    // 确认创建
    await page.click('button:has-text("确认")');

    // 验证任务创建
    await expect(page.locator('text=完成报告')).toBeVisible();
  });

  test('update task status', async ({ page }) => {
    // 假设已有任务
    const taskCard = page.locator('.task-card').first();
    await taskCard.click();

    // 更改状态
    await page.click('button:has-text("开始")');

    // 验证状态更新
    await expect(taskCard).toHaveClass(/status-doing/);
  });
});
EOF
```

### Step 4: 运行测试

```bash
# 启动应用（在一个终端窗口）
npm run tauri:dev

# 在另一个终端窗口运行测试
npx playwright test

# 查看测试报告
npx playwright show-report
```

### 更新 package.json

```json
{
  "scripts": {
    "test:e2e": "playwright test",
    "test:e2e:ui": "playwright test --ui",
    "test:e2e:debug": "playwright test --debug"
  }
}
```

## 4. 快速设置 React 组件测试

### Step 1: 安装依赖（5 分钟）

```bash
npm install -D vitest @testing-library/react @testing-library/user-event \
  @testing-library/jest-dom happy-dom @vitest/ui
```

### Step 2: 创建 Vitest 配置

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

### Step 3: 创建测试 setup 文件

```bash
mkdir -p src/test
cat > src/test/setup.ts << 'EOF'
import '@testing-library/jest-dom';
import { cleanup } from '@testing-library/react';
import { afterEach } from 'vitest';

// 每个测试后清理
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

### Step 4: 创建第一个组件测试

```bash
mkdir -p src/__tests__/components
cat > src/__tests__/components/TaskCard.test.tsx << 'EOF'
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { TaskCard } from '@/components/TaskCard';

describe('TaskCard', () => {
  const mockTask = {
    id: 1,
    title: '测试任务',
    description: '这是描述',
    status: 'todo' as const,
    priority: 'medium' as const,
    created_at: '2026-02-21T10:00:00Z',
  };

  it('renders task title and description', () => {
    render(<TaskCard task={mockTask} onSelect={() => {}} />);

    expect(screen.getByText('测试任务')).toBeInTheDocument();
    expect(screen.getByText('这是描述')).toBeInTheDocument();
  });

  it('calls onSelect when clicked', () => {
    const handleSelect = vi.fn();
    render(<TaskCard task={mockTask} onSelect={handleSelect} />);

    fireEvent.click(screen.getByText('测试任务'));
    expect(handleSelect).toHaveBeenCalledWith(mockTask);
  });

  it('displays correct status badge', () => {
    render(<TaskCard task={mockTask} onSelect={() => {}} />);

    const statusBadge = screen.getByText('待办');
    expect(statusBadge).toBeInTheDocument();
  });
});
EOF
```

### Step 5: 运行组件测试

```bash
# 运行测试
npm test

# 带 UI 界面运行
npm run test:ui

# 生成覆盖率报告
npm run test:coverage
```

### 更新 package.json

```json
{
  "scripts": {
    "test": "vitest",
    "test:ui": "vitest --ui",
    "test:coverage": "vitest --coverage"
  }
}
```

## 5. 测试脚本集合

创建一个便捷的测试脚本：

```bash
cat > test.sh << 'EOF'
#!/bin/bash

echo "🧪 Intento Test Suite Runner"
echo "=============================="

# 颜色定义
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 函数：运行 Rust 测试
run_rust_tests() {
    echo -e "\n${YELLOW}[1/4] Running Rust unit tests...${NC}"
    cd src-tauri
    if cargo test --lib; then
        echo -e "${GREEN}✓ Rust unit tests passed${NC}"
    else
        echo -e "${RED}✗ Rust unit tests failed${NC}"
        return 1
    fi
    cd ..
}

# 函数：运行 Rust 集成测试
run_rust_integration() {
    echo -e "\n${YELLOW}[2/4] Running Rust integration tests...${NC}"
    cd src-tauri
    if cargo test --tests; then
        echo -e "${GREEN}✓ Rust integration tests passed${NC}"
    else
        echo -e "${RED}✗ Rust integration tests failed${NC}"
        return 1
    fi
    cd ..
}

# 函数：运行 React 测试
run_react_tests() {
    echo -e "\n${YELLOW}[3/4] Running React component tests...${NC}"
    if npm test -- --run; then
        echo -e "${GREEN}✓ React tests passed${NC}"
    else
        echo -e "${RED}✗ React tests failed${NC}"
        return 1
    fi
}

# 函数：运行 E2E 测试
run_e2e_tests() {
    echo -e "\n${YELLOW}[4/4] Running E2E tests...${NC}"

    # 检查是否有应用在运行
    if ! curl -s http://localhost:1420 > /dev/null; then
        echo -e "${RED}✗ Application not running. Start with 'npm run tauri:dev'${NC}"
        return 1
    fi

    if npx playwright test; then
        echo -e "${GREEN}✓ E2E tests passed${NC}"
    else
        echo -e "${RED}✗ E2E tests failed${NC}"
        return 1
    fi
}

# 主流程
case "$1" in
    rust)
        run_rust_tests
        ;;
    integration)
        run_rust_integration
        ;;
    react)
        run_react_tests
        ;;
    e2e)
        run_e2e_tests
        ;;
    all)
        run_rust_tests && \
        run_rust_integration && \
        run_react_tests && \
        run_e2e_tests
        ;;
    *)
        echo "Usage: $0 {rust|integration|react|e2e|all}"
        echo ""
        echo "Examples:"
        echo "  ./test.sh rust        - Run Rust unit tests only"
        echo "  ./test.sh react       - Run React component tests only"
        echo "  ./test.sh all         - Run all tests"
        exit 1
        ;;
esac

echo -e "\n${GREEN}=============================="
echo -e "Tests completed!${NC}"
EOF

chmod +x test.sh
```

### 使用测试脚本

```bash
# 运行所有快速测试（不包括 E2E）
./test.sh all

# 只运行 Rust 测试
./test.sh rust

# 只运行 React 测试
./test.sh react

# 只运行 E2E 测试（需要先启动应用）
npm run tauri:dev &
./test.sh e2e
```

## 6. 推荐的测试工作流

### 日常开发流程

```bash
# 1. 启动应用开发模式
npm run tauri:dev

# 2. 在另一个终端，监听测试变化
npm test -- --watch

# 3. 修改代码后，测试自动运行

# 4. 提交前运行完整测试
./test.sh all
```

### 功能开发流程

```
1. 写测试用例（TDD）
   ↓
2. 运行测试（应该失败）
   ↓
3. 实现功能
   ↓
4. 运行测试（应该通过）
   ↓
5. 重构代码
   ↓
6. 再次运行测试（确保仍然通过）
   ↓
7. 提交代码
```

## 7. 常见问题

### Q: 测试运行很慢怎么办？

```bash
# 只运行特定测试文件
cargo test --test test_integration

# 只运行匹配的测试
npm test -- TaskCard

# 使用快速模式
npm test -- --run --reporter=dot
```

### Q: E2E 测试无法连接应用？

```bash
# 确保应用正在运行
npm run tauri:dev

# 检查端口
lsof -i :1420

# 等待应用完全启动后再运行测试
sleep 5 && npx playwright test
```

### Q: AI API 测试消耗配额？

```bash
# 大部分测试使用 mock，不调用真实 API
cargo test --tests

# 只在需要时运行真实 API 测试
cargo test -- --ignored
```

## 8. 下一步

现在你已经设置好基础的测试框架，可以：

1. **扩展测试覆盖** - 为更多组件添加测试
2. **设置 CI/CD** - 在 GitHub Actions 中自动运行测试
3. **添加性能测试** - 测试应用在大数据量下的表现
4. **编写测试文档** - 记录测试策略和最佳实践

参考 `docs/testing/TESTING_STRATEGY.md` 了解完整的测试策略。

---

**快速帮助:**
- 测试策略: `docs/testing/TESTING_STRATEGY.md`
- Rust 测试: `cd src-tauri && cargo test --help`
- React 测试: `npm test -- --help`
- E2E 测试: `npx playwright test --help`

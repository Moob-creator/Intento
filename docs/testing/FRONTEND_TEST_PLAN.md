# Intento 前端测试方案 - 概要设计

**版本**: v1.0
**日期**: 2026-02-21
**技术栈**: React 19 + TypeScript + Zustand + Tailwind CSS + Tauri

---

## 📋 目录

1. [测试架构概览](#1-测试架构概览)
2. [测试技术栈](#2-测试技术栈)
3. [测试范围](#3-测试范围)
4. [测试优先级](#4-测试优先级)
5. [实施计划](#5-实施计划)
6. [预期成果](#6-预期成果)

---

## 1. 测试架构概览

### 1.1 测试金字塔（前端部分）

```
                /\
               /  \
              / E2E \          组件集成测试 (5-10 个)
             /--------\        - 完整用户流程
            /          \       - 跨组件交互
           /  Integration\
          /--------------\
         /                \    组件单元测试 (30-50 个)
        /  Component Unit  \   - 渲染测试
       /--------------------\  - 用户交互
      /                      \ - Props 验证
     /   Unit Tests (Utils)   \
    /--------------------------\ 工具函数测试 (10-15 个)
                                 - 纯函数测试
                                 - 数据转换
```

### 1.2 测试层级

| 层级 | 测试内容 | 工具 | 数量目标 | 优先级 |
|------|---------|------|---------|--------|
| **工具函数** | 纯函数、数据转换 | Vitest | 10-15 | P0 |
| **组件单元** | 独立组件渲染和交互 | Vitest + RTL | 30-40 | P0 |
| **Hooks** | 自定义 Hooks | Vitest + RTL | 3-5 | P1 |
| **Store** | Zustand 状态管理 | Vitest | 5-8 | P0 |
| **组件集成** | 多组件协作 | Vitest + RTL | 5-10 | P1 |
| **E2E** | 完整用户流程 | Playwright | 3-5 | P2 |

---

## 2. 测试技术栈

### 2.1 核心测试库

```json
{
  "devDependencies": {
    // 测试框架
    "vitest": "^2.0.0",
    "@vitest/ui": "^2.0.0",

    // React 测试工具
    "@testing-library/react": "^16.0.0",
    "@testing-library/user-event": "^14.5.0",
    "@testing-library/jest-dom": "^6.4.0",

    // Mock 工具
    "happy-dom": "^15.0.0",
    "msw": "^2.0.0",

    // 覆盖率
    "@vitest/coverage-v8": "^2.0.0",

    // Tauri Mock
    "@tauri-apps/api-mock": "latest"
  }
}
```

### 2.2 测试配置文件

```typescript
// vitest.config.ts
import { defineConfig } from 'vitest/config';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  test: {
    environment: 'happy-dom',
    setupFiles: ['./src/test/setup.ts'],
    globals: true,
    css: true,
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      exclude: [
        'node_modules/',
        'src/test/',
        '**/*.d.ts',
        '**/*.config.*',
      ],
    },
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
});
```

---

## 3. 测试范围

### 3.1 前端代码结构分析

```
src/
├── components/ (28 个组件)
│   ├── TaskCard.tsx           ⭐ P0 - 核心展示组件
│   ├── TaskList.tsx           ⭐ P0 - 核心列表组件
│   ├── TaskDetailPanel.tsx    ⭐ P0 - 核心详情组件
│   ├── TaskConfirmDialog.tsx  ⭐ P0 - AI 确认对话框
│   ├── TaskOperationsConfirmDialog.tsx ⭐ P0 - 多操作确认
│   ├── CommandPalette.tsx     ⭐ P1 - 命令面板
│   ├── SummaryPanel.tsx       ⭐ P1 - 摘要面板
│   ├── SettingsPanel.tsx      ⭐ P1 - 设置面板
│   ├── CalendarView.tsx       ⭐ P1 - 日历视图
│   ├── Sidebar.tsx            ⭐ P2 - 侧边栏
│   └── ... (其他 UI 组件)     ⭐ P2
│
├── store/
│   └── taskStore.ts           ⭐ P0 - Zustand 状态管理
│
├── hooks/
│   ├── useKeyboardShortcuts.ts ⭐ P1 - 键盘快捷键
│   └── useToast.tsx           ⭐ P2 - Toast 通知
│
├── utils/
│   └── dateFormat.ts          ⭐ P0 - 日期格式化
│
└── types/
    ├── task.ts                ⭐ P0 - 类型定义
    ├── summary.ts
    └── notification.ts
```

### 3.2 测试覆盖目标

| 类别 | 文件数 | 测试目标 | 优先级 |
|------|--------|---------|--------|
| **工具函数** | 1 | 100% | P0 |
| **类型定义** | 3 | 类型安全 | P0 |
| **Store** | 1 | 90% | P0 |
| **核心组件** | 5 | 90% | P0 |
| **功能组件** | 8 | 70% | P1 |
| **UI 组件** | 14 | 50% | P2 |
| **Hooks** | 2 | 80% | P1 |

---

## 4. 测试优先级

### 4.1 P0 - 必须测试（核心功能）

#### A. 工具函数测试 (1 个文件)

**`utils/dateFormat.ts`**
```typescript
// 测试用例
describe('dateFormat', () => {
  test('formatDate - 格式化时间戳');
  test('formatRelativeTime - 相对时间');
  test('parseDeadline - 解析截止日期');
  test('isOverdue - 判断是否过期');
});
```

#### B. Store 测试 (1 个文件)

**`store/taskStore.ts`**
```typescript
describe('taskStore', () => {
  // 状态初始化
  test('初始状态正确');

  // CRUD 操作
  test('loadTasks - 加载任务列表');
  test('createTask - 创建任务');
  test('updateTask - 更新任务');
  test('deleteTask - 删除任务');
  test('selectTask - 选择任务');

  // 错误处理
  test('处理 API 错误');
  test('clearError - 清除错误');

  // Mock Tauri invoke
  test('正确调用 Tauri 命令');
});
```

#### C. 核心组件测试 (5 个组件)

**1. `TaskCard.tsx`** - 任务卡片
```typescript
describe('TaskCard', () => {
  test('渲染任务标题和描述');
  test('显示正确的状态徽章');
  test('显示正确的优先级图标');
  test('显示截止日期');
  test('显示标签列表');
  test('点击时触发 onSelect');
  test('右键显示上下文菜单');
  test('过期任务显示警告样式');
});
```

**2. `TaskList.tsx`** - 任务列表
```typescript
describe('TaskList', () => {
  test('空列表显示空状态');
  test('渲染任务列表');
  test('按状态筛选任务');
  test('搜索任务');
  test('加载状态显示 loading');
  test('滚动加载更多');
  test('选中任务高亮显示');
});
```

**3. `TaskDetailPanel.tsx`** - 任务详情
```typescript
describe('TaskDetailPanel', () => {
  test('显示完整任务信息');
  test('编辑模式切换');
  test('保存修改');
  test('取消编辑');
  test('删除任务确认');
  test('状态更新按钮');
  test('添加/删除标签');
  test('设置截止日期');
});
```

**4. `TaskConfirmDialog.tsx`** - AI 单任务确认
```typescript
describe('TaskConfirmDialog', () => {
  test('显示解析后的任务信息');
  test('显示 AI 置信度');
  test('编辑任务字段');
  test('确认创建任务');
  test('取消对话框');
  test('显示解析警告');
});
```

**5. `TaskOperationsConfirmDialog.tsx`** - AI 多操作确认
```typescript
describe('TaskOperationsConfirmDialog', () => {
  test('显示多个操作列表');
  test('操作类型图标和颜色');
  test('勾选/取消勾选操作');
  test('全选/全不选');
  test('执行选中操作');
  test('显示操作数量统计');
  test('显示图片预览');
});
```

### 4.2 P1 - 应该测试（功能组件）

#### A. 功能组件 (4 个)

1. **`CommandPalette.tsx`** - 命令面板
2. **`SummaryPanel.tsx`** - 摘要面板
3. **`SettingsPanel.tsx`** - 设置面板
4. **`CalendarView.tsx`** - 日历视图

#### B. Hooks (2 个)

1. **`useKeyboardShortcuts.ts`** - 键盘快捷键
2. **`useToast.tsx`** - Toast 通知

### 4.3 P2 - 可选测试（UI 组件）

基础 UI 组件（按钮、输入框、对话框等）

---

## 5. 实施计划

### 5.1 Phase 1: 基础设施搭建 (1-2 小时)

**目标**: 完成测试环境配置

```bash
# 1. 安装依赖
npm install -D vitest @vitest/ui @testing-library/react \
  @testing-library/user-event @testing-library/jest-dom \
  happy-dom @vitest/coverage-v8

# 2. 创建配置文件
touch vitest.config.ts
mkdir -p src/test

# 3. 创建测试 setup 文件
touch src/test/setup.ts
touch src/test/mocks/tauri.ts
touch src/test/helpers.tsx

# 4. 更新 package.json scripts
```

**交付物**:
- ✅ `vitest.config.ts`
- ✅ `src/test/setup.ts`
- ✅ `src/test/mocks/tauri.ts`
- ✅ `src/test/helpers.tsx`
- ✅ 测试可以运行

### 5.2 Phase 2: P0 测试实施 (4-6 小时)

**目标**: 完成核心功能测试

**Week 1: 工具函数 + Store (1-2 小时)**
```bash
src/utils/__tests__/dateFormat.test.ts
src/store/__tests__/taskStore.test.ts
```

**Week 2: 核心组件 (3-4 小时)**
```bash
src/components/__tests__/TaskCard.test.tsx
src/components/__tests__/TaskList.test.tsx
src/components/__tests__/TaskDetailPanel.test.tsx
src/components/__tests__/TaskConfirmDialog.test.tsx
src/components/__tests__/TaskOperationsConfirmDialog.test.tsx
```

**交付物**:
- ✅ 10-15 个工具函数测试
- ✅ 8-10 个 Store 测试
- ✅ 30-40 个组件测试
- ✅ 测试覆盖率 > 60%

### 5.3 Phase 3: P1 测试实施 (3-4 小时)

**目标**: 完成功能组件和 Hooks 测试

**功能组件**:
```bash
src/components/__tests__/CommandPalette.test.tsx
src/components/__tests__/SummaryPanel.test.tsx
src/components/__tests__/SettingsPanel.test.tsx
src/components/__tests__/CalendarView.test.tsx
```

**Hooks**:
```bash
src/hooks/__tests__/useKeyboardShortcuts.test.ts
src/hooks/__tests__/useToast.test.tsx
```

**交付物**:
- ✅ 20-30 个功能组件测试
- ✅ 5-10 个 Hooks 测试
- ✅ 测试覆盖率 > 70%

### 5.4 Phase 4: P2 测试实施 (2-3 小时)

**目标**: 完成 UI 组件测试

**基础 UI 组件**:
- Tooltip, Toast, ContextMenu
- DateTimePicker, CustomSelect
- StatusFilter, TaskSearchBar
- 等等

**交付物**:
- ✅ 10-20 个 UI 组件测试
- ✅ 测试覆盖率 > 80%

### 5.5 Phase 5: 集成测试和优化 (2-3 小时)

**集成测试场景**:
1. 创建任务完整流程
2. AI 解析任务流程
3. 任务状态更新流程
4. 摘要生成流程

**优化**:
- 重构重复的测试代码
- 提取通用 helpers
- 优化测试性能
- 添加测试文档

---

## 6. 预期成果

### 6.1 量化指标

| 指标 | Phase 1 | Phase 2 | Phase 3 | Phase 4 | Phase 5 |
|------|---------|---------|---------|---------|---------|
| **测试文件数** | 3 | 8 | 14 | 24 | 30 |
| **测试用例数** | 0 | 50 | 80 | 110 | 130 |
| **代码覆盖率** | 0% | 60% | 70% | 80% | 85% |
| **工时投入** | 2h | 8h | 12h | 15h | 18h |
| **测试执行时间** | - | <5s | <10s | <15s | <20s |

### 6.2 质量指标

✅ **测试稳定性**
- 无 flaky tests（不稳定的测试）
- 测试结果可重复

✅ **测试可读性**
- 清晰的测试名称
- AAA 模式（Arrange, Act, Assert）
- 充分的注释

✅ **测试覆盖**
- P0 功能 90% 覆盖
- P1 功能 70% 覆盖
- P2 功能 50% 覆盖

✅ **测试性能**
- 所有测试 < 20 秒
- 单个测试文件 < 2 秒

### 6.3 交付物清单

**配置文件**:
- [ ] `vitest.config.ts`
- [ ] `src/test/setup.ts`
- [ ] `src/test/mocks/tauri.ts`
- [ ] `src/test/helpers.tsx`

**测试文件 (P0)**:
- [ ] `src/utils/__tests__/dateFormat.test.ts`
- [ ] `src/store/__tests__/taskStore.test.ts`
- [ ] `src/components/__tests__/TaskCard.test.tsx`
- [ ] `src/components/__tests__/TaskList.test.tsx`
- [ ] `src/components/__tests__/TaskDetailPanel.test.tsx`
- [ ] `src/components/__tests__/TaskConfirmDialog.test.tsx`
- [ ] `src/components/__tests__/TaskOperationsConfirmDialog.test.tsx`

**测试文件 (P1)**:
- [ ] `src/components/__tests__/CommandPalette.test.tsx`
- [ ] `src/components/__tests__/SummaryPanel.test.tsx`
- [ ] `src/components/__tests__/SettingsPanel.test.tsx`
- [ ] `src/components/__tests__/CalendarView.test.tsx`
- [ ] `src/hooks/__tests__/useKeyboardShortcuts.test.ts`
- [ ] `src/hooks/__tests__/useToast.test.tsx`

**文档**:
- [ ] 前端测试指南
- [ ] 组件测试模板
- [ ] 常见问题解答

---

## 7. 技术要点

### 7.1 Mock Tauri API

```typescript
// src/test/mocks/tauri.ts
export const mockInvoke = vi.fn();

// Mock @tauri-apps/api/core
vi.mock('@tauri-apps/api/core', () => ({
  invoke: mockInvoke,
}));

// 使用示例
mockInvoke.mockResolvedValueOnce([
  { id: 1, title: 'Test Task', status: 'todo' }
]);
```

### 7.2 测试 Zustand Store

```typescript
// src/test/helpers.tsx
export const createMockStore = () => {
  return create<TaskStore>((set) => ({
    tasks: [],
    loadTasks: vi.fn(),
    // ... 其他 actions
  }));
};
```

### 7.3 测试异步操作

```typescript
import { waitFor } from '@testing-library/react';

test('加载任务', async () => {
  mockInvoke.mockResolvedValueOnce([...tasks]);

  const { result } = renderHook(() => useTaskStore());

  act(() => {
    result.current.loadTasks();
  });

  await waitFor(() => {
    expect(result.current.tasks).toHaveLength(3);
  });
});
```

### 7.4 测试用户交互

```typescript
import userEvent from '@testing-library/user-event';

test('点击任务卡片', async () => {
  const user = userEvent.setup();
  const onSelect = vi.fn();

  render(<TaskCard task={mockTask} onSelect={onSelect} />);

  await user.click(screen.getByText('Test Task'));

  expect(onSelect).toHaveBeenCalledWith(mockTask);
});
```

---

## 8. 风险和缓解

| 风险 | 影响 | 概率 | 缓解措施 |
|------|------|------|---------|
| Tauri API Mock 复杂 | 高 | 中 | 提前研究 Mock 方案 |
| 测试运行缓慢 | 中 | 中 | 优化测试配置，使用 happy-dom |
| 组件依赖过多 | 中 | 高 | 使用浅渲染，Mock 子组件 |
| 异步测试不稳定 | 高 | 低 | 使用 waitFor，避免 setTimeout |

---

## 9. 成功标准

✅ **完成标准**:
- [ ] 所有 P0 测试完成（50+ 测试用例）
- [ ] 测试覆盖率 > 60%
- [ ] 所有测试通过
- [ ] 测试执行时间 < 20 秒
- [ ] CI/CD 集成配置完成

✅ **质量标准**:
- [ ] 无 flaky tests
- [ ] 测试代码符合最佳实践
- [ ] 有完整的测试文档
- [ ] 团队成员理解测试策略

---

## 10. 下一步行动

### 立即开始 (Phase 1)

```bash
# 1. 安装依赖
npm install -D vitest @vitest/ui @testing-library/react \
  @testing-library/user-event @testing-library/jest-dom happy-dom

# 2. 创建配置文件
# 参考 docs/testing/QUICK_START.md

# 3. 运行第一个测试
npm test
```

### 渐进式实施

- **Week 1**: Phase 1 + Phase 2（工具函数 + Store）
- **Week 2**: Phase 2 继续（核心组件）
- **Week 3**: Phase 3（功能组件 + Hooks）
- **Week 4**: Phase 4 + Phase 5（UI 组件 + 优化）

---

## 附录

### A. 推荐资源

- [Vitest 官方文档](https://vitest.dev/)
- [React Testing Library 文档](https://testing-library.com/react)
- [Testing Tauri Applications](https://tauri.app/v1/guides/testing/)
- [Zustand Testing Guide](https://docs.pmnd.rs/zustand/guides/testing)

### B. 示例测试模板

参考 `docs/testing/QUICK_START.md` 第 4 节。

### C. CI/CD 配置

参考 `docs/testing/TESTING_STRATEGY.md` 第 5 节。

---

**方案制定人**: Development Team
**审核人**: ____________
**批准日期**: ____________

**版本历史**:
- v1.0 (2026-02-21) - 初始版本

#!/bin/bash

# Intento Test Runner
# 快速运行各类测试的便捷脚本

set -e

# 颜色定义
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 打印带颜色的消息
print_step() {
    echo -e "${BLUE}==>${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# 显示帮助信息
show_help() {
    cat << EOF
🧪 Intento Test Runner

用法: $0 [选项] [测试类型]

测试类型:
  rust          运行 Rust 单元测试
  rust-int      运行 Rust 集成测试（不调用 AI API）
  rust-ai       运行 Rust AI 集成测试（会调用真实 AI API）
  rust-all      运行所有 Rust 测试
  frontend      运行前端组件测试（需要先配置）
  e2e           运行端到端测试（需要先配置）
  all           运行所有快速测试（不包括 AI API 测试）

选项:
  -h, --help    显示帮助信息
  -v, --verbose 显示详细输出

示例:
  $0 rust           # 只运行 Rust 单元测试
  $0 rust-all       # 运行所有 Rust 测试
  $0 all            # 运行所有快速测试
  $0 -v rust        # 带详细输出运行 Rust 测试

环境要求:
  - Rust 测试: 需要 Cargo
  - AI 测试: 需要配置 .env 文件（OPENAI_API_KEY 或 ANTHROPIC_API_KEY）
  - 前端测试: 需要先运行 'npm install -D vitest @testing-library/react'
  - E2E 测试: 需要先运行 'npm install -D @playwright/test'
EOF
}

# 运行 Rust 单元测试
run_rust_unit_tests() {
    print_step "运行 Rust 单元测试..."
    cd src-tauri

    if [ "$VERBOSE" = true ]; then
        cargo test --lib -- --nocapture
    else
        cargo test --lib
    fi

    if [ $? -eq 0 ]; then
        print_success "Rust 单元测试通过"
    else
        print_error "Rust 单元测试失败"
        exit 1
    fi

    cd ..
}

# 运行 Rust 集成测试（不调用 AI）
run_rust_integration_tests() {
    print_step "运行 Rust 集成测试（不调用 AI API）..."
    cd src-tauri

    if [ "$VERBOSE" = true ]; then
        cargo test --tests -- --nocapture
    else
        cargo test --tests
    fi

    if [ $? -eq 0 ]; then
        print_success "Rust 集成测试通过"
    else
        print_error "Rust 集成测试失败"
        exit 1
    fi

    cd ..
}

# 运行 Rust AI 集成测试
run_rust_ai_tests() {
    print_step "运行 Rust AI 集成测试（会调用真实 AI API）..."

    # 检查 .env 文件
    if [ ! -f ".env" ]; then
        print_error "未找到 .env 文件，请先配置 API keys"
        print_warning "创建 .env 文件并添加以下内容:"
        echo "  OPENAI_API_KEY=sk-xxx"
        echo "  ANTHROPIC_API_KEY=sk-ant-xxx"
        exit 1
    fi

    cd src-tauri

    print_warning "此测试会调用真实 AI API，可能消耗配额"
    read -p "是否继续? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_warning "已取消测试"
        exit 0
    fi

    if [ "$VERBOSE" = true ]; then
        cargo test --test test_integration -- --ignored --nocapture
    else
        cargo test --test test_integration -- --ignored
    fi

    if [ $? -eq 0 ]; then
        print_success "Rust AI 集成测试通过"
    else
        print_error "Rust AI 集成测试失败"
        exit 1
    fi

    cd ..
}

# 运行所有 Rust 测试
run_all_rust_tests() {
    run_rust_unit_tests
    echo ""
    run_rust_integration_tests
}

# 运行前端测试
run_frontend_tests() {
    print_step "运行前端组件测试..."

    # 检查是否安装了 vitest
    if ! npm list vitest &> /dev/null; then
        print_error "未安装 vitest，请先运行:"
        echo "  npm install -D vitest @testing-library/react @testing-library/user-event @testing-library/jest-dom happy-dom"
        exit 1
    fi

    if [ "$VERBOSE" = true ]; then
        npm test -- --run --reporter=verbose
    else
        npm test -- --run
    fi

    if [ $? -eq 0 ]; then
        print_success "前端测试通过"
    else
        print_error "前端测试失败"
        exit 1
    fi
}

# 运行 E2E 测试
run_e2e_tests() {
    print_step "运行端到端测试..."

    # 检查是否安装了 playwright
    if ! npm list @playwright/test &> /dev/null; then
        print_error "未安装 Playwright，请先运行:"
        echo "  npm install -D @playwright/test"
        echo "  npx playwright install"
        exit 1
    fi

    # 检查应用是否在运行
    if ! curl -s http://localhost:1420 > /dev/null 2>&1; then
        print_error "应用未运行，请先在另一个终端运行:"
        echo "  npm run tauri:dev"
        exit 1
    fi

    if [ "$VERBOSE" = true ]; then
        npx playwright test --reporter=list
    else
        npx playwright test
    fi

    if [ $? -eq 0 ]; then
        print_success "E2E 测试通过"
    else
        print_error "E2E 测试失败"
        exit 1
    fi
}

# 运行所有快速测试
run_all_tests() {
    print_step "运行所有快速测试..."
    echo ""

    run_rust_unit_tests
    echo ""
    run_rust_integration_tests

    # 如果前端测试已配置，也运行
    if npm list vitest &> /dev/null; then
        echo ""
        run_frontend_tests
    else
        print_warning "跳过前端测试（未配置）"
    fi

    echo ""
    print_success "所有快速测试完成！"
}

# 显示测试统计
show_test_stats() {
    print_step "测试统计信息..."
    cd src-tauri

    echo ""
    echo "📊 Rust 测试文件:"
    echo "  单元测试: $(find src -name "tests.rs" | wc -l | tr -d ' ') 个文件"
    echo "  集成测试: $(find tests -name "*.rs" 2>/dev/null | wc -l | tr -d ' ') 个文件"

    echo ""
    echo "📊 测试用例统计:"
    cargo test --lib -- --list 2>/dev/null | grep ": test$" | wc -l | xargs echo "  单元测试:"
    cargo test --tests -- --list 2>/dev/null | grep ": test$" | wc -l | xargs echo "  集成测试:"

    cd ..

    if npm list vitest &> /dev/null; then
        echo ""
        echo "📊 前端测试:"
        find src -name "*.test.tsx" -o -name "*.test.ts" 2>/dev/null | wc -l | xargs echo "  测试文件:"
    fi
}

# 主程序
VERBOSE=false

# 解析选项
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            exit 0
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        rust)
            run_rust_unit_tests
            exit 0
            ;;
        rust-int)
            run_rust_integration_tests
            exit 0
            ;;
        rust-ai)
            run_rust_ai_tests
            exit 0
            ;;
        rust-all)
            run_all_rust_tests
            exit 0
            ;;
        frontend)
            run_frontend_tests
            exit 0
            ;;
        e2e)
            run_e2e_tests
            exit 0
            ;;
        all)
            run_all_tests
            exit 0
            ;;
        stats)
            show_test_stats
            exit 0
            ;;
        *)
            print_error "未知选项: $1"
            echo ""
            show_help
            exit 1
            ;;
    esac
done

# 如果没有参数，显示帮助
show_help

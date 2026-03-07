#!/bin/bash

# 意图识别测试脚本
# 用于测试AI是否能正确识别用户输入是否应该创建任务

set -e

# 脚本所在目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "========================================="
echo "  意图识别测试 - Intent Recognition Test"
echo "========================================="
echo ""

# 检查.env文件
if [ ! -f ".env" ]; then
    echo "❌ 错误: 未找到 .env 文件"
    echo ""
    echo "请执行以下步骤配置API密钥:"
    echo "1. cp .env.example .env"
    echo "2. 编辑 .env 文件，设置以下任一API密钥:"
    echo "   - OPENAI_API_KEY (OpenAI)"
    echo "   - ANTHROPIC_API_KEY (Anthropic Claude)"
    echo "   - MOONSHOT_API_KEY (Moonshot Kimi, 推荐)"
    echo ""
    echo "推荐使用 Kimi (国内访问更稳定):"
    echo "  AI_PROVIDER=kimi"
    echo "  MOONSHOT_API_KEY=your-key-here"
    echo ""
    exit 1
fi

# 检查是否有API密钥
source .env

HAS_API_KEY=false
if [ ! -z "$MOONSHOT_API_KEY" ]; then
    echo "✓ 检测到 Kimi API 密钥配置"
    HAS_API_KEY=true
elif [ ! -z "$OPENAI_API_KEY" ]; then
    echo "✓ 检测到 OpenAI API 密钥配置"
    HAS_API_KEY=true
elif [ ! -z "$ANTHROPIC_API_KEY" ]; then
    echo "✓ 检测到 Anthropic API 密钥配置"
    HAS_API_KEY=true
fi

if [ "$HAS_API_KEY" = false ]; then
    echo "❌ 错误: .env 文件中未配置有效的API密钥"
    echo ""
    echo "请在 .env 文件中设置以下任一 API 密钥:"
    echo "  - MOONSHOT_API_KEY (推荐，使用Kim i)"
    echo "  - OPENAI_API_KEY"
    echo "  - ANTHROPIC_API_KEY"
    echo ""
    exit 1
fi

echo ""
echo "开始运行测试..."
echo ""

# 运行测试
cd src-tauri
cargo test --test intent_recognition -- --ignored

echo ""
echo "========================================="
echo "测试完成！"
echo "结果已保存到: intent_test_results.json"
echo "========================================="

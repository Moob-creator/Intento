#!/bin/bash

# Intento 构建脚本
# 支持 macOS (DMG) 和 Windows (NSIS/MSI) 安装包

set -e

# 颜色输出
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔═══════════════════════════════════════╗${NC}"
echo -e "${BLUE}║    Intento 构建脚本                   ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════╝${NC}"
echo ""

# 检测操作系统
OS="$(uname -s)"
case "${OS}" in
    Linux*)     MACHINE=Linux;;
    Darwin*)    MACHINE=Mac;;
    CYGWIN*|MINGW*|MSYS*)    MACHINE=Windows;;
    *)          MACHINE="UNKNOWN:${OS}"
esac

echo -e "${GREEN}检测到操作系统: ${MACHINE}${NC}"
echo ""

# 解析参数
TARGET=""
CLEAN=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --target)
            TARGET="$2"
            shift 2
            ;;
        --clean)
            CLEAN=true
            shift
            ;;
        -h|--help)
            echo "用法: ./build.sh [选项]"
            echo ""
            echo "选项:"
            echo "  --target <platform>  指定构建目标 (mac, win, all)"
            echo "  --clean              构建前清理输出目录"
            echo "  -h, --help           显示帮助信息"
            echo ""
            echo "示例:"
            echo "  ./build.sh --target mac     # 仅构建 macOS 版本"
            echo "  ./build.sh --target win     # 仅构建 Windows 版本"
            echo "  ./build.sh --target all     # 构建所有平台"
            echo "  ./build.sh --clean          # 清理后构建当前平台"
            exit 0
            ;;
        *)
            echo -e "${RED}未知参数: $1${NC}"
            exit 1
            ;;
    esac
done

# 如果没有指定 target，根据当前操作系统设置默认值
if [ -z "$TARGET" ]; then
    case "${MACHINE}" in
        Mac)
            TARGET="mac"
            ;;
        Windows)
            TARGET="win"
            ;;
        *)
            echo -e "${YELLOW}在此操作系统上默认构建所有平台${NC}"
            TARGET="all"
            ;;
    esac
fi

echo -e "${GREEN}构建目标: ${TARGET}${NC}"
echo ""

# 清理输出目录
if [ "$CLEAN" = true ]; then
    echo -e "${YELLOW}清理构建输出...${NC}"
    rm -rf src-tauri/target/release/bundle
    rm -rf dist
    echo -e "${GREEN}✓ 清理完成${NC}"
    echo ""
fi

# 检查依赖
echo -e "${BLUE}检查依赖...${NC}"

if ! command -v node &> /dev/null; then
    echo -e "${RED}错误: 未找到 Node.js${NC}"
    exit 1
fi

if ! command -v npm &> /dev/null; then
    echo -e "${RED}错误: 未找到 npm${NC}"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}错误: 未找到 Rust/Cargo${NC}"
    exit 1
fi

echo -e "${GREEN}✓ 所有依赖已就绪${NC}"
echo ""

# 安装 npm 依赖
echo -e "${BLUE}检查 npm 依赖...${NC}"
if [ ! -d "node_modules" ]; then
    echo -e "${YELLOW}安装 npm 依赖...${NC}"
    npm install
else
    echo -e "${GREEN}✓ npm 依赖已安装${NC}"
fi
echo ""

# 构建前端
echo -e "${BLUE}构建前端资源...${NC}"
npm run build
echo -e "${GREEN}✓ 前端构建完成${NC}"
echo ""

# 构建函数
build_mac() {
    echo -e "${BLUE}════════════════════════════════════${NC}"
    echo -e "${BLUE}  构建 macOS 版本 (Universal)      ${NC}"
    echo -e "${BLUE}════════════════════════════════════${NC}"
    npm run tauri build -- --target universal-apple-darwin
    echo -e "${GREEN}✓ macOS 构建完成${NC}"
    echo ""
    echo -e "${GREEN}DMG 文件位置:${NC}"
    find src-tauri/target -name "*.dmg" -type f 2>/dev/null | while read file; do
        echo -e "  ${BLUE}→${NC} $file"
    done
    echo ""
}

build_win() {
    echo -e "${BLUE}════════════════════════════════════${NC}"
    echo -e "${BLUE}  构建 Windows 版本                 ${NC}"
    echo -e "${BLUE}════════════════════════════════════${NC}"
    npm run tauri build -- --target x86_64-pc-windows-msvc
    echo -e "${GREEN}✓ Windows 构建完成${NC}"
    echo ""
    echo -e "${GREEN}安装包位置:${NC}"
    find src-tauri/target -name "*.msi" -o -name "*.exe" -type f 2>/dev/null | grep -E "\.(msi|exe)$" | while read file; do
        echo -e "  ${BLUE}→${NC} $file"
    done
    echo ""
}

# 根据目标构建
case "${TARGET}" in
    mac)
        if [ "${MACHINE}" != "Mac" ]; then
            echo -e "${RED}警告: 在非 macOS 系统上构建 macOS 应用可能失败${NC}"
        fi
        build_mac
        ;;
    win)
        if [ "${MACHINE}" != "Windows" ] && [ "${MACHINE}" != "Linux" ]; then
            echo -e "${YELLOW}警告: 在 macOS 上交叉编译 Windows 应用需要额外配置${NC}"
        fi
        build_win
        ;;
    all)
        if [ "${MACHINE}" = "Mac" ]; then
            build_mac
        elif [ "${MACHINE}" = "Windows" ]; then
            build_win
        else
            # Linux 或其他系统，尝试构建两者
            echo -e "${YELLOW}尝试构建所有平台...${NC}"
            build_mac || echo -e "${YELLOW}macOS 构建失败（可能需要在 macOS 上构建）${NC}"
            build_win || echo -e "${YELLOW}Windows 构建失败${NC}"
        fi
        ;;
    *)
        echo -e "${RED}未知的构建目标: ${TARGET}${NC}"
        exit 1
        ;;
esac

echo -e "${GREEN}╔═══════════════════════════════════════╗${NC}"
echo -e "${GREEN}║    构建完成！                         ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}输出目录: src-tauri/target/release/bundle/${NC}"

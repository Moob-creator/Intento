# Intento 构建脚本 (Windows PowerShell)
# 支持 Windows (NSIS/MSI) 安装包

param(
    [string]$Target = "win",
    [switch]$Clean,
    [switch]$Help
)

$ErrorActionPreference = "Stop"

# 显示帮助
if ($Help) {
    Write-Host @"
用法: .\build.ps1 [-Target <platform>] [-Clean] [-Help]

参数:
  -Target <platform>  指定构建目标 (win, all)
  -Clean              构建前清理输出目录
  -Help               显示帮助信息

示例:
  .\build.ps1 -Target win     # 构建 Windows 版本
  .\build.ps1 -Clean          # 清理后构建
"@
    exit 0
}

Write-Host "╔═══════════════════════════════════════╗" -ForegroundColor Blue
Write-Host "║    Intento 构建脚本 (Windows)         ║" -ForegroundColor Blue
Write-Host "╚═══════════════════════════════════════╝" -ForegroundColor Blue
Write-Host ""

Write-Host "构建目标: $Target" -ForegroundColor Green
Write-Host ""

# 清理输出目录
if ($Clean) {
    Write-Host "清理构建输出..." -ForegroundColor Yellow
    if (Test-Path "src-tauri\target\release\bundle") {
        Remove-Item -Recurse -Force "src-tauri\target\release\bundle"
    }
    if (Test-Path "dist") {
        Remove-Item -Recurse -Force "dist"
    }
    Write-Host "✓ 清理完成" -ForegroundColor Green
    Write-Host ""
}

# 检查依赖
Write-Host "检查依赖..." -ForegroundColor Blue

try {
    $null = Get-Command node -ErrorAction Stop
    $null = Get-Command npm -ErrorAction Stop
    $null = Get-Command cargo -ErrorAction Stop
} catch {
    Write-Host "错误: 缺少必要的依赖 (Node.js, npm, Rust/Cargo)" -ForegroundColor Red
    exit 1
}

Write-Host "✓ 所有依赖已就绪" -ForegroundColor Green
Write-Host ""

# 安装 npm 依赖
Write-Host "检查 npm 依赖..." -ForegroundColor Blue
if (-not (Test-Path "node_modules")) {
    Write-Host "安装 npm 依赖..." -ForegroundColor Yellow
    npm install
    if ($LASTEXITCODE -ne 0) {
        Write-Host "错误: npm 安装失败" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "✓ npm 依赖已安装" -ForegroundColor Green
}
Write-Host ""

# 构建前端
Write-Host "构建前端资源..." -ForegroundColor Blue
npm run build
if ($LASTEXITCODE -ne 0) {
    Write-Host "错误: 前端构建失败" -ForegroundColor Red
    exit 1
}
Write-Host "✓ 前端构建完成" -ForegroundColor Green
Write-Host ""

# 构建 Windows 应用
Write-Host "════════════════════════════════════" -ForegroundColor Blue
Write-Host "  构建 Windows 版本                 " -ForegroundColor Blue
Write-Host "════════════════════════════════════" -ForegroundColor Blue

npm run tauri build -- --target x86_64-pc-windows-msvc

if ($LASTEXITCODE -ne 0) {
    Write-Host "错误: Tauri 构建失败" -ForegroundColor Red
    exit 1
}

Write-Host "✓ Windows 构建完成" -ForegroundColor Green
Write-Host ""

# 显示输出文件
Write-Host "安装包位置:" -ForegroundColor Green
Get-ChildItem -Path "src-tauri\target" -Recurse -Include "*.msi", "*.exe" -File -ErrorAction SilentlyContinue | ForEach-Object {
    if ($_.FullName -match "bundle") {
        Write-Host "  → $($_.FullName)" -ForegroundColor Blue
    }
}
Write-Host ""

Write-Host "╔═══════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║    构建完成！                         ║" -ForegroundColor Green
Write-Host "╚═══════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""
Write-Host "输出目录: src-tauri\target\release\bundle\" -ForegroundColor Blue

# Intento 构建指南

本文档说明如何构建 Intento 的 macOS 和 Windows 安装包。

## 环境要求

### 通用要求
- Node.js 18+ 和 npm
- Rust 1.70+ (通过 rustup 安装)
- Git

### macOS 构建要求
- macOS 10.13+
- Xcode Command Line Tools: `xcode-select --install`

### Windows 构建要求
- Windows 10+
- Visual Studio 2022 (或 Build Tools)
- WebView2 Runtime (通常已预装在 Windows 10/11)

## 快速开始

### 使用 npm scripts

```bash
# 构建当前平台
npm run tauri:build

# 构建前端 + Tauri 应用
npm run build:all

# 仅构建 macOS (Universal Binary)
npm run build:mac

# 仅构建 Windows
npm run build:win
```

### 使用构建脚本

#### macOS/Linux

```bash
# 赋予执行权限
chmod +x scripts/build.sh

# 构建当前平台
./scripts/build.sh

# 构建 macOS
./scripts/build.sh --target mac

# 构建 Windows (需要交叉编译工具链)
./scripts/build.sh --target win

# 清理后构建
./scripts/build.sh --clean

# 查看帮助
./scripts/build.sh --help
```

#### Windows (PowerShell)

```powershell
# 构建 Windows
.\scripts\build.ps1

# 清理后构建
.\scripts\build.ps1 -Clean

# 查看帮助
.\scripts\build.ps1 -Help
```

## 输出文件

构建完成后，安装包位于：

```
src-tauri/target/release/bundle/
├── dmg/                    # macOS DMG 安装包
│   └── Intento_0.1.0_universal.dmg
├── macos/                  # macOS .app 包
│   └── Intento.app
├── nsis/                   # Windows NSIS 安装程序
│   └── Intento_0.1.0_x64-setup.exe
└── msi/                    # Windows MSI 安装包
    └── Intento_0.1.0_x64_en-US.msi
```

## 构建配置

### 无签名构建（开发测试）

当前配置为**无签名构建**，适合开发和测试：

- macOS: `signingIdentity: null`
- Windows: `certificateThumbprint: null`

### macOS 注意事项

1. **Gatekeeper 警告**：未签名的 DMG 在首次打开时会显示安全警告
   - 解决方法：右键点击 → "打开" → 确认打开

2. **Universal Binary**：构建生成支持 Intel 和 Apple Silicon 的通用二进制文件

### Windows 注意事项

1. **SmartScreen 警告**：未签名的安装程序会触发 Windows Defender SmartScreen
   - 解决方法：点击 "更多信息" → "仍要运行"

2. **安装方式**：默认生成 NSIS 安装包
   - NSIS: 现代化安装向导，体积小，支持自动更新
   - 如需 MSI 格式，需要在 tauri.conf.json 中配置 `wix` 选项

## 代码签名（生产发布）

### macOS 代码签名

获得 Apple Developer 证书后：

1. 更新 `src-tauri/tauri.conf.json`:
```json
{
  "bundle": {
    "macOS": {
      "signingIdentity": "Developer ID Application: Your Name (TEAM_ID)",
      "entitlements": "entitlements.plist"
    }
  }
}
```

2. 创建 `src-tauri/entitlements.plist`

3. 公证应用：
```bash
xcrun notarytool submit Intento.dmg --apple-id "your@email.com" --password "app-specific-password" --team-id "TEAM_ID"
```

### Windows 代码签名

获得代码签名证书后：

1. 更新 `src-tauri/tauri.conf.json`:
```json
{
  "bundle": {
    "windows": {
      "certificateThumbprint": "YOUR_CERT_THUMBPRINT",
      "digestAlgorithm": "sha256",
      "timestampUrl": "http://timestamp.digicert.com"
    }
  }
}
```

2. 或使用 SignTool 手动签名：
```powershell
signtool sign /fd SHA256 /tr http://timestamp.digicert.com /td SHA256 /f certificate.pfx /p password Intento_setup.exe
```

### 自定义 NSIS 安装程序（可选）

如需自定义 NSIS 安装程序（语言、图标等），在 `tauri.conf.json` 中添加：

```json
{
  "bundle": {
    "windows": {
      "nsis": {
        "displayLanguageSelector": true
      }
    }
  }
}
```

注意：Tauri v2 的 NSIS 配置选项有限，大部分配置会自动处理。

## 故障排除

### macOS 构建失败

**问题**: `error: linker 'cc' not found`
```bash
xcode-select --install
```

**问题**: Architecture mismatch
```bash
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin
```

### Windows 构建失败

**问题**: MSVC not found
- 安装 Visual Studio 2022 或 Build Tools for Visual Studio
- 确保包含 "Desktop development with C++"

**问题**: WebView2 相关错误
```bash
# WebView2 会在构建时自动下载，确保网络通畅
```

### 通用问题

**前端构建失败**:
```bash
rm -rf node_modules dist
npm install
npm run build
```

**Rust 编译错误**:
```bash
cd src-tauri
cargo clean
cargo build --release
```

## CI/CD 集成

### GitHub Actions 示例

参考 `.github/workflows/build.yml` 配置多平台自动构建。

## 开发模式

运行开发服务器（带热重载）：

```bash
npm run tauri:dev
```

## 更多信息

- [Tauri 文档](https://tauri.app/)
- [构建配置参考](https://tauri.app/v1/api/config#buildconfig)
- [代码签名指南](https://tauri.app/v1/guides/distribution/sign-macos)

## 版本发布

更新版本号：

1. 修改 `package.json` 和 `src-tauri/Cargo.toml` 中的 `version`
2. 修改 `src-tauri/tauri.conf.json` 中的 `version`
3. 创建 git tag: `git tag v0.1.0`
4. 推送: `git push --tags`

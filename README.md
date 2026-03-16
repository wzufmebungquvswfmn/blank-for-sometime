# Meditation Timer 🧘

A simple meditation timer application that displays a blank white screen to help you focus and meditate.

[中文文档](#中文文档)

---

## Features

- **Fullscreen White Screen**: Immersive blank screen for distraction-free meditation
- **Countdown Timer**: Optional countdown display showing remaining time
- **Random Words Display**: Display custom words with random sizes, positions, and rotations
- **Configurable Duration**: Set meditation time in minutes
- **ESC to Exit**: Press ESC at any time to end the session early

## Display Options

1. **Show countdown timer**: Toggle the countdown display at the bottom of the screen
2. **Show random words**: Display custom words on the screen
   - **Animate words**: When enabled, words change positions every 3 seconds; when disabled, words stay in fixed positions for the entire session

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo

### Build from Source (Desktop – Windows / Linux / macOS)

```bash
# Clone the repository
git clone https://github.com/wzufmebungquvswfmn/blank-for-sometime.git
cd blank-for-sometime

# Build (GUI mode is the default)
cargo build --release

# Run
cargo run
```

The executable will be available at `target/release/blank_for_sometime.exe` (Windows) or `target/release/blank_for_sometime` (Linux/macOS).

### Build & Run on Android (Termux)

The app ships a **terminal UI (TUI) mode** designed for Termux and any environment without a graphical display.

```bash
# 1. Install Rust in Termux (if not already done)
pkg install rust

# 2. Clone the repository
pkg install git
git clone https://github.com/wzufmebungquvswfmn/blank-for-sometime.git
cd blank-for-sometime

# 3. Build the TUI binary (no display server required)
cargo build --release --no-default-features --features tui

# 4. Run
./target/release/blank_for_sometime
```

Or run directly without a separate build step:

```bash
cargo run --release --no-default-features --features tui
```

> **Note:** The `--no-default-features --features tui` flags switch from the GUI backend
> (`eframe`/`egui`) to the terminal backend (`crossterm`).  The GUI build requires a
> graphical display server (X11 / Wayland / Windows) and will not work in Termux.

## Usage

### Desktop (GUI mode)

1. Launch the application
2. Enter the meditation duration in minutes (default: 15)
3. Configure display options:
   - Check/uncheck "Show countdown timer"
   - Check/uncheck "Show random words"
   - If random words is enabled, enter your custom words (comma-separated)
   - If random words is enabled, optionally check "Animate words"
4. Click "Start Meditation"
5. Press ESC to exit early if needed

### Termux / Terminal (TUI mode)

1. Run the app (see build instructions above)
2. Answer the interactive prompts:
   - Duration in minutes
   - Show countdown? (Y/n)
   - Show random words? (y/N)
   - (if yes) Animate words? (y/N)
   - (if yes) Comma-separated word list
3. Press ENTER to start the meditation session
4. Press **ESC** or **q** to exit early

## Screenshots

### Input Screen
The main interface where you configure your meditation session.

### Meditation Screen
A fullscreen white display with optional countdown timer and random words.

### Completion Screen
Shown when your meditation session is complete.

## Technical Details

- Built with [Rust](https://www.rust-lang.org/)
- GUI framework: [eframe/egui](https://github.com/emilk/egui) (desktop, `gui` feature)
- TUI framework: [crossterm](https://github.com/crossterm-rs/crossterm) (terminal / Termux, `tui` feature)
- Random number generation: [rand](https://crates.io/crates/rand)

---

# 中文文档

一个简单的冥想计时器应用，显示空白白色屏幕帮助你专注和冥想。

## 功能特点

- **全屏白屏**: 沉浸式空白屏幕，无干扰冥想
- **倒计时器**: 可选的倒计时显示，展示剩余时间
- **随机词语显示**: 以随机大小、位置和旋转角度显示自定义词语
- **可配置时长**: 以分钟为单位设置冥想时间
- **ESC退出**: 随时按ESC键提前结束冥想

## 显示选项

1. **显示倒计时器**: 切换屏幕底部的倒计时显示
2. **显示随机词语**: 在屏幕上显示自定义词语
   - **动画词语**: 启用后，词语每3秒变换位置；禁用时，词语在整个冥想过程中保持固定位置

## 安装

### 桌面端（Windows / Linux / macOS）

```bash
# 克隆仓库
git clone https://github.com/wzufmebungquvswfmn/blank-for-sometime.git
cd blank-for-sometime

# 构建（默认为图形界面模式）
cargo build --release

# 运行
cargo run
```

可执行文件位于 `target/release/blank_for_sometime.exe` (Windows) 或 `target/release/blank_for_sometime` (Linux/macOS)。

### Android 手机 Termux 端

本项目内置了**终端界面（TUI）模式**，专为 Termux 及其他没有图形显示的环境设计。

```bash
# 1. 在 Termux 中安装 Rust（若尚未安装）
pkg install rust

# 2. 克隆仓库
pkg install git
git clone https://github.com/wzufmebungquvswfmn/blank-for-sometime.git
cd blank-for-sometime

# 3. 编译 TUI 版本（无需显示服务器）
cargo build --release --no-default-features --features tui

# 4. 运行
./target/release/blank_for_sometime
```

也可以直接运行（无需单独编译）：

```bash
cargo run --release --no-default-features --features tui
```

> **说明：** `--no-default-features --features tui` 会将 GUI 后端（`eframe`/`egui`，
> 需要图形显示服务器）切换为终端后端（`crossterm`）。在 Termux 中直接使用默认的
> GUI 模式会编译失败，请务必加上上述参数。

## 使用方法

### 桌面端（图形界面模式）

1. 启动应用
2. 输入冥想时长（分钟），默认为15分钟
3. 配置显示选项：
   - 勾选/取消勾选"显示倒计时器"
   - 勾选/取消勾选"显示随机词语"
   - 如果启用了随机词语，输入自定义词语（用逗号分隔）
   - 如果启用了随机词语，可选择勾选"动画词语"
4. 点击"开始冥想"
5. 如需提前退出，按ESC键

### Termux / 终端模式

1. 运行应用（见上方编译说明）
2. 按提示输入配置信息：
   - 冥想时长（分钟）
   - 是否显示倒计时（Y/n）
   - 是否显示随机词语（y/N）
   - （如启用词语）是否动画词语（y/N）
   - （如启用词语）逗号分隔的词语列表
3. 按 ENTER 开始冥想
4. 按 **ESC** 或 **q** 提前退出

## 技术细节

- 使用 [Rust](https://www.rust-lang.org/) 开发
- 图形界面框架: [eframe/egui](https://github.com/emilk/egui)（桌面端，`gui` 功能）
- 终端界面框架: [crossterm](https://github.com/crossterm-rs/crossterm)（终端/Termux，`tui` 功能）
- 随机数生成: [rand](https://crates.io/crates/rand)

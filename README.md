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

### Build from Source

```bash
# Clone the repository
git clone https://github.com/wzufmebungquvswfmn/blank-for-sometime.git
cd blank_for_sometime

# Build
cargo build --release

# Run
cargo run
```

The executable will be available at `target/release/blank_for_sometime.exe` (Windows) or `target/release/blank_for_sometime` (Linux/macOS).

## Usage

1. Launch the application
2. Enter the meditation duration in minutes (default: 15)
3. Configure display options:
   - Check/uncheck "Show countdown timer"
   - Check/uncheck "Show random words"
   - If random words is enabled, enter your custom words (comma-separated)
   - If random words is enabled, optionally check "Animate words"
4. Click "Start Meditation"
5. Press ESC to exit early if needed

## Screenshots

### Input Screen
The main interface where you configure your meditation session.

### Meditation Screen
A fullscreen white display with optional countdown timer and random words.

### Completion Screen
Shown when your meditation session is complete.

## Technical Details

- Built with [Rust](https://www.rust-lang.org/)
- GUI framework: [eframe/egui](https://github.com/emilk/egui)
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

### 前置要求

- Rust 1.70 或更高版本
- Cargo

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/wzufmebungquvswfmn/blank-for-sometime.git
cd blank_for_sometime

# 构建
cargo build --release

# 运行
cargo run
```

可执行文件位于 `target/release/blank_for_sometime.exe` (Windows) 或 `target/release/blank_for_sometime` (Linux/macOS)。

## 使用方法

1. 启动应用
2. 输入冥想时长（分钟），默认为15分钟
3. 配置显示选项：
   - 勾选/取消勾选"显示倒计时器"
   - 勾选/取消勾选"显示随机词语"
   - 如果启用了随机词语，输入自定义词语（用逗号分隔）
   - 如果启用了随机词语，可选择勾选"动画词语"
4. 点击"开始冥想"
5. 如需提前退出，按ESC键

## 技术细节

- 使用 [Rust](https://www.rust-lang.org/) 开发
- GUI框架: [eframe/egui](https://github.com/emilk/egui)
- 随机数生成: [rand](https://crates.io/crates/rand)

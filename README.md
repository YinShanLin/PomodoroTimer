# 番茄钟 - Tauri + Vue

一个跨平台桌面番茄钟应用，使用 Tauri (Rust) + Vue 3 构建。

## 技术栈

- **前端**: Vue 3 + TypeScript + Vite
- **后端**: Rust + Tauri 2
- **音效**: rodio
- **存储**: JSON 文件

## 功能

- 25 分钟专注 / 5 分钟短休息 / 15 分钟长休息
- 圆形进度环动画
- 系统托盘支持
- 自定义时长设置
- 轮次计数

## 开发环境要求

- Node.js >= 18
- Rust >= 1.70
- Tauri CLI

## 快速开始

```bash
# 安装前端依赖
npm install

# 开发模式（同时启动前端和 Tauri）
npm run tauri dev

# 构建生产版本
npm run tauri build
```

## 项目结构

```
PomodoroTimer/
├── src/                    # Vue 前端
│   ├── components/
│   │   ├── TimerRing.vue   # 倒计时圆环
│   │   ├── ControlPanel.vue # 控制按钮
│   │   └── SettingsPanel.vue # 设置面板
│   ├── composables/
│   │   └── useTimer.ts     # 计时器逻辑
│   ├── App.vue
│   └── main.ts
├── src-tauri/              # Rust 后端
│   ├── src/
│   │   └── main.rs         # Tauri 命令和计时器逻辑
│   ├── Cargo.toml
│   └── tauri.conf.json
├── package.json
└── vite.config.ts
```

## 快捷键

- `Space`: 开始/暂停计时器
- `S`: 跳过当前阶段
- `R`: 重置计时器

## 功能特性

- 圆形进度环动画
- 系统托盘（关闭窗口最小化到托盘）
- 系统通知（计时结束时提醒）
- 提示音（可关闭）
- 统计页面（今日/本周/历史记录）
- 设置面板（自定义时长）
- 窗口标题显示剩余时间
- 数据持久化（JSON 文件保存）

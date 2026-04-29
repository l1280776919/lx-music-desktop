# v-tauri

这是在不改动仓库现有 Electron 工程的前提下，新建出来的 Tauri + Rust “壳子工程”。

## 目录

- [web](file:///workspace/v-tauri/web)：Web 构建配置（把现有 `src/renderer` 打包成 Tauri 可加载的资源）
- [frontend-dist](file:///workspace/v-tauri/frontend-dist)：Web 构建输出目录（Tauri `distDir` 指向这里）
- [src-tauri](file:///workspace/v-tauri/src-tauri)：Tauri + Rust 后端

## 运行

```bash
cd v-tauri
npm install
npm run dev
```

## Web 构建

```bash
cd v-tauri
npm run build:web
```

## Linux 依赖

Linux 下 Tauri 1.x 默认使用 GTK/WebKit2GTK，需要系统依赖（不同发行版包名可能不同）。

Debian/Ubuntu 常用依赖示例：

```bash
sudo apt-get update
sudo apt-get install -y pkg-config libglib2.0-dev libwebkit2gtk-4.0-dev libgtk-3-dev libssl-dev build-essential
```

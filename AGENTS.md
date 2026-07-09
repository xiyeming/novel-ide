# Novel IDE

专业小说写作 IDE，基于 Tauri 2 + Vue 3 + SQLite 构建。

## 快速开始

```bash
# 前端开发服务器（端口 5273）
npm run dev

# 完整桌面应用（前端 + Rust 后端）
npm run tauri dev

# 生产构建
npm run build
```

## 常用命令

| 命令 | 用途 |
|------|------|
| `npm run dev` | 仅启动 Vite 开发服务器 |
| `npm run tauri dev` | 启动完整桌面应用 |
| `npm run build` | 类型检查 + 构建前端 |
| `npm run type-check` | 仅 TypeScript 检查 |
| `npm run lint` | ESLint 检查 |
| `npm run test` | Vitest 监听模式 |
| `npm run test:run` | Vitest 运行一次 |
| `cargo test` | 运行 Rust 测试 |
| `cargo check` | Rust 类型检查 |

## 架构

### 前端 (src/)
- `components/layout/` — IDE 布局（ActivityBar、Sidebar、StatusBar 等）
- `components/project/` — 项目管理（ProjectList、NewProject、EditProject）
- `components/ai/` — AI 工作室面板
- `stores/` — Pinia 状态管理（project、chapter、ai、workspace、shortcuts）
- `styles/` — CSS 变量和组件样式
- `composables/` — Vue 组合式函数（useTauriIPC）

### 后端 (src-tauri/)
- `src/commands/` — Tauri 命令处理器
- `src/db/` — SQLite 数据库（SQLx）
- `src/models/` — 数据模型
- `src/services/` — 业务逻辑
- `src/error.rs` — 错误类型

### 关键信息
- Tauri 2.11，插件：dialog、fs、shell、store、global-shortcut
- SQLite 通过 SQLx 0.9（runtime-tokio）
- Vue 3.5 + Pinia 3 + Monaco Editor
- Vite 开发服务器运行在端口 5273
- Rust edition 2024，rust-version 1.90

## 开发规范

- **交互语言**：所有 UI 文本必须使用中文（永中文）。组件名称使用英文。
- **代码注释**：使用中文编写代码注释。
- **输入文档**：输入的文档内容使用中文。
- **CSS**：使用 `src/styles/variables.css` 中的设计令牌。默认暗色主题。
- **IPC**：Tauri IPC 自动转换 snake_case（Rust）到 camelCase（JS）。前端发送 camelCase。
- **数据库**：SQLite 迁移文件在 `src-tauri/migrations/`
- **组件**：使用 `<script setup lang="ts">` 和 Vue 3 组合式 API

## 测试

- 前端：Vitest + jsdom 环境
- 后端：Rust 测试 `cargo test`
- 前端测试在 `src/test/`，Rust 测试内联在源文件中

## 注意事项

- `window.__TAURI__` 不可靠，不要用它检测 Tauri 环境。使用 `import("@tauri-apps/plugin-dialog")` 配合 try/catch。
- 原生数字输入控件不支持 CSS 样式。使用自定义 NumberInput 组件。
- 原生 `<select>` 下拉框不支持暗色主题。使用自定义 CustomSelect 组件。
- Tauri 对话框权限必须在 `src-tauri/capabilities/desktop.json` 中声明。

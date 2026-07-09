# Novel IDE UI Architecture 规范

> **版本：** 1.0.0
> **状态：** Draft
> **目标：** 建立专业级 Novel IDE 的 UI 架构规范，确保后续开发的一致性和可扩展性

---

## 目录

1. [Layout System（布局系统）](#1-layout-system)
2. [Design Token（设计令牌）](#2-design-token)
3. [Component Library（组件库）](#3-component-library)
4. [Interaction Specification（交互规范）](#4-interaction-specification)
5. [Workspace Presets（工作区预设）](#5-workspace-presets)
6. [Responsive Rules（响应式规则）](#6-responsive-rules)

---

## 1. Layout System

### 1.1 整体布局架构

Novel IDE 采用**三栏 + 可折叠面板**的 IDE 风格布局：

```
┌────────────────────────────────────────────────────────────────────────────┐
│ Title Bar (36px)                                                          │
├──────┬─────────────────────────────────────────────┬────────────────────────┤
│      │ Breadcrumb                                  │ AI Studio              │
│      ├─────────────────────────────────────────────┤  (380-460px)           │
│      │                                             │                        │
│ 56px │             Editor Workspace                │                        │
│      │             (自适应)                         │                        │
│      │                                             │                        │
│      │                                             │                        │
├──────┼─────────────────────────────────────────────┴────────────────────────┤
│ 280px│ Explorer Tree (280px)  │ Bottom Panel (150-400px, 默认收起)          │
├──────┴─────────────────────────────────────────────────────────────────────┤
│ Status Bar (24px)                                                         │
└───────────────────────────────────────────────────────────────────────────┘
```

### 1.2 面板尺寸规范

| 面板 | 默认宽度 | 最小宽度 | 最大宽度 | 可折叠 |
|------|----------|----------|----------|--------|
| Activity Bar | 56px | 56px | 56px | 否 |
| Sidebar | 280px | 200px | 500px | 是 |
| Editor | 自适应 | 400px | - | 否 |
| AI Studio | 420px | 320px | 600px | 是 |
| Bottom Panel | 200px | 100px | 400px | 是 |
| Inspector | 280px | 200px | 400px | 是 |

### 1.3 面板拖拽调整

- **拖拽手柄宽度：** 4px
- **拖拽手柄颜色：** 透明，hover 时显示 `--accent` 色
- **拖拽动画：** 无（实时响应）
- **最小/最大值限制：** 严格执行，超出时停止在边界

### 1.4 面板折叠/展开

- **折叠按钮位置：** 面板顶部右侧或拖拽手柄处
- **折叠动画：** 200ms ease-out
- **折叠状态：** 记住用户上次的面板宽度
- **快捷键：** 支持 `Ctrl+B` 折叠 Sidebar，`Ctrl+Shift+B` 折叠 AI Studio

---

## 2. Design Token

### 2.1 颜色系统

#### 2.1.1 Gray 色阶

```css
Gray-50:  #FAFBFC   /* 最浅，用于背景叠加 */
Gray-100: #F1F3F5
Gray-200: #E9ECEF
Gray-300: #DEE2E6
Gray-400: #CED4DA
Gray-500: #ADB5BD   /* 默认文本色 */
Gray-600: #868E96
Gray-700: #495057
Gray-800: #343A40
Gray-900: #212529   /* 最深，用于标题 */
```

#### 2.1.2 Blue 色阶（主色调）

```css
Blue-50:  #E7F5FF
Blue-100: #D0EBFF
Blue-200: #A5D8FF
Blue-300: #74C0FC
Blue-400: #4DABF7
Blue-500: #339AF0
Blue-600: #228BE6   /* 主按钮色 */
Blue-700: #1C7ED6
Blue-800: #1971C2
Blue-900: #1864AB
```

#### 2.1.3 语义色

```css
Success:  #40C057   /* 成功、完成 */
Warning:  #FAB005   /* 警告、待处理 */
Danger:   #FA5252   /* 错误、删除 */
Info:     #15AABF   /* 信息、提示 */
Purple:   #7950F2   /* AI、Workflow */
Teal:     #20C997   /* 知识库、RAG */
```

#### 2.1.4 Surface 色（暗色主题）

```css
Background:  #171C25   /* 最底层背景 */
Sidebar:     #1E2533   /* 侧边栏背景 */
Panel:       #252F40   /* 面板背景 */
Card:        #2A3548   /* 卡片背景 */
Hover:       #344055   /* 悬停状态 */
Active:      #3D4A60   /* 激活状态 */
Border:      #313C50   /* 边框色 */
Divider:     #2A3548   /* 分割线 */
Overlay:     rgba(0, 0, 0, 0.5)  /* 遮罩层 */
```

#### 2.1.5 Surface 色（亮色主题）

```css
Background:  #FFFFFF
Sidebar:     #F8F9FA
Panel:       #F1F3F5
Card:        #FFFFFF
Hover:       #E9ECEF
Active:      #DEE2E6
Border:      #DEE2E6
Divider:     #E9ECEF
Overlay:     rgba(0, 0, 0, 0.3)
```

### 2.2 字体系统

#### 2.2.1 字体族

```css
--font-sans: "Inter", "Segoe UI", "PingFang SC", "Microsoft YaHei", sans-serif;
--font-mono: "JetBrains Mono", "Fira Code", "Cascadia Code", monospace;
--font-serif: "Noto Serif SC", "Source Han Serif SC", serif;
```

#### 2.2.2 字体大小

```css
--font-size-xs:    11px   /* 辅助信息、徽章 */
--font-size-sm:    12px   /* 提示文本、树节点 */
--font-size-base:  13px   /* 标签、按钮 */
--font-size-md:    14px   /* 正文、输入框 */
--font-size-lg:    16px   /* 区块标题 */
--font-size-xl:    20px   /* 页面标题 */
--font-size-2xl:   24px   /* 大标题 */
--font-size-3xl:   32px   /* 展示标题 */
```

#### 2.2.3 字重

```css
--font-weight-normal:   400
--font-weight-medium:   500
--font-weight-semibold: 600
--font-weight-bold:     700
```

#### 2.2.4 行高

```css
--line-height-tight:   1.25   /* 标题 */
--line-height-normal:  1.5    /* 正文 */
--line-height-relaxed: 1.75   /* 长文本、小说内容 */
```

### 2.3 间距系统

```css
--spacing-0:   0px
--spacing-1:   4px     /* 紧凑间距 */
--spacing-2:   8px     /* 元素内间距 */
--spacing-3:   12px    /* 组件间距 */
--spacing-4:   16px    /* 区块间距 */
--spacing-5:   20px
--spacing-6:   24px    /* 大区块间距 */
--spacing-8:   32px    /* 页面边距 */
--spacing-10:  40px
--spacing-12:  48px
```

### 2.4 圆角系统

```css
--radius-none: 0px
--radius-sm:   4px     /* 小元素（按钮、输入框） */
--radius-md:   6px     /* 中等元素（卡片、对话框） */
--radius-lg:   8px     /* 大元素（面板、弹窗） */
--radius-xl:   12px    /* 特大元素（模态框） */
--radius-full: 9999px  /* 圆形（头像、徽章） */
```

### 2.5 阴影系统

```css
--shadow-xs:   0 1px 2px rgba(0, 0, 0, 0.05)
--shadow-sm:   0 1px 3px rgba(0, 0, 0, 0.1), 0 1px 2px rgba(0, 0, 0, 0.06)
--shadow-md:   0 4px 6px rgba(0, 0, 0, 0.1), 0 2px 4px rgba(0, 0, 0, 0.06)
--shadow-lg:   0 10px 15px rgba(0, 0, 0, 0.1), 0 4px 6px rgba(0, 0, 0, 0.05)
--shadow-xl:   0 20px 25px rgba(0, 0, 0, 0.1), 0 10px 10px rgba(0, 0, 0, 0.04)
```

### 2.6 动画系统

```css
--duration-fast:    100ms
--duration-normal:  200ms
--duration-slow:    300ms
--duration-slower:  500ms

--ease-in:          cubic-bezier(0.4, 0, 1, 1)
--ease-out:         cubic-bezier(0, 0, 0.2, 1)
--ease-in-out:      cubic-bezier(0.4, 0, 0.2, 1)
--ease-spring:      cubic-bezier(0.34, 1.56, 0.64, 1)
```

---

## 3. Component Library

### 3.1 Button（按钮）

#### 3.1.1 类型

| 类型 | 用途 | 样式 |
|------|------|------|
| Primary | 主要操作 | 渐变背景 `#4DABF7` → `#339AF0`，白色文字 |
| Secondary | 次要操作 | 透明背景，边框，主色文字 |
| Ghost | 轻量操作 | 透明背景，无边框，次要色文字 |
| Danger | 危险操作 | 红色背景，白色文字 |
| Icon | 图标按钮 | 仅图标，无边框 |

#### 3.1.2 尺寸

| 尺寸 | 高度 | 内边距 | 字体 |
|------|------|--------|------|
| Small | 28px | 8px 12px | 12px |
| Medium | 34px | 10px 16px | 13px |
| Large | 40px | 12px 20px | 14px |

#### 3.1.3 状态

```css
/* Default */
background: linear-gradient(135deg, #4DABF7, #339AF0)
border-radius: 6px
box-shadow: 0 2px 4px rgba(51, 154, 240, 0.3)

/* Hover */
background: linear-gradient(135deg, #339AF0, #228BE6)
box-shadow: 0 4px 8px rgba(51, 154, 240, 0.4)
transform: translateY(-1px)

/* Active */
background: linear-gradient(135deg, #228BE6, #1C7ED6)
box-shadow: 0 1px 2px rgba(51, 154, 240, 0.3)
transform: translateY(0)

/* Disabled */
opacity: 0.5
cursor: not-allowed
```

### 3.2 Input（输入框）

#### 3.2.1 类型

| 类型 | 用途 | 样式 |
|------|------|------|
| Text | 文本输入 | 单行 |
| Textarea | 多行输入 | 可调整高度 |
| Search | 搜索输入 | 带搜索图标 |
| Number | 数字输入 | 带增减按钮 |

#### 3.2.2 样式

```css
/* Default */
background: #232A36
border: 1px solid transparent
border-radius: 6px
padding: 8px 12px
color: var(--text-primary)
font-size: 14px

/* Hover */
background: #2A3442

/* Focus */
border-color: var(--accent)
box-shadow: 0 0 0 3px rgba(77, 171, 247, 0.2)
outline: none

/* Error */
border-color: var(--danger)
box-shadow: 0 0 0 3px rgba(250, 82, 82, 0.2)
```

### 3.3 Tree（树）

#### 3.3.1 结构

```
▼ 项目
    小说名称
    封面
    简介
▼ 章节
    第1章
    第2章
    第3章
▶ 世界观
▶ 角色
▶ 蓝图
▶ 知识库
```

#### 3.3.2 样式

```css
/* 节点 */
padding: 4px 8px
border-radius: 4px
cursor: pointer
transition: background 0.15s

/* Hover */
background: var(--hover)

/* Selected */
background: var(--accent)
color: var(--bg-primary)

/* 展开/折叠图标 */
width: 16px
height: 16px
transition: transform 0.15s

/* 缩进 */
每个层级: 16px
```

### 3.4 Panel（面板）

#### 3.4.1 类型

| 类型 | 用途 | 样式 |
|------|------|------|
| Sidebar | 左侧面板 | 280px 宽，可折叠 |
| Inspector | 右侧面板 | 280px 宽，可折叠 |
| Bottom | 底部面板 | 200px 高，可折叠 |
| Modal | 模态面板 | 居中，带遮罩 |

#### 3.4.2 样式

```css
/* 面板背景 */
background: var(--panel-bg)

/* 面板头部 */
height: 36px
padding: 0 12px
border-bottom: 1px solid var(--border)
font-size: 13px
font-weight: 500

/* 面板内容 */
padding: 12px
overflow-y: auto

/* 拖拽手柄 */
width: 4px
background: transparent
transition: background 0.15s

/* 拖拽手柄 Hover */
background: var(--accent)
```

### 3.5 Inspector（检查器）

#### 3.5.1 类型

| 类型 | 内容 |
|------|------|
| Chapter Inspector | 字数、标签、状态、引用角色、引用地点、引用世界观、章节摘要 |
| Character Inspector | 位置、状态、装备、目标、最近出场、AI 总结 |
| World Inspector | 名称、描述、规则、相关角色、相关章节 |
| Prompt Inspector | System、Workflow、Context、RAG、Prompt |
| Workflow Inspector | 当前阶段、进度、历史、配置 |

#### 3.5.2 样式

```css
/* Inspector 容器 */
background: var(--panel-bg)
border-left: 1px solid var(--border)

/* 属性行 */
display: flex
justify-content: space-between
padding: 8px 0
border-bottom: 1px solid var(--divider)

/* 属性名 */
font-size: 12px
color: var(--text-secondary)

/* 属性值 */
font-size: 13px
color: var(--text-primary)
```

### 3.6 AI Studio

#### 3.6.1 三层架构

```
AI Studio

━━━━━━━━━━━━━━━━━━━━

Task ▼

当前任务: 生成第35章
进度: ████████████ 42%
预计: 22 秒

━━━━━━━━━━━━━━━━━━━━

Workflow ▼

① 项目配置 ✓
② 故事前提 ✓
③ 世界观 ✓
④ 角色 ✓
⑤ 蓝图 ✓
⑥ 第12章 Writing...
⑦ 审稿 Waiting
⑧ 修稿 Waiting

━━━━━━━━━━━━━━━━━━━━

Context ▼

✔ 世界观
✔ 角色
✔ 已写章节
✔ 知识库
✔ Prompt

━━━━━━━━━━━━━━━━━━━━

Execution ▼

Agent: Writer Agent
MCP: 3 servers
Skills: 12 loaded
Hooks: 2 active

━━━━━━━━━━━━━━━━━━━━

Conversation ▼

[聊天记录]
```

### 3.7 Status Bar（状态栏）

#### 3.7.1 布局

```
┌────────────────────────────────────────────────────────────────────────────┐
│ 🤖 Claude 4 │ 📊 Context 92K │ 💬 Prompt 24K │ 🔢 Token 156K │ 📁 Novel │ 🌿 main │ 🔍 RAG ON │ 🔌 MCP 3 │ 🧩 Skills 12 │ 📝 Words 3264 │ 📄 Chars 4872 │ Ln 12, Col 5 │ UTF-8 │ CRLF │
└────────────────────────────────────────────────────────────────────────────┘
```

#### 3.7.2 区域

| 区域 | 内容 | 位置 |
|------|------|------|
| Left | Model, Context, Tokens | 左侧 |
| Center | Project, Branch, RAG, MCP, Skills | 中间 |
| Right | Words, Chars, Position, Encoding | 右侧 |

---

## 4. Interaction Specification

### 4.1 拖拽交互

#### 4.1.1 面板拖拽

- **触发方式：** 鼠标按下拖拽手柄
- **光标：** `col-resize`（垂直分割线）或 `row-resize`（水平分割线）
- **视觉反馈：** 拖拽时显示半透明预览线
- **边界限制：** 严格执行最小/最大尺寸
- **性能：** 实时更新，无延迟

#### 4.1.2 列表拖拽

- **触发方式：** 长按 500ms 或按住 `Alt` 键拖拽
- **视觉反馈：** 拖拽项半透明，目标位置显示插入线
- **动画：** 200ms ease-out 平滑移动
- **撤销：** 支持 `Ctrl+Z` 撤销拖拽操作

### 4.2 快捷键系统

#### 4.2.1 全局快捷键

| 快捷键 | 功能 |
|--------|------|
| `Ctrl+Shift+P` | Command Palette |
| `Ctrl+P` | 快速打开文件 |
| `Ctrl+B` | 折叠/展开 Sidebar |
| `Ctrl+Shift+B` | 折叠/展开 AI Studio |
| `Ctrl+J` | 折叠/展开 Bottom Panel |
| `Ctrl+K Ctrl+S` | 打开快捷键设置 |
| `F11` | 全屏 |
| `Ctrl+,` | 打开设置 |

#### 4.2.2 编辑器快捷键

| 快捷键 | 功能 |
|--------|------|
| `Ctrl+S` | 保存 |
| `Ctrl+Z` | 撤销 |
| `Ctrl+Shift+Z` | 重做 |
| `Ctrl+F` | 查找 |
| `Ctrl+H` | 替换 |
| `Ctrl+D` | 选择下一个相同内容 |
| `Ctrl+/` | 注释 |
| `Tab` | 缩进 |
| `Shift+Tab` | 取消缩进 |

#### 4.2.3 AI 快捷键

| 快捷键 | 功能 |
|--------|------|
| `Ctrl+Enter` | 发送 AI 消息 |
| `Ctrl+Shift+Enter` | AI 续写 |
| `Ctrl+Shift+R` | AI 改写 |
| `Ctrl+Shift+E` | AI 扩写 |
| `Ctrl+Shift+C` | AI 缩写 |

### 4.3 悬停交互

#### 4.3.1 Tooltip

- **延迟显示：** 500ms
- **延迟隐藏：** 100ms
- **位置：** 自动调整，避免超出视口
- **样式：** 圆角、阴影、半透明背景

#### 4.3.2 Hover 效果

```css
/* 按钮 Hover */
background: 深一度
box-shadow: 增强阴影
transform: translateY(-1px)

/* 列表项 Hover */
background: var(--hover)

/* 链接 Hover */
text-decoration: underline
color: var(--accent)
```

### 4.4 焦点管理

#### 4.4.1 焦点环

```css
/* 焦点环样式 */
outline: 2px solid var(--accent)
outline-offset: 2px
border-radius: 4px
```

#### 4.4.2 Tab 顺序

- 按照视觉顺序（从左到右，从上到下）
- 跳过不可见元素
- 支持 `Escape` 退出当前焦点区域

### 4.5 动画规范

#### 4.5.1 面板折叠/展开

```css
/* 折叠 */
transition: width 200ms ease-out, opacity 150ms ease-out
width: 0
opacity: 0

/* 展开 */
transition: width 200ms ease-out, opacity 150ms ease-out
width: 目标宽度
opacity: 1
```

#### 4.5.2 模态框

```css
/* 进入 */
animation: modal-in 200ms ease-out

/* 退出 */
animation: modal-out 150ms ease-in

@keyframes modal-in {
  from { opacity: 0; transform: scale(0.95); }
  to { opacity: 1; transform: scale(1); }
}

@keyframes modal-out {
  from { opacity: 1; transform: scale(1); }
  to { opacity: 0; transform: scale(0.95); }
}
```

#### 4.5.3 列表项

```css
/* 添加 */
animation: item-in 200ms ease-out

/* 删除 */
animation: item-out 150ms ease-in

@keyframes item-in {
  from { opacity: 0; transform: translateY(-10px); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes item-out {
  from { opacity: 1; transform: translateY(0); }
  to { opacity: 0; transform: translateY(10px); }
}
```

---

## 5. Workspace Presets

### 5.1 预设列表

| 预设 | 布局 | 用途 |
|------|------|------|
| Writing | Editor 80% + AI 20% | 日常写作 |
| Review | Diff 50% + Review 50% | 审稿校对 |
| Outline | Editor 60% + Inspector 40% | 大纲编辑 |
| Character | Editor 50% + Character Inspector 50% | 角色编辑 |
| World | Editor 50% + World Inspector 50% | 世界观编辑 |
| Research | Knowledge 40% + Editor 40% + AI 20% | 研究资料 |
| AI | AI Studio 60% + Editor 40% | AI 创作 |
| Zen | Editor 100%（全屏） | 专注写作 |

### 5.2 布局配置

```typescript
interface WorkspacePreset {
  id: string;
  name: string;
  icon: string;
  layout: {
    sidebar: { visible: boolean; width: number };
    editor: { visible: boolean; width: string };  // 百分比
    aiStudio: { visible: boolean; width: number };
    inspector: { visible: boolean; width: number; type: 'chapter' | 'character' | 'world' | 'prompt' | 'workflow' };
    bottomPanel: { visible: boolean; height: number };
  };
}
```

### 5.3 预设切换

- **触发方式：** 点击预设按钮或使用快捷键
- **动画：** 300ms ease-out 平滑过渡
- **记忆：** 记住用户上次使用的预设
- **自定义：** 支持用户创建自定义预设

---

## 6. Responsive Rules

### 6.1 断点

| 断点 | 宽度 | 布局调整 |
|------|------|----------|
| xs | < 768px | 仅显示 Editor，其他面板隐藏 |
| sm | 768-1024px | Sidebar 折叠，AI Studio 隐藏 |
| md | 1024-1280px | 正常布局 |
| lg | 1280-1440px | 正常布局 |
| xl | > 1440px | 可显示更多面板 |

### 6.2 最小宽度

```css
/* 应用最小宽度 */
min-width: 768px

/* 面板最小宽度 */
sidebar: 200px
aiStudio: 320px
inspector: 200px
```

### 6.3 面板折叠规则

| 条件 | 行为 |
|------|------|
| 宽度 < 1024px | 自动折叠 AI Studio |
| 宽度 < 768px | 自动折叠 Sidebar 和 AI Studio |
| 用户手动折叠 | 记住状态，不自动恢复 |

### 6.4 双屏支持

- **支持分屏显示**
- **支持将面板拖拽到另一个屏幕**
- **记住每个屏幕的布局配置**

---

## 附录

### A. 参考设计系统

- [VS Code](https://code.visualstudio.com/)
- [JetBrains IntelliJ](https://www.jetbrains.com/idea/)
- [Cursor](https://cursor.sh/)
- [Obsidian](https://obsidian.md/)
- [Linear](https://linear.app/)
- [Raycast](https://www.raycast.com/)

### B. 相关文档

- [Design Token JSON Schema](./design-tokens.json)
- [Component Storybook](./storybook/)
- [Interaction Test Cases](./interaction-tests/)
- [Workspace Preset Configurations](./workspace-presets.json)

---

**文档维护者：** Novel IDE Team
**最后更新：** 2026-07-09

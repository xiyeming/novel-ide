# Phase 14: UI Architecture Refactoring Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Refactor Novel IDE UI to professional IDE standards following the UI Architecture specification.

**Architecture:** Implement Layout System, Design Token, Component Library, Interaction Specification, Workspace Presets, and Responsive Rules as defined in the UI Architecture spec.

**Tech Stack:** Vue 3.5.39, TypeScript 5.7+, CSS Variables, Pinia 3.0.4

## Global Constraints
- Follow UI Architecture spec at `docs/superpowers/specs/2026-07-09-ui-architecture.md`
- All UI text in Chinese
- All component names in English
- Use existing Vue 3 + TypeScript patterns
- Maintain backward compatibility with existing stores

---

## Phase 1: Design Token System (Foundation)

### Task 1: Update CSS Variables

**Files:**
- Modify: `src/styles/variables.css`

**Steps:**

- [ ] **Step 1: Update color tokens**

```css
/* Gray 色阶 */
--gray-50: #FAFBFC;
--gray-100: #F1F3F5;
--gray-200: #E9ECEF;
--gray-300: #DEE2E6;
--gray-400: #CED4DA;
--gray-500: #ADB5BD;
--gray-600: #868E96;
--gray-700: #495057;
--gray-800: #343A40;
--gray-900: #212529;

/* Blue 色阶 */
--blue-50: #E7F5FF;
--blue-100: #D0EBFF;
--blue-200: #A5D8FF;
--blue-300: #74C0FC;
--blue-400: #4DABF7;
--blue-500: #339AF0;
--blue-600: #228BE6;
--blue-700: #1C7ED6;
--blue-800: #1971C2;
--blue-900: #1864AB;

/* 语义色 */
--success: #40C057;
--warning: #FAB005;
--danger: #FA5252;
--info: #15AABF;
--purple: #7950F2;
--teal: #20C997;
```

- [ ] **Step 2: Update surface tokens**

```css
/* 暗色主题 Surface */
--bg-background: #171C25;
--bg-sidebar: #1E2533;
--bg-panel: #252F40;
--bg-card: #2A3548;
--bg-hover: #344055;
--bg-active: #3D4A60;
--border-default: #313C50;
--border-divider: #2A3548;
--overlay: rgba(0, 0, 0, 0.5);
```

- [ ] **Step 3: Update font tokens**

```css
/* 字体族 */
--font-sans: "Inter", "Segoe UI", "PingFang SC", "Microsoft YaHei", sans-serif;
--font-mono: "JetBrains Mono", "Fira Code", "Cascadia Code", monospace;
--font-serif: "Noto Serif SC", "Source Han Serif SC", serif;

/* 字体大小 */
--font-size-xs: 11px;
--font-size-sm: 12px;
--font-size-base: 13px;
--font-size-md: 14px;
--font-size-lg: 16px;
--font-size-xl: 20px;
--font-size-2xl: 24px;
--font-size-3xl: 32px;

/* 字重 */
--font-weight-normal: 400;
--font-weight-medium: 500;
--font-weight-semibold: 600;
--font-weight-bold: 700;

/* 行高 */
--line-height-tight: 1.25;
--line-height-normal: 1.5;
--line-height-relaxed: 1.75;
```

- [ ] **Step 4: Update spacing tokens**

```css
--spacing-0: 0px;
--spacing-1: 4px;
--spacing-2: 8px;
--spacing-3: 12px;
--spacing-4: 16px;
--spacing-5: 20px;
--spacing-6: 24px;
--spacing-8: 32px;
--spacing-10: 40px;
--spacing-12: 48px;
```

- [ ] **Step 5: Update radius tokens**

```css
--radius-none: 0px;
--radius-sm: 4px;
--radius-md: 6px;
--radius-lg: 8px;
--radius-xl: 12px;
--radius-full: 9999px;
```

- [ ] **Step 6: Update shadow tokens**

```css
--shadow-xs: 0 1px 2px rgba(0, 0, 0, 0.05);
--shadow-sm: 0 1px 3px rgba(0, 0, 0, 0.1), 0 1px 2px rgba(0, 0, 0, 0.06);
--shadow-md: 0 4px 6px rgba(0, 0, 0, 0.1), 0 2px 4px rgba(0, 0, 0, 0.06);
--shadow-lg: 0 10px 15px rgba(0, 0, 0, 0.1), 0 4px 6px rgba(0, 0, 0, 0.05);
--shadow-xl: 0 20px 25px rgba(0, 0, 0, 0.1), 0 10px 10px rgba(0, 0, 0, 0.04);
```

- [ ] **Step 7: Update animation tokens**

```css
--duration-fast: 100ms;
--duration-normal: 200ms;
--duration-slow: 300ms;
--duration-slower: 500ms;

--ease-in: cubic-bezier(0.4, 0, 1, 1);
--ease-out: cubic-bezier(0, 0, 0.2, 1);
--ease-in-out: cubic-bezier(0.4, 0, 0.2, 1);
--ease-spring: cubic-bezier(0.34, 1.56, 0.64, 1);
```

- [ ] **Step 8: Commit**

```bash
git add src/styles/variables.css
git commit -m "style: update design tokens according to UI Architecture spec"
```

**Verification:** Visual check - colors, fonts, spacing should match spec

---

### Task 2: Create Component Styles

**Files:**
- Create: `src/styles/components/button.css`
- Create: `src/styles/components/input.css`
- Create: `src/styles/components/tree.css`
- Create: `src/styles/components/panel.css`
- Create: `src/styles/components/index.css`

**Steps:**

- [ ] **Step 1: Create button.css**

```css
/* Button Component Styles */
.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-2);
  border: none;
  border-radius: var(--radius-md);
  font-family: var(--font-sans);
  font-weight: var(--font-weight-medium);
  cursor: pointer;
  transition: all var(--duration-normal) var(--ease-out);
}

/* Sizes */
.btn-sm {
  height: 28px;
  padding: var(--spacing-2) var(--spacing-3);
  font-size: var(--font-size-sm);
}

.btn-md {
  height: 34px;
  padding: var(--spacing-2) var(--spacing-4);
  font-size: var(--font-size-base);
}

.btn-lg {
  height: 40px;
  padding: var(--spacing-3) var(--spacing-5);
  font-size: var(--font-size-md);
}

/* Types */
.btn-primary {
  background: linear-gradient(135deg, var(--blue-400), var(--blue-500));
  color: white;
  box-shadow: 0 2px 4px rgba(51, 154, 240, 0.3);
}

.btn-primary:hover {
  background: linear-gradient(135deg, var(--blue-500), var(--blue-600));
  box-shadow: 0 4px 8px rgba(51, 154, 240, 0.4);
  transform: translateY(-1px);
}

.btn-primary:active {
  background: linear-gradient(135deg, var(--blue-600), var(--blue-700));
  box-shadow: 0 1px 2px rgba(51, 154, 240, 0.3);
  transform: translateY(0);
}

.btn-secondary {
  background: transparent;
  border: 1px solid var(--border-default);
  color: var(--blue-500);
}

.btn-secondary:hover {
  background: var(--bg-hover);
  border-color: var(--blue-500);
}

.btn-ghost {
  background: transparent;
  color: var(--gray-500);
}

.btn-ghost:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.btn-danger {
  background: var(--danger);
  color: white;
}

.btn-danger:hover {
  background: #E03131;
}

/* Disabled */
.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none;
}
```

- [ ] **Step 2: Create input.css**

```css
/* Input Component Styles */
.input {
  width: 100%;
  background: #232A36;
  border: 1px solid transparent;
  border-radius: var(--radius-md);
  padding: var(--spacing-2) var(--spacing-3);
  color: var(--text-primary);
  font-family: var(--font-sans);
  font-size: var(--font-size-md);
  transition: all var(--duration-normal) var(--ease-out);
}

.input:hover {
  background: #2A3442;
}

.input:focus {
  border-color: var(--blue-500);
  box-shadow: 0 0 0 3px rgba(77, 171, 247, 0.2);
  outline: none;
}

.input::placeholder {
  color: var(--gray-500);
}

/* Textarea */
.textarea {
  min-height: 80px;
  resize: vertical;
}

/* Search */
.input-search {
  padding-left: var(--spacing-8);
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='%236c7086' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Ccircle cx='11' cy='11' r='8'%3E%3C/circle%3E%3Cline x1='21' y1='21' x2='16.65' y2='16.65'%3E%3C/line%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: var(--spacing-3) center;
}

/* Error */
.input-error {
  border-color: var(--danger);
  box-shadow: 0 0 0 3px rgba(250, 82, 82, 0.2);
}
```

- [ ] **Step 3: Create tree.css**

```css
/* Tree Component Styles */
.tree {
  font-family: var(--font-sans);
  font-size: var(--font-size-sm);
}

.tree-node {
  display: flex;
  align-items: center;
  gap: var(--spacing-1);
  padding: var(--spacing-1) var(--spacing-2);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-out);
  user-select: none;
}

.tree-node:hover {
  background: var(--bg-hover);
}

.tree-node.selected {
  background: var(--blue-500);
  color: white;
}

.tree-node-icon {
  width: 16px;
  height: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: transform var(--duration-fast) var(--ease-out);
}

.tree-node-icon.expanded {
  transform: rotate(90deg);
}

.tree-node-label {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.tree-children {
  padding-left: var(--spacing-4);
}
```

- [ ] **Step 4: Create panel.css**

```css
/* Panel Component Styles */
.panel {
  display: flex;
  flex-direction: column;
  background: var(--bg-panel);
  overflow: hidden;
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 36px;
  padding: 0 var(--spacing-3);
  border-bottom: 1px solid var(--border-default);
  font-size: var(--font-size-base);
  font-weight: var(--font-weight-medium);
  color: var(--text-primary);
}

.panel-content {
  flex: 1;
  padding: var(--spacing-3);
  overflow-y: auto;
}

.panel-resize-handle {
  width: 4px;
  background: transparent;
  cursor: col-resize;
  transition: background var(--duration-fast) var(--ease-out);
}

.panel-resize-handle:hover {
  background: var(--blue-500);
}

.panel-resize-handle.horizontal {
  width: 100%;
  height: 4px;
  cursor: row-resize;
}

/* Panel States */
.panel.collapsed {
  width: 0;
  opacity: 0;
  transition: all var(--duration-normal) var(--ease-out);
}

.panel.expanded {
  opacity: 1;
  transition: all var(--duration-normal) var(--ease-out);
}
```

- [ ] **Step 5: Create index.css**

```css
@import './button.css';
@import './input.css';
@import './tree.css';
@import './panel.css';
```

- [ ] **Step 6: Update main.ts to import component styles**

```typescript
// src/main.ts
import './styles/components/index.css';
```

- [ ] **Step 7: Commit**

```bash
git add src/styles/components/
git commit -m "style: add component styles according to UI Architecture spec"
```

**Verification:** Components should render with new styles

---

## Phase 2: Layout System

### Task 3: Refactor IDELayout

**Files:**
- Modify: `src/components/layout/IDELayout.vue`

**Steps:**

- [ ] **Step 1: Update layout structure**

```vue
<template>
  <div class="ide-layout">
    <TitleBar @back="emit('back')" />
    <div class="ide-main">
      <ActivityBar
        :activeView="activeView"
        @select="activeView = $event"
      />
      <div class="sidebar" :style="{ width: sidebarVisible ? `${sidebarWidth}px` : '0' }">
        <Sidebar :view="activeView" @openChapter="handleOpenChapter" />
      </div>
      <div
        v-if="sidebarVisible"
        class="resize-handle vertical"
        @mousedown="onMouseDown('sidebar', $event)"
      />
      <div class="editor-area">
        <Breadcrumb :items="breadcrumbItems" />
        <div class="editor-workspace" :style="{ height: bottomPanelVisible ? `calc(100% - ${bottomPanelHeight}px)` : '100%' }">
          <EditorPanel ref="editorPanelRef" />
        </div>
        <div
          v-if="bottomPanelVisible"
          class="resize-handle horizontal"
          @mousedown="onMouseDown('bottom', $event)"
        />
        <BottomPanel v-if="bottomPanelVisible" :height="bottomPanelHeight" />
      </div>
      <div
        v-if="aiStudioVisible"
        class="resize-handle vertical"
        @mousedown="onMouseDown('ai', $event)"
      />
      <div class="ai-studio" :style="{ width: aiStudioVisible ? `${aiStudioWidth}px` : '0' }">
        <AIStudio />
      </div>
    </div>
    <StatusBar />
  </div>
</template>
```

- [ ] **Step 2: Add new state**

```typescript
const activeView = ref<'explorer' | 'search' | 'ai' | 'plugins' | 'settings'>('explorer');
const sidebarVisible = ref(true);
const aiStudioVisible = ref(true);
const bottomPanelVisible = ref(false);
const inspectorVisible = ref(false);
const inspectorType = ref<'chapter' | 'character' | 'world' | 'prompt' | 'workflow'>('chapter');
```

- [ ] **Step 3: Update styles**

```css
.ide-layout {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  background: var(--bg-background);
}

.ide-main {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.sidebar {
  flex-shrink: 0;
  overflow: hidden;
  transition: width var(--duration-normal) var(--ease-out);
}

.editor-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.ai-studio {
  flex-shrink: 0;
  overflow: hidden;
  transition: width var(--duration-normal) var(--ease-out);
}

.resize-handle {
  width: 4px;
  cursor: col-resize;
  background: transparent;
  transition: background var(--duration-fast) var(--ease-out);
}

.resize-handle:hover {
  background: var(--blue-500);
}

.resize-handle.horizontal {
  width: 100%;
  height: 4px;
  cursor: row-resize;
}
```

- [ ] **Step 4: Commit**

```bash
git add src/components/layout/IDELayout.vue
git commit -m "refactor: update IDELayout according to UI Architecture spec"
```

**Verification:** Layout should render with new structure

---

### Task 4: Create ActivityBar Component

**Files:**
- Create: `src/components/layout/ActivityBar.vue`

**Steps:**

- [ ] **Step 1: Create ActivityBar.vue**

```vue
<script setup lang="ts">
defineProps<{
  activeView: string;
}>();

const emit = defineEmits<{
  select: [view: string];
}>();

const views = [
  { id: 'explorer', icon: '📁', label: 'Explorer' },
  { id: 'search', icon: '🔍', label: 'Search' },
  { id: 'ai', icon: '🤖', label: 'AI' },
  { id: 'plugins', icon: '🧩', label: 'Plugins' },
  { id: 'settings', icon: '⚙️', label: 'Settings' },
];
</script>

<template>
  <div class="activity-bar">
    <button
      v-for="view in views"
      :key="view.id"
      :class="['activity-item', { active: activeView === view.id }]"
      @click="emit('select', view.id)"
      :title="view.label"
    >
      <span class="activity-icon">{{ view.icon }}</span>
    </button>
  </div>
</template>

<style scoped>
.activity-bar {
  display: flex;
  flex-direction: column;
  width: 56px;
  background: var(--bg-sidebar);
  border-right: 1px solid var(--border-default);
}

.activity-item {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 56px;
  background: none;
  border: none;
  border-left: 2px solid transparent;
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-out);
}

.activity-item:hover {
  background: var(--bg-hover);
}

.activity-item.active {
  border-left-color: var(--blue-500);
  background: var(--bg-panel);
}

.activity-icon {
  font-size: 20px;
}
</style>
```

- [ ] **Step 2: Commit**

```bash
git add src/components/layout/ActivityBar.vue
git commit -m "feat: create ActivityBar component"
```

**Verification:** ActivityBar should render with icons

---

### Task 5: Create Breadcrumb Component

**Files:**
- Create: `src/components/layout/Breadcrumb.vue`

**Steps:**

- [ ] **Step 1: Create Breadcrumb.vue**

```vue
<script setup lang="ts">
defineProps<{
  items: Array<{ label: string; path?: string }>;
}>();
</script>

<template>
  <div class="breadcrumb">
    <span
      v-for="(item, index) in items"
      :key="index"
      class="breadcrumb-item"
    >
      <span v-if="index > 0" class="breadcrumb-separator">/</span>
      <span class="breadcrumb-label">{{ item.label }}</span>
    </span>
  </div>
</template>

<style scoped>
.breadcrumb {
  display: flex;
  align-items: center;
  height: 24px;
  padding: 0 var(--spacing-3);
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-default);
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.breadcrumb-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-2);
}

.breadcrumb-separator {
  color: var(--gray-500);
}

.breadcrumb-label {
  cursor: pointer;
}

.breadcrumb-label:hover {
  color: var(--text-primary);
}
</style>
```

- [ ] **Step 2: Commit**

```bash
git add src/components/layout/Breadcrumb.vue
git commit -m "feat: create Breadcrumb component"
```

**Verification:** Breadcrumb should render with items

---

### Task 6: Create StatusBar Component

**Files:**
- Create: `src/components/layout/StatusBar.vue`

**Steps:**

- [ ] **Step 1: Create StatusBar.vue**

```vue
<script setup lang="ts">
import { ref, computed } from "vue";
import { useProjectStore } from "../../stores/project";
import { useChapterStore } from "../../stores/chapter";
import { useAIStore } from "../../stores/ai";

const projectStore = useProjectStore();
const chapterStore = useChapterStore();
const aiStore = useAIStore();

const model = computed(() => aiStore.selectedModel);
const contextTokens = ref(92000);
const promptTokens = ref(24000);
const totalTokens = ref(156000);
const branch = ref("main");
const ragEnabled = ref(true);
const mcpServers = ref(3);
const skillsLoaded = ref(12);
</script>

<template>
  <div class="status-bar">
    <div class="status-left">
      <span class="status-item">🤖 {{ model }}</span>
      <span class="status-item">📊 Context {{ Math.round(contextTokens / 1000) }}K</span>
      <span class="status-item">💬 Prompt {{ Math.round(promptTokens / 1000) }}K</span>
      <span class="status-item">🔢 Token {{ Math.round(totalTokens / 1000) }}K</span>
    </div>
    <div class="status-center">
      <span class="status-item">📁 {{ projectStore.currentProject?.name || 'Novel' }}</span>
      <span class="status-item">🌿 {{ branch }}</span>
      <span class="status-item">🔍 RAG {{ ragEnabled ? 'ON' : 'OFF' }}</span>
      <span class="status-item">🔌 MCP {{ mcpServers }}</span>
      <span class="status-item">🧩 Skills {{ skillsLoaded }}</span>
    </div>
    <div class="status-right">
      <span class="status-item">Ln 1, Col 1</span>
      <span class="status-item">UTF-8</span>
      <span class="status-item">CRLF</span>
    </div>
  </div>
</template>

<style scoped>
.status-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 24px;
  padding: 0 var(--spacing-3);
  background: var(--bg-sidebar);
  border-top: 1px solid var(--border-default);
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
}

.status-left,
.status-center,
.status-right {
  display: flex;
  align-items: center;
  gap: var(--spacing-4);
}

.status-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-1);
  cursor: pointer;
}

.status-item:hover {
  color: var(--text-primary);
}
</style>
```

- [ ] **Step 2: Commit**

```bash
git add src/components/layout/StatusBar.vue
git commit -m "feat: create StatusBar component"
```

**Verification:** StatusBar should render with all items

---

## Phase 3: AI Studio

### Task 7: Create AIStudio Component

**Files:**
- Create: `src/components/ai/AIStudio.vue`

**Steps:**

- [ ] **Step 1: Create AIStudio.vue**

```vue
<script setup lang="ts">
import { ref } from "vue";
import { useAIStore } from "../../stores/ai";
import { useWorkflowStore } from "../../stores/workflow";

const aiStore = useAIStore();
const workflowStore = useWorkflowStore();

const activeSection = ref<'task' | 'workflow' | 'context' | 'execution' | 'conversation'>('task');
const input = ref("");

const sections = [
  { id: 'task', label: 'Task', icon: '📋' },
  { id: 'workflow', label: 'Workflow', icon: '⚙️' },
  { id: 'context', label: 'Context', icon: '📚' },
  { id: 'execution', label: 'Execution', icon: '🚀' },
  { id: 'conversation', label: 'Conversation', icon: '💬' },
];

const sendMessage = () => {
  if (!input.value.trim() || aiStore.streaming) return;
  aiStore.sendMessage(input.value);
  input.value = "";
};
</script>

<template>
  <div class="ai-studio">
    <div class="studio-header">
      <h3>AI Studio</h3>
    </div>

    <div class="studio-sections">
      <div
        v-for="section in sections"
        :key="section.id"
        :class="['section-header', { active: activeSection === section.id }]"
        @click="activeSection = section.id as any"
      >
        <span class="section-icon">{{ section.icon }}</span>
        <span class="section-label">{{ section.label }}</span>
        <span class="section-arrow">{{ activeSection === section.id ? '▼' : '▶' }}</span>
      </div>

      <!-- Task Section -->
      <div v-if="activeSection === 'task'" class="section-content">
        <div class="task-info">
          <div class="task-label">当前任务</div>
          <div class="task-name">生成第35章</div>
          <div class="task-progress">
            <div class="progress-bar">
              <div class="progress-fill" style="width: 42%"></div>
            </div>
            <span class="progress-text">42%</span>
          </div>
          <div class="task-eta">预计: 22 秒</div>
        </div>
      </div>

      <!-- Workflow Section -->
      <div v-if="activeSection === 'workflow'" class="section-content">
        <div class="workflow-steps">
          <div class="workflow-step completed">
            <span class="step-icon">✓</span>
            <span class="step-label">① 项目配置</span>
          </div>
          <div class="workflow-step completed">
            <span class="step-icon">✓</span>
            <span class="step-label">② 故事前提</span>
          </div>
          <div class="workflow-step completed">
            <span class="step-icon">✓</span>
            <span class="step-label">③ 世界观</span>
          </div>
          <div class="workflow-step completed">
            <span class="step-icon">✓</span>
            <span class="step-label">④ 角色</span>
          </div>
          <div class="workflow-step completed">
            <span class="step-icon">✓</span>
            <span class="step-label">⑤ 蓝图</span>
          </div>
          <div class="workflow-step active">
            <span class="step-icon">●</span>
            <span class="step-label">⑥ 第12章 Writing...</span>
          </div>
          <div class="workflow-step pending">
            <span class="step-icon">○</span>
            <span class="step-label">⑦ 审稿 Waiting</span>
          </div>
          <div class="workflow-step pending">
            <span class="step-icon">○</span>
            <span class="step-label">⑧ 修稿 Waiting</span>
          </div>
        </div>
      </div>

      <!-- Context Section -->
      <div v-if="activeSection === 'context'" class="section-content">
        <div class="context-items">
          <div class="context-item">
            <span class="context-check">✔</span>
            <span class="context-label">世界观</span>
          </div>
          <div class="context-item">
            <span class="context-check">✔</span>
            <span class="context-label">角色</span>
          </div>
          <div class="context-item">
            <span class="context-check">✔</span>
            <span class="context-label">已写章节</span>
          </div>
          <div class="context-item">
            <span class="context-check">✔</span>
            <span class="context-label">知识库</span>
          </div>
          <div class="context-item">
            <span class="context-check">✔</span>
            <span class="context-label">Prompt</span>
          </div>
        </div>
      </div>

      <!-- Execution Section -->
      <div v-if="activeSection === 'execution'" class="section-content">
        <div class="execution-items">
          <div class="execution-item">
            <span class="execution-label">Agent:</span>
            <span class="execution-value">Writer Agent</span>
          </div>
          <div class="execution-item">
            <span class="execution-label">MCP:</span>
            <span class="execution-value">3 servers</span>
          </div>
          <div class="execution-item">
            <span class="execution-label">Skills:</span>
            <span class="execution-value">12 loaded</span>
          </div>
          <div class="execution-item">
            <span class="execution-label">Hooks:</span>
            <span class="execution-value">2 active</span>
          </div>
        </div>
      </div>

      <!-- Conversation Section -->
      <div v-if="activeSection === 'conversation'" class="section-content">
        <div class="conversation-messages">
          <div
            v-for="msg in aiStore.messages"
            :key="msg.id"
            :class="['message', msg.role]"
          >
            <div class="message-content">{{ msg.content }}</div>
          </div>
        </div>
        <div class="conversation-input">
          <input
            v-model="input"
            type="text"
            placeholder="输入消息..."
            :disabled="aiStore.streaming"
            @keydown.enter="sendMessage"
          />
          <button @click="sendMessage" :disabled="aiStore.streaming">发送</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.ai-studio {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-panel);
  border-left: 1px solid var(--border-default);
}

.studio-header {
  height: 36px;
  display: flex;
  align-items: center;
  padding: 0 var(--spacing-3);
  border-bottom: 1px solid var(--border-default);
}

.studio-header h3 {
  margin: 0;
  font-size: var(--font-size-base);
  font-weight: var(--font-weight-medium);
}

.studio-sections {
  flex: 1;
  overflow-y: auto;
}

.section-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-2);
  padding: var(--spacing-2) var(--spacing-3);
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-out);
}

.section-header:hover {
  background: var(--bg-hover);
}

.section-header.active {
  background: var(--bg-active);
}

.section-icon {
  font-size: 14px;
}

.section-label {
  flex: 1;
  font-size: var(--font-size-sm);
  font-weight: var(--font-weight-medium);
}

.section-arrow {
  font-size: 10px;
  color: var(--text-secondary);
}

.section-content {
  padding: var(--spacing-3);
  border-top: 1px solid var(--border-divider);
}

/* Task Styles */
.task-info {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-2);
}

.task-label {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
}

.task-name {
  font-size: var(--font-size-md);
  font-weight: var(--font-weight-medium);
}

.task-progress {
  display: flex;
  align-items: center;
  gap: var(--spacing-2);
}

.progress-bar {
  flex: 1;
  height: 6px;
  background: var(--bg-hover);
  border-radius: var(--radius-full);
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--blue-400), var(--blue-600));
  transition: width var(--duration-normal) var(--ease-out);
}

.progress-text {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
}

.task-eta {
  font-size: var(--font-size-xs);
  color: var(--text-secondary);
}

/* Workflow Styles */
.workflow-steps {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-1);
}

.workflow-step {
  display: flex;
  align-items: center;
  gap: var(--spacing-2);
  padding: var(--spacing-1) var(--spacing-2);
  font-size: var(--font-size-sm);
}

.workflow-step.completed {
  color: var(--success);
}

.workflow-step.active {
  color: var(--blue-500);
  font-weight: var(--font-weight-medium);
}

.workflow-step.pending {
  color: var(--text-secondary);
}

.step-icon {
  width: 16px;
  text-align: center;
}

/* Context Styles */
.context-items {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-1);
}

.context-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-2);
  padding: var(--spacing-1) var(--spacing-2);
  font-size: var(--font-size-sm);
}

.context-check {
  color: var(--success);
}

/* Execution Styles */
.execution-items {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-2);
}

.execution-item {
  display: flex;
  justify-content: space-between;
  font-size: var(--font-size-sm);
}

.execution-label {
  color: var(--text-secondary);
}

.execution-value {
  color: var(--text-primary);
}

/* Conversation Styles */
.conversation-messages {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-3);
}

.message {
  margin-bottom: var(--spacing-3);
}

.message.user .message-content {
  background: var(--blue-500);
  color: white;
  margin-left: 40px;
  padding: var(--spacing-2) var(--spacing-3);
  border-radius: var(--radius-md);
}

.message.assistant .message-content {
  background: var(--bg-card);
  color: var(--text-primary);
  margin-right: 40px;
  padding: var(--spacing-2) var(--spacing-3);
  border-radius: var(--radius-md);
}

.conversation-input {
  display: flex;
  gap: var(--spacing-2);
  padding: var(--spacing-3);
  border-top: 1px solid var(--border-default);
}

.conversation-input input {
  flex: 1;
  background: var(--bg-card);
  border: 1px solid transparent;
  border-radius: var(--radius-md);
  padding: var(--spacing-2) var(--spacing-3);
  color: var(--text-primary);
  font-size: var(--font-size-sm);
}

.conversation-input input:focus {
  border-color: var(--blue-500);
  outline: none;
}

.conversation-input button {
  background: var(--blue-500);
  color: white;
  border: none;
  border-radius: var(--radius-md);
  padding: var(--spacing-2) var(--spacing-3);
  cursor: pointer;
}

.conversation-input button:hover {
  background: var(--blue-600);
}
</style>
```

- [ ] **Step 2: Commit**

```bash
git add src/components/ai/AIStudio.vue
git commit -m "feat: create AIStudio component with three-layer architecture"
```

**Verification:** AIStudio should render with all sections

---

## Phase 4: Workspace Presets

### Task 8: Create WorkspaceStore

**Files:**
- Create: `src/stores/workspace.ts`

**Steps:**

- [ ] **Step 1: Create workspace.ts**

```typescript
import { defineStore } from "pinia";
import { ref } from "vue";

interface WorkspacePreset {
  id: string;
  name: string;
  icon: string;
  layout: {
    sidebar: { visible: boolean; width: number };
    editor: { visible: boolean; width: string };
    aiStudio: { visible: boolean; width: number };
    inspector: { visible: boolean; width: number; type: 'chapter' | 'character' | 'world' | 'prompt' | 'workflow' };
    bottomPanel: { visible: boolean; height: number };
  };
}

const defaultPresets: WorkspacePreset[] = [
  {
    id: 'writing',
    name: '写作',
    icon: '✍️',
    layout: {
      sidebar: { visible: true, width: 280 },
      editor: { visible: true, width: '80%' },
      aiStudio: { visible: true, width: 320 },
      inspector: { visible: false, width: 280, type: 'chapter' },
      bottomPanel: { visible: false, height: 200 },
    },
  },
  {
    id: 'review',
    name: '审稿',
    icon: '📝',
    layout: {
      sidebar: { visible: true, width: 280 },
      editor: { visible: true, width: '50%' },
      aiStudio: { visible: false, width: 420 },
      inspector: { visible: true, width: 280, type: 'chapter' },
      bottomPanel: { visible: true, height: 200 },
    },
  },
  {
    id: 'outline',
    name: '大纲',
    icon: '📋',
    layout: {
      sidebar: { visible: true, width: 280 },
      editor: { visible: true, width: '60%' },
      aiStudio: { visible: false, width: 420 },
      inspector: { visible: true, width: 320, type: 'chapter' },
      bottomPanel: { visible: false, height: 200 },
    },
  },
  {
    id: 'character',
    name: '角色',
    icon: '👤',
    layout: {
      sidebar: { visible: true, width: 280 },
      editor: { visible: true, width: '50%' },
      aiStudio: { visible: false, width: 420 },
      inspector: { visible: true, width: 320, type: 'character' },
      bottomPanel: { visible: false, height: 200 },
    },
  },
  {
    id: 'world',
    name: '世界',
    icon: '🌍',
    layout: {
      sidebar: { visible: true, width: 280 },
      editor: { visible: true, width: '50%' },
      aiStudio: { visible: false, width: 420 },
      inspector: { visible: true, width: 320, type: 'world' },
      bottomPanel: { visible: false, height: 200 },
    },
  },
  {
    id: 'research',
    name: '研究',
    icon: '📚',
    layout: {
      sidebar: { visible: true, width: 280 },
      editor: { visible: true, width: '40%' },
      aiStudio: { visible: true, width: 320 },
      inspector: { visible: false, width: 280, type: 'chapter' },
      bottomPanel: { visible: true, height: 200 },
    },
  },
  {
    id: 'ai',
    name: 'AI',
    icon: '🤖',
    layout: {
      sidebar: { visible: true, width: 280 },
      editor: { visible: true, width: '40%' },
      aiStudio: { visible: true, width: 460 },
      inspector: { visible: false, width: 280, type: 'chapter' },
      bottomPanel: { visible: false, height: 200 },
    },
  },
  {
    id: 'zen',
    name: '专注',
    icon: '🧘',
    layout: {
      sidebar: { visible: false, width: 0 },
      editor: { visible: true, width: '100%' },
      aiStudio: { visible: false, width: 0 },
      inspector: { visible: false, width: 0, type: 'chapter' },
      bottomPanel: { visible: false, height: 0 },
    },
  },
];

export const useWorkspaceStore = defineStore("workspace", () => {
  const presets = ref<WorkspacePreset[]>(defaultPresets);
  const activePresetId = ref<string>("writing");
  const activePreset = ref<WorkspacePreset>(defaultPresets[0]);

  const setActivePreset = (presetId: string) => {
    const preset = presets.value.find((p) => p.id === presetId);
    if (preset) {
      activePresetId.value = presetId;
      activePreset.value = preset;
    }
  };

  const updateLayout = (layout: Partial<WorkspacePreset['layout']>) => {
    activePreset.value.layout = {
      ...activePreset.value.layout,
      ...layout,
    };
  };

  return {
    presets,
    activePresetId,
    activePreset,
    setActivePreset,
    updateLayout,
  };
});
```

- [ ] **Step 2: Commit**

```bash
git add src/stores/workspace.ts
git commit -m "feat: create WorkspaceStore with presets"
```

**Verification:** Store should be importable

---

### Task 9: Create WorkspaceSwitcher Component

**Files:**
- Create: `src/components/layout/WorkspaceSwitcher.vue`

**Steps:**

- [ ] **Step 1: Create WorkspaceSwitcher.vue**

```vue
<script setup lang="ts">
import { useWorkspaceStore } from "../../stores/workspace";

const workspaceStore = useWorkspaceStore();
</script>

<template>
  <div class="workspace-switcher">
    <button
      v-for="preset in workspaceStore.presets"
      :key="preset.id"
      :class="['preset-btn', { active: workspaceStore.activePresetId === preset.id }]"
      @click="workspaceStore.setActivePreset(preset.id)"
      :title="preset.name"
    >
      <span class="preset-icon">{{ preset.icon }}</span>
      <span class="preset-name">{{ preset.name }}</span>
    </button>
  </div>
</template>

<style scoped>
.workspace-switcher {
  display: flex;
  gap: var(--spacing-1);
  padding: var(--spacing-2);
  background: var(--bg-sidebar);
  border-bottom: 1px solid var(--border-default);
}

.preset-btn {
  display: flex;
  align-items: center;
  gap: var(--spacing-1);
  padding: var(--spacing-1) var(--spacing-2);
  background: transparent;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-out);
}

.preset-btn:hover {
  background: var(--bg-hover);
}

.preset-btn.active {
  background: var(--blue-500);
  color: white;
}

.preset-icon {
  font-size: 14px;
}

.preset-name {
  font-size: var(--font-size-xs);
}
</style>
```

- [ ] **Step 2: Commit**

```bash
git add src/components/layout/WorkspaceSwitcher.vue
git commit -m "feat: create WorkspaceSwitcher component"
```

**Verification:** WorkspaceSwitcher should render with presets

---

## Phase 5: Interaction System

### Task 10: Implement Keyboard Shortcuts

**Files:**
- Modify: `src/composables/useKeyboardShortcuts.ts`

**Steps:**

- [ ] **Step 1: Update shortcuts**

```typescript
const shortcuts = {
  'ctrl+shift+p': () => openCommandPalette(),
  'ctrl+p': () => openQuickOpen(),
  'ctrl+b': () => toggleSidebar(),
  'ctrl+shift+b': () => toggleAIStudio(),
  'ctrl+j': () => toggleBottomPanel(),
  'ctrl+s': () => save(),
  'ctrl+z': () => undo(),
  'ctrl+shift+z': () => redo(),
  'ctrl+f': () => find(),
  'ctrl+h': () => replace(),
  'ctrl+enter': () => sendAIMessage(),
  'ctrl+shift+enter': () => aiContinue(),
  'ctrl+shift+r': () => aiRewrite(),
  'ctrl+shift+e': () => aiExpand(),
  'ctrl+shift+c': () => aiCondense(),
  'f11': () => toggleFullscreen(),
  'ctrl+,': () => openSettings(),
};
```

- [ ] **Step 2: Commit**

```bash
git add src/composables/useKeyboardShortcuts.ts
git commit -m "feat: implement keyboard shortcuts according to spec"
```

**Verification:** Shortcuts should work

---

## Phase 6: Inspector System

### Task 11: Create Inspector Component

**Files:**
- Create: `src/components/inspector/Inspector.vue`

**Steps:**

- [ ] **Step 1: Create Inspector.vue**

```vue
<script setup lang="ts">
import { computed } from "vue";
import { useChapterStore } from "../../stores/chapter";
import { useProjectStore } from "../../stores/project";

const props = defineProps<{
  type: 'chapter' | 'character' | 'world' | 'prompt' | 'workflow';
}>();

const chapterStore = useChapterStore();
const projectStore = useProjectStore();

const chapter = computed(() => chapterStore.currentChapter);
const wordCount = computed(() => chapter.value?.content?.length || 0);
</script>

<template>
  <div class="inspector">
    <div class="inspector-header">
      <h3>{{ type }} Inspector</h3>
    </div>
    <div class="inspector-content">
      <!-- Chapter Inspector -->
      <template v-if="type === 'chapter'">
        <div class="inspector-row">
          <span class="inspector-label">字数</span>
          <span class="inspector-value">{{ wordCount }}</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">状态</span>
          <span class="inspector-value">编辑中</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">标签</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">引用角色</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">引用地点</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">章节摘要</span>
          <span class="inspector-value">-</span>
        </div>
      </template>

      <!-- Character Inspector -->
      <template v-if="type === 'character'">
        <div class="inspector-row">
          <span class="inspector-label">位置</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">状态</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">装备</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">目标</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">最近出场</span>
          <span class="inspector-value">-</span>
        </div>
      </template>

      <!-- World Inspector -->
      <template v-if="type === 'world'">
        <div class="inspector-row">
          <span class="inspector-label">名称</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">描述</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">规则</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">相关角色</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">相关章节</span>
          <span class="inspector-value">-</span>
        </div>
      </template>

      <!-- Prompt Inspector -->
      <template v-if="type === 'prompt'">
        <div class="inspector-row">
          <span class="inspector-label">System</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">Workflow</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">Context</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">RAG</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">Prompt</span>
          <span class="inspector-value">-</span>
        </div>
      </template>

      <!-- Workflow Inspector -->
      <template v-if="type === 'workflow'">
        <div class="inspector-row">
          <span class="inspector-label">当前阶段</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">进度</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">历史</span>
          <span class="inspector-value">-</span>
        </div>
        <div class="inspector-row">
          <span class="inspector-label">配置</span>
          <span class="inspector-value">-</span>
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped>
.inspector {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-panel);
  border-left: 1px solid var(--border-default);
}

.inspector-header {
  height: 36px;
  display: flex;
  align-items: center;
  padding: 0 var(--spacing-3);
  border-bottom: 1px solid var(--border-default);
}

.inspector-header h3 {
  margin: 0;
  font-size: var(--font-size-base);
  font-weight: var(--font-weight-medium);
}

.inspector-content {
  flex: 1;
  padding: var(--spacing-3);
  overflow-y: auto;
}

.inspector-row {
  display: flex;
  justify-content: space-between;
  padding: var(--spacing-2) 0;
  border-bottom: 1px solid var(--border-divider);
}

.inspector-label {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.inspector-value {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
}
</style>
```

- [ ] **Step 2: Commit**

```bash
git add src/components/inspector/Inspector.vue
git commit -m "feat: create Inspector component"
```

**Verification:** Inspector should render with all types

---

## Phase 7: Integration

### Task 12: Update Main Layout

**Files:**
- Modify: `src/components/layout/IDELayout.vue`

**Steps:**

- [ ] **Step 1: Integrate all components**

```vue
<script setup lang="ts">
import { ref, watch } from "vue";
import TitleBar from "./TitleBar.vue";
import ActivityBar from "./ActivityBar.vue";
import Sidebar from "./Sidebar.vue";
import Breadcrumb from "./Breadcrumb.vue";
import EditorPanel from "../editor/EditorPanel.vue";
import AIStudio from "../ai/AIStudio.vue";
import Inspector from "../inspector/Inspector.vue";
import BottomPanel from "./BottomPanel.vue";
import StatusBar from "./StatusBar.vue";
import WorkspaceSwitcher from "./WorkspaceSwitcher.vue";
import { useProjectStore } from "../../stores/project";
import { useChapterStore } from "../../stores/chapter";
import { useWorkspaceStore } from "../../stores/workspace";

const emit = defineEmits<{
  back: [];
}>();

const projectStore = useProjectStore();
const chapterStore = useChapterStore();
const workspaceStore = useWorkspaceStore();
const editorPanelRef = ref<InstanceType<typeof EditorPanel> | null>(null);

const activeView = ref<'explorer' | 'search' | 'ai' | 'plugins' | 'settings'>('explorer');
const sidebarVisible = ref(true);
const aiStudioVisible = ref(true);
const bottomPanelVisible = ref(false);
const inspectorVisible = ref(false);
const inspectorType = ref<'chapter' | 'character' | 'world' | 'prompt' | 'workflow'>('chapter');

const sidebarWidth = ref(280);
const aiStudioWidth = ref(420);
const bottomPanelHeight = ref(200);
const inspectorWidth = ref(280);

const isDragging = ref(false);
const dragTarget = ref<"sidebar" | "ai" | "bottom" | "inspector" | null>(null);

const onMouseDown = (target: "sidebar" | "ai" | "bottom" | "inspector", e: MouseEvent) => {
  isDragging.value = true;
  dragTarget.value = target;
  e.preventDefault();
};

const onMouseMove = (e: MouseEvent) => {
  if (!isDragging.value) return;

  if (dragTarget.value === "sidebar") {
    sidebarWidth.value = Math.max(200, Math.min(e.clientX, 500));
  } else if (dragTarget.value === "ai") {
    aiStudioWidth.value = Math.max(320, Math.min(window.innerWidth - e.clientX, 600));
  } else if (dragTarget.value === "bottom") {
    bottomPanelHeight.value = Math.max(100, Math.min(window.innerHeight - e.clientY, 400));
  } else if (dragTarget.value === "inspector") {
    inspectorWidth.value = Math.max(200, Math.min(window.innerWidth - e.clientX, 400));
  }
};

const onMouseUp = () => {
  isDragging.value = false;
  dragTarget.value = null;
};

window.addEventListener("mousemove", onMouseMove);
window.addEventListener("mouseup", onMouseUp);

watch(
  () => projectStore.currentProject,
  async (project) => {
    if (project) {
      await chapterStore.fetchChapters(project.id);
    } else {
      chapterStore.chapters = [];
      chapterStore.currentChapter = null;
    }
  },
  { immediate: true }
);

const handleOpenChapter = (chapterId: string) => {
  const chapter = chapterStore.chapters.find((c) => c.id === chapterId);
  if (chapter && editorPanelRef.value) {
    editorPanelRef.value.openTab(chapter.id, chapter.title);
  }
};

const toggleSidebar = () => {
  sidebarVisible.value = !sidebarVisible.value;
};

const toggleAIStudio = () => {
  aiStudioVisible.value = !aiStudioVisible.value;
};

const toggleBottomPanel = () => {
  bottomPanelVisible.value = !bottomPanelVisible.value;
};

const toggleInspector = () => {
  inspectorVisible.value = !inspectorVisible.value;
};

const breadcrumbItems = [
  { label: projectStore.currentProject?.name || 'Novel' },
  { label: chapterStore.currentChapter?.title || 'Editor' },
];
</script>

<template>
  <div class="ide-layout">
    <TitleBar @back="emit('back')" />
    <WorkspaceSwitcher />
    <div class="ide-main">
      <ActivityBar
        :activeView="activeView"
        @select="activeView = $event"
      />
      <div class="sidebar" :style="{ width: sidebarVisible ? `${sidebarWidth}px` : '0' }">
        <Sidebar :view="activeView" @openChapter="handleOpenChapter" />
      </div>
      <div
        v-if="sidebarVisible"
        class="resize-handle vertical"
        @mousedown="onMouseDown('sidebar', $event)"
      />
      <div class="editor-area">
        <Breadcrumb :items="breadcrumbItems" />
        <div class="editor-workspace" :style="{ height: bottomPanelVisible ? `calc(100% - ${bottomPanelHeight}px)` : '100%' }">
          <EditorPanel ref="editorPanelRef" />
        </div>
        <div
          v-if="bottomPanelVisible"
          class="resize-handle horizontal"
          @mousedown="onMouseDown('bottom', $event)"
        />
        <BottomPanel v-if="bottomPanelVisible" :height="bottomPanelHeight" />
      </div>
      <div
        v-if="inspectorVisible"
        class="resize-handle vertical"
        @mousedown="onMouseDown('inspector', $event)"
      />
      <div class="inspector" :style="{ width: inspectorVisible ? `${inspectorWidth}px` : '0' }">
        <Inspector :type="inspectorType" />
      </div>
      <div
        v-if="aiStudioVisible"
        class="resize-handle vertical"
        @mousedown="onMouseDown('ai', $event)"
      />
      <div class="ai-studio" :style="{ width: aiStudioVisible ? `${aiStudioWidth}px` : '0' }">
        <AIStudio />
      </div>
    </div>
    <StatusBar />
  </div>
</template>
```

- [ ] **Step 2: Update styles**

```css
.ide-layout {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  background: var(--bg-background);
}

.ide-main {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.sidebar {
  flex-shrink: 0;
  overflow: hidden;
  transition: width var(--duration-normal) var(--ease-out);
}

.editor-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.inspector {
  flex-shrink: 0;
  overflow: hidden;
  transition: width var(--duration-normal) var(--ease-out);
}

.ai-studio {
  flex-shrink: 0;
  overflow: hidden;
  transition: width var(--duration-normal) var(--ease-out);
}

.resize-handle {
  width: 4px;
  cursor: col-resize;
  background: transparent;
  transition: background var(--duration-fast) var(--ease-out);
}

.resize-handle:hover {
  background: var(--blue-500);
}

.resize-handle.horizontal {
  width: 100%;
  height: 4px;
  cursor: row-resize;
}
```

- [ ] **Step 3: Commit**

```bash
git add src/components/layout/IDELayout.vue
git commit -m "feat: integrate all UI components"
```

**Verification:** Full layout should render correctly

---

## Execution Summary

| Phase | Tasks | Description | Est. Time |
|-------|-------|-------------|-----------|
| 1 | 2 | Design Token System | 30 min |
| 2 | 4 | Layout System | 45 min |
| 3 | 1 | AI Studio | 30 min |
| 4 | 2 | Workspace Presets | 20 min |
| 5 | 1 | Interaction System | 15 min |
| 6 | 1 | Inspector System | 20 min |
| 7 | 1 | Integration | 20 min |
| **Total** | **12** | | **~180 min** |

## Success Criteria

- [ ] Design Token system implemented
- [ ] All component styles match spec
- [ ] Layout System works correctly
- [ ] AI Studio with three-layer architecture
- [ ] Workspace Presets switchable
- [ ] Keyboard shortcuts functional
- [ ] Inspector dynamic by type
- [ ] All tests pass
- [ ] Build succeeds

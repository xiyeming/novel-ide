<script setup lang="ts">
import { ref, computed } from "vue";
import { useTauriIPC } from "../../composables/useTauriIPC";

interface ChapterVersion {
  id: string;
  chapter_id: string;
  version_number: number;
  content: string;
  word_count: number;
  created_at: string;
}

const props = defineProps<{
  chapterId: string | null;
  currentContent: string;
}>();

const emit = defineEmits<{
  restored: [content: string];
}>();

const { call } = useTauriIPC();
const versions = ref<ChapterVersion[]>([]);
const loading = ref(false);
const selectedVersion = ref<ChapterVersion | null>(null);
const showDiff = ref(false);

const sortedVersions = computed(() => versions.value);

const fetchVersions = async () => {
  if (!props.chapterId) return;
  loading.value = true;
  try {
    versions.value = await call<ChapterVersion[]>("list_versions", {
      chapterId: props.chapterId,
    });
  } finally {
    loading.value = false;
  }
};

const selectVersion = (version: ChapterVersion) => {
  selectedVersion.value = version;
  showDiff.value = true;
};

const restoreVersion = async (version: ChapterVersion) => {
  if (!confirm(`确定恢复到版本 ${version.version_number} 吗？`)) return;
  try {
    const content = await call<string>("restore_version", {
      versionId: version.id,
    });
    emit("restored", content);
    showDiff.value = false;
    selectedVersion.value = null;
  } catch (e) {
    alert("恢复失败: " + String(e));
  }
};

const formatTime = (dateStr: string) => {
  const d = new Date(dateStr + "Z");
  const now = new Date();
  const diffMs = now.getTime() - d.getTime();
  const diffMin = Math.floor(diffMs / 60000);
  if (diffMin < 1) return "刚刚";
  if (diffMin < 60) return `${diffMin} 分钟前`;
  const diffHour = Math.floor(diffMin / 60);
  if (diffHour < 24) return `${diffHour} 小时前`;
  const diffDay = Math.floor(diffHour / 24);
  if (diffDay < 7) return `${diffDay} 天前`;
  return d.toLocaleDateString("zh-CN", { month: "short", day: "numeric", hour: "2-digit", minute: "2-digit" });
};

const closeDiff = () => {
  showDiff.value = false;
  selectedVersion.value = null;
};

defineExpose({ fetchVersions });
</script>

<template>
  <div class="version-history">
    <div class="version-header">
      <span class="version-title">版本历史</span>
    </div>

    <div v-if="!chapterId" class="version-empty">请先打开一个章节</div>

    <div v-else-if="loading && versions.length === 0" class="version-empty">加载中...</div>

    <div v-else-if="versions.length === 0" class="version-empty">暂无版本记录</div>

    <div v-else class="version-list">
      <div
        v-for="v in sortedVersions"
        :key="v.id"
        :class="['version-item', { selected: selectedVersion?.id === v.id }]"
        @click="selectVersion(v)"
      >
        <div class="version-info">
          <span class="version-num">v{{ v.version_number }}</span>
          <span class="version-time">{{ formatTime(v.created_at) }}</span>
        </div>
        <div class="version-meta">
          <span class="version-words">{{ v.word_count.toLocaleString() }} 字</span>
          <button
            class="restore-btn"
            @click.stop="restoreVersion(v)"
            title="恢复此版本"
          >
            恢复
          </button>
        </div>
      </div>
    </div>

    <div v-if="showDiff && selectedVersion" class="version-diff">
      <div class="diff-header">
        <span class="diff-title">版本 {{ selectedVersion.version_number }}</span>
        <button class="diff-close" @click="closeDiff">×</button>
      </div>
      <div class="diff-content">
        <div class="diff-label">当前内容</div>
        <pre class="diff-text">{{ currentContent || "(空)" }}</pre>
        <div class="diff-label">版本内容</div>
        <pre class="diff-text version-content">{{ selectedVersion.content || "(空)" }}</pre>
      </div>
      <div class="diff-actions">
        <button class="diff-restore-btn" @click="restoreVersion(selectedVersion)">恢复此版本</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.version-history {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-secondary);
  border-left: 1px solid var(--border);
  min-width: 240px;
}

.version-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-sm) var(--spacing-md);
  border-bottom: 1px solid var(--border);
}

.version-title {
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-primary);
}

.version-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-muted);
  font-size: var(--font-size-sm);
}

.version-list {
  flex: 1;
  overflow-y: auto;
}

.version-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-sm) var(--spacing-md);
  border-bottom: 1px solid var(--border);
  cursor: pointer;
  transition: background 0.15s;
}

.version-item:hover {
  background: var(--bg-hover);
}

.version-item.selected {
  background: var(--bg-surface);
  border-left: 2px solid var(--accent);
}

.version-info {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.version-num {
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--accent);
  font-family: var(--font-mono);
}

.version-time {
  font-size: var(--font-size-sm);
  color: var(--text-muted);
}

.version-meta {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.version-words {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  font-family: var(--font-mono);
}

.restore-btn {
  padding: 2px var(--spacing-sm);
  background: transparent;
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-secondary);
  font-size: var(--font-size-sm);
  cursor: pointer;
  transition: all 0.15s;
  opacity: 0;
}

.version-item:hover .restore-btn {
  opacity: 1;
}

.restore-btn:hover {
  background: var(--accent);
  color: var(--bg-primary);
  border-color: var(--accent);
}

.version-diff {
  border-top: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  max-height: 50%;
  min-height: 200px;
}

.diff-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-sm) var(--spacing-md);
  border-bottom: 1px solid var(--border);
}

.diff-title {
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-primary);
}

.diff-close {
  background: none;
  border: none;
  color: var(--text-muted);
  font-size: 16px;
  cursor: pointer;
  padding: 0 4px;
}

.diff-close:hover {
  color: var(--text-primary);
}

.diff-content {
  flex: 1;
  overflow-y: auto;
  padding: var(--spacing-md);
}

.diff-label {
  font-size: var(--font-size-sm);
  color: var(--text-muted);
  margin-bottom: var(--spacing-xs);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.diff-text {
  font-family: var(--font-mono);
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  white-space: pre-wrap;
  word-break: break-all;
  padding: var(--spacing-sm);
  background: var(--bg-tertiary);
  border-radius: 4px;
  max-height: 120px;
  overflow-y: auto;
  margin-bottom: var(--spacing-md);
}

.diff-text.version-content {
  background: var(--bg-surface);
  border: 1px solid var(--border);
}

.diff-actions {
  padding: var(--spacing-sm) var(--spacing-md);
  border-top: 1px solid var(--border);
  display: flex;
  justify-content: flex-end;
}

.diff-restore-btn {
  padding: var(--spacing-xs) var(--spacing-md);
  background: var(--accent);
  color: var(--bg-primary);
  border: none;
  border-radius: 4px;
  font-size: var(--font-size-sm);
  cursor: pointer;
  transition: opacity 0.15s;
}

.diff-restore-btn:hover {
  opacity: 0.9;
}
</style>

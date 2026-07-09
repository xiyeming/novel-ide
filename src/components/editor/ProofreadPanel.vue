<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useTauriIPC } from "../../composables/useTauriIPC";
import { useModelStore } from "../../stores/model";

interface ProofreadError {
  error_type: string;
  original: string;
  suggestion: string;
  line: number;
  column: number;
  confidence: number;
}

const props = defineProps<{
  chapterId: string | null;
  content: string;
}>();

const emit = defineEmits<{
  fix: [error: ProofreadError];
  fixAll: [errors: ProofreadError[]];
  jumpToLine: [line: number];
}>();

const { call } = useTauriIPC();
const modelStore = useModelStore();
const errors = ref<ProofreadError[]>([]);
const loading = ref(false);
const selectedError = ref<ProofreadError | null>(null);
const selectedProviderId = ref<string>("");

const errorTypeLabels: Record<string, string> = {
  typo: "错别字",
  grammar: "语病",
  punctuation: "标点错误",
  word_choice: "用词不当",
  logic: "逻辑错误",
};

const errorTypeIcons: Record<string, string> = {
  typo: "✏️",
  grammar: "📝",
  punctuation: "❗",
  word_choice: "💡",
  logic: "🔗",
};

const sortedErrors = computed(() =>
  [...errors.value].sort((a, b) => a.line - b.line)
);

onMounted(async () => {
  if (modelStore.providers.length === 0) {
    await modelStore.fetchProviders();
  }
  const defaultProvider = modelStore.providers.find((p) => p.is_default);
  if (defaultProvider) {
    selectedProviderId.value = defaultProvider.id;
  } else if (modelStore.providers.length > 0) {
    selectedProviderId.value = modelStore.providers[0].id;
  }
});

const runProofread = async () => {
  if (!props.content || !selectedProviderId.value) return;

  loading.value = true;
  errors.value = [];
  selectedError.value = null;

  try {
    const result = await call<ProofreadError[]>("proofread_chapter", {
      providerId: selectedProviderId.value,
      content: props.content,
    });
    errors.value = result;
  } catch (e) {
    alert("校对失败: " + String(e));
  } finally {
    loading.value = false;
  }
};

const selectError = (error: ProofreadError) => {
  selectedError.value = error;
  emit("jumpToLine", error.line);
};

const fixError = (error: ProofreadError) => {
  emit("fix", error);
  errors.value = errors.value.filter((e) => e !== error);
  if (selectedError.value === error) {
    selectedError.value = null;
  }
};

const fixAllErrors = () => {
  if (errors.value.length === 0) return;
  emit("fixAll", [...errors.value]);
  errors.value = [];
  selectedError.value = null;
};

const clearErrors = () => {
  errors.value = [];
  selectedError.value = null;
};

defineExpose({ runProofread });
</script>

<template>
  <div class="proofread-panel">
    <div class="proofread-header">
      <span class="proofread-title">校对结果</span>
      <div class="proofread-actions">
        <button
          class="proofread-btn primary"
          @click="runProofread"
          :disabled="loading || !selectedProviderId"
          title="开始校对"
        >
          {{ loading ? "校对中..." : "开始校对" }}
        </button>
        <button
          v-if="errors.length > 0"
          class="proofread-btn"
          @click="fixAllErrors"
          title="全部修复"
        >
          全部修复
        </button>
        <button
          v-if="errors.length > 0"
          class="proofread-btn"
          @click="clearErrors"
          title="清除"
        >
          清除
        </button>
      </div>
    </div>

    <div class="proofread-provider">
      <label for="provider-select">模型:</label>
      <select id="provider-select" v-model="selectedProviderId">
        <option v-for="p in modelStore.providers" :key="p.id" :value="p.id">
          {{ p.name }}
        </option>
      </select>
    </div>

    <div v-if="!selectedProviderId" class="proofread-empty">
      请先配置模型提供者
    </div>

    <div v-else-if="loading" class="proofread-empty">正在校对...</div>

    <div v-else-if="errors.length === 0" class="proofread-empty">
      {{ content ? '点击"开始校对"检查文本' : "请先输入内容" }}
    </div>

    <div v-else class="proofread-list">
      <div
        v-for="(error, index) in sortedErrors"
        :key="index"
        :class="[
          'proofread-item',
          { selected: selectedError === error },
        ]"
        @click="selectError(error)"
      >
        <div class="error-header">
          <span class="error-icon">{{ errorTypeIcons[error.error_type] || "❓" }}</span>
          <span class="error-type">{{ errorTypeLabels[error.error_type] || error.error_type }}</span>
          <span class="error-line">行 {{ error.line }}</span>
          <span class="error-confidence">{{ Math.round(error.confidence * 100) }}%</span>
        </div>
        <div class="error-content">
          <div class="error-original">
            <span class="error-label">原文:</span>
            <span class="error-text">{{ error.original }}</span>
          </div>
          <div class="error-suggestion">
            <span class="error-label">建议:</span>
            <span class="error-text">{{ error.suggestion }}</span>
          </div>
        </div>
        <button
          class="fix-btn"
          @click.stop="fixError(error)"
          title="修复此错误"
        >
          修复
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.proofread-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-secondary);
  border-left: 1px solid var(--border);
  min-width: 280px;
}

.proofread-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--spacing-sm) var(--spacing-md);
  border-bottom: 1px solid var(--border);
}

.proofread-title {
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-primary);
}

.proofread-actions {
  display: flex;
  gap: var(--spacing-xs);
}

.proofread-btn {
  padding: 2px var(--spacing-sm);
  background: transparent;
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-secondary);
  font-size: var(--font-size-sm);
  cursor: pointer;
  transition: all 0.15s;
}

.proofread-btn:hover:not(:disabled) {
  background: var(--bg-hover);
}

.proofread-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.proofread-btn.primary {
  background: var(--accent);
  color: var(--bg-primary);
  border-color: var(--accent);
}

.proofread-btn.primary:hover:not(:disabled) {
  opacity: 0.9;
}

.proofread-provider {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-sm) var(--spacing-md);
  border-bottom: 1px solid var(--border);
}

.proofread-provider label {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
}

.proofread-provider select {
  flex: 1;
  padding: var(--spacing-xs) var(--spacing-sm);
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-primary);
  font-size: var(--font-size-sm);
  outline: none;
}

.proofread-provider select:focus {
  border-color: var(--accent);
}

.proofread-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-muted);
  font-size: var(--font-size-sm);
  padding: var(--spacing-md);
  text-align: center;
}

.proofread-list {
  flex: 1;
  overflow-y: auto;
}

.proofread-item {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
  padding: var(--spacing-sm) var(--spacing-md);
  border-bottom: 1px solid var(--border);
  cursor: pointer;
  transition: background 0.15s;
  position: relative;
}

.proofread-item:hover {
  background: var(--bg-hover);
}

.proofread-item.selected {
  background: var(--bg-surface);
  border-left: 2px solid var(--accent);
}

.error-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.error-icon {
  font-size: 14px;
}

.error-type {
  font-size: var(--font-size-sm);
  font-weight: 600;
  color: var(--text-primary);
}

.error-line {
  font-size: var(--font-size-sm);
  color: var(--text-muted);
  font-family: var(--font-mono);
}

.error-confidence {
  font-size: var(--font-size-sm);
  color: var(--text-muted);
  margin-left: auto;
}

.error-content {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
  font-size: var(--font-size-sm);
}

.error-original,
.error-suggestion {
  display: flex;
  gap: var(--spacing-xs);
}

.error-label {
  color: var(--text-muted);
  min-width: 32px;
}

.error-text {
  color: var(--text-secondary);
}

.error-suggestion .error-text {
  color: var(--accent);
}

.fix-btn {
  position: absolute;
  top: var(--spacing-sm);
  right: var(--spacing-md);
  padding: 2px var(--spacing-sm);
  background: transparent;
  border: 1px solid var(--border);
  border-radius: 4px;
  color: var(--text-secondary);
  font-size: var(--font-size-sm);
  cursor: pointer;
  opacity: 0;
  transition: all 0.15s;
}

.proofread-item:hover .fix-btn {
  opacity: 1;
}

.fix-btn:hover {
  background: var(--accent);
  color: var(--bg-primary);
  border-color: var(--accent);
}
</style>

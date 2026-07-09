<script setup lang="ts">
import { ref, computed } from 'vue'
import { useAIStore } from '../../stores/ai'
import { useModelStore } from '../../stores/model'
import { useChapterStore } from '../../stores/chapter'

const aiStore = useAIStore()
const modelStore = useModelStore()
const chapterStore = useChapterStore()

const selectedFeature = ref('continue')
const instruction = ref('')
const targetWords = ref(1000)
const targetStyle = ref('古风')
const result = ref('')
const isProcessing = ref(false)
const errorMsg = ref('')

const features = [
  { id: 'continue', name: '续写', icon: '✍️', description: '继续当前内容' },
  { id: 'rewrite', name: '改写', icon: '🔄', description: '按要求改写内容' },
  { id: 'expand', name: '扩写', icon: '📝', description: '丰富细节描写' },
  { id: 'condense', name: '缩写', icon: '✂️', description: '精简内容' },
  { id: 'style', name: '风格转换', icon: '🎨', description: '转换写作风格' },
]

const stylePresets = [
  '古风', '现代', '悬疑', '科幻', '奇幻', '武侠', '都市', '历史'
]

const currentContent = computed(() => {
  return chapterStore.currentChapter?.content || ''
})

const defaultProvider = computed(() => {
  return modelStore.providers.find(p => p.is_default) || modelStore.providers[0]
})

async function processContent() {
  if (!currentContent.value || !defaultProvider.value) return

  isProcessing.value = true
  errorMsg.value = ''
  try {
    const providerId = defaultProvider.value.id

    switch (selectedFeature.value) {
      case 'continue':
        result.value = await aiStore.continueWriting(currentContent.value, providerId)
        break
      case 'rewrite':
        if (!instruction.value) {
          errorMsg.value = '请输入改写要求'
          return
        }
        result.value = await aiStore.rewriteContent(currentContent.value, providerId, instruction.value)
        break
      case 'expand':
        result.value = await aiStore.expandContent(currentContent.value, providerId, targetWords.value)
        break
      case 'condense':
        result.value = await aiStore.condenseContent(currentContent.value, providerId)
        break
      case 'style':
        result.value = await aiStore.styleTransfer(currentContent.value, providerId, targetStyle.value)
        break
    }
  } catch (e) {
    errorMsg.value = String(e)
  } finally {
    isProcessing.value = false
  }
}

function applyResult() {
  if (result.value && chapterStore.currentChapter) {
    chapterStore.updateChapterContent(
      chapterStore.currentChapter.id,
      currentContent.value + '\n\n' + result.value
    )
    result.value = ''
  }
}

defineExpose({ selectFeature })

function selectFeature(feature: string) {
  selectedFeature.value = feature
}
</script>

<template>
  <div class="ai-write-panel">
    <div class="feature-grid">
      <button
        v-for="feature in features"
        :key="feature.id"
        class="feature-btn"
        :class="{ active: selectedFeature === feature.id }"
        @click="selectedFeature = feature.id"
      >
        <span class="feature-icon">{{ feature.icon }}</span>
        <span class="feature-name">{{ feature.name }}</span>
      </button>
    </div>

    <div class="feature-options">
      <div v-if="selectedFeature === 'rewrite'" class="option-group">
        <label>改写要求</label>
        <textarea v-model="instruction" placeholder="例如：让对话更生动..." rows="3" />
      </div>

      <div v-if="selectedFeature === 'expand'" class="option-group">
        <label>目标字数: {{ targetWords }}</label>
        <input type="range" v-model.number="targetWords" min="500" max="3000" step="100" />
      </div>

      <div v-if="selectedFeature === 'style'" class="option-group">
        <label>目标风格</label>
        <div class="style-presets">
          <button
            v-for="style in stylePresets"
            :key="style"
            class="style-btn"
            :class="{ active: targetStyle === style }"
            @click="targetStyle = style"
          >
            {{ style }}
          </button>
        </div>
      </div>
    </div>

    <button
      class="process-btn"
      :disabled="isProcessing || !currentContent || !defaultProvider"
      @click="processContent"
    >
      {{ isProcessing ? '处理中...' : '开始处理' }}
    </button>

    <div v-if="errorMsg" class="error-msg">{{ errorMsg }}</div>

    <div v-if="result" class="result-section">
      <div class="result-header">
        <span>生成结果</span>
        <button class="apply-btn" @click="applyResult">应用到章节</button>
      </div>
      <div class="result-content">{{ result }}</div>
    </div>
  </div>
</template>

<style scoped>
.ai-write-panel {
  padding: 16px;
  height: 100%;
  overflow-y: auto;
}

.feature-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 8px;
  margin-bottom: 16px;
}

.feature-btn {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 12px 8px;
  border: 1px solid var(--border);
  border-radius: 8px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s;
}

.feature-btn:hover {
  border-color: var(--accent);
}

.feature-btn.active {
  border-color: var(--accent);
  background: var(--accent);
  color: white;
}

.feature-icon {
  font-size: 20px;
  margin-bottom: 4px;
}

.feature-name {
  font-size: 12px;
}

.feature-options {
  margin-bottom: 16px;
}

.option-group {
  margin-bottom: 12px;
}

.option-group label {
  display: block;
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 6px;
}

.option-group textarea,
.option-group input[type="range"] {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 13px;
  box-sizing: border-box;
}

.style-presets {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.style-btn {
  padding: 6px 12px;
  border: 1px solid var(--border);
  border-radius: 16px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  cursor: pointer;
  font-size: 12px;
  transition: all 0.2s;
}

.style-btn:hover {
  border-color: var(--accent);
}

.style-btn.active {
  border-color: var(--accent);
  background: var(--accent);
  color: white;
}

.process-btn {
  width: 100%;
  padding: 12px;
  border: none;
  border-radius: 8px;
  background: var(--accent);
  color: white;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  margin-bottom: 16px;
}

.process-btn:hover:not(:disabled) {
  opacity: 0.9;
}

.process-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.error-msg {
  padding: 8px 12px;
  margin-bottom: 12px;
  border-radius: 6px;
  background: #fee;
  color: #c00;
  font-size: 13px;
}

.result-section {
  border: 1px solid var(--border);
  border-radius: 8px;
  overflow: hidden;
}

.result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border);
  font-size: 14px;
  font-weight: 500;
}

.apply-btn {
  padding: 6px 12px;
  border: none;
  border-radius: 6px;
  background: var(--success, #22c55e);
  color: white;
  cursor: pointer;
  font-size: 12px;
}

.result-content {
  padding: 12px;
  font-size: 14px;
  line-height: 1.6;
  white-space: pre-wrap;
  max-height: 300px;
  overflow-y: auto;
}
</style>

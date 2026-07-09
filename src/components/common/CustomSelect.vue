<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";

interface Option {
  label: string;
  value: string;
}

const props = defineProps<{
  modelValue: string;
  options: Option[];
  placeholder?: string;
}>();

const emit = defineEmits<{
  "update:modelValue": [value: string];
}>();

const isOpen = ref(false);
const selectRef = ref<HTMLDivElement | null>(null);

const selectedLabel = computed(() => {
  const option = props.options.find((o) => o.value === props.modelValue);
  return option?.label || props.placeholder || "请选择";
});

const toggleDropdown = () => {
  isOpen.value = !isOpen.value;
};

const selectOption = (value: string) => {
  emit("update:modelValue", value);
  isOpen.value = false;
};

const handleClickOutside = (event: MouseEvent) => {
  if (selectRef.value && !selectRef.value.contains(event.target as Node)) {
    isOpen.value = false;
  }
};

onMounted(() => {
  document.addEventListener("mousedown", handleClickOutside);
});

onUnmounted(() => {
  document.removeEventListener("mousedown", handleClickOutside);
});
</script>

<template>
  <div class="custom-select" ref="selectRef">
    <div class="select-trigger" @click="toggleDropdown">
      <span :class="{ placeholder: !modelValue }">{{ selectedLabel }}</span>
      <span class="arrow" :class="{ open: isOpen }">▾</span>
    </div>
    <div class="select-dropdown" v-if="isOpen">
      <div
        v-for="option in options"
        :key="option.value"
        class="select-option"
        :class="{ selected: option.value === modelValue }"
        @click="selectOption(option.value)"
      >
        {{ option.label }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.custom-select {
  position: relative;
  width: 100%;
}

.select-trigger {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding: var(--spacing-sm) var(--spacing-md);
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: var(--font-size-md);
  cursor: pointer;
  transition: border-color var(--duration-fast) var(--ease-out);
}

.select-trigger:hover {
  border-color: var(--accent);
}

.placeholder {
  color: var(--text-muted);
}

.arrow {
  font-size: 12px;
  transition: transform var(--duration-fast) var(--ease-out);
}

.arrow.open {
  transform: rotate(180deg);
}

.select-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  margin-top: 4px;
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 6px;
  box-shadow: var(--shadow-lg);
  z-index: 1000;
  max-height: 200px;
  overflow-y: auto;
}

.select-option {
  padding: var(--spacing-sm) var(--spacing-md);
  color: var(--text-primary);
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-out);
}

.select-option:hover {
  background: var(--bg-hover);
}

.select-option.selected {
  background: var(--accent);
  color: white;
}
</style>

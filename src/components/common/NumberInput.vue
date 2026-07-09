<script setup lang="ts">
import { ref, computed } from "vue";

const props = defineProps<{
  modelValue: number;
  min?: number;
  max?: number;
  step?: number;
}>();

const emit = defineEmits<{
  "update:modelValue": [value: number];
}>();

const step = computed(() => props.step || 1);
const min = computed(() => props.min ?? -Infinity);
const max = computed(() => props.max ?? Infinity);

const increment = () => {
  const newVal = Math.min(props.modelValue + step.value, max.value);
  emit("update:modelValue", newVal);
};

const decrement = () => {
  const newVal = Math.max(props.modelValue - step.value, min.value);
  emit("update:modelValue", newVal);
};

const handleInput = (event: Event) => {
  const target = event.target as HTMLInputElement;
  const value = parseFloat(target.value);
  if (!isNaN(value)) {
    emit("update:modelValue", Math.min(Math.max(value, min.value), max.value));
  }
};
</script>

<template>
  <div class="number-input">
    <button class="number-btn minus" @click="decrement" :disabled="modelValue <= min">−</button>
    <input
      type="number"
      :value="modelValue"
      @input="handleInput"
      :min="min"
      :max="max"
      :step="step"
    />
    <button class="number-btn plus" @click="increment" :disabled="modelValue >= max">+</button>
  </div>
</template>

<style scoped>
.number-input {
  display: flex;
  align-items: center;
  width: 100%;
  border: 1px solid var(--border);
  border-radius: 6px;
  overflow: hidden;
}

.number-input:focus-within {
  border-color: var(--accent);
}

.number-btn {
  width: 28px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-surface);
  border: none;
  color: var(--text-primary);
  font-size: 14px;
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-out);
  flex-shrink: 0;
}

.number-btn:hover:not(:disabled) {
  background: var(--bg-hover);
}

.number-btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.number-btn.minus {
  border-right: 1px solid var(--border);
}

.number-btn.plus {
  border-left: 1px solid var(--border);
}

.number-input input {
  flex: 1;
  min-width: 0;
  padding: 0 var(--spacing-xs);
  background: var(--bg-surface);
  border: none;
  color: var(--text-primary);
  font-size: var(--font-size-md);
  text-align: center;
  -moz-appearance: textfield;
  outline: none;
}

.number-input input::-webkit-outer-spin-button,
.number-input input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
</style>

<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { useMonaco } from "../../composables/useMonaco";

const props = withDefaults(
  defineProps<{
    modelValue?: string;
    language?: string;
    readOnly?: boolean;
  }>(),
  {
    modelValue: "",
    language: "markdown",
    readOnly: false,
  }
);

const emit = defineEmits<{
  "update:modelValue": [value: string];
  save: [];
}>();

const { editor, init, setValue, getValue, setReadOnly } = useMonaco();
const editorContainer = ref<HTMLDivElement>();

onMounted(() => {
  if (editorContainer.value) {
    const ed = init(editorContainer.value, {
      value: props.modelValue,
      language: props.language,
      readOnly: props.readOnly,
    });

    ed.onDidChangeModelContent(() => {
      emit("update:modelValue", getValue());
    });

    // Ctrl+S / Cmd+S to save
    ed.addAction({
      id: "save-document",
      label: "保存",
      keybindings: [2048 | 49], // CtrlOrMeta + S
      run: () => {
        emit("save");
      },
    });
  }
});

watch(
  () => props.modelValue,
  (newVal) => {
    if (editor.value && newVal !== getValue()) {
      setValue(newVal);
    }
  }
);

watch(
  () => props.readOnly,
  (newVal) => {
    setReadOnly(newVal);
  }
);

defineExpose({ editor });
</script>

<template>
  <div ref="editorContainer" class="monaco-editor-wrapper"></div>
</template>

<style scoped>
.monaco-editor-wrapper {
  width: 100%;
  height: 100%;
}
</style>

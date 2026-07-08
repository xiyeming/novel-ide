import { onUnmounted, ref, shallowRef } from "vue";
import * as monaco from "monaco-editor";

export function useMonaco() {
  const editor = shallowRef<monaco.editor.IStandaloneCodeEditor | null>(null);
  const container = ref<HTMLDivElement | null>(null);

  const init = (
    containerEl: HTMLDivElement,
    options?: {
      value?: string;
      language?: string;
      readOnly?: boolean;
      theme?: string;
    }
  ) => {
    container.value = containerEl;
    editor.value = monaco.editor.create(containerEl, {
      value: options?.value ?? "",
      language: options?.language ?? "markdown",
      theme: options?.theme ?? "vs-dark",
      readOnly: options?.readOnly ?? false,
      minimap: { enabled: false },
      fontSize: 16,
      lineHeight: 28,
      fontFamily: "'PingFang SC', 'Microsoft YaHei', sans-serif",
      wordWrap: "on",
      padding: { top: 16, bottom: 16 },
      scrollBeyondLastLine: false,
      renderLineHighlight: "gutter",
      automaticLayout: true,
      tabSize: 2,
      suggest: {
        showWords: false,
      },
    });
    return editor.value;
  };

  const dispose = () => {
    editor.value?.dispose();
    editor.value = null;
  };

  const setValue = (value: string) => {
    if (editor.value) {
      const model = editor.value.getModel();
      if (model) {
        model.setValue(value);
      }
    }
  };

  const getValue = (): string => {
    return editor.value?.getValue() ?? "";
  };

  const setReadOnly = (readOnly: boolean) => {
    editor.value?.updateOptions({ readOnly });
  };

  onUnmounted(() => {
    dispose();
  });

  return { editor, container, init, dispose, setValue, getValue, setReadOnly };
}

import { defineStore } from "pinia";
import { ref } from "vue";
import { useTauriIPC } from "../composables/useTauriIPC";

interface Setting {
  key: string;
  value: string | null;
}

const DEFAULT_SETTINGS: Record<string, string> = {
  editor_font: "JetBrains Mono",
  editor_font_size: "14",
  editor_line_height: "1.6",
  auto_save_interval: "30",
  default_model_id: "",
  ai_temperature: "0.7",
  ai_max_tokens: "2000",
  ai_system_prompt: "你是一个专业的中文小说写作助手。",
  default_export_format: "txt",
  export_path: "",
  export_include_metadata: "true",
};

export const useSettingsStore = defineStore("settings", () => {
  const { call } = useTauriIPC();
  const settings = ref<Map<string, string>>(new Map());
  const loading = ref(false);

  const fetchSettings = async () => {
    loading.value = true;
    try {
      const list = await call<Setting[]>("get_settings");
      settings.value = new Map(list.map((s) => [s.key, s.value || ""]));
    } finally {
      loading.value = false;
    }
  };

  const updateSetting = async (key: string, value: string) => {
    await call("update_settings", { key, value });
    settings.value.set(key, value);
  };

  const getSetting = (key: string): string => {
    return settings.value.get(key) || "";
  };

  const initializeDefaults = async () => {
    for (const [key, value] of Object.entries(DEFAULT_SETTINGS)) {
      if (!settings.value.has(key)) {
        await updateSetting(key, value);
      }
    }
  };

  return {
    settings,
    loading,
    fetchSettings,
    updateSetting,
    getSetting,
    initializeDefaults,
  };
});

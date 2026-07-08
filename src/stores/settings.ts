import { defineStore } from "pinia";
import { ref } from "vue";
import { useTauriIPC } from "../composables/useTauriIPC";

interface Setting {
  key: string;
  value: string | null;
}

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
    await call("update_settings", { key, value: JSON.stringify(value) });
    settings.value.set(key, value);
  };

  const getSetting = (key: string): string => {
    return settings.value.get(key) || "";
  };

  return { settings, loading, fetchSettings, updateSetting, getSetting };
});

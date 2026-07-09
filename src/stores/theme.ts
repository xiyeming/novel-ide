import { defineStore } from "pinia";
import { ref } from "vue";
import { useTauriIPC } from "../composables/useTauriIPC";

export interface Theme {
  id: string;
  name: string;
  type: string;
  config: Record<string, string>;
  isActive: boolean;
}

const defaultThemeConfig: Record<string, string> = {
  "--bg-primary": "#0f172a",
  "--bg-secondary": "#1e293b",
  "--bg-tertiary": "#334155",
  "--text-primary": "#f8fafc",
  "--text-secondary": "#94a3b8",
  "--accent-primary": "#3b82f6",
  "--accent-secondary": "#60a5fa",
  "--border-color": "#334155",
  "--shadow-color": "rgba(0,0,0,0.3)",
};

export const useThemeStore = defineStore("theme", () => {
  const { call } = useTauriIPC();
  const themes = ref<Theme[]>([]);
  const activeTheme = ref<Theme | null>(null);
  const loading = ref(false);

  const fetchThemes = async () => {
    loading.value = true;
    try {
      themes.value = await call<Theme[]>("list_themes");
      activeTheme.value = themes.value.find((t) => t.isActive) || null;
      applyTheme(activeTheme.value);
    } finally {
      loading.value = false;
    }
  };

  const setActiveTheme = async (id: string) => {
    await call("set_active_theme", { id });
    const theme = themes.value.find((t) => t.id === id);
    if (theme) {
      activeTheme.value = theme;
      applyTheme(theme);
    }
  };

  const createCustomTheme = async (name: string, config: Record<string, string>) => {
    const result = await call<Theme>("create_custom_theme", { name, config });
    themes.value.push(result);
    return result;
  };

  const deleteTheme = async (id: string) => {
    await call("delete_theme", { id });
    themes.value = themes.value.filter((t) => t.id !== id);
  };

  const applyTheme = (theme: Theme | null) => {
    const root = document.documentElement;
    root.classList.remove("theme-dark", "theme-light");

    if (theme?.type === "light") {
      root.classList.add("theme-light");
    } else {
      root.classList.add("theme-dark");
    }

    const config = theme?.config ?? {};
    for (const [key, value] of Object.entries({ ...defaultThemeConfig, ...config })) {
      root.style.setProperty(key, value);
    }
  };

  const getThemePreview = (themeId?: string): Record<string, string> => {
    const theme = themeId
      ? themes.value.find((t) => t.id === themeId)
      : activeTheme.value;
    return { ...defaultThemeConfig, ...(theme?.config ?? {}) };
  };

  return {
    themes,
    activeTheme,
    loading,
    fetchThemes,
    setActiveTheme,
    createCustomTheme,
    deleteTheme,
    applyTheme,
    getThemePreview,
  };
});

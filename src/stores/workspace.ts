import { defineStore } from "pinia";
import { ref } from "vue";

interface WorkspacePreset {
  id: string;
  name: string;
  icon: string;
  layout: {
    sidebar: { visible: boolean; width: number };
    editor: { visible: boolean; width: string };
    aiStudio: { visible: boolean; width: number };
    inspector: { visible: boolean; width: number; type: 'chapter' | 'character' | 'world' | 'prompt' | 'workflow' };
    bottomPanel: { visible: boolean; height: number };
  };
}

const defaultPresets: WorkspacePreset[] = [
  {
    id: 'writing',
    name: '写作',
    icon: '✍️',
    layout: {
      sidebar: { visible: true, width: 280 },
      editor: { visible: true, width: '80%' },
      aiStudio: { visible: true, width: 320 },
      inspector: { visible: false, width: 280, type: 'chapter' },
      bottomPanel: { visible: false, height: 200 },
    },
  },
  {
    id: 'review',
    name: '审稿',
    icon: '📝',
    layout: {
      sidebar: { visible: true, width: 280 },
      editor: { visible: true, width: '50%' },
      aiStudio: { visible: false, width: 420 },
      inspector: { visible: true, width: 280, type: 'chapter' },
      bottomPanel: { visible: true, height: 200 },
    },
  },
  {
    id: 'outline',
    name: '大纲',
    icon: '📋',
    layout: {
      sidebar: { visible: true, width: 280 },
      editor: { visible: true, width: '60%' },
      aiStudio: { visible: false, width: 420 },
      inspector: { visible: true, width: 320, type: 'chapter' },
      bottomPanel: { visible: false, height: 200 },
    },
  },
  {
    id: 'character',
    name: '角色',
    icon: '👤',
    layout: {
      sidebar: { visible: true, width: 280 },
      editor: { visible: true, width: '50%' },
      aiStudio: { visible: false, width: 420 },
      inspector: { visible: true, width: 320, type: 'character' },
      bottomPanel: { visible: false, height: 200 },
    },
  },
  {
    id: 'world',
    name: '世界',
    icon: '🌍',
    layout: {
      sidebar: { visible: true, width: 280 },
      editor: { visible: true, width: '50%' },
      aiStudio: { visible: false, width: 420 },
      inspector: { visible: true, width: 320, type: 'world' },
      bottomPanel: { visible: false, height: 200 },
    },
  },
  {
    id: 'research',
    name: '研究',
    icon: '📚',
    layout: {
      sidebar: { visible: true, width: 280 },
      editor: { visible: true, width: '40%' },
      aiStudio: { visible: true, width: 320 },
      inspector: { visible: false, width: 280, type: 'chapter' },
      bottomPanel: { visible: true, height: 200 },
    },
  },
  {
    id: 'ai',
    name: 'AI',
    icon: '🤖',
    layout: {
      sidebar: { visible: true, width: 280 },
      editor: { visible: true, width: '40%' },
      aiStudio: { visible: true, width: 460 },
      inspector: { visible: false, width: 280, type: 'chapter' },
      bottomPanel: { visible: false, height: 200 },
    },
  },
  {
    id: 'zen',
    name: '专注',
    icon: '🧘',
    layout: {
      sidebar: { visible: false, width: 0 },
      editor: { visible: true, width: '100%' },
      aiStudio: { visible: false, width: 0 },
      inspector: { visible: false, width: 0, type: 'chapter' },
      bottomPanel: { visible: false, height: 0 },
    },
  },
];

export const useWorkspaceStore = defineStore("workspace", () => {
  const presets = ref<WorkspacePreset[]>(defaultPresets);
  const activePresetId = ref<string>("writing");
  const activePreset = ref<WorkspacePreset>(defaultPresets[0]);

  const setActivePreset = (presetId: string) => {
    const preset = presets.value.find((p) => p.id === presetId);
    if (preset) {
      activePresetId.value = presetId;
      activePreset.value = preset;
    }
  };

  const updateLayout = (layout: Partial<WorkspacePreset['layout']>) => {
    activePreset.value.layout = {
      ...activePreset.value.layout,
      ...layout,
    };
  };

  return {
    presets,
    activePresetId,
    activePreset,
    setActivePreset,
    updateLayout,
  };
});

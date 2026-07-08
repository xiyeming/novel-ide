import { defineStore } from "pinia";
import { ref } from "vue";
import { useTauriIPC } from "../composables/useTauriIPC";

export interface ModelProvider {
  id: string;
  name: string;
  provider_type: string;
  api_url: string;
  api_key: string | null;
  model_name: string;
  is_default: boolean;
  config: string | null;
  created_at: string;
  updated_at: string;
}

export const useModelStore = defineStore("model", () => {
  const { call } = useTauriIPC();
  const providers = ref<ModelProvider[]>([]);
  const loading = ref(false);

  const fetchProviders = async () => {
    loading.value = true;
    try {
      providers.value = await call<ModelProvider[]>("list_providers");
    } finally {
      loading.value = false;
    }
  };

  const createProvider = async (params: {
    name: string;
    provider_type: string;
    api_url: string;
    api_key?: string;
    model_name: string;
    is_default?: boolean;
  }) => {
    const provider = await call<ModelProvider>("create_provider", params);
    providers.value.push(provider);
    return provider;
  };

  const updateProvider = async (id: string, params: Record<string, unknown>) => {
    const provider = await call<ModelProvider>("update_provider", { id, ...params });
    const idx = providers.value.findIndex((p) => p.id === id);
    if (idx !== -1) providers.value[idx] = provider;
  };

  const deleteProvider = async (id: string) => {
    await call("delete_provider", { id });
    providers.value = providers.value.filter((p) => p.id !== id);
  };

  const testConnection = async (id: string) => {
    return await call<{ success: boolean; message: string }>("test_connection", { id });
  };

  return { providers, loading, fetchProviders, createProvider, updateProvider, deleteProvider, testConnection };
});

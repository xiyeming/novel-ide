import { defineStore } from "pinia";
import { ref } from "vue";
import { useTauriIPC } from "../composables/useTauriIPC";

export interface CloudConfig {
  id: string;
  name: string;
  provider_type: string;
  is_active: boolean;
}

export const useCloudStore = defineStore("cloud", () => {
  const { call } = useTauriIPC();
  const configs = ref<CloudConfig[]>([]);
  const loading = ref(false);

  const fetchConfigs = async () => {
    loading.value = true;
    try {
      configs.value = await call<CloudConfig[]>("list_cloud_configs");
    } finally {
      loading.value = false;
    }
  };

  const createConfig = async (name: string, providerType: string, config: Record<string, unknown>) => {
    const result = await call<CloudConfig>("create_cloud_config", {
      name,
      providerType,
      config,
    });
    configs.value.unshift(result);
    return result;
  };

  const upload = async (configId: string, localPath: string, remotePath: string) => {
    await call("upload_to_cloud", { configId, localPath, remotePath });
  };

  const download = async (configId: string, remotePath: string, localPath: string) => {
    await call("download_from_cloud", { configId, remotePath, localPath });
  };

  return {
    configs,
    loading,
    fetchConfigs,
    createConfig,
    upload,
    download,
  };
});

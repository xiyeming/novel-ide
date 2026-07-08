import { invoke } from "@tauri-apps/api/core";

export function useTauriIPC() {
  const call = async <T>(command: string, args?: Record<string, unknown>): Promise<T> => {
    return invoke<T>(command, args);
  };

  return { call };
}

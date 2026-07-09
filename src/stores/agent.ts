import { defineStore } from "pinia";
import { ref } from "vue";
import { useTauriIPC } from "../composables/useTauriIPC";

export interface Agent {
  id: string;
  name: string;
  role: string;
  system_prompt: string;
  model_provider_id?: string;
  temperature: number;
  max_tokens: number;
  knowledge_base_ids: string[];
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export const useAgentStore = defineStore("agent", () => {
  const { call } = useTauriIPC();
  const agents = ref<Agent[]>([]);
  const loading = ref(false);

  const fetchAgents = async () => {
    loading.value = true;
    try {
      agents.value = await call<Agent[]>("list_agents");
    } finally {
      loading.value = false;
    }
  };

  const createAgent = async (data: {
    name: string;
    role: string;
    systemPrompt: string;
    modelProviderId?: string;
    temperature: number;
    maxTokens: number;
    knowledgeBaseIds: string[];
  }) => {
    const agent = await call<Agent>("create_agent", {
      name: data.name,
      role: data.role,
      systemPrompt: data.systemPrompt,
      modelProviderId: data.modelProviderId,
      temperature: data.temperature,
      maxTokens: data.maxTokens,
      knowledgeBaseIds: data.knowledgeBaseIds,
    });
    agents.value.unshift(agent);
    return agent;
  };

  const updateAgent = async (id: string, data: Partial<Agent>) => {
    const agent = await call<Agent>("update_agent", { id, ...data });
    const idx = agents.value.findIndex((a) => a.id === id);
    if (idx !== -1) agents.value[idx] = agent;
    return agent;
  };

  const deleteAgent = async (id: string) => {
    await call("delete_agent", { id });
    agents.value = agents.value.filter((a) => a.id !== id);
  };

  return {
    agents,
    loading,
    fetchAgents,
    createAgent,
    updateAgent,
    deleteAgent,
  };
});

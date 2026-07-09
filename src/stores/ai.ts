import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useTauriIPC } from "../composables/useTauriIPC";

interface Message {
  id: string;
  role: "user" | "assistant";
  content: string;
  timestamp: number;
}

export const useAIStore = defineStore("ai", () => {
  const { call } = useTauriIPC();
  const messages = ref<Message[]>([]);
  const selectedModel = ref<string>("deepseek-chat");
  const streaming = ref(false);
  const activeAIFeature = ref<string | null>(null);

  function setActiveFeature(feature: string) {
    activeAIFeature.value = feature;
  }

  function clearActiveFeature() {
    activeAIFeature.value = null;
  }

  const addMessage = (role: "user" | "assistant", content: string) => {
    messages.value.push({
      id: crypto.randomUUID(),
      role,
      content,
      timestamp: Date.now(),
    });
  };

  const clearMessages = () => {
    messages.value = [];
  };

  const sendMessage = async (content: string) => {
    if (!content.trim() || streaming.value) return;

    addMessage("user", content);

    const assistantId = crypto.randomUUID();
    messages.value.push({
      id: assistantId,
      role: "assistant",
      content: "",
      timestamp: Date.now(),
    });

    streaming.value = true;

    try {
      const apiMessages = messages.value
        .filter((m) => m.id !== assistantId)
        .map((m) => ({ role: m.role, content: m.content }));

      let unlisten: (() => void) | undefined;

      unlisten = await listen<{ content: string; done: boolean }>(
        "ai:chunk",
        (event) => {
          const msg = messages.value.find((m) => m.id === assistantId);
          if (msg) {
            msg.content += event.payload.content;
          }
          if (event.payload.done) {
            streaming.value = false;
            unlisten?.();
          }
        }
      );

      await invoke("chat_stream", {
        providerId: selectedModel.value,
        messages: apiMessages,
      });
    } catch (error) {
      const msg = messages.value.find((m) => m.id === assistantId);
      if (msg) {
        msg.content = `Error: ${error}`;
      }
      streaming.value = false;
    }
  };

  async function continueWriting(content: string, providerId: string, style?: string) {
    return await call<string>("continue_writing", { content, providerId, style });
  }

  async function rewriteContent(content: string, providerId: string, instruction: string) {
    return await call<string>("rewrite_content", { content, providerId, instruction });
  }

  async function expandContent(content: string, providerId: string, targetWords?: number) {
    return await call<string>("expand_content", { content, providerId, targetWords });
  }

  async function condenseContent(content: string, providerId: string) {
    return await call<string>("condense_content", { content, providerId });
  }

  async function styleTransfer(content: string, providerId: string, targetStyle: string) {
    return await call<string>("style_transfer", { content, providerId, targetStyle });
  }

  return {
    messages,
    selectedModel,
    streaming,
    activeAIFeature,
    setActiveFeature,
    clearActiveFeature,
    addMessage,
    clearMessages,
    sendMessage,
    continueWriting,
    rewriteContent,
    expandContent,
    condenseContent,
    styleTransfer,
  };
});

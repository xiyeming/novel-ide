import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface Message {
  id: string;
  role: "user" | "assistant";
  content: string;
  timestamp: number;
}

export const useAIStore = defineStore("ai", () => {
  const messages = ref<Message[]>([]);
  const selectedModel = ref<string>("deepseek-chat");
  const streaming = ref(false);

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

  return {
    messages,
    selectedModel,
    streaming,
    addMessage,
    clearMessages,
    sendMessage,
  };
});

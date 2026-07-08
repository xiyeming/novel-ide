import { defineStore } from "pinia";
import { ref } from "vue";

interface Message {
  id: string;
  role: "user" | "assistant";
  content: string;
  timestamp: number;
}

export const useAIStore = defineStore("ai", () => {
  const messages = ref<Message[]>([]);
  const selectedModel = ref<string>("deepseek-chat");
  const isGenerating = ref(false);

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

  return { messages, selectedModel, isGenerating, addMessage, clearMessages };
});

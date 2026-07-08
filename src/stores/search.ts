import { defineStore } from "pinia";
import { ref } from "vue";
import { useTauriIPC } from "../composables/useTauriIPC";

interface SearchResult {
  chapter_id: string;
  chapter_title: string;
  snippet: string;
  rank: number;
}

export const useSearchStore = defineStore("search", () => {
  const { call } = useTauriIPC();
  const query = ref("");
  const results = ref<SearchResult[]>([]);
  const loading = ref(false);

  const search = async (projectId: string, q: string) => {
    query.value = q;
    if (!q.trim()) {
      results.value = [];
      return;
    }
    loading.value = true;
    try {
      results.value = await call<SearchResult[]>("searchChapters", {
        projectId,
        query: q,
      });
    } finally {
      loading.value = false;
    }
  };

  const clear = () => {
    query.value = "";
    results.value = [];
  };

  return { query, results, loading, search, clear };
});

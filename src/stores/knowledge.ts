import { defineStore } from "pinia";
import { ref } from "vue";
import { useTauriIPC } from "../composables/useTauriIPC";

export interface KnowledgeDocument {
  id: string;
  project_id: string;
  title: string;
  content: string;
  file_path: string | null;
  doc_type: string;
  chunk_count: number;
  created_at: string;
  updated_at: string;
}

export interface KnowledgeSearchResult {
  doc_id: string;
  title: string;
  snippet: string;
  source_type: string;
  rank: number;
}

export const useKnowledgeStore = defineStore("knowledge", () => {
  const { call } = useTauriIPC();
  const documents = ref<KnowledgeDocument[]>([]);
  const searchResults = ref<KnowledgeSearchResult[]>([]);
  const loading = ref(false);
  const searching = ref(false);

  const fetchDocuments = async (projectId: string) => {
    loading.value = true;
    try {
      documents.value = await call<KnowledgeDocument[]>("list_knowledge", {
        projectId,
      });
    } finally {
      loading.value = false;
    }
  };

  const importDocument = async (
    projectId: string,
    title: string,
    content: string,
    filePath?: string
  ) => {
    const doc = await call<KnowledgeDocument>("import_knowledge", {
      projectId,
      title,
      content,
      filePath: filePath || null,
    });
    documents.value.unshift(doc);
    return doc;
  };

  const deleteDocument = async (id: string) => {
    await call("delete_knowledge", { id });
    documents.value = documents.value.filter((d) => d.id !== id);
  };

  const search = async (projectId: string, query: string, limit?: number) => {
    if (!query.trim()) {
      searchResults.value = [];
      return;
    }
    searching.value = true;
    try {
      searchResults.value = await call<KnowledgeSearchResult[]>(
        "search_knowledge",
        {
          projectId,
          query,
          limit: limit || 20,
        }
      );
    } finally {
      searching.value = false;
    }
  };

  const clearSearch = () => {
    searchResults.value = [];
  };

  return {
    documents,
    searchResults,
    loading,
    searching,
    fetchDocuments,
    importDocument,
    deleteDocument,
    search,
    clearSearch,
  };
});

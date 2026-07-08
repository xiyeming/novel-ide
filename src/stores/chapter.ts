import { defineStore } from "pinia";
import { ref } from "vue";
import { useTauriIPC } from "../composables/useTauriIPC";

interface Chapter {
  id: string;
  project_id: string;
  title: string;
  content: string;
  sort_order: number;
  word_count: number;
  created_at: string;
  updated_at: string;
}

export const useChapterStore = defineStore("chapter", () => {
  const { call } = useTauriIPC();
  const chapters = ref<Chapter[]>([]);
  const currentChapter = ref<Chapter | null>(null);
  const loading = ref(false);

  const fetchChapters = async (projectId: string) => {
    loading.value = true;
    try {
      chapters.value = await call<Chapter[]>("list_chapters", { projectId });
    } finally {
      loading.value = false;
    }
  };

  const createChapter = async (
    projectId: string,
    title?: string,
    sortOrder?: number
  ) => {
    const chapter = await call<Chapter>("create_chapter", {
      projectId,
      title,
      sortOrder,
    });
    chapters.value.push(chapter);
    chapters.value.sort((a, b) => a.sort_order - b.sort_order);
    return chapter;
  };

  const openChapter = async (chapterId: string) => {
    const chapter = await call<Chapter>("get_chapter", { chapterId });
    currentChapter.value = chapter;
    return chapter;
  };

  const updateChapterContent = async (chapterId: string, content: string) => {
    const chapter = await call<Chapter>("update_chapter", {
      chapterId,
      content,
    });
    // Update in list
    const idx = chapters.value.findIndex((c) => c.id === chapterId);
    if (idx !== -1) chapters.value[idx] = chapter;
    if (currentChapter.value?.id === chapterId) currentChapter.value = chapter;
  };

  const updateChapterTitle = async (chapterId: string, title: string) => {
    const chapter = await call<Chapter>("update_chapter", {
      chapterId,
      title,
    });
    const idx = chapters.value.findIndex((c) => c.id === chapterId);
    if (idx !== -1) chapters.value[idx] = chapter;
    if (currentChapter.value?.id === chapterId) currentChapter.value = chapter;
  };

  const deleteChapter = async (chapterId: string) => {
    await call("delete_chapter", { chapterId });
    chapters.value = chapters.value.filter((c) => c.id !== chapterId);
    if (currentChapter.value?.id === chapterId) {
      currentChapter.value = chapters.value[0] || null;
    }
  };

  return {
    chapters,
    currentChapter,
    loading,
    fetchChapters,
    createChapter,
    openChapter,
    updateChapterContent,
    updateChapterTitle,
    deleteChapter,
  };
});

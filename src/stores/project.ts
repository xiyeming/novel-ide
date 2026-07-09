import { defineStore } from "pinia";
import { ref } from "vue";
import { useTauriIPC } from "../composables/useTauriIPC";

interface Project {
  id: string;
  name: string;
  path: string;
  genre: string | null;
  sub_genre: string | null;
  target_readers: string | null;
  total_chapters: number | null;
  words_per_chapter: number | null;
  narrative_pov: string | null;
  story_structure: string | null;
  core_outline: string | null;
  world_settings: string | null;
  character_profiles: string | null;
  golden_finger: string | null;
  writing_constraints: string | null;
  style_constraints: string | null;
  created_at: string;
  updated_at: string;
}

export { type Project };

export const useProjectStore = defineStore("project", () => {
  const { call } = useTauriIPC();
  const projects = ref<Project[]>([]);
  const currentProject = ref<Project | null>(null);
  const loading = ref(false);

  const fetchProjects = async () => {
    loading.value = true;
    try {
      projects.value = await call<Project[]>("list_projects");
    } finally {
      loading.value = false;
    }
  };

  const createProject = async (params: {
    name: string;
    path: string;
    genre?: string;
    sub_genre?: string;
    subGenre?: string;
    target_readers?: string;
    targetReaders?: string;
    total_chapters?: number;
    totalChapters?: number;
    words_per_chapter?: number;
    wordsPerChapter?: number;
    narrative_pov?: string;
    narrativePov?: string;
    story_structure?: string;
    storyStructure?: string;
  }) => {
    const project = await call<Project>("create_project", params);
    projects.value.unshift(project);
    return project;
  };

  const openProject = async (projectId: string) => {
    const project = await call<Project>("open_project", { projectId });
    currentProject.value = project;
    return project;
  };

  const deleteProject = async (projectId: string) => {
    await call("delete_project", { projectId });
    projects.value = projects.value.filter((p) => p.id !== projectId);
    if (currentProject.value?.id === projectId) {
      currentProject.value = null;
    }
  };

  const updateProject = async (
    projectId: string,
    params: Partial<Omit<Project, "id" | "name" | "path" | "created_at" | "updated_at">>
  ) => {
    const project = await call<Project>("update_project", {
      projectId,
      ...params,
    });
    currentProject.value = project;
    const idx = projects.value.findIndex((p) => p.id === projectId);
    if (idx !== -1) {
      projects.value[idx] = project;
    }
    return project;
  };

  return {
    projects,
    currentProject,
    loading,
    fetchProjects,
    createProject,
    openProject,
    deleteProject,
    updateProject,
  };
});

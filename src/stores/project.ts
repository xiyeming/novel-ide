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
  created_at: string;
  updated_at: string;
}

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
    target_readers?: string;
    total_chapters?: number;
    words_per_chapter?: number;
    narrative_pov?: string;
    story_structure?: string;
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

  return {
    projects,
    currentProject,
    loading,
    fetchProjects,
    createProject,
    openProject,
    deleteProject,
  };
});

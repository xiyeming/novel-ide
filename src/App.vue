<script setup lang="ts">
import { ref } from "vue";
import IDELayout from "./components/layout/IDELayout.vue";
import ProjectList from "./components/project/ProjectList.vue";
import { useProjectStore } from "./stores/project";

const projectStore = useProjectStore();
const showIDE = ref(false);

const openProject = async (projectId: string) => {
  await projectStore.openProject(projectId);
  showIDE.value = true;
};

const backToHome = () => {
  showIDE.value = false;
  projectStore.currentProject = null;
};
</script>

<template>
  <ProjectList v-if="!showIDE" @open-project="openProject" />
  <IDELayout v-else @back="backToHome" />
</template>

<style>
html,
body,
#app {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
  font-family: "Segoe UI", "PingFang SC", "Microsoft YaHei", sans-serif;
}
</style>

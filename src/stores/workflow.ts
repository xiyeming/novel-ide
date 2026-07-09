import { defineStore } from "pinia";
import { ref } from "vue";
import { useTauriIPC } from "../composables/useTauriIPC";

export interface WorkflowStage {
  name: string;
  stage_type: string;
  agent_id?: string;
  model_provider_id?: string;
  system_prompt?: string;
  temperature: number;
  max_tokens: number;
}

export interface Workflow {
  id: string;
  name: string;
  description?: string;
  stages: WorkflowStage[];
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface WorkflowExecution {
  id: string;
  workflow_id: string;
  chapter_id: string;
  status: string;
  current_stage: number;
  results: Record<string, string>;
  error?: string;
  started_at?: string;
  completed_at?: string;
  created_at: string;
}

export const useWorkflowStore = defineStore("workflow", () => {
  const { call } = useTauriIPC();
  const workflows = ref<Workflow[]>([]);
  const currentExecution = ref<WorkflowExecution | null>(null);
  const loading = ref(false);

  const fetchWorkflows = async () => {
    loading.value = true;
    try {
      workflows.value = await call<Workflow[]>("list_workflows");
    } finally {
      loading.value = false;
    }
  };

  const createWorkflow = async (
    name: string,
    description: string,
    stages: WorkflowStage[],
  ) => {
    const workflow = await call<Workflow>("create_workflow", {
      name,
      description,
      stages,
    });
    workflows.value.unshift(workflow);
    return workflow;
  };

  const deleteWorkflow = async (id: string) => {
    await call("delete_workflow", { id });
    workflows.value = workflows.value.filter((w) => w.id !== id);
  };

  const executeWorkflow = async (workflowId: string, chapterId: string) => {
    const execution = await call<WorkflowExecution>("execute_workflow", {
      workflowId,
      chapterId,
    });
    currentExecution.value = execution;
    return execution;
  };

  return {
    workflows,
    currentExecution,
    loading,
    fetchWorkflows,
    createWorkflow,
    deleteWorkflow,
    executeWorkflow,
  };
});

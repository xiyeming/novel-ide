use novel_ide_lib::models::workflow::{Workflow, WorkflowExecution, WorkflowStage};
use std::collections::HashMap;

#[tokio::test]
async fn test_workflow_stage_creation() {
    let stage = WorkflowStage {
        name: "测试阶段".to_string(),
        stage_type: "draft".to_string(),
        agent_id: None,
        model_provider_id: Some("test-provider".to_string()),
        system_prompt: Some("测试提示".to_string()),
        temperature: 0.7,
        max_tokens: 2000,
    };

    assert_eq!(stage.name, "测试阶段");
    assert_eq!(stage.stage_type, "draft");
    assert_eq!(stage.model_provider_id.as_deref(), Some("test-provider"));
    assert_eq!(stage.system_prompt.as_deref(), Some("测试提示"));
    assert_eq!(stage.temperature, 0.7);
    assert_eq!(stage.max_tokens, 2000);
    assert!(stage.agent_id.is_none());
}

#[tokio::test]
async fn test_workflow_stage_serialization() {
    let stage = WorkflowStage {
        name: "大纲阶段".to_string(),
        stage_type: "outline".to_string(),
        agent_id: Some("agent-1".to_string()),
        model_provider_id: None,
        system_prompt: None,
        temperature: 0.5,
        max_tokens: 4000,
    };

    let json = serde_json::to_string(&stage).unwrap();
    let deserialized: WorkflowStage = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.name, "大纲阶段");
    assert_eq!(deserialized.stage_type, "outline");
    assert_eq!(deserialized.agent_id.as_deref(), Some("agent-1"));
    assert!(deserialized.model_provider_id.is_none());
    assert!(deserialized.system_prompt.is_none());
    assert_eq!(deserialized.temperature, 0.5);
    assert_eq!(deserialized.max_tokens, 4000);
}

#[tokio::test]
async fn test_workflow_creation() {
    let stages = vec![
        WorkflowStage {
            name: "大纲".to_string(),
            stage_type: "outline".to_string(),
            agent_id: None,
            model_provider_id: Some("provider-1".to_string()),
            system_prompt: None,
            temperature: 0.7,
            max_tokens: 2000,
        },
        WorkflowStage {
            name: "初稿".to_string(),
            stage_type: "draft".to_string(),
            agent_id: None,
            model_provider_id: Some("provider-1".to_string()),
            system_prompt: Some("写初稿".to_string()),
            temperature: 0.8,
            max_tokens: 5000,
        },
    ];

    let workflow = Workflow {
        id: "wf-001".to_string(),
        name: "标准写作流程".to_string(),
        description: Some("包含大纲和初稿的流程".to_string()),
        stages,
        is_active: true,
        created_at: "2026-01-01T00:00:00Z".to_string(),
        updated_at: "2026-01-01T00:00:00Z".to_string(),
    };

    assert_eq!(workflow.id, "wf-001");
    assert_eq!(workflow.name, "标准写作流程");
    assert_eq!(workflow.stages.len(), 2);
    assert!(workflow.is_active);
}

#[tokio::test]
async fn test_workflow_serialization() {
    let workflow = Workflow {
        id: "wf-002".to_string(),
        name: "测试流程".to_string(),
        description: None,
        stages: vec![WorkflowStage {
            name: "校对".to_string(),
            stage_type: "proofread".to_string(),
            agent_id: None,
            model_provider_id: None,
            system_prompt: Some("校对文本".to_string()),
            temperature: 0.3,
            max_tokens: 1000,
        }],
        is_active: false,
        created_at: "2026-06-01T00:00:00Z".to_string(),
        updated_at: "2026-06-01T00:00:00Z".to_string(),
    };

    let json = serde_json::to_string(&workflow).unwrap();
    let deserialized: Workflow = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.id, "wf-002");
    assert_eq!(deserialized.name, "测试流程");
    assert!(deserialized.description.is_none());
    assert_eq!(deserialized.stages.len(), 1);
    assert_eq!(deserialized.stages[0].name, "校对");
    assert_eq!(deserialized.stages[0].stage_type, "proofread");
    assert!(!deserialized.is_active);
}

#[tokio::test]
async fn test_workflow_execution_creation() {
    let mut results = HashMap::new();
    results.insert("大纲".to_string(), "大纲内容".to_string());
    results.insert("初稿".to_string(), "初稿内容".to_string());

    let execution = WorkflowExecution {
        id: "exec-001".to_string(),
        workflow_id: "wf-001".to_string(),
        chapter_id: "ch-001".to_string(),
        status: "completed".to_string(),
        current_stage: 2,
        results,
        error: None,
        started_at: Some("2026-01-01T00:00:00Z".to_string()),
        completed_at: Some("2026-01-01T01:00:00Z".to_string()),
        created_at: "2026-01-01T00:00:00Z".to_string(),
    };

    assert_eq!(execution.id, "exec-001");
    assert_eq!(execution.workflow_id, "wf-001");
    assert_eq!(execution.chapter_id, "ch-001");
    assert_eq!(execution.status, "completed");
    assert_eq!(execution.current_stage, 2);
    assert_eq!(execution.results.len(), 2);
    assert!(execution.error.is_none());
}

#[tokio::test]
async fn test_workflow_execution_serialization() {
    let execution = WorkflowExecution {
        id: "exec-002".to_string(),
        workflow_id: "wf-002".to_string(),
        chapter_id: "ch-002".to_string(),
        status: "failed".to_string(),
        current_stage: 1,
        results: HashMap::new(),
        error: Some("模型调用失败".to_string()),
        started_at: None,
        completed_at: None,
        created_at: "2026-06-01T00:00:00Z".to_string(),
    };

    let json = serde_json::to_string(&execution).unwrap();
    let deserialized: WorkflowExecution = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.status, "failed");
    assert_eq!(deserialized.error.as_deref(), Some("模型调用失败"));
    assert!(deserialized.started_at.is_none());
    assert!(deserialized.completed_at.is_none());
}

#[tokio::test]
async fn test_workflow_stage_types() {
    let stage_types = vec!["outline", "draft", "proofread", "edit", "custom"];

    for stage_type in stage_types {
        let stage = WorkflowStage {
            name: format!("{}阶段", stage_type),
            stage_type: stage_type.to_string(),
            agent_id: None,
            model_provider_id: None,
            system_prompt: None,
            temperature: 0.7,
            max_tokens: 2000,
        };

        assert_eq!(stage.stage_type, stage_type);
    }
}

#[tokio::test]
async fn test_workflow_execution_status_transitions() {
    let statuses = vec!["pending", "running", "completed", "failed"];

    for status in statuses {
        let execution = WorkflowExecution {
            id: "exec-test".to_string(),
            workflow_id: "wf-test".to_string(),
            chapter_id: "ch-test".to_string(),
            status: status.to_string(),
            current_stage: 0,
            results: HashMap::new(),
            error: None,
            started_at: None,
            completed_at: None,
            created_at: "2026-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(execution.status, status);
    }
}

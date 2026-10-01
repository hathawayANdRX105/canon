//! flow MCP 工具面：16 个工具（读 4 + 写 8 + 规范 4），JSON schema 定义 +
//! 分发。wire 层在 `mcp` crate（手写 JSON-RPC），本模块只做参数解析与
//! 结果 JSON 组装；所有业务在本 crate 的 Store 上。

use crate::{FlowError, FlowResult, Store};
use serde_json::{Value, json};

pub const TOOL_NAMES: &[&str] = &[
    // 读
    "board_view",
    "task_get",
    "project_list",
    "journal",
    // 写
    "project_create",
    "project_update",
    "task_create",
    "task_claim",
    "task_transition",
    "step_add",
    "step_mark",
    "event_note",
    // 规范
    "spec_list",
    "spec_run",
    "spec_bind",
    "template_list",
];

pub fn is_flow_tool(name: &str) -> bool {
    TOOL_NAMES.contains(&name)
}

/// mcp.rs 分发入口：开缺省库、跑工具、返回 pretty JSON 文本。
pub fn call(name: &str, args: &Value) -> Result<String, (i64, String)> {
    let store = Store::open_default().map_err(rpc_err)?;
    let out = dispatch(&store, name, args).map_err(rpc_err)?;
    Ok(serde_json::to_string_pretty(&out).unwrap_or_default())
}

fn rpc_err(e: FlowError) -> (i64, String) {
    (-32000, format!("{}: {}", e.code, e.msg))
}

fn dispatch(s: &Store, name: &str, a: &Value) -> Result<Value, FlowError> {
    let out: Value = match name {
        "board_view" => board_view(s, a)?,
        "task_get" => {
            let task = req_str(a, "task")?;
            json!(s.task_get(&task)?)
        }
        "project_list" => json!(s.projects()?),
        "journal" => {
            let task = req_str(a, "task")?;
            let limit = opt_usize(a, "limit").unwrap_or(100);
            json!(s.journal(&task, limit)?)
        }
        "project_create" => {
            let name = req_str(a, "name")?;
            let repo_path = opt_str(a, "repo_path").unwrap_or_else(|| ".".into());
            let kind = opt_str(a, "kind").unwrap_or_else(|| "general".into());
            let states = opt_str_array(a, "states");
            json!(s.project_create(&name, &repo_path, &kind, states.as_deref())?)
        }
        "project_update" => {
            let sel = req_str(a, "project")?;
            let rename = opt_str(a, "rename");
            let states = opt_str_array(a, "states");
            json!(s.project_update(&sel, rename.as_deref(), states.as_deref())?)
        }
        "task_create" => {
            let project = req_str(a, "project")?;
            let title = req_str(a, "title")?;
            let tpl = opt_str(a, "from_template");
            let priority = opt_i64(a, "priority").unwrap_or(0);
            json!(s.task_create(&project, &title, tpl.as_deref(), priority)?)
        }
        "task_claim" => {
            let task = req_str(a, "task")?;
            let claimant = req_str(a, "claimant")?;
            json!(s.claim(&task, &claimant)?)
        }
        "task_transition" => {
            let task = req_str(a, "task")?;
            let to = req_str(a, "to")?;
            let reason = opt_str(a, "reason").unwrap_or_default();
            let actor = opt_str(a, "actor").unwrap_or_else(|| "agent".into());
            json!(s.transition(&task, &to, &reason, &actor)?)
        }
        "step_add" => {
            let task = req_str(a, "task")?;
            let title = req_str(a, "title")?;
            let kind = opt_str(a, "kind").unwrap_or_else(|| "work".into());
            let spec_ref = opt_str(a, "spec_ref");
            let hint = opt_str(a, "state_hint");
            let actor = opt_str(a, "actor").unwrap_or_else(|| "agent".into());
            json!(s.step_add(
                &task,
                &title,
                &kind,
                spec_ref.as_deref(),
                hint.as_deref(),
                &actor
            )?)
        }
        "step_mark" => {
            let task = req_str(a, "task")?;
            let step = req_i64(a, "step")?;
            let evidence = opt_str(a, "evidence");
            let actor = opt_str(a, "actor").unwrap_or_else(|| "agent".into());
            json!(s.step_mark(&task, step, evidence.as_deref(), &actor)?)
        }
        "event_note" => {
            let task = req_str(a, "task")?;
            let text = req_str(a, "text")?;
            let actor = opt_str(a, "actor").unwrap_or_else(|| "agent".into());
            json!(s.note(&task, &text, &actor)?)
        }
        "spec_list" => {
            let project = req_str(a, "project")?;
            let rules = s.spec_rules(&project)?;
            json!({
                "project": project,
                "count": rules.len(),
                "rules": rules.iter().map(rule_json).collect::<Vec<_>>(),
            })
        }
        "spec_run" => {
            let project = req_str(a, "project")?;
            let task = opt_str(a, "task");
            let step = opt_i64(a, "step");
            let names = opt_str_array(a, "names").unwrap_or_default();
            let sla = opt_str(a, "sla").unwrap_or_else(|| "l1".into());
            let actor = opt_str(a, "actor").unwrap_or_else(|| "agent".into());
            s.spec_run(&project, task.as_deref(), step, names, &sla, &actor)?
        }
        "spec_bind" => {
            let task = req_str(a, "task")?;
            let step = req_i64(a, "step")?;
            let spec_ref = req_str(a, "spec_ref")?;
            let actor = opt_str(a, "actor").unwrap_or_else(|| "agent".into());
            json!(s.step_bind(&task, step, &spec_ref, &actor)?)
        }
        "template_list" => template_list()?,
        other => {
            return Err(FlowError::new(
                "UNKNOWN_TOOL",
                format!("unknown flow tool {other}"),
            ));
        }
    };
    Ok(out)
}

fn board_view(s: &Store, a: &Value) -> Result<Value, FlowError> {
    let project = opt_str(a, "project");
    let state = opt_str(a, "state");
    let mut projects = s.projects()?;
    if let Some(p) = &project {
        projects.retain(|pr| &pr.name == p || pr.id.to_string() == *p);
    }
    let panels = projects
        .iter()
        .map(|p| {
            let tasks = s
                .tasks_by_project(p.id)?
                .into_iter()
                .filter(|t| state.as_deref().is_none_or(|st| t.state == st))
                .map(|t| {
                    let (done, total) = s.step_stats(t.id)?;
                    Ok(json!({
                        "id": t.id,
                        "title": t.title,
                        "state": t.state,
                        "claimant": t.claimant,
                        "priority": t.priority,
                        "steps_done": done,
                        "steps_total": total,
                        "updated_at": t.updated_at,
                    }))
                })
                .collect::<Result<Vec<_>, FlowError>>()?;
            Ok(json!({
                "id": p.id,
                "name": p.name,
                "kind": p.kind,
                "states": p.states,
                "tasks": tasks,
            }))
        })
        .collect::<Result<Vec<_>, FlowError>>()?;
    Ok(json!({ "projects": panels }))
}

fn template_list() -> Result<Value, FlowError> {
    let names = crate::templates::available();
    let details = names
        .iter()
        .filter_map(|n| crate::templates::resolve(n).ok())
        .map(|t| {
            json!({
                "name": t.name,
                "states": t.states,
                "step_count": t.steps.len(),
            })
        })
        .collect::<Vec<_>>();
    Ok(json!({ "templates": names, "details": details }))
}

/// spec_list 行：与 mcp 的 spec_catalog 同款（id/severity/sla/hooks/mode/why）。
fn rule_json(r: &gate::catalog::Rule) -> Value {
    json!({
        "rule_id": r.id,
        "severity": r.severity.as_str(),
        "sla": r.sla,
        "hooks": r.hooks,
        "mode": r.mode,
        "source": r.source,
        "why": r.why.lines().next().unwrap_or(""),
    })
}

// ---- 参数小件 ----

fn req_str(a: &Value, k: &str) -> FlowResult<String> {
    a.get(k)
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| FlowError::new("BAD_INPUT", format!("missing string argument {k}")))
}

fn opt_str(a: &Value, k: &str) -> Option<String> {
    a.get(k).and_then(Value::as_str).map(str::to_string)
}

fn req_i64(a: &Value, k: &str) -> FlowResult<i64> {
    opt_i64(a, k)
        .ok_or_else(|| FlowError::new("BAD_INPUT", format!("missing integer argument {k}")))
}

/// 数字或数字串都收（MCP 客户端 JSON 编码不保真）。
fn opt_i64(a: &Value, k: &str) -> Option<i64> {
    let v = a.get(k)?;
    if let Some(n) = v.as_i64() {
        return Some(n);
    }
    v.as_str()?.parse().ok()
}

fn opt_usize(a: &Value, k: &str) -> Option<usize> {
    opt_i64(a, k).map(|n| n.max(0) as usize)
}

fn opt_str_array(a: &Value, k: &str) -> Option<Vec<String>> {
    a.get(k).and_then(Value::as_array).map(|arr| {
        arr.iter()
            .filter_map(Value::as_str)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect()
    })
}

// ---- tools/list 的 JSON schema 定义 ----

fn str_prop(desc: &str) -> Value {
    json!({"type": "string", "description": desc})
}

pub fn tool_defs() -> Vec<Value> {
    vec![
        json!({
            "name": "board_view",
            "description": "Kanban snapshot grouped by project: every project panel with its tasks (state, claimant, step progress). Filter by project or task state.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project": str_prop("Project id or name (omit for all)"),
                    "state": str_prop("Only tasks in this state")
                },
                "required": []
            }
        }),
        json!({
            "name": "task_get",
            "description": "One task with its project pipeline, all steps, and the legal transitions from its current state.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "task": str_prop("Task selector: numeric id, or an exact title")
                },
                "required": ["task"]
            }
        }),
        json!({
            "name": "project_list",
            "description": "List every registered project with its state pipeline.",
            "inputSchema": {"type": "object", "properties": {}, "required": []}
        }),
        json!({
            "name": "journal",
            "description": "A task's append-only workflow record: state transitions, step completions, spec runs, notes — newest `limit` events in time order (P2 timeline data source).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "task": str_prop("Task selector: id or exact title"),
                    "limit": {"type": "integer", "description": "Max events (default 100)"}
                },
                "required": ["task"]
            }
        }),
        json!({
            "name": "project_create",
            "description": "Register a project. `states` is its ordered workflow pipeline (kanban columns); omitted → the kind template's default pipeline.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "name": str_prop("Unique project name"),
                    "repo_path": str_prop("Repo root for spec runs (default: .)"),
                    "kind": {"type": "string", "enum": ["backend", "frontend", "general"], "description": "Picks the default template (default general)"},
                    "states": {"type": "array", "items": {"type": "string"}, "description": "Custom state pipeline, e.g. [规划, 开发, 审查, 代码清洁, 完成]"}
                },
                "required": ["name"]
            }
        }),
        json!({
            "name": "project_update",
            "description": "Rename a project and/or replace its state pipeline. Renamed/dropped states migrate existing task.state values (recorded in each task's journal).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project": str_prop("Project id or name"),
                    "rename": str_prop("New project name"),
                    "states": {"type": "array", "items": {"type": "string"}, "description": "New ordered pipeline"}
                },
                "required": ["project"]
            }
        }),
        json!({
            "name": "task_create",
            "description": "Create a task at the pipeline's first state; `from_template` lays out a todo checklist (built-in template name or a user template).",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project": str_prop("Project id or name"),
                    "title": str_prop("Task title"),
                    "from_template": {"type": "string", "description": "Template name (backend/frontend/general or a user file)"},
                    "priority": {"type": "integer"}
                },
                "required": ["project", "title"]
            }
        }),
        json!({
            "name": "task_claim",
            "description": "Claim a task: writes the claimant; the task stays in its current state. A different claimant cannot steal it.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "task": str_prop("Task selector: id or exact title"),
                    "claimant": str_prop("Who claims it (agent or human name)")
                },
                "required": ["task", "claimant"]
            }
        }),
        json!({
            "name": "task_transition",
            "description": "Move a task along its pipeline: forward only to the next state, backward to any earlier state (kick-back). Entering the terminal state requires every step done.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "task": str_prop("Task selector: id or exact title"),
                    "to": str_prop("Target state name (must be in the project pipeline)"),
                    "reason": str_prop("Why (recorded in the journal)"),
                    "actor": str_prop("Who moved it (default agent)")
                },
                "required": ["task", "to"]
            }
        }),
        json!({
            "name": "step_add",
            "description": "Append a todo step to a task. kind: work/spec/record; spec steps require spec_ref (checklist name, or 'preflight').",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "task": str_prop("Task selector: id or exact title"),
                    "title": str_prop("Step title"),
                    "kind": {"type": "string", "enum": ["work", "spec", "record"]},
                    "spec_ref": str_prop("Checklist name for spec steps"),
                    "state_hint": str_prop("Pipeline state this step belongs to (P2 panel)")
                },
                "required": ["task", "title"]
            }
        }),
        json!({
            "name": "step_mark",
            "description": "Mark a step done (irreversible). Spec steps refuse without evidence — run spec_run first, it attaches the result automatically.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "task": str_prop("Task selector: id or exact title"),
                    "step": {"type": "integer", "description": "Step id"},
                    "evidence": str_prop("Completion evidence (required for spec steps)"),
                    "actor": str_prop("Who closed it (default agent)")
                },
                "required": ["task", "step"]
            }
        }),
        json!({
            "name": "event_note",
            "description": "Record a free-form decision/note on a task's journal timeline.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "task": str_prop("Task selector: id or exact title"),
                    "text": str_prop("Note text"),
                    "actor": str_prop("Who wrote it (default agent)")
                },
                "required": ["task", "text"]
            }
        }),
        json!({
            "name": "spec_list",
            "description": "List the checklists a project's spec tree enforces (id, severity, sla, hooks, why). Call before binding spec steps.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project": str_prop("Project id or name")
                },
                "required": ["project"]
            }
        }),
        json!({
            "name": "spec_run",
            "description": "Run checklists in the project's repo now (in-process engine, cwd-anchored). `names` empty = everything under the sla ceiling. With task/step args the result is journaled and attached as the step's evidence.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "project": str_prop("Project id or name"),
                    "task": str_prop("Task to journal the run against"),
                    "step": {"type": "integer", "description": "Spec step to attach the result to"},
                    "names": {"type": "array", "items": {"type": "string"}, "description": "Checklist names (empty = all under sla)"},
                    "sla": {"type": "string", "enum": ["l1", "l2", "l3"]},
                    "actor": str_prop("Who ran it (default agent)")
                },
                "required": ["project"]
            }
        }),
        json!({
            "name": "spec_bind",
            "description": "Bind a checklist name to a step; the step becomes kind=spec and cannot be closed without evidence.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "task": str_prop("Task selector: id or exact title"),
                    "step": {"type": "integer", "description": "Step id"},
                    "spec_ref": str_prop("Checklist name to bind")
                },
                "required": ["task", "step", "spec_ref"]
            }
        }),
        json!({
            "name": "template_list",
            "description": "List available step templates (built-in backend/frontend/general + user files) with their state pipeline and step count.",
            "inputSchema": {"type": "object", "properties": {}, "required": []}
        }),
    ]
}

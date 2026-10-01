//! flow store — SQLite 持久层（feature `store`）。纯模型在 crate 根
//! （无 DB 依赖，wasm/web 只拉模型 + 模板 + 转移规则）。

use std::path::Path;
use std::path::PathBuf;

use rusqlite::Connection;
use rusqlite::OptionalExtension;
use rusqlite::params;
use serde_json::Value;
use serde_json::json;

use crate::templates;
use crate::{
    EventRow, FlowError, FlowResult, Project, Step, Task, TaskBundle, allowed_transitions,
    home_dir, known_kind, now, parse_states, serde_json_states, validate_states,
};

pub(crate) fn db_err(e: rusqlite::Error) -> FlowError {
    FlowError::new("DB_ERROR", e.to_string())
}
/// 写事件钩子（serve 特性装 SSE/WS 广播；缺省无钩子，纯 store 无噪音）。
static EVENT_HOOK: std::sync::Mutex<Option<Box<dyn Fn(&EventRow) + Send>>> =
    std::sync::Mutex::new(None);

pub fn set_event_hook(f: impl Fn(&EventRow) + Send + 'static) {
    *EVENT_HOOK.lock().unwrap() = Some(Box::new(f));
}
// ---------------------------------------------------------------------------
// Store
// ---------------------------------------------------------------------------

pub struct Store {
    conn: Connection,
}

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS project (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  repo_path TEXT NOT NULL,
  kind TEXT NOT NULL DEFAULT 'general',
  spec_dir TEXT,
  states_text TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS task (
  id INTEGER PRIMARY KEY,
  project_id INTEGER NOT NULL REFERENCES project(id) ON DELETE CASCADE,
  title TEXT NOT NULL,
  state TEXT NOT NULL,
  claimant TEXT,
  claim_ts INTEGER,
  priority INTEGER NOT NULL DEFAULT 0,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS step (
  id INTEGER PRIMARY KEY,
  task_id INTEGER NOT NULL REFERENCES task(id) ON DELETE CASCADE,
  seq INTEGER NOT NULL,
  title TEXT NOT NULL,
  kind TEXT NOT NULL DEFAULT 'work',
  done INTEGER NOT NULL DEFAULT 0,
  completed_at INTEGER,
  state_hint TEXT,
  spec_ref TEXT,
  evidence TEXT,
  updated_at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS event (
  id INTEGER PRIMARY KEY,
  task_id INTEGER NOT NULL REFERENCES task(id) ON DELETE CASCADE,
  ts INTEGER NOT NULL,
  state_at TEXT NOT NULL,
  kind TEXT NOT NULL,
  payload_json TEXT NOT NULL,
  actor TEXT NOT NULL DEFAULT 'agent'
);
CREATE INDEX IF NOT EXISTS idx_task_project ON task(project_id, updated_at);
CREATE INDEX IF NOT EXISTS idx_step_task ON step(task_id, seq);
CREATE INDEX IF NOT EXISTS idx_event_task ON event(task_id, ts);
"#;

impl Store {
    pub fn open(path: &Path) -> FlowResult<Store> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| FlowError::new("DB_OPEN_FAILED", e.to_string()))?;
        }
        let conn = Connection::open(path).map_err(db_err)?;
        conn.pragma_update(None, "journal_mode", "WAL")
            .map_err(|e| FlowError::new("DB_OPEN_FAILED", e.to_string()))?;
        conn.pragma_update(None, "foreign_keys", "ON")
            .map_err(|e| FlowError::new("DB_OPEN_FAILED", e.to_string()))?;
        conn.execute_batch(SCHEMA).map_err(db_err)?;
        Ok(Store { conn })
    }

    /// 缺省库路径：`CANON_FLOW_DB` 优先，否则 XDG config 下 canon 目录。
    pub fn default_path() -> PathBuf {
        if let Ok(p) = std::env::var("CANON_FLOW_DB")
            && !p.is_empty()
        {
            return PathBuf::from(p);
        }
        let xdg = std::env::var("XDG_CONFIG_HOME")
            .ok()
            .map(PathBuf::from)
            .filter(|p| p.is_absolute());
        xdg.unwrap_or_else(|| home_dir().join(".config"))
            .join("canon")
            .join("flow.db")
    }

    pub fn open_default() -> FlowResult<Store> {
        Store::open(&Self::default_path())
    }

    pub(crate) fn conn(&self) -> &Connection {
        &self.conn
    }

    // ---- project ----

    fn project_by_name(&self, name: &str) -> FlowResult<Option<Project>> {
        let row = self
            .conn()
            .query_row(
                "SELECT id, name, repo_path, kind, spec_dir, states_text, created_at, updated_at \
                 FROM project WHERE name = ?1",
                params![name],
                project_row,
            )
            .optional()
            .map_err(db_err)?;
        Ok(row)
    }

    fn project_by_id(&self, id: i64) -> FlowResult<Option<Project>> {
        let row = self
            .conn()
            .query_row(
                "SELECT id, name, repo_path, kind, spec_dir, states_text, created_at, updated_at \
                 FROM project WHERE id = ?1",
                params![id],
                project_row,
            )
            .optional()
            .map_err(db_err)?;
        Ok(row)
    }

    pub fn get_project(&self, sel: &str) -> FlowResult<Project> {
        if let Ok(id) = sel.parse::<i64>() {
            return self
                .project_by_id(id)?
                .ok_or_else(|| FlowError::new("PROJECT_NOT_FOUND", format!("no project id {id}")));
        }
        self.project_by_name(sel)?
            .ok_or_else(|| FlowError::new("PROJECT_NOT_FOUND", format!("no project named {sel}")))
    }

    pub fn project_create(
        &self,
        name: &str,
        repo_path: &str,
        kind: &str,
        states: Option<&[String]>,
    ) -> FlowResult<Project> {
        if !known_kind(kind) {
            return Err(FlowError::new(
                "INVALID_KIND",
                format!("kind must be backend/frontend/general, got {kind}"),
            ));
        }
        if self.project_by_name(name)?.is_some() {
            return Err(FlowError::new(
                "DUPLICATE_PROJECT",
                format!("project {name} already exists"),
            ));
        }
        let states = match states {
            Some(s) => validate_states(s)?,
            None => templates::for_kind(kind).states,
        };
        let ts = now();
        self.conn()
            .execute(
                "INSERT INTO project (name, repo_path, kind, spec_dir, states_text, created_at, updated_at) \
                 VALUES (?1, ?2, ?3, NULL, ?4, ?5, ?5)",
                params![name, repo_path, kind, serde_json_states(&states), ts],
            )
            .map_err(db_err)?;
        match self.project_by_name(name)? {
            Some(p) => Ok(p),
            None => Err(FlowError::new(
                "DB_ERROR",
                "insert lost: project not readable",
            )),
        }
    }

    pub fn projects(&self) -> FlowResult<Vec<Project>> {
        let mut stmt = self
            .conn()
            .prepare(
                "SELECT id, name, repo_path, kind, spec_dir, states_text, created_at, updated_at \
                 FROM project ORDER BY id",
            )
            .map_err(db_err)?;
        let rows = stmt
            .query_map([], project_row)
            .map_err(db_err)?
            .collect::<Result<Vec<Project>, _>>()
            .map_err(db_err)?;
        Ok(rows)
    }

    /// 改管道/改名；改名与管道变更都会同步迁移存量 task.state（进 journal）。
    pub fn project_update(
        &self,
        sel: &str,
        rename: Option<&str>,
        states: Option<&[String]>,
    ) -> FlowResult<Project> {
        let p = self.get_project(sel)?;
        if let Some(name) = rename
            && name != p.name.as_str()
        {
            if self.project_by_name(name)?.is_some() {
                return Err(FlowError::new(
                    "DUPLICATE_PROJECT",
                    format!("project {name} already exists"),
                ));
            }
            self.conn()
                .execute(
                    "UPDATE project SET name = ?1, updated_at = ?2 WHERE id = ?3",
                    params![name, now(), p.id],
                )
                .map_err(db_err)?;
        }
        if let Some(new_states) = states {
            let new_states = validate_states(new_states)?;
            self.migrate_task_states(p.id, &p.states, &new_states)?;
            self.conn()
                .execute(
                    "UPDATE project SET states_text = ?1, updated_at = ?2 WHERE id = ?3",
                    params![serde_json_states(&new_states), now(), p.id],
                )
                .map_err(db_err)?;
        }
        self.get_project(&p.id.to_string())
    }

    /// 管道缩短/改名后，state 不在新管道的 task：按旧下标钳位到新管道
    /// （old_idx.min(len-1)），每次迁移写一条 journal 事件。
    fn migrate_task_states(
        &self,
        project_id: i64,
        old: &[String],
        new: &[String],
    ) -> FlowResult<usize> {
        let tasks = self.tasks_by_project(project_id)?;
        let mut migrated = 0usize;
        for t in &tasks {
            if new.iter().any(|s| s == &t.state) {
                continue;
            }
            let old_idx = old.iter().position(|s| s == &t.state).unwrap_or(0);
            let target = new[old_idx.min(new.len() - 1)].clone();
            self.conn()
                .execute(
                    "UPDATE task SET state = ?1, updated_at = ?2 WHERE id = ?3",
                    params![target, now(), t.id],
                )
                .map_err(db_err)?;
            self.push_event(
                t.id,
                &target,
                "note",
                &json!({
                    "action": "migrate_state",
                    "from": t.state,
                    "to": target,
                    "why": "project states updated"
                }),
                "system",
            )?;
            migrated += 1;
        }
        Ok(migrated)
    }
}

// ---------------------------------------------------------------------------
// 行映射器
// ---------------------------------------------------------------------------

fn project_row(r: &rusqlite::Row) -> rusqlite::Result<Project> {
    let states_text: String = r.get(5)?;
    Ok(Project {
        id: r.get(0)?,
        name: r.get(1)?,
        repo_path: r.get(2)?,
        kind: r.get(3)?,
        spec_dir: r.get(4)?,
        states: parse_states(&states_text),
        created_at: r.get(6)?,
        updated_at: r.get(7)?,
    })
}

fn task_row(r: &rusqlite::Row) -> rusqlite::Result<Task> {
    Ok(Task {
        id: r.get(0)?,
        project_id: r.get(1)?,
        title: r.get(2)?,
        state: r.get(3)?,
        claimant: r.get(4)?,
        claim_ts: r.get(5)?,
        priority: r.get(6)?,
        created_at: r.get(7)?,
        updated_at: r.get(8)?,
    })
}

impl Store {
    // ---- task ----

    pub fn task_by_id(&self, id: i64) -> FlowResult<Task> {
        let row = self
            .conn()
            .query_row(
                "SELECT id, project_id, title, state, claimant, claim_ts, priority, created_at, updated_at \
                 FROM task WHERE id = ?1",
                params![id],
                task_row,
            )
            .optional()
            .map_err(db_err)?;
        row.ok_or_else(|| FlowError::new("TASK_NOT_FOUND", format!("no task id {id}")))
    }

    pub fn tasks_by_project(&self, project_id: i64) -> FlowResult<Vec<Task>> {
        let mut stmt = self
            .conn()
            .prepare(
                "SELECT id, project_id, title, state, claimant, claim_ts, priority, created_at, updated_at \
                 FROM task WHERE project_id = ?1 ORDER BY updated_at DESC, id DESC",
            )
            .map_err(db_err)?;
        let rows = stmt
            .query_map(params![project_id], task_row)
            .map_err(db_err)?
            .collect::<Result<Vec<Task>, _>>()
            .map_err(db_err)?;
        Ok(rows)
    }

    /// task 选择器：数字 = id，否则全库按 title 精确匹配（撞名要求用 id）。
    pub fn resolve_task(&self, sel: &str) -> FlowResult<Task> {
        if let Ok(id) = sel.parse::<i64>() {
            return self.task_by_id(id);
        }
        let mut stmt = self
            .conn()
            .prepare(
                "SELECT id, project_id, title, state, claimant, claim_ts, priority, created_at, updated_at \
                 FROM task WHERE title = ?1",
            )
            .map_err(db_err)?;
        let matches = stmt
            .query_map(params![sel], task_row)
            .map_err(db_err)?
            .collect::<Result<Vec<Task>, _>>()
            .map_err(db_err)?;
        match matches.len() {
            0 => Err(FlowError::new(
                "TASK_NOT_FOUND",
                format!("no task titled {sel}"),
            )),
            1 => Ok(matches.into_iter().next().unwrap()),
            n => Err(FlowError::new(
                "TASK_AMBIGUOUS",
                format!("{n} tasks titled {sel}; select one by numeric task id"),
            )),
        }
    }

    pub fn task_create(
        &self,
        project_sel: &str,
        title: &str,
        from_template: Option<&str>,
        priority: i64,
    ) -> FlowResult<Task> {
        let p = self.get_project(project_sel)?;
        let state0 = p.states.first().cloned().unwrap_or_default();
        let ts = now();
        self.conn()
            .execute(
                "INSERT INTO task (project_id, title, state, priority, created_at, updated_at) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
                params![p.id, title, state0, priority, ts],
            )
            .map_err(db_err)?;
        let task = Task {
            id: self.conn().last_insert_rowid(),
            project_id: p.id,
            title: title.to_string(),
            state: state0.clone(),
            claimant: None,
            claim_ts: None,
            priority,
            created_at: ts,
            updated_at: ts,
        };
        self.push_event(
            task.id,
            &state0,
            "note",
            &json!({"action": "create", "title": title, "priority": priority}),
            "system",
        )?;
        if let Some(tpl_name) = from_template {
            let tpl = templates::resolve(tpl_name)?;
            self.insert_template_steps(&task, &tpl, &p)?;
        }
        Ok(task)
    }

    fn insert_template_steps(
        &self,
        task: &Task,
        tpl: &templates::Template,
        p: &Project,
    ) -> FlowResult<()> {
        for (i, s) in tpl.steps.iter().enumerate() {
            let state_hint = s
                .state
                .as_ref()
                .filter(|h| p.states.iter().any(|x| x.as_str() == h.as_str()))
                .cloned();
            self.conn()
                .execute(
                    "INSERT INTO step (task_id, seq, title, kind, state_hint, spec_ref, updated_at) \
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    params![task.id, i as i64 + 1, s.title, s.kind, state_hint, s.spec_ref, now()],
                )
                .map_err(db_err)?;
            let step_id = self.conn().last_insert_rowid();
            self.push_event(
                task.id,
                &task.state,
                "step",
                &json!({
                    "action": "add",
                    "step_id": step_id,
                    "title": s.title,
                    "kind": s.kind,
                    "from_template": tpl.name
                }),
                "system",
            )?;
        }
        Ok(())
    }

    pub fn task_get(&self, sel: &str) -> FlowResult<TaskBundle> {
        let t = self.resolve_task(sel)?;
        let p = self
            .project_by_id(t.project_id)?
            .ok_or_else(|| FlowError::new("PROJECT_NOT_FOUND", "task project missing"))?;
        let steps = self.steps_by_task(t.id)?;
        let allowed = allowed_transitions(&p.states, &t.state);
        Ok(TaskBundle {
            task: t,
            project: p,
            steps,
            allowed_transitions: allowed,
        })
    }

    /// claim：写 claimant，task 留当前 state。他人已领 → ALREADY_CLAIMED。
    pub fn claim(&self, sel: &str, claimant: &str) -> FlowResult<Task> {
        let t = self.resolve_task(sel)?;
        if let Some(c) = &t.claimant
            && c != claimant
        {
            return Err(FlowError::new(
                "ALREADY_CLAIMED",
                format!(
                    "task {} is claimed by {c}; only the same claimant can re-claim",
                    t.id
                ),
            ));
        }
        let ts = now();
        self.conn()
            .execute(
                "UPDATE task SET claimant = ?1, claim_ts = ?2, updated_at = ?2 WHERE id = ?3",
                params![claimant, ts, t.id],
            )
            .map_err(db_err)?;
        self.push_event(
            t.id,
            &t.state,
            "note",
            &json!({"action": "claim", "claimant": claimant}),
            claimant,
        )?;
        self.task_by_id(t.id)
    }

    /// 管道校验：未知 state / 非合法转移 / 末态前还有 open step，逐一结构化拒绝。
    pub fn transition(&self, sel: &str, to: &str, reason: &str, actor: &str) -> FlowResult<Task> {
        let t = self.resolve_task(sel)?;
        let p = self
            .project_by_id(t.project_id)?
            .ok_or_else(|| FlowError::new("PROJECT_NOT_FOUND", "task project missing"))?;
        if t.state == to {
            return Err(FlowError::new(
                "SAME_STATE",
                format!("task {} is already in {to}", t.id),
            ));
        }
        if !p.states.iter().any(|s| s == to) {
            return Err(FlowError::new(
                "BAD_STATE",
                format!("unknown state {to}; pipeline: {}", p.states.join("→")),
            ));
        }
        let allowed = allowed_transitions(&p.states, &t.state);
        if !allowed.iter().any(|s| s == to) {
            return Err(FlowError::new(
                "ILLEGAL_TRANSITION",
                format!(
                    "task {} in {} may move to [{}], not {to}",
                    t.id,
                    t.state,
                    allowed.join(", ")
                ),
            ));
        }
        let open = self.open_step_count(t.id)?;
        if to == p.states.last().map(String::as_str).unwrap_or("") && open > 0 {
            return Err(FlowError::new(
                "NOT_COMPLETE",
                format!("cannot enter terminal state {to} with {open} step(s) still open"),
            ));
        }
        let from = t.state.clone();
        self.conn()
            .execute(
                "UPDATE task SET state = ?1, updated_at = ?2 WHERE id = ?3",
                params![to, now(), t.id],
            )
            .map_err(db_err)?;
        self.push_event(
            t.id,
            to,
            "transition",
            &json!({"from": from, "to": to, "reason": reason}),
            actor,
        )?;
        self.task_by_id(t.id)
    }

    fn open_step_count(&self, task_id: i64) -> FlowResult<i64> {
        self.conn()
            .query_row(
                "SELECT COUNT(*) FROM step WHERE task_id = ?1 AND done = 0",
                params![task_id],
                |r| r.get(0),
            )
            .map_err(db_err)
    }

    /// P1 卡片进度：(done, total)。
    pub fn step_stats(&self, task_id: i64) -> FlowResult<(i64, i64)> {
        self.conn()
            .query_row(
                "SELECT SUM(CASE WHEN done = 1 THEN 1 ELSE 0 END), COUNT(*) \
                 FROM step WHERE task_id = ?1",
                params![task_id],
                |r| {
                    Ok((
                        r.get::<_, Option<i64>>(0)?.unwrap_or(0),
                        r.get::<_, i64>(1)?,
                    ))
                },
            )
            .map_err(db_err)
    }

    // ---- step ----

    pub fn steps_by_task(&self, task_id: i64) -> FlowResult<Vec<Step>> {
        let mut stmt = self
            .conn()
            .prepare(
                "SELECT id, task_id, seq, title, kind, done, completed_at, state_hint, spec_ref, evidence, updated_at \
                 FROM step WHERE task_id = ?1 ORDER BY seq",
            )
            .map_err(db_err)?;
        let rows = stmt
            .query_map(params![task_id], step_row)
            .map_err(db_err)?
            .collect::<Result<Vec<Step>, _>>()
            .map_err(db_err)?;
        Ok(rows)
    }

    pub(crate) fn step_by_id_and_task(&self, step_id: i64, task_id: i64) -> FlowResult<Step> {
        let row = self
            .conn()
            .query_row(
                "SELECT id, task_id, seq, title, kind, done, completed_at, state_hint, spec_ref, evidence, updated_at \
                 FROM step WHERE id = ?1 AND task_id = ?2",
                params![step_id, task_id],
                step_row,
            )
            .optional()
            .map_err(db_err)?;
        row.ok_or_else(|| {
            FlowError::new(
                "STEP_NOT_FOUND",
                format!("step {step_id} not found (or belongs to another task)"),
            )
        })
    }

    fn next_step_seq(&self, task_id: i64) -> FlowResult<i64> {
        self.conn()
            .query_row(
                "SELECT COALESCE(MAX(seq), 0) + 1 FROM step WHERE task_id = ?1",
                params![task_id],
                |r| r.get(0),
            )
            .map_err(db_err)
    }

    pub fn step_add(
        &self,
        sel: &str,
        title: &str,
        kind: &str,
        spec_ref: Option<&str>,
        state_hint: Option<&str>,
        actor: &str,
    ) -> FlowResult<Step> {
        let t = self.resolve_task(sel)?;
        if !matches!(kind, "work" | "spec" | "record") {
            return Err(FlowError::new(
                "BAD_STEP_KIND",
                format!("kind must be work/spec/record, got {kind}"),
            ));
        }
        if kind == "spec" && spec_ref.is_none_or(str::is_empty) {
            return Err(FlowError::new(
                "SPEC_REF_MISSING",
                "spec steps require spec_ref (a checklist name, or 'preflight' for everything)",
            ));
        }
        let p = self
            .project_by_id(t.project_id)?
            .ok_or_else(|| FlowError::new("PROJECT_NOT_FOUND", "task project missing"))?;
        let hint = state_hint
            .map(str::to_string)
            .filter(|h| p.states.iter().any(|s| s == h));
        let seq = self.next_step_seq(t.id)?;
        self.conn()
            .execute(
                "INSERT INTO step (task_id, seq, title, kind, state_hint, spec_ref, updated_at) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![t.id, seq, title, kind, hint, spec_ref, now()],
            )
            .map_err(db_err)?;
        let id = self.conn().last_insert_rowid();
        self.push_event(
            t.id,
            &t.state,
            "step",
            &json!({"action": "add", "step_id": id, "title": title, "kind": kind}),
            actor,
        )?;
        self.step_by_id_and_task(id, t.id)
    }

    /// 二态 step 只能往前标 done（journal append-only，无反悔）。
    pub fn step_mark(
        &self,
        sel: &str,
        step_id: i64,
        evidence: Option<&str>,
        actor: &str,
    ) -> FlowResult<Step> {
        let t = self.resolve_task(sel)?;
        let step = self.step_by_id_and_task(step_id, t.id)?;
        if step.done {
            return Err(FlowError::new(
                "ALREADY_DONE",
                format!("step {step_id} is already done"),
            ));
        }
        if step.kind == "spec" && evidence.is_none_or(str::is_empty) {
            return Err(FlowError::new(
                "EVIDENCE_REQUIRED",
                format!(
                    "spec step {step_id} needs evidence (a spec_run result snapshot) before it can be done"
                ),
            ));
        }
        let ts = now();
        self.conn()
            .execute(
                "UPDATE step SET done = 1, completed_at = ?1, evidence = ?2, updated_at = ?1 WHERE id = ?3",
                params![ts, evidence, step_id],
            )
            .map_err(db_err)?;
        self.push_event(
            t.id,
            &t.state,
            "step",
            &json!({"action": "done", "step_id": step_id, "title": step.title}),
            actor,
        )?;
        self.step_by_id_and_task(step_id, t.id)
    }

    /// spec_bind：把 checklist 名绑到 step，step.kind 自动 = spec。
    pub fn step_bind(
        &self,
        sel: &str,
        step_id: i64,
        spec_ref: &str,
        actor: &str,
    ) -> FlowResult<Step> {
        let t = self.resolve_task(sel)?;
        let _ = self.step_by_id_and_task(step_id, t.id)?;
        self.conn()
            .execute(
                "UPDATE step SET kind = 'spec', spec_ref = ?1, updated_at = ?2 WHERE id = ?3",
                params![spec_ref, now(), step_id],
            )
            .map_err(db_err)?;
        self.push_event(
            t.id,
            &t.state,
            "step",
            &json!({"action": "bind", "step_id": step_id, "spec_ref": spec_ref}),
            actor,
        )?;
        self.step_by_id_and_task(step_id, t.id)
    }

    // ---- journal ----

    pub(crate) fn push_event(
        &self,
        task_id: i64,
        state_at: &str,
        kind: &str,
        payload: &Value,
        actor: &str,
    ) -> FlowResult<()> {
        let ts = now();
        self.conn()
            .execute(
                "INSERT INTO event (task_id, ts, state_at, kind, payload_json, actor) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![task_id, ts, state_at, kind, payload.to_string(), actor],
            )
            .map_err(db_err)?;
        if let Some(hook) = EVENT_HOOK.lock().unwrap().as_ref() {
            hook(&EventRow {
                id: self.conn().last_insert_rowid(),
                task_id,
                ts,
                state_at: state_at.to_string(),
                kind: kind.to_string(),
                payload: payload.clone(),
                actor: actor.to_string(),
            });
        }
        Ok(())
    }

    pub fn note(&self, sel: &str, text: &str, actor: &str) -> FlowResult<EventRow> {
        let t = self.resolve_task(sel)?;
        self.push_event(t.id, &t.state, "note", &json!({"text": text}), actor)?;
        self.last_event(t.id)
    }

    /// 时间线：按 ts 升序（P2 栏内排序的数据源），`limit` 取最近 N 条。
    pub fn journal(&self, sel: &str, limit: usize) -> FlowResult<Vec<EventRow>> {
        let t = self.resolve_task(sel)?;
        let mut stmt = self
            .conn()
            .prepare(
                "SELECT id, task_id, ts, state_at, kind, payload_json, actor \
                 FROM event WHERE task_id = ?1 ORDER BY ts DESC, id DESC LIMIT ?2",
            )
            .map_err(db_err)?;
        let mut rows = stmt
            .query_map(params![t.id, limit as i64], event_row)
            .map_err(db_err)?
            .collect::<Result<Vec<EventRow>, _>>()
            .map_err(db_err)?;
        rows.reverse();
        Ok(rows)
    }

    fn last_event(&self, task_id: i64) -> FlowResult<EventRow> {
        let row = self
            .conn()
            .query_row(
                "SELECT id, task_id, ts, state_at, kind, payload_json, actor \
                 FROM event WHERE task_id = ?1 ORDER BY id DESC LIMIT 1",
                params![task_id],
                event_row,
            )
            .optional()
            .map_err(db_err)?;
        row.ok_or_else(|| FlowError::new("DB_ERROR", "journal row missing after insert"))
    }
}

fn step_row(r: &rusqlite::Row) -> rusqlite::Result<Step> {
    let done: i64 = r.get(5)?;
    Ok(Step {
        id: r.get(0)?,
        task_id: r.get(1)?,
        seq: r.get(2)?,
        title: r.get(3)?,
        kind: r.get(4)?,
        done: done != 0,
        completed_at: r.get(6)?,
        state_hint: r.get(7)?,
        spec_ref: r.get(8)?,
        evidence: r.get(9)?,
        updated_at: r.get(10)?,
    })
}

fn event_row(r: &rusqlite::Row) -> rusqlite::Result<EventRow> {
    let payload_json: String = r.get(5)?;
    Ok(EventRow {
        id: r.get(0)?,
        task_id: r.get(1)?,
        ts: r.get(2)?,
        state_at: r.get(3)?,
        kind: r.get(4)?,
        payload: serde_json::from_str(&payload_json).unwrap_or(Value::Null),
        actor: r.get(6)?,
    })
}

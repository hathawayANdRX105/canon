//! spec 执行壳：`spec_run` chdir 进目标仓 + 同进程调引擎（`engine::run_named_in`）。
//! 进程 CWD 是全局状态，`CWD_LOCK` 把这些运行串行化。

use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::MutexGuard;

use rusqlite::params;
use serde_json::{Value, json};

use gate::catalog;
use gate::engine;
use gate::shared;

use super::{FlowError, FlowResult, Project, Store, db_err, now};

static CWD_LOCK: Mutex<()> = Mutex::new(());

struct CwdGuard {
    _lock: MutexGuard<'static, ()>,
    prev: PathBuf,
}

impl CwdGuard {
    fn enter(dir: &Path) -> Result<Self, std::io::Error> {
        // 毒化恢复：guard 内不 panic（drop 是尽力恢复），毒锁直接接管
        let lock = match CWD_LOCK.lock() {
            Ok(g) => g,
            Err(poisoned) => poisoned.into_inner(),
        };
        let prev = std::env::current_dir()?;
        std::env::set_current_dir(dir)?;
        Ok(CwdGuard { _lock: lock, prev })
    }
}

impl Drop for CwdGuard {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.prev);
    }
}

impl Store {
    // ---- spec（引擎壳，零检测逻辑）----

    /// 目标仓 spec 树：显式 spec_dir 列优先，缺省 `<repo_path>/.githooks/spec`。
    pub fn spec_dir_of(p: &Project) -> PathBuf {
        if let Some(d) = &p.spec_dir {
            return PathBuf::from(d);
        }
        PathBuf::from(&p.repo_path).join(".githooks").join("spec")
    }

    pub fn spec_rules(&self, sel: &str) -> FlowResult<Vec<catalog::Rule>> {
        let p = self.get_project(sel)?;
        let dir = Self::spec_dir_of(&p);
        if !dir.is_dir() {
            return Err(FlowError::new(
                "SPEC_DIR_MISSING",
                format!(
                    "no spec tree at {} — run `canon init` in that repo",
                    dir.display()
                ),
            ));
        }
        Ok(catalog::load(&dir))
    }

    fn spec_names_under(dir: &Path, ceiling: engine::SlaLevel) -> Vec<String> {
        catalog::load(dir)
            .into_iter()
            .filter(|r| r.source.contains("checklist_"))
            .filter(|r| {
                r.sla
                    .as_deref()
                    .is_none_or(|s| engine::SlaLevel::parse(s) <= ceiling)
            })
            .map(|r| r.id)
            .collect()
    }

    /// 在目标仓里跑 checklist：chdir（串行）+ 同进程引擎；结果可挂 step
    /// evidence + 落 journal。`names` 为空 = 该 sla 天花板下全部 checklist。
    pub fn spec_run(
        &self,
        sel: &str,
        task_sel: Option<&str>,
        step_id: Option<i64>,
        names: Vec<String>,
        sla: &str,
        actor: &str,
    ) -> FlowResult<Value> {
        let p = self.get_project(sel)?;
        let repo = PathBuf::from(&p.repo_path);
        if !repo.is_dir() {
            return Err(FlowError::new(
                "REPO_PATH_MISSING",
                format!("project repo path {} does not exist", p.repo_path),
            ));
        }
        let dir = Self::spec_dir_of(&p);
        if !dir.is_dir() {
            return Err(FlowError::new(
                "SPEC_DIR_MISSING",
                format!(
                    "no spec tree at {} — run `canon init` in that repo",
                    dir.display()
                ),
            ));
        }
        let guard =
            CwdGuard::enter(&repo).map_err(|e| FlowError::new("CHDIR_FAILED", e.to_string()))?;
        let ceiling = engine::SlaLevel::parse(sla);
        let names = if names.is_empty() {
            Self::spec_names_under(&dir, ceiling)
        } else {
            names
        };
        let mut findings = engine::run_named_in(&dir, &names, ceiling);
        shared::apply_global_overrides(&mut findings);
        let blocking = findings
            .iter()
            .filter(|f| f.severity == shared::Severity::Fail)
            .count();
        let payload = json!({
            "project": p.name,
            "sla": sla,
            "names": names,
            "blocking": blocking,
            "would_block": blocking > 0,
            "ts": now(),
            "findings": findings.iter().map(|f| f.to_json()).collect::<Vec<_>>(),
        });
        if let Some(task_sel) = task_sel {
            let t = self.resolve_task(task_sel)?;
            if let Some(sid) = step_id {
                let step = self.step_by_id_and_task(sid, t.id)?;
                if step.kind == "spec" {
                    self.conn()
                        .execute(
                            "UPDATE step SET evidence = ?1, updated_at = ?2 WHERE id = ?3",
                            params![payload.to_string(), now(), sid],
                        )
                        .map_err(db_err)?;
                }
            }
            self.push_event(
                t.id,
                &t.state,
                "spec",
                &json!({"sla": sla, "names": names, "blocking": blocking}),
                actor,
            )?;
        }
        drop(guard);
        Ok(payload)
    }
}

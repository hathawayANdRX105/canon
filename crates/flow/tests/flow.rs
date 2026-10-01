//! flow Store 的行为测试：管道转移（每边 + 拒边）、step 证据规则、领取锁、
//! 模板展开、管道变更迁移、journal 时间线、spec_run 真跑 checklist。

fn fresh() -> (flow::Store, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let store = flow::Store::open(&dir.path().join("flow.db")).unwrap();
    (store, dir)
}

/// 管道 = 规划→开发→审查→代码清洁→完成（用户示例口径）。
fn pipeline_store() -> (flow::Store, tempfile::TempDir, String) {
    let (store, dir) = fresh();
    let states: Vec<String> = vec![
        "规划".into(),
        "开发".into(),
        "审查".into(),
        "代码清洁".into(),
        "完成".into(),
    ];
    let p = store
        .project_create("demo", ".", "general", Some(&states))
        .unwrap();
    (store, dir, p.name)
}

// ---- 转移表 ----

#[test]
fn allowed_transitions_forward_next_and_backward_any_earlier() {
    let states: Vec<String> = ["a", "b", "c", "d"].into_iter().map(String::from).collect();
    assert_eq!(flow::allowed_transitions(&states, "b"), vec!["c", "a"]);
    // 末态：没有前进，只剩打回
    assert_eq!(flow::allowed_transitions(&states, "d"), vec!["a", "b", "c"]);
    // 未知 state：无处可去
    assert!(flow::allowed_transitions(&states, "zz").is_empty());
}

#[test]
fn forward_must_be_exactly_the_next_state() {
    let (s, _dir, project) = pipeline_store();
    let t = s.task_create(&project, "t", None, 0).unwrap();
    let sel = t.id.to_string();
    s.transition(&sel, "开发", "", "agent").unwrap();
    // 跳态（开发→代码清洁）被拒
    let e = s.transition(&sel, "代码清洁", "", "agent").unwrap_err();
    assert_eq!(e.code, "ILLEGAL_TRANSITION");
    // 未知 state 被拒
    let e = s.transition(&sel, "不存在", "", "agent").unwrap_err();
    assert_eq!(e.code, "BAD_STATE");
    // 原地不动被拒
    let e = s.transition(&sel, "开发", "", "agent").unwrap_err();
    assert_eq!(e.code, "SAME_STATE");
}

#[test]
fn backward_to_any_earlier_state_is_kickback() {
    let (s, _dir, project) = pipeline_store();
    let t = s.task_create(&project, "t", None, 0).unwrap();
    let sel = t.id.to_string();
    s.transition(&sel, "开发", "", "agent").unwrap();
    s.transition(&sel, "审查", "", "agent").unwrap();
    s.transition(&sel, "规划", "打回", "agent").unwrap(); // 跨两级打回
    assert_eq!(s.task_by_id(t.id).unwrap().state, "规划");
}

#[test]
fn terminal_state_requires_every_step_done() {
    let (s, _dir, project) = pipeline_store();
    let t = s.task_create(&project, "t", None, 0).unwrap();
    s.step_add(&t.id.to_string(), "做事", "work", None, None, "agent")
        .unwrap();
    let sel = t.id.to_string();
    for to in ["开发", "审查", "代码清洁"] {
        s.transition(&sel, to, "", "agent").unwrap();
    }
    let e = s.transition(&sel, "完成", "", "agent").unwrap_err();
    assert_eq!(e.code, "NOT_COMPLETE");
    // 关掉 step 后放行
    let step = s.task_get(&sel).unwrap().steps.into_iter().next().unwrap();
    s.step_mark(&sel, step.id, None, "agent").unwrap();
    assert_eq!(s.task_by_id(t.id).unwrap().state, "代码清洁");
    s.transition(&sel, "完成", "", "agent").unwrap();
}

// ---- step 二态与证据 ----

#[test]
fn spec_step_cannot_finish_without_evidence() {
    let (s, _dir, project) = pipeline_store();
    let t = s.task_create(&project, "t", None, 0).unwrap();
    let sel = t.id.to_string();
    // spec step 没绑 spec_ref 建不进来
    let e = s
        .step_add(&sel, "检查", "spec", None, None, "agent")
        .unwrap_err();
    assert_eq!(e.code, "SPEC_REF_MISSING");
    let step = s
        .step_add(&sel, "检查", "spec", Some("ccn"), None, "agent")
        .unwrap();
    let e = s.step_mark(&sel, step.id, None, "agent").unwrap_err();
    assert_eq!(e.code, "EVIDENCE_REQUIRED");
    s.step_mark(&sel, step.id, Some("checklist ccn: 0 findings"), "agent")
        .unwrap();
    // 二态：done 不可重标
    let e = s.step_mark(&sel, step.id, None, "agent").unwrap_err();
    assert_eq!(e.code, "ALREADY_DONE");
}

#[test]
fn step_add_rejects_unknown_kind_and_filters_dangling_state_hint() {
    let (s, _dir, project) = pipeline_store();
    let t = s.task_create(&project, "t", None, 0).unwrap();
    let sel = t.id.to_string();
    let e = s
        .step_add(&sel, "x", "bogus", None, None, "agent")
        .unwrap_err();
    assert_eq!(e.code, "BAD_STEP_KIND");
    // hint 指向管道里不存在的 state → 丢弃（P2 归当前栏）
    let step = s
        .step_add(&sel, "x", "work", None, Some("不存在的state"), "agent")
        .unwrap();
    assert!(step.state_hint.is_none());
    let step = s
        .step_add(&sel, "y", "work", None, Some("开发"), "agent")
        .unwrap();
    assert_eq!(step.state_hint.as_deref(), Some("开发"));
}

// ---- 领取 ----

#[test]
fn claim_is_locked_to_its_claimant() {
    let (s, _dir, project) = pipeline_store();
    let t = s.task_create(&project, "t", None, 0).unwrap();
    let sel = t.id.to_string();
    s.claim(&sel, "alice").unwrap();
    let e = s.claim(&sel, "bob").unwrap_err();
    assert_eq!(e.code, "ALREADY_CLAIMED");
    assert!(s.claim(&sel, "alice").is_ok()); // 同 claimant 幂等
}

// ---- 选择器 ----

#[test]
fn task_selector_resolves_id_and_title_with_ambiguity() {
    let (s, _dir, project) = pipeline_store();
    let a = s.task_create(&project, "同名", None, 0).unwrap();
    let b = s.task_create(&project, "同名", None, 0).unwrap();
    assert_eq!(s.resolve_task(&a.id.to_string()).unwrap().title, "同名");
    let e = s.resolve_task("同名").unwrap_err();
    assert_eq!(e.code, "TASK_AMBIGUOUS");
    let e = s.resolve_task("999999").unwrap_err();
    assert_eq!(e.code, "TASK_NOT_FOUND");
    assert_ne!(a.id, b.id);
}

// ---- 管道变更迁移 ----

#[test]
fn project_update_migrates_task_states_by_index_clamp() {
    let (s, _dir, project) = pipeline_store();
    let t1 = s.task_create(&project, "in-dev", None, 0).unwrap();
    s.transition(&t1.id.to_string(), "开发", "", "agent")
        .unwrap();
    let t2 = s.task_create(&project, "in-review", None, 0).unwrap();
    s.transition(&t2.id.to_string(), "开发", "", "agent")
        .unwrap();
    s.transition(&t2.id.to_string(), "审查", "", "agent")
        .unwrap();

    // 新管道：规划→实现→完成（缩短 + 改名）
    let new_states: Vec<String> = vec!["规划".into(), "实现".into(), "完成".into()];
    s.project_update(&project, None, Some(&new_states)).unwrap();

    // t1 在 开发（旧 idx1）→ 新[1] = 实现
    assert_eq!(s.task_by_id(t1.id).unwrap().state, "实现");
    // t2 在 审查（旧 idx2）→ 钳位 新[2] = 完成
    assert_eq!(s.task_by_id(t2.id).unwrap().state, "完成");
    // 迁移进了 journal
    let ev = s.journal(&t1.id.to_string(), 10).unwrap();
    assert!(ev.iter().any(|e| e.payload["action"] == "migrate_state"));
}

#[test]
fn project_update_rejects_duplicate_rename_and_empty_pipeline() {
    let (s, _dir, project) = pipeline_store();
    s.project_create("other", ".", "general", None).unwrap();
    let e = s.project_update(&project, Some("other"), None).unwrap_err();
    assert_eq!(e.code, "DUPLICATE_PROJECT");
    let empty: Vec<String> = vec![];
    let e = s.project_update(&project, None, Some(&empty)).unwrap_err();
    assert_eq!(e.code, "BAD_INPUT");
}

// ---- 模板 ----

#[test]
fn task_create_from_template_lays_out_the_checklist() {
    let (s, _dir) = fresh();
    s.project_create("demo", ".", "general", None).unwrap();
    let tpl = flow::templates::for_kind("general");
    let t = s
        .task_create("demo", "from-tpl", Some("general"), 0)
        .unwrap();
    let bundle = s.task_get(&t.id.to_string()).unwrap();
    assert_eq!(bundle.steps.len(), tpl.steps.len());
    assert_eq!(bundle.task.state, tpl.states[0]);
    // 模板铺的 step 各有一条 journal「add」事件
    let ev = s.journal(&t.id.to_string(), 50).unwrap();
    let adds: Vec<_> = ev
        .iter()
        .filter(|e| e.payload.get("action").and_then(|a| a.as_str()) == Some("add"))
        .collect();
    assert_eq!(adds.len(), tpl.steps.len());
}

#[test]
fn unknown_template_is_structured_error() {
    let (s, _dir) = fresh();
    s.project_create("demo", ".", "general", None).unwrap();
    let e = s.task_create("demo", "t", Some("nope-tpl"), 0).unwrap_err();
    assert_eq!(e.code, "TEMPLATE_NOT_FOUND");
}

#[test]
fn builtins_and_kind_fallback_resolve_sane_templates() {
    let names = flow::templates::available();
    assert!(names.contains(&"backend".into()));
    assert!(names.contains(&"frontend".into()));
    assert!(names.contains(&"general".into()));
    // 未知 kind 落 general
    assert_eq!(
        flow::templates::for_kind("mystery").states,
        flow::templates::for_kind("general").states
    );
}

// ---- journal 时间线 ----

#[test]
fn journal_records_transitions_in_arrival_state_and_orders_chronologically() {
    let (s, _dir, project) = pipeline_store();
    let t = s.task_create(&project, "t", None, 0).unwrap();
    let sel = t.id.to_string();
    s.transition(&sel, "开发", "开工", "agent").unwrap();
    s.note(&sel, "中途记一笔", "agent").unwrap();
    s.transition(&sel, "规划", "打回", "agent").unwrap();

    let ev = s.journal(&sel, 100).unwrap();
    let kinds: Vec<&str> = ev.iter().map(|e| e.kind.as_str()).collect();
    assert!(kinds.contains(&"transition"));
    assert!(kinds.contains(&"note"));
    // transition 事件记「到达」的 state；note 记当时所在 state
    let into_dev = ev
        .iter()
        .find(|e| e.kind == "transition" && e.payload["to"] == "开发")
        .unwrap();
    assert_eq!(into_dev.state_at, "开发");
    let note = ev
        .iter()
        .find(|e| {
            e.kind == "note" && e.payload.get("text").and_then(|t| t.as_str()) == Some("中途记一笔")
        })
        .unwrap();
    assert_eq!(note.state_at, "开发");
    let kickback = ev
        .iter()
        .find(|e| e.kind == "transition" && e.payload["to"] == "规划")
        .unwrap();
    assert_eq!(kickback.state_at, "规划");
    // 升序
    let ts: Vec<i64> = ev.iter().map(|e| e.ts).collect();
    let mut sorted = ts.clone();
    sorted.sort();
    assert_eq!(ts, sorted);
}

// ---- spec 壳 ----

fn spec_repo_fixture() -> (tempfile::TempDir, std::path::PathBuf) {
    let repo = tempfile::tempdir().unwrap();
    let spec_dir = repo.path().join(".githooks").join("spec");
    std::fs::create_dir_all(spec_dir.join("quality")).unwrap();
    std::fs::write(
        spec_dir.join("quality").join("checklist_smoke.yaml"),
        "enabled: true\nsla: l1\nmode: grep\nharness:\n  command: sh\n  args: [\"-c\", \"echo '[]'\"]\n",
    )
    .unwrap();
    (repo, spec_dir)
}

#[test]
fn spec_rules_missing_tree_is_structured_error() {
    let (s, _dir, _project) = pipeline_store(); // repo_path = "."，spec 树在 canon 仓
    let bare = tempfile::tempdir().unwrap();
    s.project_create("nospespec", bare.path().to_str().unwrap(), "general", None)
        .unwrap();
    let e = s.spec_rules("nospespec").unwrap_err();
    assert_eq!(e.code, "SPEC_DIR_MISSING");
}

#[test]
fn spec_run_executes_checklist_journals_and_attaches_evidence() {
    let (s, _dir) = fresh();
    let (repo, _spec) = spec_repo_fixture();
    s.project_create("smoke", repo.path().to_str().unwrap(), "general", None)
        .unwrap();
    let t = s.task_create("smoke", "t", None, 0).unwrap();
    let sel = t.id.to_string();
    let step = s
        .step_add(&sel, "smoke 检查", "spec", Some("smoke"), None, "agent")
        .unwrap();

    let payload = s
        .spec_run("smoke", Some(&sel), Some(step.id), vec![], "l1", "agent")
        .unwrap();
    assert_eq!(payload["blocking"], 0);
    assert_eq!(payload["names"][0], "smoke");

    // 结果挂到 step evidence，journal 有 spec 事件
    let bundle = s.task_get(&sel).unwrap();
    let st = bundle.steps.into_iter().find(|x| x.id == step.id).unwrap();
    assert!(st.evidence.is_some());
    let ev = s.journal(&sel, 50).unwrap();
    assert!(
        ev.iter()
            .any(|e| e.kind == "spec" && e.payload["blocking"] == 0)
    );
}

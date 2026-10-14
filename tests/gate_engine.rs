//! `gate::engine` 单元测试：并行调度、harness 协议、severity 归并。

use std::time::Duration;

use canon::gate::engine::{
    ChecklistSpec, MAX_CHECK_PARALLELISM, Mode, SlaLevel, check_parallelism, find_specs,
    findings_from_stdout, load_spec, parallel_map, run_harness,
};
use canon::gate::shared::{Finding, Severity};

#[test]
fn parallel_map_preserves_input_order_under_completion_skew() {
    // Reversed input + descending sleeps: a naive unordered collect would
    // emit the fastest job first. Findings must stay in input order.
    let jobs: Vec<usize> = (0..12).rev().collect();
    let out = parallel_map(&jobs, |i| {
        std::thread::sleep(Duration::from_millis((12 - i) as u64 * 4));
        vec![Finding::new(&format!("job-{i}"), Severity::Info, "")]
    });
    let got: Vec<String> = out.into_iter().flatten().map(|f| f.rule_id).collect();
    let want: Vec<String> = jobs.iter().map(|i| format!("job-{i}")).collect();
    assert_eq!(
        got, want,
        "findings must stay in input order regardless of completion order"
    );
}
#[test]
fn parallel_map_observes_more_than_one_worker() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let live = AtomicUsize::new(0);
    let peak = AtomicUsize::new(0);
    let jobs: Vec<usize> = (0..8).collect();
    parallel_map(&jobs, |_| {
        let now_live = live.fetch_add(1, Ordering::SeqCst) + 1;
        peak.fetch_max(now_live, Ordering::SeqCst);
        std::thread::sleep(Duration::from_millis(80));
        live.fetch_sub(1, Ordering::SeqCst);
        vec![]
    });
    assert!(
        peak.load(Ordering::SeqCst) > 1,
        "checklist harnesses must overlap when parallelism > 1"
    );
}
#[test]
fn check_parallelism_stays_bounded() {
    assert_eq!(check_parallelism(0), 1);
    assert_eq!(check_parallelism(1), 1);
    // Env is not mutated in tests (edition 2024 set_var is unsafe), so
    // this asserts the invariants rather than a specific default.
    for jobs in [2usize, 3, 9, 64] {
        let n = check_parallelism(jobs);
        assert!(
            (1..=jobs).contains(&n) && n <= MAX_CHECK_PARALLELISM,
            "parallelism {n} out of bounds for {jobs} jobs"
        );
    }
}
#[test]
fn run_harness_captures_output_larger_than_pipe_buffer() {
    // 70 KB > the 64 KiB pipe buffer. With piped stdout that the engine
    // only drains after exit, this deadlocked until the harness timeout
    // (the 15-minute merge runs); with file capture it must exit cleanly
    // and keep every byte.
    let s = spec(
        "harness: {command: python3, args: [\"-c\", \"import sys; sys.stdout.write('a'*70000)\"]}\ntimeout: 20",
    );
    let (rc, out) = run_harness(&s, b"");
    assert_eq!(rc, 0, "harness should exit cleanly, rc={rc}, out={out:?}");
    assert!(
        out.len() >= 70_000,
        "harness output must not be truncated: {} bytes",
        out.len()
    );
}
fn spec(yaml: &str) -> ChecklistSpec {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("checklist_t.yaml");
    std::fs::write(&path, yaml).unwrap();
    load_spec(&path).unwrap()
}
#[test]
fn spec_parses_defaults() {
    let s = spec("harness: {command: sh, args: []}");
    assert_eq!(s.mode, Mode::Diff);
    assert_eq!(s.base_severity, Severity::Warn);
    assert_eq!(s.sla, SlaLevel::L1);
    assert!(s.optional);
    assert!(
        s.hooks.is_empty(),
        "no implicit hooks — yaml is the only router"
    );
}
#[test]
fn broken_spec_is_loud_not_silent() {
    // A typo'd key used to drop the whole spec silently (fail-open).
    // It must now surface as an Err so run_all can hard-block on it.
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("checklist_bad.yaml");
    std::fs::write(&path, "harness: {command: sh}\nbad_key: 1\n").unwrap();
    let specs = find_specs(dir.path());
    assert_eq!(specs.len(), 1);
    assert!(
        specs[0].1.is_err(),
        "typo'd key must fail loudly, not silently skip"
    );
    assert!(specs[0].1.as_ref().unwrap_err().contains("deserialize"));
}
#[test]
fn spec_parses_overrides() {
    let s = spec(
        "mode: grep\nfail_severity: FAIL\nsla: l3\noptional: false\nhooks: [merge]\ntimeout: 5",
    );
    assert_eq!(s.mode, Mode::Grep);
    assert_eq!(s.base_severity, Severity::Fail);
    assert_eq!(s.sla, SlaLevel::L3);
    assert!(!s.optional);
    assert_eq!(s.hooks, vec!["merge".to_string()]);
    assert_eq!(s.timeout_secs, 5);
}
#[test]
fn finding_json_roundtrip() {
    let spec = spec("harness: {command: sh, args: []}\nfail_severity: FAIL");
    let out = findings_from_stdout(
        &spec,
        r#"[{"id":"HS-01","severity":"WARN","path":"a.py","line":3,"message":"secret","confidence":0.9}]"#,
    );
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].rule_id, "checklist.t.HS-01");
    // max(yaml base, harness report): FAIL base + WARN report → FAIL blocks
    assert_eq!(out[0].severity, Severity::Fail);
    assert_eq!(out[0].line_hint, Some(3));
    assert_eq!(
        out[0].extra.get("confidence").map(String::as_str),
        Some("0.9")
    );
}

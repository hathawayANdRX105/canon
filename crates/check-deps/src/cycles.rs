//! 依赖无环：workspace 内 normal path 依赖边成环即 finding（确定性，零误报，可 FAIL）。
//! 对应 ArchUnit 的 `noCycles()` / dependency-cruiser 的 `no-circular` 规则族；
//! 判定走 Tarjan 强连通分量（>1 即环），输入复用 check-deps 的 cargo metadata 建模。

use check_core::context::Ctx;
use check_core::finding::{Class, Finding, Severity};
use check_core::kinds::KindFactory;
use check_core::rule::Rule;
use serde::Deserialize;

use crate::metadata;

pub struct CyclesFactory;

impl KindFactory for CyclesFactory {
    fn kind(&self) -> &'static str {
        "dep-cycles"
    }

    fn instantiate(&self, name: &str, params: &toml::Table) -> Result<Box<dyn Rule>, String> {
        let _parsed: CyclesParams = Deserialize::deserialize(toml::Value::Table(params.clone()))
            .map_err(|e| format!("bad params: {e}"))?;
        Ok(Box::new(Cycles {
            id: format!("DEP-CYCLES.{name}"),
        }))
    }
}

/// 无参数（deny_unknown_fields：空表即合法，占位以便未来加豁免清单）。
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CyclesParams {}

pub struct Cycles {
    id: String,
}

impl Rule for Cycles {
    fn id(&self) -> &str {
        &self.id
    }

    fn class(&self) -> Class {
        Class::Deterministic
    }

    fn title(&self) -> &str {
        "依赖无环"
    }

    fn doc(&self) -> &str {
        "workspace 内 normal path 依赖图存在环（强连通分量 >1）即 finding，\
         每个 SCC 报一条，成员按字典序列出。环会让改动扩散不可控、编译顺序\
         无法解耦——确定性判据，适合 FAIL。cargo metadata 失败静默（情报缺失）。\
         参数见 checks.toml 的 [kinds.dep-cycles.<name>]（当前无参数）。"
    }

    fn default_severity(&self) -> Severity {
        Severity::Fail
    }

    fn run(&self, ctx: &Ctx) -> Vec<Finding> {
        let Some(graph) = metadata::load(ctx.root()) else {
            return Vec::new();
        };
        let cycles = tarjan_scc(&graph.edges);
        cycles
            .into_iter()
            .filter(|scc| scc.len() > 1)
            .map(|mut scc| {
                scc.sort();
                let members = scc
                    .iter()
                    .map(|p| {
                        graph
                            .package_dirs
                            .get(p)
                            .cloned()
                            .unwrap_or_else(|| (*p).to_string())
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                Finding::new(
                    self.id.as_str(),
                    Severity::Fail,
                    "Cargo.toml",
                    0,
                    format!("依赖环（{} 个包互相可达）：{members}", scc.len()),
                )
                .with_suggestion("拆出共享的底层 crate，或把公共逻辑上移，切断环")
            })
            .collect()
    }
}

/// Tarjan 强连通分量；输入 (src, dst) 边表。返回所有 SCC（size>1 即环；自环单点也算）。
fn tarjan_scc(edges: &[(String, String)]) -> Vec<Vec<String>> {
    let mut adj: std::collections::BTreeMap<&str, Vec<&str>> = Default::default();
    for (s, d) in edges {
        adj.entry(s.as_str()).or_default().push(d.as_str());
    }

    #[derive(Default)]
    struct State<'a> {
        index: usize,
        indices: std::collections::BTreeMap<&'a str, usize>,
        lowlink: std::collections::BTreeMap<&'a str, usize>,
        stack: Vec<&'a str>,
        on_stack: std::collections::BTreeSet<&'a str>,
        sccs: Vec<Vec<&'a str>>,
    }

    // 迭代版 Tarjan（免深递归栈溢出）。
    fn strongconnect<'a>(
        state: &mut State<'a>,
        v: &'a str,
        adj: &std::collections::BTreeMap<&'a str, Vec<&'a str>>,
    ) {
        let mut call_stack: Vec<(&'a str, usize)> = vec![(v, 0)];
        state.indices.insert(v, state.index);
        state.lowlink.insert(v, state.index);
        state.index += 1;
        state.stack.push(v);
        state.on_stack.insert(v);

        while let Some(entry) = call_stack.last_mut() {
            let node = entry.0;
            let child_i = &mut entry.1;
            let children = adj.get(node).cloned().unwrap_or_default();
            if *child_i < children.len() {
                let w = children[*child_i];
                *child_i += 1;
                if !state.indices.contains_key(w) {
                    state.indices.insert(w, state.index);
                    state.lowlink.insert(w, state.index);
                    state.index += 1;
                    state.stack.push(w);
                    state.on_stack.insert(w);
                    call_stack.push((w, 0));
                } else if state.on_stack.contains(w) {
                    let lw = state.lowlink[w];
                    let ln = state.lowlink[node];
                    state.lowlink.insert(node, ln.min(lw));
                }
            } else {
                call_stack.pop();
                if let Some(&(parent, _)) = call_stack.last() {
                    let ln = state.lowlink[node];
                    let lp = state.lowlink[parent];
                    state.lowlink.insert(parent, lp.min(ln));
                }
                if state.lowlink[node] == state.indices[node] {
                    let mut scc = Vec::new();
                    while let Some(w) = state.stack.pop() {
                        state.on_stack.remove(w);
                        scc.push(w);
                        if w == node {
                            break;
                        }
                    }
                    state.sccs.push(scc);
                }
            }
        }
    }

    let mut state = State::default();
    let nodes: Vec<&str> = adj.keys().copied().collect();
    for n in &nodes {
        if !state.indices.contains_key(n) {
            strongconnect(&mut state, n, &adj);
        }
    }
    state
        .sccs
        .into_iter()
        .map(|scc| scc.into_iter().map(str::to_string).collect())
        .collect()
}

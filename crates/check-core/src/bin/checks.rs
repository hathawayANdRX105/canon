//! canon workspace 的开发入口；成员仓 bin 组装自己的规则集后复用 `cli::run`。

fn main() {
    let frontend = check_core::cli::Frontend {
        static_rules: Vec::new(),
        factories: check_core::kinds::builtin_factories(),
    };
    std::process::exit(check_core::cli::run(frontend));
}

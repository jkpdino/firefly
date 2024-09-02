use firefly_driver::{pass::{hir_lower::LowerHirPass, lower::{LinkPass, LowerCodePass, LowerDefsPass}, parse::ParsePass, tycheck::TyCheckPass, IgnorePass}, Driver};

fn main() {
    let mut driver = Driver::new();

    driver.parse_args();
    driver.run_pipeline((
        ParsePass,
        LinkPass,
        LowerDefsPass,
        LowerCodePass,
        IgnorePass::new(),
        TyCheckPass,
        LowerHirPass,
    ));
    driver.output();
}
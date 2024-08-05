use firefly_driver::{pass::{lower::{LinkPass, LowerCodePass, LowerDefsPass}, parse::ParsePass, vir_lower::LowerHirPass, IgnorePass}, Driver};

fn main() {
    let mut driver = Driver::new();

    driver.parse_args();
    driver.run_pipeline((
        ParsePass,
        LinkPass,
        LowerDefsPass,
        LowerCodePass,
        IgnorePass::new(),
        LowerHirPass,
    ));
    driver.output();
}
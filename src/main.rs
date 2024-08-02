use firefly_driver::{pass::{lower::{LinkPass, LowerCodePass, LowerDefsPass}, parse::ParsePass}, Driver};

fn main() {
    let mut driver = Driver::new();

    driver.parse_args();
    driver.run_pipeline((
        ParsePass,
        LinkPass,
        LowerDefsPass,
        LowerCodePass,
    ));
    driver.output();
}
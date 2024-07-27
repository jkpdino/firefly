use firefly_driver::{pass::{lower::{LinkPass, LowerPass}, parse::ParsePass}, Driver};

fn main() {
    let mut driver = Driver::new();

    driver.parse_args();
    driver.run_pipeline((
        ParsePass,
        LinkPass,
        LowerPass,
    ));
    driver.output();
}

/*
chain:
    Parse (SourceFile -> AST)
    Lower (AST -> HIR)

*/

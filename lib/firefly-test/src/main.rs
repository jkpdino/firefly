extern crate libtest_mimic;

mod display;
mod test;

use std::{path::Path, process::ExitCode};

use libtest_mimic::{Arguments, Trial};
use test::{run::run_test, source::{walker::TestWalker, TestSource}};

 fn main() -> ExitCode {
    let args = Arguments::from_args();

    let root = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests")).canonicalize().unwrap();
    let tester = TestWalker { root: &root };
    let test_suite = tester.gather();

    let tests = test_suite
        .into_tests()
        .into_iter()
        .filter(|test| test.directives.is_test)
        .map(|test| Trial::test(&test.name.clone(), move || run_test(&test).into()))
        .collect();

    libtest_mimic::run(&args, tests).exit_code() 
 }
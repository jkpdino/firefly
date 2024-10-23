
/*

History
Status
Substatus
[======>                 ]


[ ] Running test ...
[ ] Running test ...

*/

use indicatif::{ProgressBar, ProgressStyle};
use std::collections::VecDeque;

pub struct TestProgress {
    total_tests: usize,
    completed_tests: usize,
    history: VecDeque<String>, // Keeps history of all tests and their status
    current_test: Option<String>, // Tracks the current test being run
    progress_bar: ProgressBar, // Handles the progress bar
}

impl TestProgress {
    pub fn new(total_tests: usize) -> Self {
        let progress_bar = ProgressBar::new(total_tests as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("[{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
                .unwrap(),
        );

        Self {
            total_tests,
            completed_tests: 0,
            history: VecDeque::new(),
            current_test: None,
            progress_bar,
        }
    }

    // Mark the start of a test
    pub fn start_test(&mut self, test_name: &str) {
        self.current_test = Some(test_name.to_string());
        self.update_display();
        println!("Substatus: Running test: {}", test_name);
    }

    // Mark the test as successful
    pub fn success(&mut self, test_name: &str) {
        self.completed_tests += 1;
        self.history.push_back(format!("[✔] Test succeeded: {}", test_name));
        self.progress_bar.inc(1);
        self.current_test = None;
        self.update_display();
    }

    // Mark the test as failed
    pub fn error(&mut self, test_name: &str) {
        self.completed_tests += 1;
        self.history.push_back(format!("[✘] Test failed: {}", test_name));
        self.progress_bar.inc(1);
        self.current_test = None;
        self.update_display();
    }

    // Display the progress and history
    fn update_display(&self) {
        print!("\x1B[2J\x1B[1;1H");
        self.print_history();
        self.print_status();
        self.progress_bar.println("");
    }

    // Print the history of tests that have run so far
    fn print_history(&self) {
        println!("History:");
        for entry in &self.history {
            println!("{}", entry);
        }
    }

    // Print the current status (group of tests)
    fn print_status(&self) {
        println!("\nStatus:");
        if let Some(current_test) = &self.current_test {
            println!("[ ] Running test: {}", current_test);
        } else {
            println!("[✔] No test is currently running");
        }
    }

    // Mark the end of all tests
    pub fn finish(&mut self) {
        self.progress_bar.finish_with_message("All tests completed");
        self.update_display();
        println!("Substatus: All tests finished!");
    }
}
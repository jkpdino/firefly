use super::Test;

pub struct TestSuite {
    tests: Vec<Test>,
}

impl TestSuite {
    pub fn new(tests: Vec<Test>) -> Self {
        Self { tests }
    }

    pub fn with_prefix(&self, prefix: &str) -> Self {
        let filtered_tests = self.tests.iter()
            .filter(|test| Self::matches_prefix(&test.name, prefix))
            .cloned() // Clone to collect new `Vec<Test>`
            .collect();

        TestSuite::new(filtered_tests)
    }

    // Helper function to check if the test name matches the prefix
    fn matches_prefix(test_name: &str, prefix: &str) -> bool {
        test_name.starts_with(prefix) && {
            let remaining = &test_name[prefix.len()..];
            remaining.is_empty() || remaining.starts_with('.')
        }
    }

    pub fn into_tests(self) -> Vec<Test> {
        self.tests
    }
}

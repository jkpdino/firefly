use comments::CommentParser;

mod comments;

#[derive(Debug, Clone)]
pub struct TestDirectives {
  pub is_test: bool,
  pub expect_lines: Vec<String>,
  pub include_files: Vec<String>,
  pub error_lines: Vec<(String, usize)>,
}

pub fn parse_test_directives(source: &str) -> TestDirectives {
  let comments = CommentParser::new(source).parse_comments(source);

  let mut directives = TestDirectives::default();

  for comment in comments {
    let trimmed = comment.content.trim();
    if trimmed.starts_with("@notest") {
      // we have a notest directive
      directives.is_test = false;
    }

    else if trimmed.starts_with("@expect ") {
      let expect_line = &trimmed["@expect ".len()..];

      directives.expect_lines.push(expect_line.to_string());
    }

    else if trimmed.starts_with("@include ") {
      let include_line = &trimmed["@include ".len()..];

      directives.include_files.push(include_line.to_string());
    }

    else if trimmed.starts_with("@error ") {
      let error_line = &trimmed["@error ".len()..];
      let span = comment.end;

      directives.error_lines.push((error_line.to_string(), span));
    }
  }

  return directives;
}

impl Default for TestDirectives {
    fn default() -> Self {
        Self { is_test: true, expect_lines: Default::default(), include_files: Default::default(), error_lines: Default::default() }
    }
}
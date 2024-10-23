use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq)]
pub struct CommentSpan {
    pub start: usize,
    pub end: usize,
    pub content: String,
}

impl CommentSpan {
    fn new(start: usize, end: usize, content: String) -> Self {
        Self {
            start,
            end,
            content: content.trim().to_string(),
        }
    }
}

#[derive(Debug)]
pub struct CommentParser<'a> {
    chars: Peekable<Chars<'a>>,
    position: usize,
    nesting_level: usize,
}

impl<'a> CommentParser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            chars: s.chars().peekable(),
            position: 0,
            nesting_level: 0,
        }
    }

    fn peek_next(&mut self) -> Option<char> {
        self.chars.clone().nth(1).clone()
    }

    fn current_char(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn advance(&mut self) {
        self.chars.next();
    }

    fn is_comment_start(&mut self) -> bool {
        matches!(
            (self.current_char(), self.peek_next()),
            (Some('/'), Some('*'))
        )
    }

    fn is_comment_end(&mut self) -> bool {
        matches!(
            (self.current_char(), self.peek_next()),
            (Some('*'), Some('/'))
        )
    }

    fn extract_comment(&self, start: usize, end: usize, input: &str) -> CommentSpan {
        let content = input[start + 2..end].to_string();
        CommentSpan::new(start, end + 2, content)
    }

    pub fn parse_comments(mut self, input: &str) -> Vec<CommentSpan> {
        let mut comments = Vec::new();
        let mut current_start: Option<usize> = None;

        while self.current_char().is_some() {
            if self.is_comment_start() {
                if self.nesting_level == 0 {
                    current_start = Some(self.position);
                }
                self.nesting_level += 1;
                self.advance();
                self.advance();
            } else if self.is_comment_end() {
                if self.nesting_level > 0 {
                    self.nesting_level -= 1;
                    if self.nesting_level == 0 {
                        if let Some(start) = current_start {
                            let comment = self.extract_comment(start, self.position, input);
                            comments.push(comment);
                            current_start = None;
                        }
                    }
                }
                self.advance();
                self.advance();
            } else {
                self.advance();
            }
        }

        comments
    }
}
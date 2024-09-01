use firefly_error_messages::DiagnosticMessage;
use firefly_span::Span;

pub enum AnnotationKind {
    Suggestion,
    Message,
    None,

}

pub struct Annotation {
    pub kind:    AnnotationKind,
    pub message: DiagnosticMessage,
    pub loc:     Span,
}
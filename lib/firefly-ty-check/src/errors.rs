use firefly_error_messages::DiagnosticMessage;
use firefly_errors::{annotation::{Annotation, AnnotationKind}, diagnostic::{Diagnostic, DiagnosticId, Level}};
use firefly_hir::{resolve::Symbol, ty::{Ty, TyKind}, value::Value, HirContext};
use itertools::Itertools;

pub enum TypeCheckError<'a> {
    /// E0502
    /// Mismatched return type
    /// ^ expected ``, found ``
    /// ^ return type declared here
    WrongReturnType(&'a Ty, &'a Value),

    /// E0504
    /// Type of initial value does't match variable type
    /// ^ expected ``, found ``
    /// ^ variable declared here
    BindingTypeMismatch(&'a Ty, &'a Value),

    /// E0505
    /// Mismatched types
    /// ^ expeced ``, found ``
    MismatchedType(&'a Ty, &'a Value),

    /// E0506
    /// If statement condition must be a boolean
    /// ^ expected `bool`, found ``
    IfConditionBool(&'a Value),

    /// E0507
    /// While statement condition must be a boolean
    /// ^ expected `bool`, found ``
    WhileConditionBool(&'a Value),

    /// E0508
    /// n arguments missing in invocation of function
    /// -- n arguments of type are missing
    MissingFunctionArgs(&'a Value, &'a [Ty]),

    /// E0509
    /// n extra arguments found in invocation of function
    /// ^^^ hint: remove this argument
    ExtraFunctionArgs(&'a [Value]),

    /// E0510
    /// invalid arguments found in invocation of function
    /// ^^^ expected ``, found ``
    WrongFunctionArgs(&'a [(&'a Ty, &'a Value)])
}

impl TypeCheckError<'_> {
    pub fn into_diagnostic(self, context: &HirContext) -> Diagnostic {      
        match self {
            Self::WrongReturnType(return_ty, value) => {
                Diagnostic::new(
                    Level::Error,
                    DiagnosticMessage::from_str(
                        "mismatched return type"
                    )
                )
                .with_error_code(DiagnosticId::new("E0502"))
                .with_message(value.span, DiagnosticMessage::Str(format!(
                    "expected {}, found {}",
                    self.format_type(return_ty, context),
                    self.format_type(&value.ty, context)
                )))
                .with_info(return_ty.span, DiagnosticMessage::from_str("return type declared here"))
            }
            Self::BindingTypeMismatch(ty, value) => {
                Diagnostic::new(Level::Error, DiagnosticMessage::Str(format!(
                    "type of initial value doesn't match binding type"
                )))
                .with_error_code(DiagnosticId::new("E0504"))
                .with_message(value.span, DiagnosticMessage::Str(format!(
                    "expected {}, found {}",
                    self.format_type(ty, context),
                    self.format_type(&value.ty, context)
                )))
                .with_info(ty.span, DiagnosticMessage::from_str(
                    "type declared here"
                ))
            }

            Self::MismatchedType(ty, value) => {
                Diagnostic::new(
                    Level::Error,
                    DiagnosticMessage::from_str(
                        "mismatched types"
                    )
                )
                .with_error_code(DiagnosticId::new("E0505"))
                .with_message(value.span, DiagnosticMessage::Str(
                    format!(
                        "expected {}, found {}",
                        self.format_type(ty, context),
                        self.format_type(&value.ty, context)
                    )
                ))
                .with_info(ty.span, DiagnosticMessage::from_str(
                    "expected due to this"
                ))
            }

            Self::IfConditionBool(condition) => {
                Diagnostic::new(
                    Level::Error,
                    DiagnosticMessage::from_str(
                        "if statement condition must be a boolean"
                    )
                )
                .with_error_code(DiagnosticId::new("E0506"))
                .with_message(condition.span, DiagnosticMessage::Str(
                    format!(
                        "expected `bool`, found {}",
                        self.format_type(&condition.ty, context)
                    )
                ))
            }

            Self::WhileConditionBool(condition) => {
                Diagnostic::new(
                    Level::Error,
                    DiagnosticMessage::from_str(
                        "while statement condition must be a boolean"
                    )
                )
                .with_error_code(DiagnosticId::new("E0507"))
                .with_message(condition.span, DiagnosticMessage::Str(
                    format!(
                        "expected `bool`, found {}",
                        self.format_type(&condition.ty, context)
                    )
                ))
            }

            Self::MissingFunctionArgs(func, missing_args) => {
                Diagnostic::new(
                    Level::Error,
                    DiagnosticMessage::Str(format!(
                        "{} arguments missing in invocation of function",
                        missing_args.len()
                    ))
                )
                .with_error_code(DiagnosticId::new("E0508"))
                .with_message(func.span, DiagnosticMessage::Str(format!(
                    "{} arguments of type {} are missing",
                    missing_args.len(),
                    missing_args.iter()
                                .map(|arg| self.format_type(arg, context))
                                .join(", ")
                )))
            }

            Self::ExtraFunctionArgs(extra_args) => {
                Diagnostic::new(
                    Level::Error,
                    DiagnosticMessage::Str(format!(
                        "{} extra arguments found in invocation of function",
                        extra_args.len()
                    ))
                )
                .with_error_code(DiagnosticId::new("E0509"))
                .with_annotations(
                    extra_args.iter()
                        .map(|arg| Annotation {
                            kind: AnnotationKind::Suggestion,
                            message: DiagnosticMessage::from_str("hint: remove this argument"),
                            loc: arg.span,
                        }).collect_vec()
                )
            }

            Self::WrongFunctionArgs(wrong_args) => {
                Diagnostic::new(
                    Level::Error,
                    DiagnosticMessage::from_str(
                        "invalid arguments found in invocation of function",
                    )
                )
                .with_error_code(DiagnosticId::new("E0510"))
                .with_annotations(
                    wrong_args.iter()
                        .map(|(param, arg)| Annotation {
                            kind: AnnotationKind::Message,
                            message: DiagnosticMessage::Str(format!(
                                "expected {}, found {}",
                                self.format_type(param, context),
                                self.format_type(&arg.ty, context)
                            )),
                            loc: arg.span,
                        }).collect_vec()
                    )
            }
        }
    }

    fn format_type(&self, ty: &Ty, context: &HirContext) -> String {
        match &ty.kind {
            TyKind::Unit => format!("type `()`"),
            TyKind::Tuple(items) => {
                let items = items.iter()
                                 .map(|ty| self.display_type(ty, context))
                                 .join(", ");

                format!("tuple `({items})`")
            }
            TyKind::StructDef(id) => {
                let Some(Symbol { name, .. }) = context.try_get(*id) else {
                    return format!("anonymous struct");
                };

                return format!("struct `{}`", name.name);
            }
            TyKind::Func(params, return_ty) => {
                let params = params.iter()
                                   .map(|ty| self.display_type(ty, context))
                                   .join(", ");

                format!("function `({params}) -> {}`", self.display_type(return_ty, context))
            }
            TyKind::Integer => format!("int"),
            TyKind::String => format!("string"),
            TyKind::Bool => format!("bool"),
            TyKind::Float => format!("float"),
            TyKind::Never => todo!("type `!`"),
        }
    }

    fn display_type(&self, ty: &Ty, context: &HirContext) -> String {
        match &ty.kind {
            TyKind::Unit => format!("()"),
            TyKind::Tuple(items) => {
                let items = items.iter()
                                 .map(|ty| self.display_type(ty, context))
                                 .join(", ");

                format!("({items})")
            }
            TyKind::StructDef(id) => {
                let Some(Symbol { name, .. }) = context.try_get(*id) else {
                    return format!("anonymous struct");
                };

                return name.name.clone();
            }
            TyKind::Func(params, return_ty) => {
                let params = params.iter()
                                   .map(|ty| self.display_type(ty, context))
                                   .join(", ");

                format!("func ({params}) -> {}", self.display_type(return_ty, context))
            }

            TyKind::Integer => format!("int"),
            TyKind::String => format!("string"),
            TyKind::Bool => format!("bool"),
            TyKind::Float => format!("float"),

            TyKind::Never => format!("!"),
        }
    }
}
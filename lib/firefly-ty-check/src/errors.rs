use firefly_error_messages::DiagnosticMessage;
use firefly_errors::diagnostic::{Diagnostic, Level};
use firefly_hir::{resolve::Symbol, ty::{Ty, TyKind}, value::Value, HirContext};
use itertools::Itertools;

pub enum TypeCheckError<'a> {
    /// E0502
    /// Cannot assign to this value
    /// ^ This value cannot be assigned to
    CannotAssign,

    /// E0503
    /// Cannot assign to immutable value
    /// ^ this value is immutable
    CannotAssignImmutable,

    /// E0504
    /// Type of initial value does't match variable type
    /// ^ variable declared here
    BindingTypeMismatch(&'a Ty, &'a Value),

    /// E0505
    /// Mismatched types
    /// ^ expeced ``, found ``
    MismatchedType,

    /// E0506
    /// If statement condition must be a boolean
    /// ^ expected `bool`, found ``
    IfConditionBool,

    /// E0507
    /// While statement condition must be a boolean
    /// ^ expected `bool`, found ``
    WhileConditionBool,
}

impl TypeCheckError<'_> {
    pub fn into_diagnostic(self, context: &HirContext) -> Diagnostic {
        match self {
            Self::BindingTypeMismatch(ty, value) => {
                Diagnostic::new(Level::Error, DiagnosticMessage::Str(format!(
                    "type of initial value doesn't match binding type"
                )))
                .with_source(source)
            }

            _ => todo!(),
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
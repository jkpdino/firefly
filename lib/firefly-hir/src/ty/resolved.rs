pub enum ResolvedType {
    Unit,
    Tuple(Vec<Type>),

    Integer { bits: u32 },
    Float { bits: u32 },
}
//
// StaticDelegate provides a way to delegate symbol resolution to another scope.
// When resolving a symbol, if the target entity has a StaticDelegate, the resolution
// will also check the delegated scope. This is primarily used to allow struct
// initializers to be called directly with the struct name, without requiring an
// explicit "init" method call.
//
#[derive(Debug, Clone)]
pub struct StaticDelegate;

component!(static_delegates: StaticDelegate);

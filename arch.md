How should an init function work?

An init function has an initializer symbol, showing that it is an initializer.

It also has a new ParentSymbolProxy component, which allows it to be accessed as the parent.

Structs would also get the initializer symbol, so they can be initialized.

The Initializer component represents a transformation on a function: basically, it will call that function, but with a new self parameter.
The new self parameter, will then be returned.

When building the symbol table, the resolver will search for ParentSymbolProxies, and if it finds one, it will add it as a symbol in the parent scope.

Init:
Callable
HasBody
HasVisibility
What others should it have?
No Symbol
New component that lets it be called from the outside

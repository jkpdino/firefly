Define things closer to how they are defined in the AST
Then define different nodes on top of them:

Function
    Signature { }
    VisibleTo { }
    MemberOf  { }

Struct

Field

# Firefly-HIR

Everything is represented by an Entity. There are two kinds of entities:
- 
use std::fmt::Display;

use code::{BasicBlock, BasicBlockId, Function, FunctionSignature, Global, Local};
use ty::{struct_def::StructDef, Ty};

pub mod ty;
pub mod value;
pub mod code;
pub mod builder;
mod util;

pub use util::*;

pub struct MirContext {
    pub(crate) basic_blocks: UniqueContainer<BasicBlock>,
    pub(crate) functions:    UniqueContainer<Function>,
    pub(crate) structs:      UniqueContainer<StructDef>,
    pub(crate) globals:      UniqueContainer<Global>,
}

impl MirContext {
    pub fn new() -> Self {
        Self {
            basic_blocks: UniqueContainer::new(),
            functions:    UniqueContainer::new(),
            structs:      UniqueContainer::new(),
            globals:      UniqueContainer::new(),
        }
    }

    /// Create a struct in the MirContext
    pub fn create_struct(
        &mut self,
        name:   String,
    ) -> UniqueId<StructDef>
    {
        let id = self.structs.next();

        let struct_def = StructDef {
            id,
            name,
            fields: Vec::new(),
        };

        self.structs.push(struct_def);

        return id;
    }

    /// Create a field in a struct
    pub fn create_field(
        &mut self,
        struct_def: Id<StructDef>,
        ty: Ty) -> usize
    {
        let struct_def = self.get_struct_mut(struct_def);

        struct_def.fields.push(ty);

        return struct_def.fields.len() - 1
    }

    /// Create a function in the MirContext
    pub fn create_function(
        &mut self,
        name:      &str,
        params:    Vec<Ty>,
        return_ty: Ty) -> UniqueId<Function>
    {
        let id = self.functions.next();

        let func = Function {
            id,
            name: name.to_string(),
            signature: FunctionSignature {
                parameters: params.clone(),
                return_ty,
            },
            basic_blocks: Vec::new(),
            bb_factory: IdFactory::new(),
            locals: Container::new(),
        };

        self.functions.push(func);

        return id;
    }

    /// Create a basic block in a function
    pub fn create_basic_block(
        &mut self,
        func: UniqueId<Function>) -> BasicBlockId
    {
        let function = self.get_function_mut(func);

        let func_id = function.id;
        let local_id = function.bb_factory.next();
        let global_id = self.basic_blocks.next();

        let bb_id = BasicBlockId { local_id, global_id, func_id };

        self.get_function_mut(func).basic_blocks.push(bb_id);
        self.basic_blocks.push(BasicBlock::new(bb_id));

        return bb_id;
    }

    /// Create a local in a function
    pub fn create_local(
        &mut self,
        func: UniqueId<Function>,
        ty: Ty) -> &Local
    {
        let function = self.get_function_mut(func);

        let id = function.locals.next();
        function.locals.push(Local { id, ty });

        &function.locals[function.locals.len() - 1]
    }

    /// Create a global variable
    pub fn create_global(
        &mut self,
        name: &str,
        ty: Ty) -> UniqueId<Global>
    {
        let id = self.globals.next();

        let global = Global {
            id,
            name: name.to_string(),
            ty
        };

        self.globals.push(global);

        return id;
    }

    /// Gets a reference to a function by id
    pub fn get_function(&self, id: UniqueId<Function>) -> &Function {
        self.functions
            .get_by_id(id)
            .expect("internal compiler error: function not found")
    }

    /// Gets a mutable reference to a function by id
    pub fn get_function_mut(&mut self, id: UniqueId<Function>) -> &mut Function {
        self.functions
            .get_mut_by_id(id)
            .expect("internal compiler error: function not found")
    }

    /// Gets a reference to a basic block by id
    pub fn get_basic_block(&self, id: BasicBlockId) -> &BasicBlock {
        self.basic_blocks
            .get_by_id(id.global_id)
            .expect("internal compiler error: basic block not found")
    }

    /// Gets a mutable reference to a basic block by id
    pub fn get_basic_block_mut(&mut self, id: BasicBlockId) -> &mut BasicBlock {
        self.basic_blocks
            .get_mut_by_id(id.global_id)
            .expect("internal compiler error: basic block not found")
    }

    /// Gets a reference to a struct by id
    pub fn get_struct(&self, id: UniqueId<StructDef>) -> &StructDef {
        self.structs
            .get_by_id(id)
            .expect("internal compiler error: struct not found")
    }

    /// Gets a mutable reference to a struct by id
    pub fn get_struct_mut(&mut self, id: UniqueId<StructDef>) -> &mut StructDef {
        self.structs
            .get_mut_by_id(id)
            .expect("internal compiler error: struct not found")
    }

    /// Gets a reference to a global by id
    pub fn get_global(&self, id: UniqueId<Global>) -> &Global {
        self.globals
            .get_by_id(id)
            .expect("internal compiler error: global not found")
    }

    /// Gets a mutable reference to a global by id
    pub fn get_global_mut(&mut self, id: UniqueId<Global>) -> &mut Global {
        self.globals
            .get_mut_by_id(id)
            .expect("internal compiler error: global not found")
    }

    /// Get a list of globals
    pub fn globals(&self) -> &Vec<Global> {
        &self.globals
    }
}

impl Display for MirContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for global in self.globals.iter() {
            writeln!(f, "{}", self.display(global))?;
        }

        for struct_def in self.structs.iter() {
            writeln!(f, "{}", self.display(struct_def))?;
        }

        for func in self.functions.iter() {
            writeln!(f, "{}", self.display(func))?;
        }

        Ok(())
    }
}
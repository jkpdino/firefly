use code::{BasicBlock, BasicBlockId, Function, FunctionSignature, Local};
use ty::Ty;

use crate::util::{Container, Id, IdFactory, UniqueContainer, UniqueId};

pub mod ty;
pub mod value;
pub mod code;

pub struct VirContext {
    pub(crate) basic_blocks: UniqueContainer<BasicBlock>,
    pub(crate) functions:    UniqueContainer<Function>,
}

impl VirContext {
    pub fn new() -> Self {
        Self {
            basic_blocks: UniqueContainer::new(),
            functions:    UniqueContainer::new(),
        }
    }

    /// Create a function in the VirContext
    pub fn create_function(
        &mut self,
        name:      &str,
        params:    Vec<Ty>,
        return_ty: Ty)
    {
        let id = self.functions.next();

        let func = Function {
            id,
            name: name.to_string(),
            signature: FunctionSignature {
                parameters: params,
                return_ty,
            },
            basic_blocks: Vec::new(),
            bb_factory: IdFactory::new(),
            locals: Container::new(),
        };

        self.functions.push(func);
    }

    /// Create a basic block in a function
    pub fn create_basic_block(
        &mut self,
        func: UniqueId<Function>) -> BasicBlockId
    {
        let function = self.function_mut(func);

        let func_id = function.id;
        let local_id = function.bb_factory.next();
        let global_id = self.basic_blocks.next();

        BasicBlockId { local_id, global_id, func_id }
    }

    /// Create a local in a function
    pub fn create_local(
        &mut self,
        func: UniqueId<Function>,
        ty: &Ty) -> Id<Local>
    {
        todo!()
    }

    /// Gets a reference to a function by id
    pub fn function(&self, id: UniqueId<Function>) -> &Function {
        self.functions
            .get_by_id(id)
            .expect("internal compiler error: function not found")
    }

    /// Gets a mutable reference to a function by id
    pub fn function_mut(&mut self, id: UniqueId<Function>) -> &mut Function {
        self.functions
            .get_mut_by_id(id)
            .expect("internal compiler error: function not found")
    }

    /// Gets a reference to a basic block by id
    pub fn basic_block(&self, id: BasicBlockId) -> &BasicBlock {
        self.basic_blocks
            .get_by_id(id.global_id)
            .expect("internal compiler error: basic block not found")
    }

    /// Gets a mutable reference to a basic block by id
    pub fn basic_block_mut(&mut self, id: BasicBlockId) -> &mut BasicBlock {
        self.basic_blocks
            .get_mut_by_id(id.global_id)
            .expect("internal compiler error: basic block not found")
    }
}
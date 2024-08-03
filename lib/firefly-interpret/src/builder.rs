use crate::{ir::{code::{BasicBlockId, Function, Local, Terminator}, ty::Ty, value::{Immediate, Place}, VirContext}, util::{Id, UniqueId}};

pub struct Builder<'a> {
    context: &'a mut VirContext,

    current_func: Option<UniqueId<Function>>,
    current_bb: Option<BasicBlockId>,
}

impl<'a> Builder<'a> {
    pub fn new(context: &'a mut VirContext) -> Self {
        Self {
            context,
            current_func: None,
            current_bb: None
        }
    }

    pub fn context_mut(&mut self) -> &mut VirContext {
        &mut self.context
    }

    /// Position the builder on a function
    pub fn select_func(&mut self, func: Id<Function>) {
        self.current_func = Some(func);
        self.current_bb = None;
    }

    /// Position the builder on a basic block
    pub fn select_basic_block(&mut self, bb: BasicBlockId) {
        self.current_func = Some(bb.func_id);
        self.current_bb = Some(bb);
    }

    /// Appends a basic block to the selected function
    pub fn append_basic_block(&mut self) -> BasicBlockId {
        let func_id = self.current_func_id();

        let bb_id = self.context_mut().create_basic_block(func_id);
        self.current_bb = Some(bb_id);

        return bb_id;
    }

    /// Builds a local variable in the selected function
    pub fn build_local(&mut self, ty: Ty) -> &Local {
        let func_id = self.current_func_id();

        self.context_mut().create_local(func_id, ty)
    }

    /// Gets a local value
    pub fn get_local(&self, index: Id<Local>) -> &Local {
        let func_id = self.current_func_id();
        let func = self.context.function(func_id);

        func.locals.get_by_id(index).expect("internal compiler error: local not found")
    }

    /// Builds an assign operation in the selected basic block
    pub fn build_assign(&mut self, place: Place, imm: Immediate) {
        let id = self.current_basic_block_id();
        let basic_block = self.context_mut().basic_block_mut(id);

        basic_block.append_assign(place, imm);
    }

    /// Builds an evaluate operation in the selected basic block
    pub fn build_eval(&mut self, imm: Immediate) {
        let id = self.current_basic_block_id();
        let basic_block = self.context_mut().basic_block_mut(id);

        basic_block.append_eval(imm);
    }

    /// Builds a terminator operation for the selected basic block
    pub fn build_terminator(&mut self, terminator: Terminator) {
        let id = self.current_basic_block_id();
        let basic_block = self.context_mut().basic_block_mut(id);

        basic_block.append_terminator(terminator);

        self.current_bb = None;
    }




    fn current_func_id(&self) -> Id<Function> {
        self.current_func.expect("internal compiler error: no vir id selected")
    }

    fn current_basic_block_id(&self) -> BasicBlockId {
        self.current_bb.expect("internal compiler error: no vir id selected")
    }
}
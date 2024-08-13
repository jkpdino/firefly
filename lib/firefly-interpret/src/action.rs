use firefly_mir::code::BasicBlockId;

use super::value::Value;

pub enum Action {
    Jump(BasicBlockId),
    Return(Value),
    ReturnVoid,
}
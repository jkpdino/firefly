use action::Action;
use stack_frame::StackFrame;
use value::{InnerValue, Value};

use firefly_mir::{code::{BasicBlockId, Function, InstructionKind, TerminatorKind}, value::{BinaryIntrinsic, BooleanBinaryOp, Comparison, ConstantValue, FloatBinaryOp, Immediate, ImmediateKind, IntegerBinaryOp, Place, PlaceKind, StringBinaryOp, UnaryIntrinsic}, Id, MirContext};

pub mod value;
mod stack_frame;
mod action;

pub struct ExecutionEngine<'a> {
    context: &'a MirContext,
    globals: StackFrame
}

impl<'a> ExecutionEngine<'a> {
    pub fn new(context: &'a MirContext) -> Self {
        let globals = StackFrame::new(context.globals().len(), Vec::new());

        Self {
            context,
            globals
        }
    }

    pub fn execute(&mut self) {
        // todo: check if there is a function 
        self.execute_function(Id::new(0), Vec::new());
    }

    fn execute_function(&mut self, id: Id<Function>, args: Vec<Value>) -> Value {
        // create the stack frame
        let function = self.context.get_function(id);

        let mut stack_frame = StackFrame::new(function.locals().len(), args);
        let mut current_bb = function.basic_blocks().first().cloned();

        while let Some(bb) = current_bb {
            let action = self.execute_basic_block(bb, &mut stack_frame);

            match action {
                Action::Jump(bb) => {
                    current_bb = Some(bb);
                }
                Action::Return(value) => {
                    return value;
                }
                Action::ReturnVoid => {
                    return Value::new(value::InnerValue::Undefined);
                }
            }
        }

        unreachable!();
    }

    fn execute_basic_block(&mut self, bb: BasicBlockId, frame: &mut StackFrame) -> Action {
        let bb = self.context.get_basic_block(bb);

        for instruction in bb.instructions() {
            match &instruction.kind {
                InstructionKind::Assign(place, imm) => {
                    let imm = self.eval_immediate(imm, frame);
                    let place = self.eval_place(place, frame);

                    *place = imm;
                }
                InstructionKind::Eval(imm) => {
                    self.eval_immediate(imm, frame);
                }
            }
        }

        let Some(terminator) = bb.terminator() else {
            return Action::ReturnVoid;
        };
        match &terminator.kind {
            TerminatorKind::Branch(bb) => {
                Action::Jump(*bb)
            }
            TerminatorKind::BranchIf(cond, then, otherwise) => {
                let value = self.eval_immediate(cond, frame);

                match value.as_ref() {
                    InnerValue::Boolean(true) => Action::Jump(*then),
                    InnerValue::Boolean(false) => Action::Jump(*otherwise),
                    _ => panic!(),
                }
            }
            TerminatorKind::Return(value) => {
                let value = self.eval_immediate(value, frame);

                Action::Return(value)
            }
            TerminatorKind::ReturnVoid => {
                Action::ReturnVoid
            }
        }
    }

    fn eval_immediate(&mut self, imm: &Immediate, frame: &mut StackFrame) -> Value {
        let inner =
        match imm.kind.as_ref() {
            ImmediateKind::Void => InnerValue::Void,
            ImmediateKind::Constant(ConstantValue::Integer(i)) => InnerValue::Integer(*i),
            ImmediateKind::Constant(ConstantValue::Bool(b)) => InnerValue::Boolean(*b),
            ImmediateKind::Constant(ConstantValue::String(s)) => InnerValue::String(s.clone()),
            ImmediateKind::Constant(ConstantValue::Float(f)) => InnerValue::Float(*f),

            ImmediateKind::Move(place) => return self.eval_place(place, frame).clone(),

            ImmediateKind::Call(func, args) => {
                let args = args.iter().map(|arg| self.eval_immediate(arg, frame)).collect();
                let value = self.execute_function(*func, args);
                return value;
            }

            ImmediateKind::Binary(op, left, right) => {
                let left = self.eval_immediate(left, frame);
                let right = self.eval_immediate(right, frame);

                match op {
                    BinaryIntrinsic::Boolean(op) => {
                        let (InnerValue::Boolean(left), InnerValue::Boolean(right)) = (left.as_ref(), right.as_ref()) else {
                            panic!();
                        };

                        return self.eval_bool_op(*op, *left, *right);
                    }
                    BinaryIntrinsic::Integer(op) => {
                        let (InnerValue::Integer(left), InnerValue::Integer(right)) = (left.as_ref(), right.as_ref()) else {
                            panic!();
                        };

                        return self.eval_int_op(*op, *left, *right);
                    }
                    BinaryIntrinsic::Float(op) => {
                        let (InnerValue::Float(left), InnerValue::Float(right)) = (left.as_ref(), right.as_ref()) else {
                            panic!();
                        };

                        return self.eval_float_op(*op, *left, *right);
                    }
                    BinaryIntrinsic::String(op) => {
                        let (InnerValue::String(left), InnerValue::String(right)) = (left.as_ref(), right.as_ref()) else {
                            panic!();
                        };

                        return self.eval_string_op(*op, left, right);
                    }
                    BinaryIntrinsic::Compare(op) => {
                        let result = match op {
                            Comparison::Equal => left == right,
                            Comparison::NotEqual => left != right,
                            Comparison::LessThan => left < right,
                            Comparison::LessThanOrEqual => left <= right,
                            Comparison::GreaterThan => left > right,
                            Comparison::GreaterThanOrEqual => left >= right,
                        };

                        return Value::new(InnerValue::Boolean(result));
                    }
                }
            }

            ImmediateKind::Unary(op, value) => {
                self.eval_unary(value, frame, op)
            }
        };

        Box::new(inner)
    }

    fn eval_unary(&mut self, value: &Immediate, frame: &mut StackFrame, op: &UnaryIntrinsic) -> InnerValue {
        let operand = self.eval_immediate(value, frame);

        match (operand.as_ref(), op) {
            (InnerValue::Integer(i), UnaryIntrinsic::BitNot) => {
                return InnerValue::Integer(!i);
            }

            (InnerValue::Boolean(b), UnaryIntrinsic::Not) => {
                return InnerValue::Boolean(!b);
            }

            (InnerValue::String(s), UnaryIntrinsic::Len) => {
                return InnerValue::Integer(s.len() as u64);
            }

            (InnerValue::Integer(i), UnaryIntrinsic::Format) => {
                return InnerValue::String(i.to_string());
            }

            (InnerValue::String(s), UnaryIntrinsic::Parse) => {
                return InnerValue::Integer(s.parse().unwrap());
            }

            (InnerValue::Boolean(b), UnaryIntrinsic::Format) => {
                return InnerValue::String(b.to_string());
            }

            (InnerValue::String(s), UnaryIntrinsic::Print) => {
                println!("{s}");
                return InnerValue::Void;
            }

            _ => unreachable!()
        }
    }
    
    fn eval_place<'b>(&'b mut self, place: &Place, frame: &'b mut StackFrame) -> &'b mut Value {
        match place.kind.as_ref() {
            PlaceKind::Local(index) => {
                return frame.get_value_mut(index.index())
            }
            PlaceKind::Global(index) => {
                return self.globals.get_value_mut(index.index());
            }
            PlaceKind::Field(_, _) => todo!(),
        }
    }

    fn eval_bool_op(&mut self, bool_op: BooleanBinaryOp, left: bool, right: bool) -> Value {
        let result = match bool_op {
            BooleanBinaryOp::And => left && right,
            BooleanBinaryOp::Or => left || right,
            BooleanBinaryOp::Xor => left ^ right,
        };

        Value::new(InnerValue::Boolean(result))
    }

    fn eval_int_op(&mut self, int_op: IntegerBinaryOp, left: u64, right: u64) -> Value {
        let result = match int_op {
            IntegerBinaryOp::Add => left + right,
            IntegerBinaryOp::Sub => left - right,
            IntegerBinaryOp::Mul => left * right,
            IntegerBinaryOp::Div => left / right,
            IntegerBinaryOp::Rem => left % right,
            IntegerBinaryOp::ShiftLeft => left << right,
            IntegerBinaryOp::ShiftRight => left >> right,
            IntegerBinaryOp::BitOr => left | right,
            IntegerBinaryOp::BitAnd => left & right,
            IntegerBinaryOp::BitXor => left ^ right,
        };

        Value::new(InnerValue::Integer(result))
    }

    fn eval_float_op(&mut self, float_op: FloatBinaryOp, left: f64, right: f64) -> Value {
        let result = match float_op {
            FloatBinaryOp::Add => left + right,
            FloatBinaryOp::Sub => left - right,
            FloatBinaryOp::Mul => left * right,
            FloatBinaryOp::Div => left / right,
            FloatBinaryOp::Rem => left % right,
            FloatBinaryOp::Pow => left.powf(right),
        };

        Value::new(InnerValue::Float(result))
    }

    fn eval_string_op(&mut self, string_op: StringBinaryOp, left: &str, right: &str) -> Value {
        let result = match string_op {
            StringBinaryOp::Concat => format!("{left}{right}")
        };

        Value::new(InnerValue::String(result))
    }
}
use super::types::*;
use super::instructions::OpCode;
use super::modules::Context;

pub(super) struct Validator {
    pub(super) ctx: Context,
    pub vals: Vec<ValType>,
    pub ctrls: Vec<CtrlFrame>,
}

#[derive(Clone)]
pub(super) struct CtrlFrame {
    pub opcode: OpCode,
    pub start_types: Vec<ValType>,
    pub end_types: Vec<ValType>,
    pub height: usize,
    unreachable: bool
}

fn validator<'a>() -> Validator {
    Validator{
        ctx: Context::new(),
        vals: Vec::new(),
        ctrls: Vec::new()
    }
}

impl Validator {
    pub fn push_val(&mut self, val: ValType) {
        self.vals.push(val);
    }

    pub fn pop_next_val(&mut self) -> ValType {
        if self.vals.len() == self.ctrls[0].height && self.ctrls[0].unreachable {
            return ValType::Unknown;
        } else if self.vals.len() == self.ctrls[0].height {
            panic!("vals.size() == ctrls[0].height")
        } else {
            match self.vals.pop() {
                Some(val) => val,
                None => panic!("vals.pop() == None")
            }
        }
    }

    pub fn pop_val(&mut self, expect: ValType) -> ValType {
        let actual = self.pop_next_val();

        if matches!(actual, expect) || matches!(actual, ValType::Unknown) || matches!(expect, ValType::Unknown) {
            actual
        } else {
            panic!("actual != expect")
        }
    }

    pub fn push_vals(&mut self, vals: Vec<ValType>) {
        for val in vals.iter().rev() {
            self.push_val(*val)
        }
    }

    pub fn pop_vals(&mut self, vals: Vec<ValType>) -> Vec<ValType> {
        let mut ret_vals: Vec<ValType> = Vec::new();
        for val in vals.iter().rev() {
            ret_vals.push(self.pop_val(*val))
        }

        return ret_vals
    }

    pub fn push_ctrl(&mut self, opcode: OpCode, inputs: Vec<ValType>, outputs: Vec<ValType>) {
        let frame = CtrlFrame{
            opcode: opcode,
            start_types: inputs,
            end_types: outputs,
            height: self.vals.len(),
            unreachable: false
        };

        self.ctrls.push(frame);
        self.push_vals(inputs);
    }

    pub fn pop_ctrl(&mut self) -> CtrlFrame {
        self.pop_vals(self.ctrls[0].end_types);
        
        if self.vals.len() != self.ctrls[0].height {
            panic!("vals.len() != frame.height")
        }

        match self.ctrls.pop() {
            Some(frame) => frame,
            None => panic!("ctrls.pop() == None")
        }
    }

    pub fn label_types(&self, frame: CtrlFrame) -> Vec<ValType> {
        if matches!(frame.opcode, OpCode::Loop) {
            return frame.start_types;
        } else {
            return frame.end_types;
        }
    }

    pub fn vals_resize(&mut self, size: usize) {
        loop {
            if self.vals.len() > size {
                self.pop_next_val();
            } else {
                break;
            }
        }
    }

    pub fn unreachable(&mut self) {
        self.vals_resize(self.ctrls[0].height);
        self.ctrls[0].unreachable = true;
    }
}

impl Context {
    pub fn get_block_type(&self, blocktype: BlockType) -> FuncType {
        match blocktype {
            BlockType::TypeIdx(typeidx) => vec![self.types[typeidx]],
            BlockType::ValType(None) => FuncType{inputs: Vec::new(), returns: Vec::new()},
            BlockType::ValType(Some(valtype)) => FuncType {
                inputs: vec![valtype],
                returns: vec![valtype]
            }
        }
        
    }
}
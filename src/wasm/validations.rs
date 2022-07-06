use super::types::*;
use super::modules::Module;

pub(super) struct Validator {
    pub(super) module: Module,
    vals: Vec<ValType>,
    ctrls: Vec<CtrlFrame>,
}

#[derive(Clone)]
pub(super) struct CtrlFrame {
    opcode: OpCode,
    start_types: Vec<ValType>,
    end_types: Vec<ValType>,
    height: usize,
    unreachable: bool
}

fn validator<'a>() -> Validator {
    Validator{
        module: Module::new(),
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
        for val in vals {
            self.push_val(val)
        }
    }

    pub fn pop_vals(&mut self, vals: Vec<ValType>) -> Vec<ValType> {
        let mut ret_vals: Vec<ValType> = Vec::new();
        for val in vals {
            ret_vals.push(self.pop_val(val))
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

    pub fn label_types(frame: CtrlFrame) -> Vec<ValType> {
        if matches!(frame.opcode, OpCode::Loop) {
            return frame.start_types;
        } else {
            return frame.end_types;
        }
    }

    pub fn unreachable(&mut self) {
        loop {
            if self.vals.len() > self.ctrls[0].height {
                self.pop_next_val();
            } else {
                break;
            }
        }
        self.ctrls[0].unreachable = false;
    }
}
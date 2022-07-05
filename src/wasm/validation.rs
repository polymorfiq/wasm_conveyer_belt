use super::types::*;

struct Validator {
    vals: Vec<ValTypes>,
    ctrls: Vec<CtrlFrame>,
}

#[derive(Clone)]
struct CtrlFrame {
    opcode: OpCode,
    start_types: Vec<ValTypes>,
    end_types: Vec<ValTypes>,
    height: usize,
    unreachable: bool
}

impl<'a> Validator {
    pub fn push_val(&mut self, val: ValTypes) {
        self.vals.push(val);
    }

    pub fn pop_val(&mut self) -> ValTypes {
        if self.ctrls.len() == 0 { panic!("ctrls.len() == 0") }

        if self.vals.len() == self.ctrls[0].height && self.ctrls[0].unreachable {
            return ValTypes::Unknown;
        } else if self.vals.len() == self.ctrls[0].height {
            panic!("vals.size() == ctrls[0].height")
        } else {
            match self.vals.pop() {
                Some(val) => val,
                None => panic!("vals.pop() == None")
            }
        }
    }

    pub fn pop_val_expect(&mut self, expect: ValTypes) -> ValTypes {
        let actual = self.pop_val();

        if matches!(actual, expect) || matches!(actual, ValTypes::Unknown) || matches!(expect, ValTypes::Unknown) {
            actual
        } else {
            panic!("actual != expect")
        }
    }

    pub fn push_vals(&mut self, vals: Vec<ValTypes>) {
        for val in vals {
            self.push_val(val)
        }
    }

    pub fn pop_vals(&mut self, vals: Vec<ValTypes>) -> Vec<ValTypes> {
        let mut ret_vals: Vec<ValTypes> = Vec::new();
        for val in vals {
            ret_vals.push(self.pop_val_expect(val))
        }

        return ret_vals
    }

    pub fn push_ctrl(&mut self, opcode: OpCode, inputs: Vec<ValTypes>, outputs: Vec<ValTypes>) {
        let frame = CtrlFrame{
            opcode: opcode,
            start_types: inputs.clone(),
            end_types: outputs,
            height: self.vals.len(),
            unreachable: false
        };

        self.ctrls.push(frame);
        self.push_vals(inputs);
    }

    pub fn pop_ctrl(&mut self) -> CtrlFrame {
        if self.ctrls.len() == 0 {
            panic!("ctrls.len() == 0")
        }

        let frame = self.ctrls[0].clone();
        self.pop_vals(frame.end_types.clone());
        
        if self.vals.len() != frame.height {
            panic!("vals.len() != frame.height")
        }

        match self.ctrls.pop() {
            Some(frame) => frame,
            None => panic!("ctrls.pop() == None")
        }
    }

    pub fn label_types(frame: CtrlFrame) -> Vec<ValTypes> {
        if matches!(frame.opcode, OpCode::Loop) {
            return frame.start_types;
        } else {
            return frame.end_types;
        }
    }

    pub fn unreachable(&mut self) {
        if self.ctrls.len() == 0 { panic!("ctrls.len() == 0") }

        loop {
            if self.vals.len() > self.ctrls[0].height {
                self.pop_val();
            } else {
                break;
            }
        }
        self.ctrls[0].unreachable = false;
    }
}
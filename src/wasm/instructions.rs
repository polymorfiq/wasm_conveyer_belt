use super::types::*;
use super::validations::Validator;

#[derive(std::fmt::Debug)]
pub(super) enum Instr {
    //
    // Numeric Instructions
    //
    // I32 Instructions
    I32Const(u32),
    // I32 Unary Operations
    I32Clz,
    I32Ctz,
    I32PopCnt,
    // I32 Binary Operations
    I32Add,
    I32Sub,
    I32Mul,
    I32DivU,
    I32DivS,
    I32RemU,
    I32RemS,
    // I32 Test Operations
    I32Eqz,
    // I32 Comparison Operations
    I32Eq,
    I32Ne,
    I32LtU,
    I32LtS,
    I32GtU,
    I32GtS,
    I32LeU,
    I32LeS,
    I32GeU,
    I32GeS,
    // I32 Convert Operations
    I32Extend8S,
    I32Extend16S,
    I32WrapI64,
    I32TruncF32U,
    I32TruncF32S,
    I32TruncF64U,
    I32TruncF64S,
    I32TruncSatF32U,
    I32TruncSatF32S,
    I32TruncSatF64U,
    I32TruncSatF64S,
    I32ReinterpretF32,

    // I64 Instructions
    I64Const(u64),
    // I64 Unary Operations
    I64Clz,
    I64Ctz,
    I64PopCnt,
    // I64 Binary Operations
    I64Add,
    I64Sub,
    I64Mul,
    I64DivU,
    I64DivS,
    I64RemU,
    I64RemS,
    // I64 Test Operations
    I64Eqz,
    // I64 Comparison Operations
    I64Eq,
    I64Ne,
    I64LtU,
    I64LtS,
    I64GtU,
    I64GtS,
    I64LeU,
    I64LeS,
    I64GeU,
    I64GeS,
    // I64 Convert Operations
    I64Extend8S,
    I64Extend16S,
    I64Extend32S,
    I64ExtendI32U,
    I64ExtendI32S,
    I64TruncF32U,
    I64TruncF32S,
    I64TruncF64U,
    I64TruncF64S,
    I64TruncSatF32U,
    I64TruncSatF32S,
    I64TruncSatF64U,
    I64TruncSatF64S,
    I64ReinterpretF64,

    // F32 Instructions
    F32Const(f32),
    // F32 Unary Operations
    F32Abs,
    F32Neg,
    F32Sqrt,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    // F32 Binary Operations
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,
    // F32 Comparison Operations
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
    // F32 Convert Operations
    F32DemoteF64,
    F32ConvertI32U,
    F32ConvertI32S,
    F32ConvertI64U,
    F32ConvertI64S,
    F32ReinterpretI32,
    F32ReinterpretI64,

    // F64 Instructions
    F64Const(f64),
    // F64 Unary Operations
    F64Abs,
    F64Neg,
    F64Sqrt,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    // F64 Binary Operations
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64CopySign,
    // F64 Comparison Operations
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,
    // F64 Convert Operations
    F64PromoteF32,
    F64ConvertI32U,
    F64ConvertI32S,
    F64ConvertI64U,
    F64ConvertI64S,
    F64ReinterpretI32,
    F64ReinterpretI64,

    //
    // Vector Instructions
    //
    // V128 Instructions
    V128Const(u128),
    // V128 Unary Operations
    V128Not,
    // V128 Binary Operations
    V128And,
    V128AndNot,
    V128Or,
    V128Xor,
    // V128 Ternary Operations
    V128Bitselect,
    // V128 Test Operations
    V128AnyTrue,

    // VecI8x16 Instructions
    VecI8x16Shuffle([LaneIdx; 16]),
    VecI8x16Swizzle,
    VecI8x16Splat,
    VecI8x16ExtractLaneU(LaneIdx),
    VecI8x16ExtractLaneS(LaneIdx),
    VecI8x16ReplaceLane(LaneIdx),
    VecI8x16Bitmask,
    VecI8x16NarrowI16x8U,
    VecI8x16NarrowI16x8S,
    // VecI8x16 Unary Operations
    VecI8x16Abs,
    VecI8x16Neg,
    VecI8x16PopCnt,
    // VecI8x16 Binary Operations
    VecI8x16Add,
    VecI8x16Sub,
    VecI8x16AvgrU,
    // VecI8x16 Compare Operations
    VecI8x16Eq,
    VecI8x16Ne,
    VecI8x16LtU,
    VecI8x16LtS,
    VecI8x16GtU,
    VecI8x16GtS,
    VecI8x16LeU,
    VecI8x16LeS,
    VecI8x16GeU,
    VecI8x16GeS,
    // VecI8x16 Test Operations
    VecI8x16AllTrue,
    // VecI8x16 Shift Operations
    VecI8x16Shl,
    VecI8x16ShrU,
    VecI8x16ShrS,
    // VecI8x16 MinMax Operations
    VecI8x16MinU,
    VecI8x16MinS,
    VecI8x16MaxU,
    VecI8x16MaxS,
    // VecI8x16 Sat Binary Operations
    VecI8x16AddSatU,
    VecI8x16AddSatS,
    VecI8x16SubSatU,
    VecI8x16SubSatS,

    // VecI16x8 Instructions
    VecI16x8Splat,
    VecI16x8ExtractLaneU(LaneIdx),
    VecI16x8ExtractLaneS(LaneIdx),
    VecI16x8ReplaceLane(LaneIdx),
    VecI16x8Bitmask,
    VecI16x8NarrowI32x4U,
    VecI16x8NarrowI32x4S,
    // VecI16x8 Unary Operations
    VecI16x8Abs,
    VecI16x8Neg,
    // VecI16x8 Binary Operations
    VecI16x8Add,
    VecI16x8Sub,
    VecI16x8Mul,
    VecI16x8AvgrU,
    VecI16x8Q15MulrSatS,
    // VecI16x8 Test Operations
    VecI16x8AllTrue,
    // VecI16x8 Compare Operations
    VecI16x8Eq,
    VecI16x8Ne,
    VecI16x8LtU,
    VecI16x8LtS,
    VecI16x8GtU,
    VecI16x8GtS,
    VecI16x8LeU,
    VecI16x8LeS,
    VecI16x8GeU,
    VecI16x8GeS,
    // VecI16x8 Shift Operations
    VecI16x8Shl,
    VecI16x8ShrU,
    VecI16x8ShrS,
    // VecI16x8 MinMax Operations
    VecI16x8MinU,
    VecI16x8MinS,
    VecI16x8MaxU,
    VecI16x8MaxS,
    // VecI16x8 Sat Binary Operations
    VecI16x8AddSatU,
    VecI16x8AddSatS,
    VecI16x8SubSatU,
    VecI16x8SubSatS,
    // Vec16x8 Convert Operations
    VecI16x8ExtendHalfI8x16U,
    VecI16x8ExtendHalfI8x16S,
    VecI16x8ExtMulHalfI8x16U,
    VecI16x8ExtMulHalfI8x16S,
    VecI16x8ExtAddPairwiseI8x16U,
    VecI16x8ExtAddPairwiseI8x16S,

    // VecI32x4 Instructions
    VecI32x4Splat,
    VecI32x4ExtractLane(LaneIdx),
    VecI32x4ReplaceLane(LaneIdx),
    VecI32x4Bitmask,
    VecI32x4DotI16x8S,
    // VecI32x4 Unary Operations
    VecI32x4Abs,
    VecI32x4Neg,
    // VecI32x4 Binary Operations
    VecI32x4Add,
    VecI32x4Sub,
    VecI32x4Mul,
    // VecI32x4 Test Operations
    VecI32x4AllTrue,
    // VecI32x4 Compare Operations
    VecI32x4Eq,
    VecI32x4Ne,
    VecI32x4LtU,
    VecI32x4LtS,
    VecI32x4GtU,
    VecI32x4GtS,
    VecI32x4LeU,
    VecI32x4LeS,
    VecI32x4GeU,
    VecI32x4GeS,
    // VecI32x4 Shift Operations
    VecI32x4Shl,
    VecI32x4ShrU,
    VecI32x4ShrS,
    // VecI32x4 MinMax Operations
    VecI32x4MinU,
    VecI32x4MinS,
    VecI32x4MaxU,
    VecI32x4MaxS,
    // VecI32x4 Convert Operations
    VecI32x4ExtendHalfI16x8U,
    VecI32x4ExtendHalfI16x8S,
    VecI32x4ExtMulHalfI16x8U,
    VecI32x4ExtMulHalfI16x8S,
    VecI32x4ExtAddPairwiseI16x8U,
    VecI32x4ExtAddPairwiseI16x8S,
    VecI32x4TruncSatF32x4U,
    VecI32x4TruncSatF32x4S,
    VecI32x4TruncSatF64x2UZero,
    VecI32x4TruncSatF64x2SZero,

    // VecI64x2 Instructions
    VecI64x2Splat,
    VecI64x2ExtractLane(LaneIdx),
    VecI64x2ReplaceLane(LaneIdx),
    VecI64x2Bitmask,
    // VecI64x2 Unary Operations
    VecI64x2Abs,
    VecI64x2Neg,
    // VecI64x2 Binary Operations
    VecI64x2Add,
    VecI64x2Sub,
    VecI64x2Mul,
    // VecI64x2 Test Operations
    VecI64x2AllTrue,
    // VecI64x2 Compare Operations
    VecI64x2Eq,
    VecI64x2Ne,
    VecI64x2LtS,
    VecI64x2GtS,
    VecI64x2LeS,
    VecI64x2GeS,
    // VecI64x2 Shift Operations
    VecI64x2Shl,
    VecI64x2ShrU,
    VecI64x2ShrS,
    // VecI64x2 Convert Operations
    VecI64x2ExtendHalfI32x4U,
    VecI64x2ExtendHalfI32x4S,
    VecI64x2ExtMulHalfI32x4U,
    VecI64x2ExtMulHalfI32x4S,

    // VecF32x4 Instructions
    VecF32x4Splat,
    VecF32x4ExtractLane(LaneIdx),
    VecF32x4ReplaceLane(LaneIdx),
    // VecF32x4 Unary Operations
    VecF32x4Abs,
    VecF32x4Neg,
    VecF32x4Sqrt,
    VecF32x4Ceil,
    VecF32x4Floor,
    VecF32x4Trunc,
    VecF32x4Nearest,
    // VecF32x4 Binary Operations
    VecF32x4Add,
    VecF32x4Sub,
    VecF32x4Mul,
    VecF32x4Div,
    VecF32x4Min,
    VecF32x4Max,
    VecF32x4PMin,
    VecF32x4PMax,
    // VecF32x4 Compare Operations
    VecF32x4Eq,
    VecF32x4Ne,
    VecF32x4Lt,
    VecF32x4Gt,
    VecF32x4Le,
    VecF32x4Ge,
    // VecF32x4 Convert Operations
    VecF32x4ConvertI32x4U,
    VecF32x4ConvertI32x4S,
    VecF32x4DemoteF64x2Zero,

    // VecF64x2 Instructions
    VecF64x2Splat,
    VecF64x2ExtractLane(LaneIdx),
    VecF64x2ReplaceLane(LaneIdx),
    // VecF64x2 Unary Operations
    VecF64x2Abs,
    VecF64x2Neg,
    VecF64x2Sqrt,
    VecF64x2Ceil,
    VecF64x2Floor,
    VecF64x2Trunc,
    VecF64x2Nearest,
    // VecF64x2 Binary Operations
    VecF64x2Add,
    VecF64x2Sub,
    VecF64x2Mul,
    VecF64x2Div,
    VecF64x2Min,
    VecF64x2Max,
    VecF64x2PMin,
    VecF64x2PMax,
    // VecF64x2 Compare Operations
    VecF64x2Eq,
    VecF64x2Ne,
    VecF64x2Lt,
    VecF64x2Gt,
    VecF64x2Le,
    VecF64x2Ge,
    // VecF64x2 Convert Operations
    VecF64x2ConvertLowI32x4U,
    VecF64x2ConvertLowI32x4S,
    VecF64x2PromotLowF32x4,

    //
    // Reference Instructions
    //
    RefIsNull,
    RefFunc(FuncIdx),
    RefNull(ValType),

    //
    // Parametric Instructions
    //
    Drop,
    Select(Option<Vec<ValType>>),

    //
    // Variable Instructions
    //
    LocalGet(LocalIdx),
    LocalSet(LocalIdx),
    LocalTee(LocalIdx),
    GlobalGet(GlobalIdx),
    GlobalSet(GlobalIdx),

    //
    // Table Instructions
    //
    TableGet(TableIdx),
    TableSet(TableIdx),
    TableSize(TableIdx),
    TableGrow(TableIdx),
    TableFill(TableIdx),
    TableCopy(TableIdx, TableIdx),
    TableInit(TableIdx, ElemIdx),
    TableElemDrop(ElemIdx),

    //
    // Memory Instructions
    //
    MemorySize,
    MemoryGrow,
    MemoryFill,
    MemoryCopy,
    MemoryInit(DataIdx),
    DataDrop(DataIdx),
    MemI32Load(MemArg),
    MemI64Load(MemArg),
    MemF32Load(MemArg),
    MemF64Load(MemArg),
    MemI32Load8U(MemArg),
    MemI32Load8S(MemArg),
    MemI32Load16U(MemArg),
    MemI32Load16S(MemArg),
    MemI64Load8U(MemArg),
    MemI64Load8S(MemArg),
    MemI64Load16U(MemArg),
    MemI64Load16S(MemArg),
    MemI64Load32U(MemArg),
    MemI64Load32S(MemArg),
    MemI32Store(MemArg),
    MemI64Store(MemArg),
    MemF32Store(MemArg),
    MemF64Store(MemArg),
    MemI32Store8(MemArg),
    MemI32Store16(MemArg),
    MemI64Store8(MemArg),
    MemI64Store16(MemArg),
    MemI64Store32(MemArg),
    MemV128Load(MemArg),
    MemV128Load8x8U(MemArg),
    MemV128Load8x8S(MemArg),
    MemV128Load16x4U(MemArg),
    MemV128Load16x4S(MemArg),
    MemV128Load32x2U(MemArg),
    MemV128Load32x2S(MemArg),
    MemV128Load32Zero(MemArg),
    MemV128Load64Zero(MemArg),
    MemV128Load8Splat(MemArg),
    MemV128Load16Splat(MemArg),
    MemV128Load32Splat(MemArg),
    MemV128Load64Splat(MemArg),
    MemV128Load8Lane(MemArg, LaneIdx),
    MemV128Load16Lane(MemArg, LaneIdx),
    MemV128Load32Lane(MemArg, LaneIdx),
    MemV128Load64Lane(MemArg, LaneIdx),
    MemV128Store(MemArg),
    MemV128Store8Lane(MemArg, LaneIdx),
    MemV128Store16Lane(MemArg, LaneIdx),
    MemV128Store32Lane(MemArg, LaneIdx),
    MemV128Store64Lane(MemArg, LaneIdx),

    //
    // Control Instructions
    //
    Nop,
    Unreachable,
    Return,
    End,
    Else,
    Loop(BlockType, Vec<Instr>),
    Block(BlockType, Vec<Instr>),
    If(BlockType, Vec<Instr>, Vec<Instr>),
    Br(LabelIdx),
    BrIf(LabelIdx),
    BrTable(Vec<LabelIdx>, LabelIdx),
    Call(FuncIdx),
    CallIndirect(TableIdx, TypeIdx),

    // Administrative Instructions
    Trap,
    Ref(FuncAddr),
    RefExtern(ExternAddr),
    Invoke(FuncAddr)
}

// OpCode Types
pub(super) type Expr = Vec<Instr>;

#[derive(Clone, Copy)]
pub(super) enum OpCode {
    Block,
    Loop,
    If,
    Else
}

impl Instr {
    pub fn validate(&self, v: &mut Validator) {
        let mut validated = false;
        validated = validated || self.validate_numeric(v);
        validated = validated || self.validate_reference(v);
        validated = validated || self.validate_vector(v);
        validated = validated || self.validate_parametric(v);
        validated = validated || self.validate_table(v);
        validated = validated || self.validate_memory(v);
        validated = validated || self.validate_control(v);

        if !validated {
            panic!("Invalid instruction received: {:?}", self);
        }
    }

    fn validate_numeric(&self, v: &Validator) -> bool {
        let mut skipped = false;

        use Instr::*;
        use ValType::*;
        match *self {
            // t.const
            I32Const(c) => { v.push_val(I32); },
            I64Const(c) => { v.push_val(I64); },
            F32Const(c) => { v.push_val(F32); },
            F64Const(c) => { v.push_val(F64); },

            // t.unop
            I32Clz => { v.pop_val(I32); v.push_val(I32); },
            I32Ctz => { v.pop_val(I32); v.push_val(I32); },
            I32PopCnt => { v.pop_val(I32); v.push_val(I32); },
            I64Clz => { v.pop_val(I64); v.push_val(I64); },
            I64Ctz => { v.pop_val(I64); v.push_val(I64); },
            I64PopCnt => { v.pop_val(I64); v.push_val(I64); },
            F32Abs => { v.pop_val(F32); v.push_val(F32); },
            F32Neg => { v.pop_val(F32); v.push_val(F32); },
            F32Sqrt => { v.pop_val(F32); v.push_val(F32); },
            F32Ceil => { v.pop_val(F32); v.push_val(F32); },
            F32Floor => { v.pop_val(F32); v.push_val(F32); },
            F32Trunc => { v.pop_val(F32); v.push_val(F32); },
            F32Nearest => { v.pop_val(F32); v.push_val(F32); },
            F64Abs => { v.pop_val(F64); v.push_val(F64); },
            F64Neg => { v.pop_val(F64); v.push_val(F64); },
            F64Sqrt => { v.pop_val(F64); v.push_val(F64); },
            F64Ceil => { v.pop_val(F64); v.push_val(F64); },
            F64Floor => { v.pop_val(F64); v.push_val(F64); },
            F64Trunc => { v.pop_val(F64); v.push_val(F64); },
            F64Nearest => { v.pop_val(F64); v.push_val(F64); },

            // t.binop
            I32Add => { v.pop_val(I32); v.pop_val(I32); v.push_val(I32); },
            I32Sub => { v.pop_val(I32); v.pop_val(I32); v.push_val(I32); },
            I32Mul => { v.pop_val(I32); v.pop_val(I32); v.push_val(I32); },
            I32DivU => { v.pop_val(I32); v.pop_val(I32); v.push_val(I32); },
            I32DivS => { v.pop_val(I32); v.pop_val(I32); v.push_val(I32); },
            I32RemU => { v.pop_val(I32); v.pop_val(I32); v.push_val(I32); },
            I32RemS => { v.pop_val(I32); v.pop_val(I32); v.push_val(I32); },
            I64Add => { v.pop_val(I64); v.pop_val(I64); v.push_val(I64); },
            I64Sub => { v.pop_val(I64); v.pop_val(I64); v.push_val(I64); },
            I64Mul => { v.pop_val(I64); v.pop_val(I64); v.push_val(I64); },
            I64DivU => { v.pop_val(I64); v.pop_val(I64); v.push_val(I64); },
            I64DivS => { v.pop_val(I64); v.pop_val(I64); v.push_val(I64); },
            I64RemU => { v.pop_val(I64); v.pop_val(I64); v.push_val(I64); },
            I64RemS => { v.pop_val(I64); v.pop_val(I64); v.push_val(I64); },
            F32Add => { v.pop_val(F32); v.pop_val(F32); v.push_val(F32); },
            F32Sub => { v.pop_val(F32); v.pop_val(F32); v.push_val(F32); },
            F32Mul => { v.pop_val(F32); v.pop_val(F32); v.push_val(F32); },
            F32Div => { v.pop_val(F32); v.pop_val(F32); v.push_val(F32); },
            F32Min => { v.pop_val(F32); v.pop_val(F32); v.push_val(F32); },
            F32Max => { v.pop_val(F32); v.pop_val(F32); v.push_val(F32); },
            F32Copysign => { v.pop_val(F32); v.pop_val(F32); v.push_val(F32); },
            F64Add => { v.pop_val(F64); v.pop_val(F64); v.push_val(F64); },
            F64Sub => { v.pop_val(F64); v.pop_val(F64); v.push_val(F64); },
            F64Mul => { v.pop_val(F64); v.pop_val(F64); v.push_val(F64); },
            F64Div => { v.pop_val(F64); v.pop_val(F64); v.push_val(F64); },
            F64Min => { v.pop_val(F64); v.pop_val(F64); v.push_val(F64); },
            F64Max => { v.pop_val(F64); v.pop_val(F64); v.push_val(F64); },
            F64Copysign => { v.pop_val(F64); v.pop_val(F64); v.push_val(F64); },

            // t.testop
            I32Eqz => { v.pop_val(I32); v.push_val(I32); },
            I64Eqz => { v.pop_val(I64); v.push_val(I32); },

            // t.relop
            I32Eq => { v.pop_val(I32); v.pop_val(I32); v.push_val(I32); },
            I32Ne => { v.pop_val(I32); v.pop_val(I32); v.push_val(I32); },
            I32LtU => { v.pop_val(I32); v.pop_val(I32); v.push_val(I32); },
            I32LtS => { v.pop_val(I32); v.pop_val(I32); v.push_val(I32); },
            I32GtU => { v.pop_val(I32); v.pop_val(I32); v.push_val(I32); },
            I32GtS => { v.pop_val(I32); v.pop_val(I32); v.push_val(I32); },
            I32LeU => { v.pop_val(I32); v.pop_val(I32); v.push_val(I32); },
            I32LeS => { v.pop_val(I32); v.pop_val(I32); v.push_val(I32); },
            I32GeU => { v.pop_val(I32); v.pop_val(I32); v.push_val(I32); }
            I32GeS => { v.pop_val(I32); v.pop_val(I32); v.push_val(I32); },
            I64Eq => { v.pop_val(I64); v.pop_val(I64); v.push_val(I32); },
            I64Ne => { v.pop_val(I64); v.pop_val(I64); v.push_val(I32); },
            I64LtU => { v.pop_val(I64); v.pop_val(I64); v.push_val(I32); },
            I64LtS => { v.pop_val(I64); v.pop_val(I64); v.push_val(I32); },
            I64GtU => { v.pop_val(I64); v.pop_val(I64); v.push_val(I32); },
            I64GtS => { v.pop_val(I64); v.pop_val(I64); v.push_val(I32); },
            I64LeU => { v.pop_val(I64); v.pop_val(I64); v.push_val(I32); },
            I64LeS => { v.pop_val(I64); v.pop_val(I64); v.push_val(I32); },
            I64GeU => { v.pop_val(I64); v.pop_val(I64); v.push_val(I32); },
            I64GeS => { v.pop_val(I64); v.pop_val(I64); v.push_val(I32); },
            F32Eq => { v.pop_val(F32); v.pop_val(F32); v.push_val(I32); },
            F32Ne => { v.pop_val(F32); v.pop_val(F32); v.push_val(I32); },
            F32Lt => { v.pop_val(F32); v.pop_val(F32); v.push_val(I32); },
            F32Gt => { v.pop_val(F32); v.pop_val(F32); v.push_val(I32); },
            F32Le => { v.pop_val(F32); v.pop_val(F32); v.push_val(I32); },
            F32Ge => { v.pop_val(F32); v.pop_val(F32); v.push_val(I32); },
            F64Eq => { v.pop_val(F64); v.pop_val(F64); v.push_val(I32); },
            F64Ne => { v.pop_val(F64); v.pop_val(F64); v.push_val(I32); },
            F64Lt => { v.pop_val(F64); v.pop_val(F64); v.push_val(I32); },
            F64Gt => { v.pop_val(F64); v.pop_val(F64); v.push_val(I32); },
            F64Le => { v.pop_val(F64); v.pop_val(F64); v.push_val(I32); },
            F64Ge => { v.pop_val(F64); v.pop_val(F64); v.push_val(I32); },

            // t.cvtop_t1_sx
            I32Extend8S => { v.pop_val(I32); v.push_val(I32); },
            I32Extend16S => { v.pop_val(I32); v.push_val(I32); },
            I32WrapI64 => { v.pop_val(I64); v.push_val(I32); },
            I32TruncF32U => { v.pop_val(F32); v.push_val(I32); },
            I32TruncF32S => { v.pop_val(F32); v.push_val(I32); },
            I32TruncF64U => { v.pop_val(F64); v.push_val(I32); },
            I32TruncF64S => { v.pop_val(F64); v.push_val(I32); },
            I32TruncSatF32U => { v.pop_val(F32); v.push_val(I32); },
            I32TruncSatF32S => { v.pop_val(F32); v.push_val(I32); },
            I32TruncSatF64U => { v.pop_val(F64); v.push_val(I32); },
            I32TruncSatF64S => { v.pop_val(F64); v.push_val(I32); },
            I32ReinterpretF32 => { v.pop_val(F32); v.push_val(I32); },
            I64Extend8S => { v.pop_val(I32); v.push_val(I64); },
            I64Extend16S => { v.pop_val(I32); v.push_val(I64); },
            I64Extend32S => { v.pop_val(I32); v.push_val(I64); },
            I64ExtendI32U => { v.pop_val(I32); v.push_val(I64); },
            I64ExtendI32S => { v.pop_val(I32); v.push_val(I64); },
            I64TruncF32U => { v.pop_val(F32); v.push_val(I64); },
            I64TruncF32S => { v.pop_val(F32); v.push_val(I64); },
            I64TruncF64U => { v.pop_val(F64); v.push_val(I64); },
            I64TruncF64S => { v.pop_val(F64); v.push_val(I64); },
            I64TruncSatF32U => { v.pop_val(F32); v.push_val(I64); },
            I64TruncSatF32S => { v.pop_val(F32); v.push_val(I64); },
            I64TruncSatF64U => { v.pop_val(F64); v.push_val(I64); },
            I64TruncSatF64S => { v.pop_val(F64); v.push_val(I64); },
            I64ReinterpretF64 => { v.pop_val(F64); v.push_val(I64); },
            F32DemoteF64 => { v.pop_val(F64); v.push_val(F32); },
            F32ConvertI32U => { v.pop_val(I32); v.push_val(F32); },
            F32ConvertI32S => { v.pop_val(I32); v.push_val(F32); },
            F32ConvertI64U => { v.pop_val(I64); v.push_val(F32); },
            F32ConvertI64S => { v.pop_val(I64); v.push_val(F32); },
            F32ReinterpretI32 => { v.pop_val(I32); v.push_val(F32); },
            F32ReinterpretI64 => { v.pop_val(I64); v.push_val(F32); },
            F64PromoteF32 => { v.pop_val(I64); v.push_val(F64); },
            F64ConvertI32U => { v.pop_val(I32); v.push_val(F64); },
            F64ConvertI32S => { v.pop_val(I32); v.push_val(F64); },
            F64ConvertI64U => { v.pop_val(I64); v.push_val(F64); },
            F64ConvertI64S => { v.pop_val(I64); v.push_val(F64); },
            F64ReinterpretI32 => { v.pop_val(I32); v.push_val(F64); },
            F64ReinterpretI64 => { v.pop_val(I64); v.push_val(F64); },

            _ => { skipped = true; }
        }

        !skipped
    }

    fn validate_reference(&self, v: &Validator) -> bool {
        let mut skipped = false;

        use Instr::*;
        use ValType::*;
        match *self {
            RefNull(ValType::FuncRef) => { v.push_val(ValType::FuncRef); },
            RefNull(ValType::ExternRef) => { v.push_val(ValType::ExternRef); },
            RefIsNull => {
                let val = v.pop_val(Unknown);
                
                if val.is_ref() {
                    v.push_val(I32)
                } else {
                    panic!("RefIsNull: expected funcref or externref, got {:?}", val)
                }
            },

            RefFunc(funcidx) => {
                if v.ctx.funcs.len() <= funcidx {
                    panic!("RefFunc: C.funcs - index {} out of bounds", funcidx)
                }

                if v.ctx.refs.contains(&funcidx) {
                    panic!("RefFunc: C.refs - does not contain funcidx ({})", funcidx)
                }

                v.push_val(ValType::FuncRef);
            }

            _ => { skipped = true; }
        }

        !skipped
    }

    fn validate_vector(&self, v: &Validator) -> bool {
        let mut skipped = false;

        use Instr::*;
        use ValType::*;
        match *self {
            V128Const(c) => { v.push_val(V128); },

            // v128.vvternop
            V128Bitselect => { v.pop_vals(vec![V128, V128, V128]); v.push_val(V128); },
            
            // VecI8x16 Instructions
            VecI8x16Swizzle => { v.pop_vals(vec![V128, V128]); v.push_val(V128); }
            VecI8x16Shuffle(laneindices) => {
                for laneidx in laneindices {
                    if laneidx >= 32 {
                        panic!("For all laneidxi​, in laneidx16, laneidxi​ must be smaller than 32.")
                    }

                    v.pop_vals(vec![V128, V128]); v.push_val(V128);
                }
            }

            // shape.splat
            VecI8x16Splat => { v.pop_val(I32); v.push_val(V128); },
            VecI16x8Splat => { v.pop_val(I32); v.push_val(V128); },
            VecI32x4Splat => { v.pop_val(I32); v.push_val(V128); },
            VecI64x2Splat => { v.pop_val(I64); v.push_val(V128); },
            VecF32x4Splat => { v.pop_val(F32); v.push_val(V128); },
            VecF64x2Splat => { v.pop_val(F64); v.push_val(V128); },
            
            // shape.extract_lane_sx
            VecI8x16ExtractLaneU(laneidx) => {
                if laneidx >= 16 { panic!("laneidx must be smaller than 16.") }
                v.pop_val(V128); v.push_val(I32);
            },
            VecI8x16ExtractLaneS(laneidx) => {
                if laneidx >= 16 { panic!("laneidx must be smaller than 16.") }
                v.pop_val(V128); v.push_val(I32);
            },
            VecI16x8ExtractLaneU(laneidx) => {
                if laneidx >= 8 { panic!("laneidx must be smaller than 8.") }
                v.pop_val(V128); v.push_val(I32);
            }
            VecI16x8ExtractLaneS(laneidx) => {
                if laneidx >= 8 { panic!("laneidx must be smaller than 8.") }
                v.pop_val(V128); v.push_val(I32);
            }
            VecI32x4ExtractLane(laneidx) => {
                if laneidx >= 4 { panic!("laneidx must be smaller than 4.") }
                v.pop_val(V128); v.push_val(I32);
            }
            VecI64x2ExtractLane(laneidx) => {
                if laneidx >= 2 { panic!("laneidx must be smaller than 4.") }
                v.pop_val(V128); v.push_val(I64);
            }
            VecF32x4ExtractLane(laneidx) => {
                if laneidx >= 4 { panic!("laneidx must be smaller than 4.") }
                v.pop_val(V128); v.push_val(F32);
            }
            VecF64x2ExtractLane(laneidx) => {
                if laneidx >= 2 { panic!("laneidx must be smaller than 2.") }
                v.pop_val(V128); v.push_val(F64);
            }

            // shape.replace_lane
            VecI8x16ReplaceLane(laneidx) => {
                if laneidx >= 16 { panic!("laneidx must be smaller than 16.") }
                v.pop_vals(vec![V128, I32]); v.push_val(V128);
            },
            VecI16x8ReplaceLane(laneidx) => {
                if laneidx >= 8 { panic!("laneidx must be smaller than 8.") }
                v.pop_vals(vec![V128, I32]); v.push_val(V128);
            },
            VecI32x4ReplaceLane(laneidx) => {
                if laneidx >= 4 { panic!("laneidx must be smaller than 4.") }
                v.pop_vals(vec![V128, I32]); v.push_val(V128);
            },
            VecI64x2ReplaceLane(laneidx) => {
                if laneidx >= 2 { panic!("laneidx must be smaller than 2.") }
                v.pop_vals(vec![V128, I64]); v.push_val(V128);
            },
            VecF32x4ReplaceLane(laneidx) => {
                if laneidx >= 4 { panic!("laneidx must be smaller than 4.") }
                v.pop_vals(vec![V128, F32]); v.push_val(V128);
            },
            VecF64x2ReplaceLane(laneidx) => {
                if laneidx >= 2 { panic!("laneidx must be smaller than 2.") }
                v.pop_vals(vec![V128, F64]); v.push_val(V128);
            },

            // shape.vunop
            V128Not => { v.pop_val(V128); v.push_val(V128); },
            VecI8x16Abs => { v.pop_val(V128); v.push_val(V128); },
            VecI8x16Neg => { v.pop_val(V128); v.push_val(V128); },
            VecI8x16PopCnt => { v.pop_val(V128); v.push_val(V128); },
            VecI16x8Abs => { v.pop_val(V128); v.push_val(V128); },
            VecI16x8Neg => { v.pop_val(V128); v.push_val(V128); },
            VecI32x4Abs => { v.pop_val(V128); v.push_val(V128); },
            VecI32x4Neg => { v.pop_val(V128); v.push_val(V128); },
            VecI64x2Abs => { v.pop_val(V128); v.push_val(V128); },
            VecI64x2Neg => { v.pop_val(V128); v.push_val(V128); },
            VecF32x4Abs => { v.pop_val(V128); v.push_val(V128); },
            VecF32x4Neg => { v.pop_val(V128); v.push_val(V128); },
            VecF32x4Sqrt => { v.pop_val(V128); v.push_val(V128); },
            VecF32x4Ceil => { v.pop_val(V128); v.push_val(V128); },
            VecF32x4Floor => { v.pop_val(V128); v.push_val(V128); },
            VecF32x4Trunc => { v.pop_val(V128); v.push_val(V128); },
            VecF32x4Nearest => { v.pop_val(V128); v.push_val(V128); },
            VecF64x2Abs => { v.pop_val(V128); v.push_val(V128); },
            VecF64x2Neg => { v.pop_val(V128); v.push_val(V128); },
            VecF64x2Sqrt => { v.pop_val(V128); v.push_val(V128); },
            VecF64x2Ceil => { v.pop_val(V128); v.push_val(V128); },
            VecF64x2Floor => { v.pop_val(V128); v.push_val(V128); },
            VecF64x2Trunc => { v.pop_val(V128); v.push_val(V128); },
            VecF64x2Nearest => { v.pop_val(V128); v.push_val(V128); },

            // shape.vbinop        
            V128And => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            V128AndNot => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            V128Or => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            V128Xor => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI8x16Add => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI8x16Sub => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI8x16AvgrU => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI16x8Add => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI16x8Sub => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI16x8Mul => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI16x8AvgrU => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI16x8Q15MulrSatS => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI32x4Add => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI32x4Sub => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI32x4Mul => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI64x2Add => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI64x2Sub => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI64x2Mul => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF32x4Add => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF32x4Sub => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF32x4Mul => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF32x4Div => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF32x4Min => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF32x4Max => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF32x4PMin => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF32x4PMax => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF64x2Add => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF64x2Sub => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF64x2Mul => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF64x2Div => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF64x2Min => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF64x2Max => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF64x2PMin => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF64x2PMax => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },

            // shape.vrelop
            VecI8x16Eq => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI8x16Ne => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI8x16LtU => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI8x16LtS => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI8x16GtU => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI8x16GtS => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI8x16LeU => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI8x16LeS => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI8x16GeU => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI8x16GeS => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI16x8Eq => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI16x8Ne => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI16x8LtU => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI16x8LtS => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI16x8GtU => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI16x8GtS => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI16x8LeU => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI16x8LeS => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI16x8GeU => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI16x8GeS => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI32x4Eq => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI32x4Ne => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI32x4LtU => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI32x4LtS => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI32x4GtU => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI32x4GtS => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI32x4LeU => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI32x4LeS => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI32x4GeU => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI32x4GeS => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI64x2Eq => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI64x2Ne => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI64x2LtS => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI64x2GtS => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI64x2LeS => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI64x2GeS => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF32x4Eq => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF32x4Ne => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF32x4Lt => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF32x4Gt => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF32x4Le => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF32x4Ge => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF64x2Eq => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF64x2Ne => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF64x2Lt => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF64x2Gt => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF64x2Le => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecF64x2Ge => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            
            // ishape.vshiftop
            VecI8x16Shl => { v.pop_vals(vec![V128, I32]); v.push_val(V128); },
            VecI8x16ShrU => { v.pop_vals(vec![V128, I32]); v.push_val(V128); },
            VecI8x16ShrS => { v.pop_vals(vec![V128, I32]); v.push_val(V128); },
            VecI16x8Shl => { v.pop_vals(vec![V128, I32]); v.push_val(V128); },
            VecI16x8ShrU => { v.pop_vals(vec![V128, I32]); v.push_val(V128); },
            VecI16x8ShrS => { v.pop_vals(vec![V128, I32]); v.push_val(V128); },
            VecI32x4Shl => { v.pop_vals(vec![V128, I32]); v.push_val(V128); },
            VecI32x4ShrU => { v.pop_vals(vec![V128, I32]); v.push_val(V128); },
            VecI32x4ShrS => { v.pop_vals(vec![V128, I32]); v.push_val(V128); },
            VecI64x2Shl => { v.pop_vals(vec![V128, I32]); v.push_val(V128); },
            VecI64x2ShrU => { v.pop_vals(vec![V128, I32]); v.push_val(V128); },
            VecI64x2ShrS => { v.pop_vals(vec![V128, I32]); v.push_val(V128); },

            // shape.vtestop
            V128AnyTrue => { v.pop_val(V128); v.push_val(I32) },
            VecI8x16AllTrue => { v.pop_val(V128); v.push_val(I32) },
            VecI16x8AllTrue => { v.pop_val(V128); v.push_val(I32) },
            VecI32x4AllTrue => { v.pop_val(V128); v.push_val(I32) },
            VecI64x2AllTrue => { v.pop_val(V128); v.push_val(I32) },

            // shape.vcvtop
            VecI16x8ExtendHalfI8x16U => { v.pop_val(V128); v.push_val(V128) },
            VecI16x8ExtendHalfI8x16S => { v.pop_val(V128); v.push_val(V128) },
            VecI32x4ExtendHalfI16x8U => { v.pop_val(V128); v.push_val(V128) },
            VecI32x4ExtendHalfI16x8S => { v.pop_val(V128); v.push_val(V128) },
            VecI32x4TruncSatF32x4U => { v.pop_val(V128); v.push_val(V128) },
            VecI32x4TruncSatF32x4S => { v.pop_val(V128); v.push_val(V128) },
            VecI32x4TruncSatF64x2UZero => { v.pop_val(V128); v.push_val(V128) },
            VecI32x4TruncSatF64x2SZero => { v.pop_val(V128); v.push_val(V128) },
            VecI64x2ExtendHalfI32x4U => { v.pop_val(V128); v.push_val(V128) },
            VecI64x2ExtendHalfI32x4S => { v.pop_val(V128); v.push_val(V128) },
            VecF32x4ConvertI32x4U => { v.pop_val(V128); v.push_val(V128) },
            VecF32x4ConvertI32x4S => { v.pop_val(V128); v.push_val(V128) },
            VecF32x4DemoteF64x2Zero => { v.pop_val(V128); v.push_val(V128) },
            VecF64x2ConvertLowI32x4U => { v.pop_val(V128); v.push_val(V128) },
            VecF64x2ConvertLowI32x4S => { v.pop_val(V128); v.push_val(V128) },
            VecF64x2PromotLowF32x4 => { v.pop_val(V128); v.push_val(V128) },

            // ishape.narrow
            VecI8x16NarrowI16x8U => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI8x16NarrowI16x8S => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI16x8NarrowI32x4U => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI16x8NarrowI32x4S => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },

            // ishape.bitmask
            VecI8x16Bitmask => { v.pop_val(V128); v.push_val(I32) },
            VecI16x8Bitmask => { v.pop_val(V128); v.push_val(I32) },
            VecI32x4Bitmask => { v.pop_val(V128); v.push_val(I32) },
            VecI64x2Bitmask => { v.pop_val(V128); v.push_val(I32) },

            // ishape.dot
            VecI32x4DotI16x8S => { v.pop_vals(vec![V128, V128]); v.push_val(V128)},

            // ishape.extmul
            VecI16x8ExtMulHalfI8x16U => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI16x8ExtMulHalfI8x16S => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI32x4ExtMulHalfI16x8U => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI32x4ExtMulHalfI16x8S => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI64x2ExtMulHalfI32x4U => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },
            VecI64x2ExtMulHalfI32x4S => { v.pop_vals(vec![V128, V128]); v.push_val(V128); },

            // ishape.extadd_pairwise
            VecI16x8ExtAddPairwiseI8x16U => { v.pop_val(V128); v.push_val(V128) },
            VecI16x8ExtAddPairwiseI8x16S => { v.pop_val(V128); v.push_val(V128) },
            VecI32x4ExtAddPairwiseI16x8U => { v.pop_val(V128); v.push_val(V128) },
            VecI32x4ExtAddPairwiseI16x8S => { v.pop_val(V128); v.push_val(V128) },

            _ => { skipped = true; }
        }

        !skipped
    }

    fn validate_parametric(&self, v: &Validator) -> bool {
        let mut skipped = false;

        use Instr::*;
        use ValType::*;
        match *self {
            Drop => { v.pop_val(Unknown); },
            Select(None) => {
                v.pop_val(I32);
                let t1 = v.pop_val(Unknown);
                let t2 = v.pop_val(Unknown);
        
                let areNum = t1.is_num() && t2.is_num();
                let areVec = t1.is_vec() && t2.is_vec();
                if !areNum && !areVec {
                    panic!("Select: operands must be numeric or vector");
                }
        
                if !matches!(t1, t2) && !matches!(t1, ValType::Unknown) && !matches!(t2, ValType::Unknown) {
                    panic!("Select: Operands must be the same type");
                }
        
                v.push_val( if matches!(t1, ValType::Unknown) { t2 } else { t1 } );
            }

            Select(Some(valtypes)) => {
                if valtypes.len() != 1 {
                    panic!("Select: must be given exactly one value type");
                }

                v.pop_vals(vec![valtypes[0], valtypes[0], I32]);
                v.push_val(valtypes[0]);
            }

            LocalGet(x) => {
                if v.ctx.locals.len() <= x { panic!("local {} out of range", x); }

                let t = v.ctx.locals[x];
                v.push_val(t);
            }

            LocalSet(x) => {
                if v.ctx.locals.len() <= x { panic!("local {} out of range", x); }

                let t = v.ctx.locals[x];
                v.pop_val(t);
            }

            LocalTee(x) => {
                if v.ctx.locals.len() <= x { panic!("local {} out of range", x); }

                let t = v.ctx.locals[x];
                v.pop_val(t);
                v.push_val(t);
            }

            GlobalGet(x) => {
                if v.ctx.globals.len() <= x { panic!("global {} out of range", x); }

                let glbl = v.ctx.globals[x];
                let t = glbl.valtype;
                v.push_val(t);
            }

            GlobalSet(x) => {
                if v.ctx.globals.len() <= x { panic!("global {} out of range", x); }

                let glbl = v.ctx.globals[x];
                let m = glbl.valmut;
                if !matches!(m, Mut::Var) {
                    panic!("global {} is immutable", x);
                }

                let t = glbl.valtype;
                v.pop_val(t);
            }

            _ => { skipped = true; }
        }

        !skipped
    }

    fn validate_table(&self, v: &Validator) -> bool {
        let mut skipped = false;

        use Instr::*;
        use ValType::*;
        match *self {
            TableGet(x) => {
                if v.ctx.tables.len() <= x { panic!("table {} out of range", x); }
                let t = v.ctx.tables[x].reftype;

                v.pop_val(I32);
                v.push_val(t);
            }

            TableSet(x) => {
                if v.ctx.tables.len() <= x { panic!("table {} out of range", x); }
                let t = v.ctx.tables[x].reftype;

                v.pop_val(I32);
                v.pop_val(t);
            }

            TableSize(x) => {
                if v.ctx.tables.len() <= x { panic!("table {} out of range", x); }
                v.push_val(I32);
            }

            TableGrow(x) => {
                if v.ctx.tables.len() <= x { panic!("table {} out of range", x); }
                let t = v.ctx.tables[x].reftype;

                v.pop_vals(vec![t, I32]);
                v.push_val(I32);
            }

            TableFill(x) => {
                if v.ctx.tables.len() <= x { panic!("table {} out of range", x); }
                let t = v.ctx.tables[x].reftype;

                v.pop_vals(vec![I32, t, I32]);
            }

            TableCopy(x, y) => {
                if v.ctx.tables.len() <= x { panic!("table {} out of range (argument 1)", x); }
                if v.ctx.tables.len() <= y { panic!("table {} out of range (argument 2)", y); }

                let t1 = v.ctx.tables[x].reftype;
                let t2 = v.ctx.tables[y].reftype;
                if !matches!(t1, t2) {
                    panic!("TableCopy: operands must be the same type");
                }

                v.pop_vals(vec![I32, I32, I32]);
            }

            TableInit(x, y) => {
                if v.ctx.tables.len() <= x { panic!("table {} out of range", x); }
                if v.ctx.elems.len() <= y { panic!("element {} out of range", y); }

                let t1 = v.ctx.tables[x].reftype;
                let t2 = v.ctx.elems[y];
                if !matches!(t1, t2) {
                    panic!("TableInit: operands must be the same type");
                }

                v.pop_vals(vec![I32, I32, I32]);
            }

            TableElemDrop(x) => {
                if v.ctx.elems.len() <= x { panic!("element {} out of range", x); }
            }

            _ => { skipped = true; }
        }

        !skipped
    }

    fn validate_memory(&self, v: &Validator) -> bool {
        let mut skipped = false;

        use Instr::*;
        use ValType::*;
        match *self {
            MemorySize => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                v.push_val(I32);
            }
            MemoryGrow => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                v.pop_val(I32); v.push_val(I32);
            }
            MemoryFill => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                v.pop_vals(vec![I32, I32, I32]);
            }
            MemoryCopy => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                v.pop_vals(vec![I32, I32, I32]);
            }
            MemoryInit(x) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if v.ctx.datas.len() <= x { panic!("datas {} out of range", x); }
                v.pop_vals(vec![I32, I32, I32]);
            }
            DataDrop(x) => {
                if v.ctx.datas.len() <= x { panic!("datas {} out of range", x); }
            }

            // t.load
            MemI32Load(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (32/8) { panic!("align must be < 32/8"); }
                v.pop_val(I32); v.push_val(I32);
            }
            MemI64Load(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (64/8) { panic!("align must be < 64/8"); }
                v.pop_val(I32); v.push_val(I64);
            }
            MemF32Load(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (32/8) { panic!("align must be < 32/8"); }
                v.pop_val(I32); v.push_val(F32);
            }
            MemF64Load(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (64/8) { panic!("align must be < 64/8"); }
                v.pop_val(I32); v.push_val(F64);
            }
            MemV128Load(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (128/8) { panic!("align must be < 128/8"); }
                v.pop_val(I32); v.push_val(V128);
            }

            // t.loadN_sx
            MemI32Load8U(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (8/8) { panic!("align must be < 8/8"); }
                v.pop_val(I32); v.push_val(I32);
            }
            MemI32Load8S(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (8/8) { panic!("align must be < 8/8"); }
                v.pop_val(I32); v.push_val(I32);
            }
            MemI32Load16U(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (16/8) { panic!("align must be < 16/8"); }
                v.pop_val(I32); v.push_val(I32);
            }
            MemI32Load16S(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (16/8) { panic!("align must be < 16/8"); }
                v.pop_val(I32); v.push_val(I32);
            }
            MemI64Load8U(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (8/8) { panic!("align must be < 8/8"); }
                v.pop_val(I32); v.push_val(I64);
            }
            MemI64Load8S(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (8/8) { panic!("align must be < 8/8"); }
                v.pop_val(I32); v.push_val(I64);
            }
            MemI64Load16S(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (16/8) { panic!("align must be < 16/8"); }
                v.pop_val(I32); v.push_val(I64);
            }
            MemI64Load32U(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (32/8) { panic!("align must be < 32/8"); }
                v.pop_val(I32); v.push_val(I64);
            }
            MemI64Load32S(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (32/8) { panic!("align must be < 32/8"); }
                v.pop_val(I32); v.push_val(I64);
            }

            // t.store
            MemI32Store(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (32/8) { panic!("align must be < 32/8"); }
                v.pop_vals(vec![I32, I32]);
            }
            MemI64Store(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (64/8) { panic!("align must be < 64/8"); }
                v.pop_vals(vec![I32, I64]);
            }
            MemF32Store(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (32/8) { panic!("align must be < 32/8"); }
                v.pop_vals(vec![I32, F32]);
            }
            MemF64Store(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (64/8) { panic!("align must be < 64/8"); }
                v.pop_vals(vec![I32, F64]);
            }
            MemV128Store(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (128/8) { panic!("align must be < 128/8"); }
                v.pop_vals(vec![I32, V128]);
            }

            // t.storeN
            MemI32Store8(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (8/8) { panic!("align must be < 8/8"); }
                v.pop_vals(vec![I32, I32]);
            }
            MemI32Store16(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (16/8) { panic!("align must be < 16/8"); }
                v.pop_vals(vec![I32, I32]);
            }
            MemI64Store8(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (8/8) { panic!("align must be < 8/8"); }
                v.pop_vals(vec![I32, I64]);
            }
            MemI64Store16(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (16/8) { panic!("align must be < 16/8"); }
                v.pop_vals(vec![I32, I64]);

            }
            MemI64Store32(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= (32/8) { panic!("align must be < 32/8"); }
                v.pop_vals(vec![I32, I64]);
            }

            // v128.loadNxM
            MemV128Load8x8U(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= ((8/8)*8) { panic!("align must be < (8/8) * 8"); }
                v.pop_val(I32); v.push_val(V128);
            }
            MemV128Load8x8S(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= ((8/8)*8) { panic!("align must be < (8/8) * 8"); }
                v.pop_val(I32); v.push_val(V128);
            }
            MemV128Load16x4U(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= ((16/8)*4) { panic!("align must be < (16/8) * 4"); }
                v.pop_val(I32); v.push_val(V128);
            }
            MemV128Load16x4S(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= ((16/8)*4) { panic!("align must be < (16/8) * 4"); }
                v.pop_val(I32); v.push_val(V128);
            }
            MemV128Load32x2U(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= ((32/8)*2) { panic!("align must be < (32/8) * 2"); }
                v.pop_val(I32); v.push_val(V128);
            }
            MemV128Load32x2S(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= ((32/8)*2) { panic!("align must be < (32/8) * 2"); }
                v.pop_val(I32); v.push_val(V128);
            }

            // v128.loadN_splat
            MemV128Load8Splat(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= 8/8 { panic!("align must be < 8/8"); }
                v.pop_val(I32); v.push_val(V128);
            }
            MemV128Load16Splat(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= 16/8 { panic!("align must be < 16/8"); }
                v.pop_val(I32); v.push_val(V128);
            }
            MemV128Load32Splat(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= 32/8 { panic!("align must be < 32/8"); }
                v.pop_val(I32); v.push_val(V128);
            }
            MemV128Load64Splat(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= 64/8 { panic!("align must be < 64/8"); }
                v.pop_val(I32); v.push_val(V128);
            }

            // V128.loadN_zero
            MemV128Load32Zero(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= 32/8 { panic!("align must be < 32/8"); }
                v.pop_val(I32); v.push_val(V128);
            }
            MemV128Load64Zero(memarg) => {
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= 64/8 { panic!("align must be < 64/8"); }
                v.pop_val(I32); v.push_val(V128);
            }

            // v128.loadN_lane
            MemV128Load8Lane(memarg, laneidx) => {
                if laneidx >= 128/8 { panic!("laneidx must be < 128/8"); }
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= 8/8 { panic!("align must be < 8/8"); }
                v.pop_vals(vec![I32, V128]); v.push_val(V128);
            }
            MemV128Load16Lane(memarg, laneidx) => {
                if laneidx >= 128/16 { panic!("laneidx must be < 128/16"); }
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= 16/8 { panic!("align must be < 16/8"); }
                v.pop_vals(vec![I32, V128]); v.push_val(V128);
            }
            MemV128Load32Lane(memarg, laneidx) => {
                if laneidx >= 128/32 { panic!("laneidx must be < 128/32"); }
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= 32/8 { panic!("align must be < 32/8"); }
                v.pop_vals(vec![I32, V128]); v.push_val(V128);
            }
            MemV128Load64Lane(memarg, laneidx) => {
                if laneidx >= 128/64 { panic!("laneidx must be < 128/64"); }
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= 64/8 { panic!("align must be < 64/8"); }
                v.pop_vals(vec![I32, V128]); v.push_val(V128);
            }

            // v128.storeN_lane
            MemV128Store8Lane(memarg, laneidx) => {
                if laneidx >= 128/8 { panic!("laneidx must be < 128/8"); }
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= 8/8 { panic!("align must be < 8/8"); }
                v.pop_vals(vec![I32, V128]); v.push_val(V128);
            }
            MemV128Store16Lane(memarg, laneidx) => {
                if laneidx >= 128/16 { panic!("laneidx must be < 128/16"); }
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= 16/8 { panic!("align must be < 16/8"); }
                v.pop_vals(vec![I32, V128]); v.push_val(V128);
            }
            MemV128Store32Lane(memarg, laneidx) => {
                if laneidx >= 128/32 { panic!("laneidx must be < 128/32"); }
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= 32/8 { panic!("align must be < 32/8"); }
                v.pop_vals(vec![I32, V128]); v.push_val(V128);
            }
            MemV128Store64Lane(memarg, laneidx) => {
                if laneidx >= 128/64 { panic!("laneidx must be < 128/64"); }
                if v.ctx.mems.len() <= 0 { panic!("C.mems[0] not defined"); }
                if memarg.align >= 64/8 { panic!("align must be < 64/8"); }
                v.pop_vals(vec![I32, V128]); v.push_val(V128);
            }


            _ => { skipped = true; }
        }

        !skipped
    }

    fn validate_control(&self, v: &Validator) -> bool {
        let mut skipped = false;

        use Instr::*;
        use ValType::*;
        match *self {
            Nop => (),
            
            Unreachable => v.unreachable(),

            End => {
                let frame = v.pop_ctrl();
                v.push_vals(frame.end_types);
            }

            Else => {
                let frame = v.pop_ctrl();
                if !matches!(frame.opcode, OpCode::If) {
                    panic!("else must follow if");
                }

                v.push_ctrl(OpCode::Else, frame.start_types, frame.end_types);
            }

            Return => {
                if v.ctx._return.is_none() {
                    panic!("C._return is absent")
                }

                match v.ctx._return {
                    None => panic!("_return values are absent from context"),
                    Some(t) => {
                        v.pop_vals(t);
                        v.vals_resize(v.ctrls[0].height);
                    }
                }

            }

            Call(x) => {
                if v.ctx.funcs.len() <= x { panic!("func {} out of range", x); }
                v.pop_vals(v.ctx.funcs[x].inputs);
                v.push_vals(v.ctx.funcs[x].returns);
            }

            CallIndirect(x, y) => {
                if v.ctx.tables.len() <= x { panic!("table {} out of range", x); }

                let t = v.ctx.tables[x].reftype;
                if !matches!(t, FuncRef) { panic!("table {} is not a function table", x); }
                if v.ctx.types.len() <= y { panic!("type {} out of range", y); }

                let functype = v.ctx.types[y];
                v.pop_val(I32);
                v.pop_vals(functype.inputs);
                v.push_vals(functype.returns);

            }

            Block(blocktype, instrs) => {
                let func_type = v.ctx.get_block_type(blocktype);
                v.pop_vals(func_type.inputs);
                v.push_ctrl(OpCode::Block, func_type.inputs, func_type.returns);
            }

            Loop(blocktype, instrs) => {
                let func_type = v.ctx.get_block_type(blocktype);
                v.pop_vals(func_type.inputs);
                v.push_ctrl(OpCode::Loop, func_type.inputs, func_type.returns);
            }

            If(blocktype, then_instrs, else_instrs) => {
                let func_type = v.ctx.get_block_type(blocktype);
                v.pop_val(ValType::I32);
                v.pop_vals(func_type.inputs);
                v.push_ctrl(OpCode::If, func_type.inputs, func_type.returns);
            }

            Br(idx) => {
                if v.ctrls.len() <= idx {
                    panic!("branch out of range");
                }

                v.pop_vals(v.label_types(v.ctrls[idx]));
                v.unreachable()
            }

            BrIf(idx) => {
                if v.ctrls.len() <= idx {
                    panic!("branch_if out of range");
                }

                v.pop_val(ValType::I32);
                v.pop_vals(v.label_types(v.ctrls[idx]));
                v.push_vals(v.label_types(v.ctrls[idx]));
            }

            BrTable(indices, m) => {
                v.pop_val(ValType::I32);
                if v.ctrls.len() <= m {
                    panic!("branch_table out of range");
                }

                let arity = v.label_types(v.ctrls[m]).len();
                for n in indices {
                    if v.ctrls.len() <= n {
                        panic!("branch_table out of range");
                    } else if v.label_types(v.ctrls[n]).len() != arity {
                        panic!("branch_table: label types must match");
                    }

                    v.push_vals(v.pop_vals(v.label_types(v.ctrls[n])));
                }

                v.pop_vals(v.label_types(v.ctrls[m]));
                v.unreachable()
            }

            _ => { skipped = true; }
        }

        !skipped
    }
}
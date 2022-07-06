use super::types::*;
use super::validations::Validator;

pub(super) enum Instr {
    //
    // Numeric Instructions
    //
    // I32 Instructions
    I32Const(u32),
    // I32 Unary Operations
    I32Clz(I32),
    I32Ctz(I32),
    I32PopCnt(I32),
    // I32 Binary Operations
    I32Add(I32, I32),
    I32Sub(I32, I32),
    I32Mul(I32, I32),
    I32DivU(I32, I32),
    I32DivS(I32, I32),
    I32RemU(I32, I32),
    I32RemS(I32, I32),
    // I32 Test Operations
    I32Eqz(I32),
    // I32 Comparison Operations
    I32Eq(I32, I32),
    I32Ne(I32, I32),
    I32LtU(I32, I32),
    I32LtS(I32, I32),
    I32GtU(I32, I32),
    I32GtS(I32, I32),
    I32LeU(I32, I32),
    I32LeS(I32, I32),
    I32GeU(I32, I32),
    I32GeS(I32, I32),
    // I32 Convert Operations
    I32Extend8S(I32),
    I32Extend16S(I32),
    I32WrapI64(I64),
    I32TruncF32U(F32),
    I32TruncF32S(F32),
    I32TruncF64U(F64),
    I32TruncF64S(F64),
    I32TruncSatF32U(F32),
    I32TruncSatF32S(F32),
    I32TruncSatF64U(F64),
    I32TruncSatF64S(F64),
    I32ReinterpretF32(F32),

    // I64 Instructions
    I64Const(u64),
    // I64 Unary Operations
    I64Clz(I64),
    I64Ctz(I64),
    I64PopCnt(I64),
    // I64 Binary Operations
    I64Add(I64, I64),
    I64Sub(I64, I64),
    I64Mul(I64, I64),
    I64DivU(I64, I64),
    I64DivS(I64, I64),
    I64RemU(I64, I64),
    I64RemS(I64, I64),
    // I64 Test Operations
    I64Eqz(I64),
    // I64 Comparison Operations
    I64Eq(I64, I64),
    I64Ne(I64, I64),
    I64LtU(I64, I64),
    I64LtS(I64, I64),
    I64GtU(I64, I64),
    I64GtS(I64, I64),
    I64LeU(I64, I64),
    I64LeS(I64, I64),
    I64GeU(I64, I64),
    I64GeS(I64, I64),
    // I64 Convert Operations
    I64Extend8S(I32),
    I64Extend16S(I32),
    I64Extend32S(I32),
    I64ExtendI32U(I32),
    I64ExtendI32S(I32),
    I64TruncF32U(F32),
    I64TruncF32S(F32),
    I64TruncF64U(F64),
    I64TruncF64S(F64),
    I64TruncSatF32U(F32),
    I64TruncSatF32S(F32),
    I64TruncSatF64U(F64),
    I64TruncSatF64S(F64),
    I64ReinterpretF64(F64),

    // F32 Instructions
    F32Const(f32),
    // F32 Unary Operations
    F32Abs(F32),
    F32Neg(F32),
    F32Sqrt(F32),
    F32Ceil(F32),
    F32Floor(F32),
    F32Trunc(F32),
    F32Nearest(F32),
    // F32 Binary Operations
    F32Add(F32, F32),
    F32Sub(F32, F32),
    F32Mul(F32, F32),
    F32Div(F32, F32),
    F32Min(F32, F32),
    F32Max(F32, F32),
    F32Copysign(F32, F32),
    // F32 Comparison Operations
    F32Eq(F32, F32),
    F32Ne(F32, F32),
    F32Lt(F32, F32),
    F32Gt(F32, F32),
    F32Le(F32, F32),
    F32Ge(F32, F32),
    // F32 Convert Operations
    F32DemoteF64(F64),
    F32ConvertI32U(I32),
    F32ConvertI32S(I32),
    F32ConvertI64U(I64),
    F32ConvertI64S(I64),
    F32ReinterpretI32(I32),
    F32ReinterpretI64(I64),

    // F64 Instructions
    F64Const(f64),
    // F64 Unary Operations
    F64Abs(F64),
    F64Neg(F64),
    F64Sqrt(F64),
    F64Ceil(F64),
    F64Floor(F64),
    F64Trunc(F64),
    F64Nearest(F64),
    // F64 Binary Operations
    F64Add(F64, F64),
    F64Sub(F64, F64),
    F64Mul(F64, F64),
    F64Div(F64, F64),
    F64Min(F64, F64),
    F64Max(F64, F64),
    F64Copysign(F64, F64),
    // F64 Comparison Operations
    F64Eq(F64, F64),
    F64Ne(F64, F64),
    F64Lt(F64, F64),
    F64Gt(F64, F64),
    F64Le(F64, F64),
    F64Ge(F64, F64),
    // F64 Convert Operations
    F64PromoteF32(F32),
    F64ConvertI32U(I32),
    F64ConvertI32S(I32),
    F64ConvertI64U(I64),
    F64ConvertI64S(I64),
    F64ReinterpretI32(I32),
    F64ReinterpretI64(I64),

    //
    // Vector Instructions
    //
    // Vec128 Instructions
    Vec128Const(u128),
    // Vec128 Unary Operations
    Vec128Not(Vec128),
    // Vec128 Binary Operations
    Vec128And(Vec128, Vec128),
    Vec128AndNot(Vec128, Vec128),
    Vec128Or(Vec128, Vec128),
    Vec128Xor(Vec128, Vec128),
    // Vec128 Comparison Operations
    Vec128Bitselect(Vec128, Vec128, Vec128),
    // Vec128 Test Operations
    Vec128AnyTrue(Vec128),

    // VecI8x16 Instructions
    VecI8x16Shuffle(VecI8x16),
    VecI8x16Swizzle(VecI8x16),
    VecI8x16Splat(u128),
    VecI8x16ExtractLaneU(VecI8x16, LaneIdx),
    VecI8x16ExtractLaneS(VecI8x16, LaneIdx),
    VecI8x16ReplaceLane(VecI8x16, LaneIdx),
    VecI8x16Bitmask(VecI8x16),
    VecI8x16NarrowI16x8U(VecI8x16),
    VecI8x16NarrowI16x8S(VecI8x16),
    // VecI8x16 Unary Operations
    VecI8x16Abs(VecI8x16),
    VecI8x16Neg(VecI8x16),
    VecI8x16PopCnt(VecI8x16),
    // VecI8x16 Binary Operations
    VecI8x16Add(VecI8x16, VecI8x16),
    VecI8x16Sub(VecI8x16, VecI8x16),
    VecI8x16AvgrU(VecI8x16, VecI8x16),
    // VecI8x16 Compare Operations
    VecI8x16Eq(VecI8x16, VecI8x16),
    VecI8x16Ne(VecI8x16, VecI8x16),
    VecI8x16LtU(VecI8x16, VecI8x16),
    VecI8x16LtS(VecI8x16, VecI8x16),
    VecI8x16GtU(VecI8x16, VecI8x16),
    VecI8x16GtS(VecI8x16, VecI8x16),
    VecI8x16LeU(VecI8x16, VecI8x16),
    VecI8x16LeS(VecI8x16, VecI8x16),
    VecI8x16GeU(VecI8x16, VecI8x16),
    VecI8x16GeS(VecI8x16, VecI8x16),
    // VecI8x16 Test Operations
    VecI8x16AllTrue(VecI8x16),
    // VecI8x16 Shift Operations
    VecI8x16Shl(VecI8x16, I32),
    VecI8x16ShrU(VecI8x16, I32),
    VecI8x16ShrS(VecI8x16, I32),
    // VecI8x16 MinMax Operations
    VecI8x16MinU(VecI8x16, VecI8x16),
    VecI8x16MinS(VecI8x16, VecI8x16),
    VecI8x16MaxU(VecI8x16, VecI8x16),
    VecI8x16MaxS(VecI8x16, VecI8x16),
    // VecI8x16 Sat Binary Operations
    VecI8x16AddSatU(VecI8x16),
    VecI8x16AddSatS(VecI8x16),
    VecI8x16SubSatU(VecI8x16),
    VecI8x16SubSatS(VecI8x16),

    // VecI16x8 Instructions
    VecI16x8Splat(u128),
    VecI16x8ExtractLaneU(VecI16x8, LaneIdx),
    VecI16x8ExtractLaneS(VecI16x8, LaneIdx),
    VecI16x8ReplaceLane(VecI16x8, LaneIdx),
    VecI16x8Bitmask(VecI16x8),
    VecI16x8NarrowI32x4U(VecI16x8),
    VecI16x8NarrowI32x4S(VecI16x8),
    // VecI16x8 Unary Operations
    VecI16x8Abs(VecI16x8),
    VecI16x8Neg(VecI16x8),
    // VecI16x8 Binary Operations
    VecI16x8Add(VecI16x8, VecI16x8),
    VecI16x8Sub(VecI16x8, VecI16x8),
    VecI16x8Mul(VecI16x8, VecI16x8),
    VecI16x8AvgrU(VecI16x8, VecI16x8),
    VecI16x8Q15MulrSatS(VecI16x8, VecI16x8),
    // VecI16x8 Test Operations
    VecI16x8AllTrue(VecI16x8),
    // VecI16x8 Compare Operations
    VecI16x8Eq(VecI16x8, VecI16x8),
    VecI16x8Ne(VecI16x8, VecI16x8),
    VecI16x8LtU(VecI16x8, VecI16x8),
    VecI16x8LtS(VecI16x8, VecI16x8),
    VecI16x8GtU(VecI16x8, VecI16x8),
    VecI16x8GtS(VecI16x8, VecI16x8),
    VecI16x8LeU(VecI16x8, VecI16x8),
    VecI16x8LeS(VecI16x8, VecI16x8),
    VecI16x8GeU(VecI16x8, VecI16x8),
    VecI16x8GeS(VecI16x8, VecI16x8),
    // VecI16x8 Shift Operations
    VecI16x8Shl(VecI16x8, I32),
    VecI16x8ShrU(VecI16x8, I32),
    VecI16x8ShrS(VecI16x8, I32),
    // VecI16x8 MinMax Operations
    VecI16x8MinU(VecI16x8, VecI16x8),
    VecI16x8MinS(VecI16x8, VecI16x8),
    VecI16x8MaxU(VecI16x8, VecI16x8),
    VecI16x8MaxS(VecI16x8, VecI16x8),
    // VecI16x8 Sat Binary Operations
    VecI16x8AddSatU(VecI16x8),
    VecI16x8AddSatS(VecI16x8),
    VecI16x8SubSatU(VecI16x8),
    VecI16x8SubSatS(VecI16x8),
    // Vec16x8 Convert Operations
    VecI16x8ExtendHalfI8x16U(VecI8x16),
    VecI16x8ExtendHalfI8x16S(VecI8x16),
    VecI16x8ExtMulHalfI8x16U(VecI8x16),
    VecI16x8ExtMulHalfI8x16S(VecI8x16),
    VecI16x8ExtAddPairwiseI8x16U(VecI8x16),
    VecI16x8ExtAddPairwiseI8x16S(VecI8x16),

    // VecI32x4 Instructions
    VecI32x4Splat(u128),
    VecI32x4ExtractLane(VecI32x4, LaneIdx),
    VecI32x4ReplaceLane(VecI32x4, LaneIdx),
    VecI32x4Bitmask(VecI32x4),
    VecI32x4DotI16x8S(VecI16x8),
    // VecI32x4 Unary Operations
    VecI32x4Abs(VecI32x4),
    VecI32x4Neg(VecI32x4),
    // VecI32x4 Binary Operations
    VecI32x4Add(VecI32x4, VecI32x4),
    VecI32x4Sub(VecI32x4, VecI32x4),
    VecI32x4Mul(VecI32x4, VecI32x4),
    // VecI32x4 Test Operations
    VecI32x4AllTrue(VecI32x4),
    // VecI32x4 Compare Operations
    VecI32x4Eq(VecI32x4, VecI32x4),
    VecI32x4Ne(VecI32x4, VecI32x4),
    VecI32x4LtU(VecI32x4, VecI32x4),
    VecI32x4LtS(VecI32x4, VecI32x4),
    VecI32x4GtU(VecI32x4, VecI32x4),
    VecI32x4GtS(VecI32x4, VecI32x4),
    VecI32x4LeU(VecI32x4, VecI32x4),
    VecI32x4LeS(VecI32x4, VecI32x4),
    VecI32x4GeU(VecI32x4, VecI32x4),
    VecI32x4GeS(VecI32x4, VecI32x4),
    // VecI32x4 Shift Operations
    VecI32x4Shl(VecI32x4, I32),
    VecI32x4ShrU(VecI32x4, I32),
    VecI32x4ShrS(VecI32x4, I32),
    // VecI32x4 MinMax Operations
    VecI32x4MinU(VecI32x4, VecI32x4),
    VecI32x4MinS(VecI32x4, VecI32x4),
    VecI32x4MaxU(VecI32x4, VecI32x4),
    VecI32x4MaxS(VecI32x4, VecI32x4),
    // VecI32x4 Convert Operations
    VecI32x4ExtendHalfI16x8U(VecI16x8),
    VecI32x4ExtendHalfI16x8S(VecI16x8),
    VecI32x4ExtMulHalfI16x8U(VecI16x8),
    VecI32x4ExtMulHalfI16x8S(VecI16x8),
    VecI32x4ExtAddPairwiseI16x8U(VecI16x8),
    VecI32x4ExtAddPairwiseI16x8S(VecI16x8),
    VecI32x4TruncSatF32x4U(VecF32x4),
    VecI32x4TruncSatF32x4S(VecF32x4),
    VecI32x4TruncSatF64x2UZero(VecF64x2),
    VecI32x4TruncSatF64x2SZero(VecF64x2),

    // VecI64x2 Instructions
    VecI64x2Splat(u128),
    VecI64x2ExtractLane(VecI64x2, LaneIdx),
    VecI64x2ReplaceLane(VecI64x2, LaneIdx),
    VecI64x2Bitmask(VecI64x2),
    // VecI64x2 Unary Operations
    VecI64x2Abs(VecI64x2),
    VecI64x2Neg(VecI64x2),
    // VecI64x2 Binary Operations
    VecI64x2Add(VecI64x2, VecI64x2),
    VecI64x2Sub(VecI64x2, VecI64x2),
    VecI64x2Mul(VecI64x2, VecI64x2),
    // VecI64x2 Test Operations
    VecI64x2AllTrue(VecI64x2),
    // VecI64x2 Compare Operations
    VecI64x2Eq(VecI64x2, VecI64x2),
    VecI64x2Ne(VecI64x2, VecI64x2),
    VecI64x2LtS(VecI64x2, VecI64x2),
    VecI64x2GtS(VecI64x2, VecI64x2),
    VecI64x2LeS(VecI64x2, VecI64x2),
    VecI64x2GeS(VecI64x2, VecI64x2),
    // VecI64x2 Shift Operations
    VecI64x2Shl(VecI64x2, I32),
    VecI64x2ShrU(VecI64x2, I32),
    VecI64x2ShrS(VecI64x2, I32),
    // VecI64x2 Convert Operations
    VecI64x2ExtendHalfI32x4U(VecI32x4),
    VecI64x2ExtendHalfI32x4S(VecI32x4),
    VecI64x2ExtMulHalfI32x4U(VecI32x4),
    VecI64x2ExtMulHalfI32x4S(VecI32x4),

    // VecF32x4 Instructions
    VecF32x4Splat(u128),
    VecF32x4ExtractLane(VecF32x4, LaneIdx),
    VecF32x4ReplaceLane(VecF32x4, LaneIdx),
    // VecF32x4 Unary Operations
    VecF32x4Abs(VecF32x4),
    VecF32x4Neg(VecF32x4),
    VecF32x4Sqrt(VecF32x4),
    VecF32x4Ceil(VecF32x4),
    VecF32x4Floor(VecF32x4),
    VecF32x4Trunc(VecF32x4),
    VecF32x4Nearest(VecF32x4),
    // VecF32x4 Binary Operations
    VecF32x4Add(VecF32x4, VecF32x4),
    VecF32x4Sub(VecF32x4, VecF32x4),
    VecF32x4Mul(VecF32x4, VecF32x4),
    VecF32x4Div(VecF32x4, VecF32x4),
    VecF32x4Min(VecF32x4, VecF32x4),
    VecF32x4Max(VecF32x4, VecF32x4),
    VecF32x4PMin(VecF32x4, VecF32x4),
    VecF32x4PMax(VecF32x4, VecF32x4),
    // VecF32x4 Compare Operations
    VecF32x4Eq(VecF32x4, VecF32x4),
    VecF32x4Ne(VecF32x4, VecF32x4),
    VecF32x4Lt(VecF32x4, VecF32x4),
    VecF32x4Gt(VecF32x4, VecF32x4),
    VecF32x4Le(VecF32x4, VecF32x4),
    VecF32x4Ge(VecF32x4, VecF32x4),
    // VecF32x4 Convert Operations
    VecF32x4ConvertI32x4U(VecI32x4),
    VecF32x4ConvertI32x4S(VecI32x4),
    VecF32x4DemoteF64x2Zero(VecF64x2),

    // VecF64x2 Instructions
    VecF64x2Splat(u128),
    VecF64x2ExtractLane(VecF64x2, LaneIdx),
    VecF64x2ReplaceLane(VecF64x2, LaneIdx),
    // VecF64x2 Unary Operations
    VecF64x2Abs(VecF64x2),
    VecF64x2Neg(VecF64x2),
    VecF64x2Sqrt(VecF64x2),
    VecF64x2Ceil(VecF64x2),
    VecF64x2Floor(VecF64x2),
    VecF64x2Trunc(VecF64x2),
    VecF64x2Nearest(VecF64x2),
    // VecF64x2 Binary Operations
    VecF64x2Add(VecF64x2, VecF64x2),
    VecF64x2Sub(VecF64x2, VecF64x2),
    VecF64x2Mul(VecF64x2, VecF64x2),
    VecF64x2Div(VecF64x2, VecF64x2),
    VecF64x2Min(VecF64x2, VecF64x2),
    VecF64x2Max(VecF64x2, VecF64x2),
    VecF64x2PMin(VecF64x2, VecF64x2),
    VecF64x2PMax(VecF64x2, VecF64x2),
    // VecF64x2 Compare Operations
    VecF64x2Eq(VecF64x2, VecF64x2),
    VecF64x2Ne(VecF64x2, VecF64x2),
    VecF64x2Lt(VecF64x2, VecF64x2),
    VecF64x2Gt(VecF64x2, VecF64x2),
    VecF64x2Le(VecF64x2, VecF64x2),
    VecF64x2Ge(VecF64x2, VecF64x2),
    // VecF64x2 Convert Operations
    VecF64x2ConvertLowI32x4U(VecI32x4),
    VecF64x2ConvertLowI32x4S(VecI32x4),
    VecF64x2PromotLowF32x4(VecF32x4),

    //
    // Reference Instructions
    //
    FuncRefNull(),
    FuncRef(RefType),
    ExternRefNull(),
    ExternRef(RefType),

    //
    // Parametric Instructions
    //
    Drop(),
    Select(Option<ValType>),

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
    MemSize(),
    MemGrow(),
    MemFill(),
    MemCopy(),
    MemInit(DataIdx),
    DataDrop(DataIdx),
    MemI32Load(Offset, Align),
    MemI64Load(Offset, Align),
    MemF32Load(Offset, Align),
    MemF64Load(Offset, Align),
    MemI32Load8U(Offset, Align),
    MemI32Load8S(Offset, Align),
    MemI32Load16U(Offset, Align),
    MemI32Load16S(Offset, Align),
    MemI64Load8U(Offset, Align),
    MemI64Load8S(Offset, Align),
    MemI64Load16U(Offset, Align),
    MemI64Load16S(Offset, Align),
    MemI64Load32U(Offset, Align),
    MemI64Load32S(Offset, Align),
    MemI32Store(Offset, Align),
    MemI64Store(Offset, Align),
    MemF32Store(Offset, Align),
    MemF64Store(Offset, Align),
    MemI32Store8(Offset, Align),
    MemI32Store16(Offset, Align),
    MemI64Store8(Offset, Align),
    MemI64Store16(Offset, Align),
    MemI64Store32(Offset, Align),
    MemV128Load(Offset, Align),
    MemV128Load8x8U(Offset, Align),
    MemV128Load8x8S(Offset, Align),
    MemV128Load16x4U(Offset, Align),
    MemV128Load16x4S(Offset, Align),
    MemV128Load32x2U(Offset, Align),
    MemV128Load32x2S(Offset, Align),
    MemV128Load32Zero(Offset, Align),
    MemV128Load64Zero(Offset, Align),
    MemV128Load8Splat(Offset, Align),
    MemV128Load16Splat(Offset, Align),
    MemV128Load32Splat(Offset, Align),
    MemV128Load64Splat(Offset, Align),
    MemV128Load8Lane(Offset, Align),
    MemV128Load16Lane(Offset, Align),
    MemV128Load32Lane(Offset, Align),
    MemV128Load64Lane(Offset, Align),
    MemV128Store(Offset, Align),
    MemV128Store8Lane(Offset, Align),
    MemV128Store16Lane(Offset, Align),
    MemV128Store32Lane(Offset, Align),
    MemV128Store64Lane(Offset, Align),

    //
    // Control Instructions
    //
    Nop,
    Unreachable,
    Loop(BlockType, Vec<Instr>),
    Block(BlockType, Vec<Instr>),
    If(BlockType, Vec<Instr>, Vec<Instr>),
    Br(LabelIdx),
    BrIf(LabelIdx),
    BrTable(Vec<LabelIdx>, LabelIdx),
    Return,
    Call(FuncIdx),
    CallIndirect(TableIdx, TypeIdx),
}

pub(super) type Expr = Vec<Instr>;

impl Instr {
    pub fn validate(&self, v: &mut Validator) {
        match *self {
            Unreachable => v.unreachable(),
            Drop => { v.pop_val(ValType::Any); },

            Instr::Select(None) => {
                v.pop_val(ValType::I32);
                let t1 = v.pop_val(ValType::Any);
                let t2 = v.pop_val(ValType::Any);
        
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

            Instr::Select(Some(t)) => {
                v.pop_val(ValType::I32);
                v.pop_val(t);
                v.pop_val(t);
                v.push_val(t);
            }

            Instr::Block(blocktype, instrs) => {
                let func_type = v.module.get_block_type(blocktype);
                v.pop_vals(func_type.inputs);
                v.push_ctrl(OpCode::Block, func_type.inputs, func_type.returns);
            }

        };
    }
}
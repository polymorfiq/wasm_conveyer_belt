use super::types::*;

pub(super) enum Instr {
    NumericInst(NumericInst),
    VecInst(VecInst),
    ParametricInst(ParametricInst),
    VariableInst(VariableInst),
    TableInst(TableInst),
    MemoryInst(MemoryInst),
    ControlInst(ControlInst),
    ExprInst(ExprInst),
}

//
// Numeric Instructions
//
pub(super) enum NumericInst {
    I32Inst(I32Inst),
    I64Inst(I64Inst),
    F32Inst(F32Inst),
    F64Inst(F64Inst),
}

// Unary Operations
// Consume one operand and produce one result of the respective type.
pub(super) enum UnOp {
    I32UnaryOp(I32UnaryOp),
    I64UnaryOp(I64UnaryOp),
    F32UnaryOp(F32UnaryOp),
    F64UnaryOp(F64UnaryOp),
}

// Binary Operations
// Consume two operands and produce one result of the respective type.
pub(super) enum BinOp {
    I32BinaryOp(I32BinaryOp),
    I64BinaryOp(I64BinaryOp),
    F32BinaryOp(F32BinaryOp),
    F64BinaryOp(F64BinaryOp),
}

// Test Operations
// Consume one operand of the respective type and produce a Boolean integer result.
pub(super) enum TestOp {
    I32TestOp(I32TestOp),
    I64TestOp(I64TestOp),
}

// Comparison Operations
// Consume two operands of the respective type and produce a Boolean integer result.
pub(super) enum RelOp {
    I32RelOp(I32RelOp),
    I64RelOp(I64RelOp),
    F32RelOp(F32RelOp),
    F64RelOp(F64RelOp),
}

// Conversion Operations
// Consume a value of one type and produce a result of another.
pub (super) enum CvtOp {
    I32CvtOp(I32CvtOp),
    I64CvtOp(I64CvtOp),
    F32CvtOp(F32CvtOp),
    F64CvtOp(F64CvtOp),
}

// I32 Instructions
pub(super) enum I32Inst {
    Const(u32),

    I32UnaryOp(I32UnaryOp),
    I32BinaryOp(I32BinaryOp),
    I32TestOp(I32TestOp),
    I32RelOp(I32RelOp),
    I32CvtOp(I32CvtOp),
}

pub(super) enum I32UnaryOp {
    Clz(I32),
    Ctz(I32),
    PopCnt(I32),
}

pub(super) enum I32BinaryOp {
    Add(I32, I32),
    Sub(I32, I32),
    Mul(I32, I32),
    DivU(I32, I32),
    DivS(I32, I32),
    RemU(I32, I32),
    RemS(I32, I32),
}

pub(super) enum I32TestOp {
    EqZ(I32),
}

pub(super) enum I32RelOp {
    Eq(I32, I32),
    Ne(I32, I32),
    LtU(I32, I32),
    LtS(I32, I32),
    GtU(I32, I32),
    GtS(I32, I32),
    LeU(I32, I32),
    LeS(I32, I32),
    GeU(I32, I32),
    GeS(I32, I32),
}

pub(super) enum I32CvtOp {
    Extend8S(I32),
    Extend16S(I32),
    WrapI64(I64),
    TruncF32U(I32),
    TruncF32S(I32),
    TruncF64U(I32),
    TruncF64S(I32),
    TruncSatF32U(I32),
    TruncSatF32S(I32),
    TruncSatF64U(I32),
    TruncSatF64S(I32),
    ReinterpritF32(I32),
}

// I64 Instructions
pub(super) enum I64Inst {
    Const(u64),

    I64UnaryOp(I64UnaryOp),
    I64BinaryOp(I64BinaryOp),
    I64TestOp(I64TestOp),
    I64RelOp(I64RelOp),
    I64CvtOp(I64CvtOp),
}

pub(super) enum I64UnaryOp {
    Clz(I64),
    Ctz(I64),
    PopCnt(I64),
}

pub(super) enum I64BinaryOp {
    Add(I64, I64),
    Sub(I64, I64),
    Mul(I64, I64),
    DivU(I64, I64),
    DivS(I64, I64),
    RemU(I64, I64),
    RemS(I64, I64),
}

pub(super) enum I64TestOp {
    EqZ(I64),
}

pub(super) enum I64RelOp {
    Eq(I64, I64),
    Ne(I64, I64),
    LtU(I64, I64),
    LtS(I64, I64),
    GtU(I64, I64),
    GtS(I64, I64),
    LeU(I64, I64),
    LeS(I64, I64),
    GeU(I64, I64),
    GeS(I64, I64),
}

pub(super) enum I64CvtOp {
    Extend8S(I64),
    Extend16S(I64),
    Extend32S(I64),
    ExtendI32U(I32),
    ExtendI32S(I32),
    TruncF32U(I64),
    TruncF32S(I64),
    TruncF64U(I64),
    TruncF64S(I64),
    TruncSatF32U(I64),
    TruncSatF32S(I64),
    TruncSatF64U(I64),
    TruncSatF64S(I64),
    ReinterpritF64(I64),
}

// F32 Instructions
pub(super) enum F32Inst {
    Const(f32),

    F32UnaryOp(F32UnaryOp),
    F32BinaryOp(F32BinaryOp),
    F32RelOp(F32RelOp),
    F32CvtOp(F32CvtOp),
}

pub(super) enum F32UnaryOp {
    Abs(F32),
    Neg(F32),
    Sqrt(F32),
    Ceil(F32),
    Floor(F32),
    Trunc(F32),
    Nearest(F32),
}

pub(super) enum F32BinaryOp {
    Add(F32, F32),
    Sub(F32, F32),
    Mul(F32, F32),
    Div(F32, F32),
    Min(F32, F32),
    Max(F32, F32),
    CopySign(F32, F32),
}

pub(super) enum F32RelOp {
    Eq(F32, F32),
    Ne(F32, F32),
    Lt(F32, F32),
    Gt(F32, F32),
    Le(F32, F32),
    Ge(F32, F32),
}

pub(super) enum F32CvtOp {
    DemoteF64(F64),
    ConvertI32U(I32),
    ConvertI32S(I32),
    ConvertI64U(I64),
    ConvertI64S(I64),
    ReinterpritI32(I32),
    ReinterpritI64(I64),
}

// F64 Instructions
pub(super) enum F64Inst {
    Const(f64),

    F64UnaryOp(F64UnaryOp),
    F64BinaryOp(F64BinaryOp),
    F64RelOp(F64RelOp),
    F64CvtOp(F64CvtOp),
}

pub(super) enum F64UnaryOp {
    Abs(F64),
    Neg(F64),
    Sqrt(F64),
    Ceil(F64),
    Floor(F64),
    Trunc(F64),
    Nearest(F64),
}

pub(super) enum F64BinaryOp {
    Add(F64, F64),
    Sub(F64, F64),
    Mul(F64, F64),
    Div(F64, F64),
    Min(F64, F64),
    Max(F64, F64),
    CopySign(F64, F64),
}

pub(super) enum F64RelOp {
    Eq(F64, F64),
    Ne(F64, F64),
    Lt(F64, F64),
    Gt(F64, F64),
    Le(F64, F64),
    Ge(F64, F64),
}

pub(super) enum F64CvtOp {
    PromoteF32(F32),
    ConvertI32U(I32),
    ConvertI32S(I32),
    ConvertI64U(I64),
    ConvertI64S(I64),
    ReinterpritI32(I32),
    ReinterpritI64(I32),
}

//
// Vector instructions
//
pub(super) enum VecInst {
    Vec128Inst(Vec128Inst),
    VecI8X16Inst(VecI8X16Inst),
    VecI16X8Inst(VecI16X8Inst),
    VecI32x4Inst(VecI32X4Inst),
    VecI64x2Inst(VecI64X2Inst),
    VecF32X4Inst(VecF32X4Inst),
    VecF64X2Inst(VecF64X2Inst),
}

pub(super) enum VUnOp {
    Vec128UnaryOp(Vec128UnaryOp),
    VecI8X16UnaryOp(VecI8X16UnaryOp),
    VecI16X8UnaryOp(VecI16X8UnaryOp),
    VecI32X4UnaryOp(VecI32X4UnaryOp),
    VecI64X2UnaryOp(VecI64X2UnaryOp),
    VecF32X4UnaryOp(VecF32X4UnaryOp),
    VecF64X2UnaryOp(VecF64X2UnaryOp),
}

pub(super) enum VBinOp {
    Vec128BinaryOp(Vec128BinaryOp),

    VecI8X16BinaryOp(VecI8X16BinaryOp),
    VecI8X16MinMaxOp(VecI8X16MinMaxOp),
    VecI8X16SatBinaryOp(VecI8X16SatBinaryOp),

    VecI16X8BinaryOp(VecI16X8BinaryOp),
    VecI16X8MinMaxOp(VecI16X8MinMaxOp),
    VecI16X8SatBinaryOp(VecI16X8SatBinaryOp),

    VecI32X4BinaryOp(VecI32X4BinaryOp),
    VecI32X4MinMaxOp(VecI32X4MinMaxOp),

    VecI64X2BinaryOp(VecI64X2BinaryOp),

    VecF32X4BinaryOp(VecF32X4BinaryOp),
    VecF64X2BinaryOp(VecF64X2BinaryOp),
}

pub(super) enum VTestOp {
    VecI8X16TestOp(VecI8X16TestOp),
    VecI16X8TestOp(VecI16X8TestOp),
    VecI32X4TestOp(VecI32X4TestOp),
    VecI64X2TestOp(VecI64X2TestOp),
}

pub(super) enum VRelOp {
    VecI8X16RelOp(VecI8X16RelOp),
    VecI16X8RelOp(VecI16X8RelOp),
    VecI32X4RelOp(VecI32X4RelOp),
    VecI64X2RelOp(VecI64X2RelOp),
    VecF32X4RelOp(VecF32X4RelOp),
    VecF64X2RelOp(VecF64X2RelOp),
}

pub(super) enum VCvtOp {
    VecI16X8ConvertOp(VecI16X8ConvertOp),
    VecI32X4ConvertOp(VecI32X4ConvertOp),
    VecF32X4ConvertOp(VecF32X4ConvertOp),
    VecF64X2ConvertOp(VecF64X2ConvertOp),
}

// Vec128 Instructions
pub(super) enum Vec128Inst {
    Const(u128),
    
    Vec128UnaryOp(Vec128UnaryOp),
    Vec128BinaryOp(Vec128BinaryOp),
    Vec128TernaryOp(Vec128TernaryOp),
    Vec128TestOp(Vec128TestOp),
}

pub(super) enum Vec128UnaryOp {
    Not(Vec128),
}

pub(super) enum Vec128BinaryOp {
    And(Vec128, Vec128),
    AndNot(Vec128, Vec128),
    Or(Vec128, Vec128),
    Xor(Vec128, Vec128),
}

pub(super) enum Vec128TernaryOp {
    Bitselect(Vec128, Vec128, Vec128),
}

pub(super) enum Vec128TestOp {
    AnyTrue(Vec128),
}

// VecI8X16 Instructions
pub(super) enum VecI8X16Inst {
    Shuffle(VecI8X16),
    Swizzle(VecI8X16),
    Splat(I128),
    ExtractLaneU(VecI8X16, LaneIdx),
    ExtractLaneS(VecI8X16, LaneIdx),
    ReplaceLane(VecI8X16, LaneIdx),
    Bitmask(VecI8X16),
    NarrowI16X8U(VecI16X8),
    NarrowI16X8S(VecI16X8),

    VecI8X16RelOp(VecI8X16RelOp),
    VecI8X16UnaryOp(VecI8X16UnaryOp),
    VecI8X16BinaryOp(VecI8X16BinaryOp),
    VecI8X16TestOp(VecI8X16TestOp),
    VecI8X16ShiftOp(VecI8X16ShiftOp),
    VecI8X16MinMaxOp(VecI8X16MinMaxOp),
    VecI8X16SatBinaryOp(VecI8X16SatBinaryOp),
}

pub(super) enum VecI8X16RelOp {
    Eq(VecI8X16, VecI8X16),
    Ne(VecI8X16, VecI8X16),
    LtU(VecI8X16, VecI8X16),
    LtS(VecI8X16, VecI8X16),
    GtU(VecI8X16, VecI8X16),
    GtS(VecI8X16, VecI8X16),
    LeU(VecI8X16, VecI8X16),
    LeS(VecI8X16, VecI8X16),
    GeU(VecI8X16, VecI8X16),
    GeS(VecI8X16, VecI8X16),
}

pub(super) enum VecI8X16UnaryOp {
    Abs(VecI8X16),
    Neg(VecI8X16),
    PopCnt(VecI8X16),
}

pub(super) enum VecI8X16BinaryOp {
    Add(VecI8X16, VecI8X16),
    Sub(VecI8X16, VecI8X16),
    AvgrU(VecI8X16, VecI8X16),
}

pub(super) enum VecI8X16TestOp {
    AllTrue(VecI8X16),
}

pub(super) enum VecI8X16ShiftOp {
    Shl(VecI8X16, I32),
    ShrU(VecI8X16, I32),
    ShrS(VecI8X16, I32),
}

pub(super) enum VecI8X16MinMaxOp {
    MinU(VecI8X16),
    MinS(VecI8X16),
    MaxU(VecI8X16),
    MaxS(VecI8X16),
}

pub(super) enum VecI8X16SatBinaryOp {
    AddSatU(VecI8X16),
    AddSatS(VecI8X16),
    SubSatU(VecI8X16),
    SubSatS(VecI8X16),
}

// VecI16X8 Instructions
pub(super) enum VecI16X8Inst {
    Splat(I128),
    ExtractLaneU(VecI16X8, LaneIdx),
    ExtractLaneS(VecI16X8, LaneIdx),
    ReplaceLane(VecI16X8, LaneIdx),
    Bitmask(VecI16X8),
    NarrowI32X4U(VecI16X8),
    NarrowI32X4S(VecI16X8),

    VecI16X8RelOp(VecI16X8RelOp),
    VecI16X8UnaryOp(VecI16X8UnaryOp),
    VecI16X8BinaryOp(VecI16X8BinaryOp),
    VecI16X8TestOp(VecI16X8TestOp),
    VecI16X8ShiftOp(VecI16X8ShiftOp),
    VecI16X8MinMaxOp(VecI16X8MinMaxOp),
    VecI16X8SatBinaryOp(VecI16X8SatBinaryOp),
    VecI16X8ConvertOp(VecI16X8ConvertOp),
}

pub(super) enum VecI16X8RelOp {
    Eq(VecI16X8, VecI16X8),
    Ne(VecI16X8, VecI16X8),
    LtU(VecI16X8, VecI16X8),
    LtS(VecI16X8, VecI16X8),
    GtU(VecI16X8, VecI16X8),
    GtS(VecI16X8, VecI16X8),
    LeU(VecI16X8, VecI16X8),
    LeS(VecI16X8, VecI16X8),
    GeU(VecI16X8, VecI16X8),
    GeS(VecI16X8, VecI16X8),
}

pub(super) enum VecI16X8UnaryOp {
    Abs(VecI16X8),
    Neg(VecI16X8)
}

pub(super) enum VecI16X8BinaryOp {
    Add(VecI16X8, VecI16X8),
    Sub(VecI16X8, VecI16X8),
    Mul(VecI16X8, VecI16X8),
    AvgrU(VecI16X8, VecI16X8),
    Q15MulrSatS(VecI16X8, VecI16X8),
}

pub(super) enum VecI16X8TestOp {
    AllTrue(VecI16X8),
}

pub(super) enum VecI16X8ShiftOp {
    Shl(VecI16X8, I32),
    ShrU(VecI16X8, I32),
    ShrS(VecI16X8, I32),
}

pub(super) enum VecI16X8MinMaxOp {
    MinU(VecI16X8),
    MinS(VecI16X8),
    MaxU(VecI16X8),
    MaxS(VecI16X8),
}

pub(super) enum VecI16X8SatBinaryOp {
    AddSatU(VecI16X8),
    AddSatS(VecI16X8),
    SubSatU(VecI16X8),
    SubSatS(VecI16X8),
}

pub(super) enum VecI16X8ConvertOp {
    ExtendHalfI8X16U(VecI8X16),
    ExtendHalfI8X16S(VecI8X16),
    ExtMulHalfI8X16U(VecI8X16),
    ExtMulHalfI8X16S(VecI8X16),
    ExtAddPairwiseI8X16U(VecI8X16),
    ExtAddPairwiseI8X16S(VecI8X16),
}

// VecI32X4 Instructions
pub(super) enum VecI32X4Inst {
    Splat(I128),
    ExtractLane(VecI32X4, LaneIdx),
    ReplaceLane(VecI32X4, LaneIdx),
    Bitmask(VecI32X4),
    DotI16X8S(VecI32X4),

    VecI32X4RelOp(VecI32X4RelOp),
    VecI32X4UnaryOp(VecI32X4UnaryOp),
    VecI32X4BinaryOp(VecI32X4BinaryOp),
    VecI32X4TestOp(VecI32X4TestOp),
    VecI32X4ShiftOp(VecI32X4ShiftOp),
    VecI32X4MinMaxOp(VecI32X4MinMaxOp),
    VecI32X4ConvertOp(VecI32X4ConvertOp),
}

pub(super) enum VecI32X4RelOp {
    Eq(VecI32X4, VecI32X4),
    Ne(VecI32X4, VecI32X4),
    LtU(VecI32X4, VecI32X4),
    LtS(VecI32X4, VecI32X4),
    GtU(VecI32X4, VecI32X4),
    GtS(VecI32X4, VecI32X4),
    LeU(VecI32X4, VecI32X4),
    LeS(VecI32X4, VecI32X4),
    GeU(VecI32X4, VecI32X4),
    GeS(VecI32X4, VecI32X4),
}

pub(super) enum VecI32X4UnaryOp {
    Abs(VecI32X4),
    Neg(VecI32X4)
}

pub(super) enum VecI32X4BinaryOp {
    Add(VecI32X4, VecI32X4),
    Sub(VecI32X4, VecI32X4),
    Mul(VecI32X4, VecI32X4),
}

pub(super) enum VecI32X4TestOp {
    AllTrue(VecI32X4),
}

pub(super) enum VecI32X4ShiftOp {
    Shl(VecI32X4, I32),
    ShrU(VecI32X4, I32),
    ShrS(VecI32X4, I32),
}

pub(super) enum VecI32X4MinMaxOp {
    MinU(VecI32X4),
    MinS(VecI32X4),
    MaxU(VecI32X4),
    MaxS(VecI32X4),
}

pub(super) enum VecI32X4ConvertOp {
    ExtendHalfI16X8U(VecI16X8),
    ExtendHalfI16X8S(VecI16X8),
    ExtMulHalfI16X8U(VecI16X8),
    ExtMulHalfI16X8S(VecI16X8),
    ExtAddPairwiseI16X8U(VecI16X8),
    ExtAddPairwiseI16X8S(VecI16X8),
    TruncSatF32X4U(VecF32X4),
    TruncSatF32X4S(VecF32X4),
    TruncSatF64X2UZero(VecF64X2),
    TruncsatF64X2SZero(VecF64X2),
}

// VecI64X2 Instructions
pub(super) enum VecI64X2Inst {
    Splat(I128),
    ExtractLane(VecI64X2, LaneIdx),
    ReplaceLane(VecI64X2, LaneIdx),
    Bitmask(VecI64X2),

    VecI64X2RelOp(VecI64X2RelOp),
    VecI64X2UnaryOp(VecI64X2UnaryOp),
    VecI64X2BinaryOp(VecI64X2BinaryOp),
    VecI64X2TestOp(VecI64X2TestOp),
    VecI64X2ShiftOp(VecI64X2ShiftOp),
    VecI64X2ConvertOp(VecI64X2ConvertOp),
}

pub(super) enum VecI64X2RelOp {
    Eq(VecI64X2, VecI64X2),
    Ne(VecI64X2, VecI64X2),
    LtS(VecI64X2, VecI64X2),
    GtS(VecI64X2, VecI64X2),
    LeS(VecI64X2, VecI64X2),
    GeS(VecI64X2, VecI64X2),
}

pub(super) enum VecI64X2UnaryOp {
    Abs(VecI64X2),
    Neg(VecI64X2)
}

pub(super) enum VecI64X2BinaryOp {
    Add(VecI64X2, VecI64X2),
    Sub(VecI64X2, VecI64X2),
    Mul(VecI64X2, VecI64X2),
}

pub(super) enum VecI64X2TestOp {
    AllTrue(VecI64X2),
}

pub(super) enum VecI64X2ShiftOp {
    Shl(VecI64X2, I32),
    ShrU(VecI64X2, I32),
    ShrS(VecI64X2, I32),
}

pub(super) enum VecI64X2ConvertOp {
    ExtendHalfI32X4U(VecI32X4),
    ExtendHalfI32X4S(VecI32X4),
    ExtMulHalfI32X4U(VecI32X4),
    ExtMulHalfI32X4S(VecI32X4),
}

// VecF32X4 Instructions
pub(super) enum VecF32X4Inst {
    Splat(I128),
    ExtractLane(VecF32X4, LaneIdx),
    ReplaceLane(VecF32X4, LaneIdx),

    VecF32X4RelOp(VecF32X4RelOp),
    VecF32X4UnaryOp(VecF32X4UnaryOp),
    VecF32X4BinaryOp(VecF32X4BinaryOp),
    VecF32X4ConvertOp(VecF32X4ConvertOp),
}

pub(super) enum VecF32X4RelOp {
    Eq(VecF32X4, VecF32X4),
    Ne(VecF32X4, VecF32X4),
    Lt(VecF32X4, VecF32X4),
    Gt(VecF32X4, VecF32X4),
    Le(VecF32X4, VecF32X4),
    Ge(VecF32X4, VecF32X4),
}

pub(super) enum VecF32X4UnaryOp {
    Abs(VecF32X4),
    Neg(VecF32X4),
    Sqrt(VecF32X4),
    Ceil(VecF32X4),
    Floor(VecF32X4),
    Trunc(VecF32X4),
    Nearest(VecF32X4)
}

pub(super) enum VecF32X4BinaryOp {
    Add(VecF32X4, VecF32X4),
    Sub(VecF32X4, VecF32X4),
    Mul(VecF32X4, VecF32X4),
    Div(VecF32X4, VecF32X4),
    Min(VecF32X4, VecF32X4),
    Max(VecF32X4, VecF32X4),
    PMin(VecF32X4, VecF32X4),
    PMax(VecF32X4, VecF32X4),
}

pub(super) enum VecF32X4ConvertOp {
    ConvertI32X4U(VecI32X4),
    ConvertI32X4S(VecI32X4),
    DemoteF64X2Zero(VecF64X2),
}

// VecF64X2 Instructions
pub(super) enum VecF64X2Inst {
    Splat(I128),
    ExtractLane(VecF64X2, LaneIdx),
    ReplaceLane(VecF64X2, LaneIdx),

    VecF64X2RelOp(VecF64X2RelOp),
    VecF64X2UnaryOp(VecF64X2UnaryOp),
    VecF64X2BinaryOp(VecF64X2BinaryOp),
    VecF64X2ConvertOp(VecF64X2ConvertOp),
}

pub(super) enum VecF64X2RelOp {
    Eq(VecF64X2, VecF64X2),
    Ne(VecF64X2, VecF64X2),
    Lt(VecF64X2, VecF64X2),
    Gt(VecF64X2, VecF64X2),
    Le(VecF64X2, VecF64X2),
    Ge(VecF64X2, VecF64X2),
}

pub(super) enum VecF64X2UnaryOp {
    Abs(VecF64X2),
    Neg(VecF64X2),
    Sqrt(VecF64X2),
    Ceil(VecF64X2),
    Floor(VecF64X2),
    Trunc(VecF64X2),
    Nearest(VecF64X2)
}

pub(super) enum VecF64X2BinaryOp {
    Add(VecF64X2, VecF64X2),
    Sub(VecF64X2, VecF64X2),
    Mul(VecF64X2, VecF64X2),
    Div(VecF64X2, VecF64X2),
    Min(VecF64X2, VecF64X2),
    Max(VecF64X2, VecF64X2),
    PMin(VecF64X2, VecF64X2),
    PMax(VecF64X2, VecF64X2),
}

pub(super) enum VecF64X2ConvertOp {
    ConvertLowI32X4U(VecI32X4),
    ConvertLowI32X4S(VecI32X4),
    PromoteLowF32X4(VecF32X4),
}

//
// Reference Instructions
//
pub(super) enum RefInst {
    FuncRefInst(FuncRefInst),
    ExternRefInst(ExternRefInst)
}

pub(super) enum FuncRefInst {
    Null(),
    IsNull(FuncRef),
    FuncRef(FuncRef)
}

pub(super) enum ExternRefInst {
    Null(),
    IsNull(ExternRef),
    ExternRef(ExternRef)
}

//
// Parametric Instructions
//
pub(super) enum ParametricInst {
    Drop(),
    Select(Vec<ValType>)
}

//
// Variable Instructions
//
pub(super) enum VariableInst {
    LocalGet(LocalIdx),
    LocalSet(LocalIdx),
    LocalTee(LocalIdx),
    GlobalGet(GlobalIdx),
    GlobalSet(GlobalIdx),
}

//
// Table Instructions
//
pub(super) enum TableInst {
    Get(TableIdx),
    Set(TableIdx),
    Size(TableIdx),
    Grow(TableIdx),
    Fill(TableIdx),
    Copy(TableIdx, TableIdx),
    Init(TableIdx, ElemIdx),
    ElemDrop(ElemIdx)
}

//
// Memory Instructions
//
pub(super) enum MemoryInst {
    I32Load(Offset, Align),
    I32Store(Offset, Align),
    I32Load8U(Offset, Align),
    I32Load8S(Offset, Align),
    I32Store8(Offset, Align),
    I32Load16U(Offset, Align),
    I32Load16S(Offset, Align),
    I32Store16(Offset, Align),

    I64Load(Offset, Align),
    I64Store(Offset, Align),
    I64Load8U(Offset, Align),
    I64Load8S(Offset, Align),
    I64Store8(Offset, Align),
    I64Load16U(Offset, Align),
    I64Load16S(Offset, Align),
    I64Store16(Offset, Align),
    I64Load32U(Offset, Align),
    I64Load32S(Offset, Align),
    I64Store32(Offset, Align),

    F32Load(Offset, Align),
    F32Store(Offset, Align),

    F64Load(Offset, Align),
    F64Store(Offset, Align),

    V128Load(Offset, Align),
    V128Load8X8U(Offset, Align),
    V128Load8X8S(Offset, Align),
    V128Load16X4U(Offset, Align),
    V128Load16X4S(Offset, Align),
    V128Load32X2U(Offset, Align),
    V128Load32X2S(Offset, Align),
    V128Load32Zero(Offset, Align),
    V128Load64Zero(Offset, Align),
    V128Load8Splat(Offset, Align),
    V128Load16Splat(Offset, Align),
    V128Load32Splat(Offset, Align),
    V128Load64Splat(Offset, Align),
    V128Load8Lane(Offset, Align, LaneIdx),
    V128Load16Lane(Offset, Align, LaneIdx),
    V128Load32Lane(Offset, Align, LaneIdx),
    V128Load64Lane(Offset, Align, LaneIdx),
    V128Store(Offset, Align),
    V128Store8Lane(Offset, Align, LaneIdx),
    V128Store16Lane(Offset, Align, LaneIdx),
    V128Store32Lane(Offset, Align, LaneIdx),
    V128Store64Lane(Offset, Align, LaneIdx),

    Size(),
    Grow(),
    Fill(),
    Copy(),
    Init(DataIdx),
    DataDrop(DataIdx)
}

//
// Control Instructions
//
pub(super) enum ControlInst {
    Nop(),
    Unreachable(),
    Block(BlockType, Vec<Instr>),
    Loop(BlockType, Vec<Instr>),
    If(BlockType, Vec<Instr>, Vec<Instr>),
    Br(LabelIdx),
    BrIf(LabelIdx),
    BrTable(Vec<LabelIdx>, LabelIdx),
    Return(),
    Call(FuncIdx),
    CallIndirect(TableIdx, TypeIdx)
}

//
// Expr Instructions
//
pub(super) enum ExprInst {
    Expr(Vec<Instr>)
}
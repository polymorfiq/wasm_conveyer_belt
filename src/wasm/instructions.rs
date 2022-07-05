use super::types::*;

pub(super) trait Instr {
    fn exec(&self, inputs: dyn ResultType) -> dyn ResultType;
}

//
// Numeric Instructions
//

// I32 Instructions
pub(super) struct I32Const {a: u32}

// I32 Unary Operations
pub(super) struct I32Clz {a: I32}
pub(super) struct I32Ctz {a: I32}
pub(super) struct I32Popcnt {a: I32}

// I32 Binary Operations
pub(super) struct I32Add {a: I32, b: I32}
pub(super) struct I32Sub {a: I32, b: I32}
pub(super) struct I32Mul {a: I32, b: I32}
pub(super) struct I32DivU {a: I32, b: I32}
pub(super) struct I32DivS {a: I32, b: I32}
pub(super) struct I32RemU {a: I32, b: I32}
pub(super) struct I32RemS {a: I32, b: I32}

// I32 Test Operations
pub(super) struct I32Eqz {a: I32}

// I32 Comparison Operations
pub(super) struct I32Eq {a: I32, b: I32}
pub(super) struct I32Ne {a: I32, b: I32}
pub(super) struct I32LtU {a: I32, b: I32}
pub(super) struct I32LtS {a: I32, b: I32}
pub(super) struct I32GtU {a: I32, b: I32}
pub(super) struct I32GtS {a: I32, b: I32}
pub(super) struct I32LeU {a: I32, b: I32}
pub(super) struct I32LeS {a: I32, b: I32}
pub(super) struct I32GeU {a: I32, b: I32}
pub(super) struct I32GeS {a: I32, b: I32}

// I32 Convert Operations
pub(super) struct I32Extend8S {a: I32}
pub(super) struct I32Extend16S {a: I32}
pub(super) struct I32WrapI64 {a: I64}
pub(super) struct I32TruncF32U {a: F32}
pub(super) struct I32TruncF32S {a: F32}
pub(super) struct I32TruncF64U {a: F64}
pub(super) struct I32TruncF64S {a: F64}
pub(super) struct I32TruncSatF32U {a: F32}
pub(super) struct I32TruncSatF32S {a: F32}
pub(super) struct I32TruncSatF64U {a: F64}
pub(super) struct I32TruncSatF64S {a: F64}
pub(super) struct I32ReinterpretF32 {a: F32}

// I64 Instructions
pub(super) struct I64Const {a: u64}

// I64 Unary Operations
pub(super) struct I64Clz {a: I64}
pub(super) struct I64Ctz {a: I64}
pub(super) struct I64Popcnt {a: I64}

// I64 Binary Operations
pub(super) struct I64Add {a: I64, b: I64}
pub(super) struct I64Sub {a: I64, b: I64}
pub(super) struct I64Mul {a: I64, b: I64}
pub(super) struct I64DivU {a: I64, b: I64}
pub(super) struct I64DivS {a: I64, b: I64}
pub(super) struct I64RemU {a: I64, b: I64}
pub(super) struct I64RemS {a: I64, b: I64}

// I64 Test Operations
pub(super) struct I64Eqz {a: I64}

// I64 Comparison Operations
pub(super) struct I64Eq {a: I64, b: I64}
pub(super) struct I64Ne {a: I64, b: I64}
pub(super) struct I64LtU {a: I64, b: I64}
pub(super) struct I64LtS {a: I64, b: I64}
pub(super) struct I64GtU {a: I64, b: I64}
pub(super) struct I64GtS {a: I64, b: I64}
pub(super) struct I64LeU {a: I64, b: I64}
pub(super) struct I64LeS {a: I64, b: I64}
pub(super) struct I64GeU {a: I64, b: I64}
pub(super) struct I64GeS {a: I64, b: I64}

// I64 Convert Operations
pub(super) struct I64Extend8S {a: I8}
pub(super) struct I64Extend16S {a: I16}
pub(super) struct I64Extend32S {a: I32}
pub(super) struct I64ExtendI32U {a: I32}
pub(super) struct I64ExtendI32S {a: I32}
pub(super) struct I64TruncF32U {a: F32}
pub(super) struct I64TruncF32S {a: F32}
pub(super) struct I64TruncF64U {a: F64}
pub(super) struct I64TruncF64S {a: F64}
pub(super) struct I64TruncSatF32U {a: F32}
pub(super) struct I64TruncSatF32S {a: F32}
pub(super) struct I64TruncSatF64U {a: F64}
pub(super) struct I64TruncSatF64S {a: F64}
pub(super) struct I64ReinterpretF64 {a: F64}

// F32 Instructions
pub(super) struct F32Const {a: f32}

// F32 Unary Operations
pub(super) struct F32Abs {a: F32}
pub(super) struct F32Neg {a: F32}
pub(super) struct F32Sqrt {a: F32}
pub(super) struct F32Ceil {a: F32}
pub(super) struct F32Floor {a: F32}
pub(super) struct F32Trunc {a: F32}
pub(super) struct F32Nearest {a: F32}

// F32 Binary Operations
pub(super) struct F32Add {a: F32, b: F32}
pub(super) struct F32Sub {a: F32, b: F32}
pub(super) struct F32Mul {a: F32, b: F32}
pub(super) struct F32Div {a: F32, b: F32}
pub(super) struct F32Min {a: F32, b: F32}
pub(super) struct F32Max {a: F32, b: F32}
pub(super) struct F32CopySign {a: F32, b: F32}

// F32 Comparison Operations
pub(super) struct F32Eq {a: F32, b: F32}
pub(super) struct F32Ne {a: F32, b: F32}
pub(super) struct F32Lt {a: F32, b: F32}
pub(super) struct F32Gt {a: F32, b: F32}
pub(super) struct F32Le {a: F32, b: F32}
pub(super) struct F32Ge {a: F32, b: F32}

// F32 Convert Operations
pub(super) struct F32DemoteF64 {a: F64}
pub(super) struct F32ConvertI32U {a: I32}
pub(super) struct F32ConvertI32S {a: I32}
pub(super) struct F32ConvertI64U {a: I64}
pub(super) struct F32ConvertI64S {a: I64}
pub(super) struct F32ReinterpretI32 {a: I32}
pub(super) struct F32ReinterpretI64 {a: I64}

// F64 Instructions
pub(super) struct F64Const {a: f64}

// F64 Unary Operations
pub(super) struct F64Abs {a: F64}
pub(super) struct F64Neg {a: F64}
pub(super) struct F64Sqrt {a: F64}
pub(super) struct F64Ceil {a: F64}
pub(super) struct F64Floor {a: F64}
pub(super) struct F64Trunc {a: F64}
pub(super) struct F64Nearest {a: F64}

// F64 Binary Operations
pub(super) struct F64Add {a: F64, b: F64}
pub(super) struct F64Sub {a: F64, b: F64}
pub(super) struct F64Mul {a: F64, b: F64}
pub(super) struct F64Div {a: F64, b: F64}
pub(super) struct F64Min {a: F64, b: F64}
pub(super) struct F64Max {a: F64, b: F64}
pub(super) struct F64CopySign {a: F64, b: F64}

// F64 Comparison Operations
pub(super) struct F64Eq {a: F64, b: F64}
pub(super) struct F64Ne {a: F64, b: F64}
pub(super) struct F64Lt {a: F64, b: F64}
pub(super) struct F64Gt {a: F64, b: F64}
pub(super) struct F64Le {a: F64, b: F64}
pub(super) struct F64Ge {a: F64, b: F64}

// F64 Convert Operations
pub(super) struct F64PromoteF32 {a: F32}
pub(super) struct F64ConvertI32U {a: I32}
pub(super) struct F64ConvertI32S {a: I32}
pub(super) struct F64ConvertI64U {a: I64}
pub(super) struct F64ConvertI64S {a: I64}
pub(super) struct F64ReinterpretI32 {a: I32}
pub(super) struct F64ReinterpretI64 {a: I64}

//
// Vector instructions
//

// Vec128 Instructions
pub(super) struct Vec128Const {a: u128}

// Vec128 Unary Operations
pub(super) struct Vec128Not {a: Vec128}

// Vec128 Binary Operations
pub(super) struct Vec128And {a: Vec128, b: Vec128}
pub(super) struct Vec128AndNot {a: Vec128, b: Vec128}
pub(super) struct Vec128Or {a: Vec128, b: Vec128}
pub(super) struct Vec128Xor {a: Vec128, b: Vec128}

// Vec128 Comparison Operations
pub(super) struct Vec128Bitselect {a: Vec128, b: Vec128, c: Vec128}

// Vec128 Test Operations
pub(super) struct Vec128AnyTrue {a: Vec128}

// VecI8x16 Instructions
pub(super) struct VecI8x16Shuffle {a: VecI8x16}
pub(super) struct VecI8x16Swizzle {a: VecI8x16}
pub(super) struct VecI8x16Splat {a: VecI8x16}
pub(super) struct VecI8x16ExtractLaneU {a: VecI8x16, b: LaneIdx}
pub(super) struct VecI8x16ExtractLaneS {a: VecI8x16, b: LaneIdx}
pub(super) struct VecI8x16ReplaceLane {a: VecI8x16, b: LaneIdx}
pub(super) struct VecI8x16Bitmask {a: VecI8x16}
pub(super) struct VecI8x16NarrowI16x8U {a: VecI16x8}
pub(super) struct VecI8x16NarrowI16x8S {a: VecI16x8}

// VecI8x16 Unary Operations
pub(super) struct VecI8x16Abs {a: VecI8x16}
pub(super) struct VecI8x16Neg {a: VecI8x16}
pub(super) struct VecI8x16PopCnt {a: VecI8x16}

// VecI8x16 Binary Operations
pub(super) struct VecI8x16Add {a: VecI8x16, b: VecI8x16}
pub(super) struct VecI8x16Sub {a: VecI8x16, b: VecI8x16}
pub(super) struct VecI8x16AvgrU {a: VecI8x16, b: VecI8x16}

// VecI8x16 Compare Operations
pub(super) struct VecI8x16Eq {a: VecI8x16, b: VecI8x16}
pub(super) struct VecI8x16Ne {a: VecI8x16, b: VecI8x16}
pub(super) struct VecI8x16LtU {a: VecI8x16, b: VecI8x16}
pub(super) struct VecI8x16LtS {a: VecI8x16, b: VecI8x16}
pub(super) struct VecI8x16GtU {a: VecI8x16, b: VecI8x16}
pub(super) struct VecI8x16GtS {a: VecI8x16, b: VecI8x16}
pub(super) struct VecI8x16LeU {a: VecI8x16, b: VecI8x16}
pub(super) struct VecI8x16LeS {a: VecI8x16, b: VecI8x16}
pub(super) struct VecI8x16GeU {a: VecI8x16, b: VecI8x16}
pub(super) struct VecI8x16GeS {a: VecI8x16, b: VecI8x16}

// VecI8x16 Test Operations
pub(super) struct VecI8x16AllTrue {a: VecI8x16}

// VecI8x16 Shift Operations
pub(super) struct VecI8x16Shl {a: VecI8x16, b: I32}
pub(super) struct VecI8x16ShrU {a: VecI8x16, b: I32}
pub(super) struct VecI8x16ShrS {a: VecI8x16, b: I32}

// VecI8x16 MinMax Operations
pub(super) struct VecI8x16MinU {a: VecI8x16, b: VecI8x16}
pub(super) struct VecI8x16MinS {a: VecI8x16, b: VecI8x16}
pub(super) struct VecI8x16MaxU {a: VecI8x16, b: VecI8x16}
pub(super) struct VecI8x16MaxS {a: VecI8x16, b: VecI8x16}

// VecI8x16 Sat Binary Operations
pub(super) struct VecI8x16AddSatU {a: VecI8x16}
pub(super) struct VecI8x16AddSatS {a: VecI8x16}
pub(super) struct VecI8x16SubSatU {a: VecI8x16}
pub(super) struct VecI8x16SubSatS {a: VecI8x16}

// VecI16x8 Instructions
pub(super) struct VecI16x8Splat {a: u128}
pub(super) struct VecI16x8ExtractLaneU {a: VecI16x8, b: LaneIdx}
pub(super) struct VecI16x8ExtractLaneS {a: VecI16x8, b: LaneIdx}
pub(super) struct VecI16x8ReplaceLane {a: VecI16x8, b: LaneIdx}
pub(super) struct VecI16x8Bitmask {a: VecI16x8}
pub(super) struct VecI16x8NarrowI32x4U {a: VecI32x4}
pub(super) struct VecI16x8NarrowI32x4S {a: VecI32x4}

// VecI16x8 Unary Operations
pub(super) struct VecI16x8Abs {a: VecI16x8}
pub(super) struct VecI16x8Neg {a: VecI16x8}

// VecI16x8 Binary Operations
pub(super) struct VecI16x8Add {a: VecI16x8, b: VecI16x8}
pub(super) struct VecI16x8Sub {a: VecI16x8, b: VecI16x8}
pub(super) struct VecI16x8Mul {a: VecI16x8, b: VecI16x8}
pub(super) struct VecI16x8AvgrU {a: VecI16x8, b: VecI16x8}
pub(super) struct VecI16x8Q15MulrSatS {a: VecI16x8, b: VecI16x8}

// VecI16x8 Test Operations
pub(super) struct VecI16x8AllTrue {a: VecI16x8}

// VecI16x8 Compare Operations
pub(super) struct VecI16x8Eq {a: VecI16x8, b: VecI16x8}
pub(super) struct VecI16x8Ne {a: VecI16x8, b: VecI16x8}
pub(super) struct VecI16x8LtU {a: VecI16x8, b: VecI16x8}
pub(super) struct VecI16x8LtS {a: VecI16x8, b: VecI16x8}
pub(super) struct VecI16x8GtU {a: VecI16x8, b: VecI16x8}
pub(super) struct VecI16x8GtS {a: VecI16x8, b: VecI16x8}
pub(super) struct VecI16x8LeU {a: VecI16x8, b: VecI16x8}
pub(super) struct VecI16x8LeS {a: VecI16x8, b: VecI16x8}
pub(super) struct VecI16x8GeU {a: VecI16x8, b: VecI16x8}
pub(super) struct VecI16x8GeS {a: VecI16x8, b: VecI16x8}

// VecI16x8 Shift Operations
pub(super) struct VecI16x8Shl {a: VecI16x8, b: I32}
pub(super) struct VecI16x8ShrU {a: VecI16x8, b: I32}
pub(super) struct VecI16x8ShrS {a: VecI16x8, b: I32}

// VecI16x8 MinMax Operations
pub(super) struct VecI16x8MinU {a: VecI16x8, b: VecI16x8}
pub(super) struct VecI16x8MinS {a: VecI16x8, b: VecI16x8}
pub(super) struct VecI16x8MaxU {a: VecI16x8, b: VecI16x8}
pub(super) struct VecI16x8MaxS {a: VecI16x8, b: VecI16x8}

// VecI16x8 Sat Binary Operations
pub(super) struct VecI16x8AddSatU {a: VecI16x8}
pub(super) struct VecI16x8AddSatS {a: VecI16x8}
pub(super) struct VecI16x8SubSatU {a: VecI16x8}
pub(super) struct VecI16x8SubSatS {a: VecI16x8}

// Vec16x8 Convert Operations
pub(super) struct VecI16x8ExtendHalfI8x16U {a: VecI8x16}
pub(super) struct VecI16x8ExtendHalfI8x16S {a: VecI8x16}
pub(super) struct VecI16x8ExtMulHalfI8x16U {a: VecI8x16}
pub(super) struct VecI16x8ExtMulHalfI8x16S {a: VecI8x16}
pub(super) struct VecI16x8ExtAddPairwiseI8x16U {a: VecI8x16}
pub(super) struct VecI16x8ExtAddPairwiseI8x16S {a: VecI8x16}

// VecI32x4 Instructions
pub(super) struct VecI32x4Splat {a: u128}
pub(super) struct VecI32x4ExtractLane {a: VecI32x4, b: LaneIdx}
pub(super) struct VecI32x4ReplaceLane {a: VecI32x4, b: LaneIdx}
pub(super) struct VecI32x4Bitmask {a: VecI32x4}
pub(super) struct VecI32x4DotI16x8S {a: VecI16x8}

// VecI32x4 Unary Operations
pub(super) struct VecI32x4Abs {a: VecI32x4}
pub(super) struct VecI32x4Neg {a: VecI32x4}

// VecI32x4 Binary Operations
pub(super) struct VecI32x4Add {a: VecI32x4, b: VecI32x4}
pub(super) struct VecI32x4Sub {a: VecI32x4, b: VecI32x4}
pub(super) struct VecI32x4Mul {a: VecI32x4, b: VecI32x4}

// VecI32x4 Test Operations
pub(super) struct VecI32x4AllTrue {a: VecI32x4}

// VecI32x4 Compare Operations
pub(super) struct VecI32x4Eq {a: VecI32x4, b: VecI32x4}
pub(super) struct VecI32x4Ne {a: VecI32x4, b: VecI32x4}
pub(super) struct VecI32x4LtU {a: VecI32x4, b: VecI32x4}
pub(super) struct VecI32x4LtS {a: VecI32x4, b: VecI32x4}
pub(super) struct VecI32x4GtU {a: VecI32x4, b: VecI32x4}
pub(super) struct VecI32x4GtS {a: VecI32x4, b: VecI32x4}
pub(super) struct VecI32x4LeU {a: VecI32x4, b: VecI32x4}
pub(super) struct VecI32x4LeS {a: VecI32x4, b: VecI32x4}
pub(super) struct VecI32x4GeU {a: VecI32x4, b: VecI32x4}
pub(super) struct VecI32x4GeS {a: VecI32x4, b: VecI32x4}

// VecI32x4 Shift Operations
pub(super) struct VecI32x4Shl {a: VecI32x4, b: I32}
pub(super) struct VecI32x4ShrU {a: VecI32x4, b: I32}
pub(super) struct VecI32x4ShrS {a: VecI32x4, b: I32}

// VecI32x4 MinMax Operations
pub(super) struct VecI32x4MinU {a: VecI32x4, b: VecI32x4}
pub(super) struct VecI32x4MinS {a: VecI32x4, b: VecI32x4}
pub(super) struct VecI32x4MaxU {a: VecI32x4, b: VecI32x4}
pub(super) struct VecI32x4MaxS {a: VecI32x4, b: VecI32x4}

// VecI32x4 Convert Operations
pub(super) struct VecI32x4ExtendHalfI16x8U {a: VecI16x8}
pub(super) struct VecI32x4ExtendHalfI16x8S {a: VecI16x8}
pub(super) struct VecI32x4ExtMulHalfI16x8U {a: VecI16x8}
pub(super) struct VecI32x4ExtMulHalfI16x8S {a: VecI16x8}
pub(super) struct VecI32x4ExtAddPairwiseI16x8U {a: VecI16x8}
pub(super) struct VecI32x4ExtAddPairwiseI16x8S {a: VecI16x8}
pub(super) struct VecI32x4TruncSatF32x4U {a: VecF32x4}
pub(super) struct VecI32x4TruncSatF32x4S {a: VecF32x4}
pub(super) struct VecI32x4TruncSatF64x2UZero {a: VecF64x2}
pub(super) struct VecI32x4TruncSatF64x2SZero {a: VecF64x2}

// VecI64x2 Instructions
pub(super) struct VecI64x2Splat {a: u128}
pub(super) struct VecI64x2ExtractLane {a: VecI64x2, b: LaneIdx}
pub(super) struct VecI64x2ReplaceLane {a: VecI64x2, b: LaneIdx}
pub(super) struct VecI64x2Bitmask {a: VecI64x2}

// VecI64x2 Unary Operations
pub(super) struct VecI64x2Abs {a: VecI64x2}
pub(super) struct VecI64x2Neg {a: VecI64x2}

// VecI64x2 Binary Operations
pub(super) struct VecI64x2Add {a: VecI64x2, b: VecI64x2}
pub(super) struct VecI64x2Sub {a: VecI64x2, b: VecI64x2}
pub(super) struct VecI64x2Mul {a: VecI64x2, b: VecI64x2}

// VecI64x2 Test Operations
pub(super) struct VecI64x2AllTrue {a: VecI64x2}

// VecI64x2 Compare Operations
pub(super) struct VecI64x2Eq {a: VecI64x2, b: VecI64x2}
pub(super) struct VecI64x2Ne {a: VecI64x2, b: VecI64x2}
pub(super) struct VecI64x2LtS {a: VecI64x2, b: VecI64x2}
pub(super) struct VecI64x2GtS {a: VecI64x2, b: VecI64x2}
pub(super) struct VecI64x2LeS {a: VecI64x2, b: VecI64x2}
pub(super) struct VecI64x2GeS {a: VecI64x2, b: VecI64x2}

// VecI64x2 Shift Operations
pub(super) struct VecI64x2Shl {a: VecI64x2, b: I32}
pub(super) struct VecI64x2ShrU {a: VecI64x2, b: I32}
pub(super) struct VecI64x2ShrS {a: VecI64x2, b: I32}

// VecI64x2 Convert Operations
pub(super) struct VecI64x2ExtendHalfI32x4U {a: VecI32x4}
pub(super) struct VecI64x2ExtendHalfI32x4S {a: VecI32x4}
pub(super) struct VecI64x2ExtMulHalfI32x4U {a: VecI32x4}
pub(super) struct VecI64x2ExtMulHalfI32x4S {a: VecI32x4}

// VecF32x4 Instructions
pub(super) struct VecF32x4Splat {a: u128}
pub(super) struct VecF32x4ExtractLane {a: VecF32x4, b: LaneIdx}
pub(super) struct VecF32x4ReplaceLane {a: VecF32x4, b: LaneIdx}

// VecF32x4 Unary Operations
pub(super) struct VecF32x4Abs {a: VecF32x4}
pub(super) struct VecF32x4Neg {a: VecF32x4}
pub(super) struct VecF32x4Sqrt {a: VecF32x4}
pub(super) struct VecF32x4Ceil {a: VecF32x4}
pub(super) struct VecF32x4Floor {a: VecF32x4}
pub(super) struct VecF32x4Trunc {a: VecF32x4}
pub(super) struct VecF32x4Nearest {a: VecF32x4}

// VecF32x4 Binary Operations
pub(super) struct VecF32x4Add {a: VecF32x4, b: VecF32x4}
pub(super) struct VecF32x4Sub {a: VecF32x4, b: VecF32x4}
pub(super) struct VecF32x4Mul {a: VecF32x4, b: VecF32x4}
pub(super) struct VecF32x4Div {a: VecF32x4, b: VecF32x4}
pub(super) struct VecF32x4Min {a: VecF32x4, b: VecF32x4}
pub(super) struct VecF32x4Max {a: VecF32x4, b: VecF32x4}
pub(super) struct VecF32x4PMin {a: VecF32x4, b: VecF32x4}
pub(super) struct VecF32x4PMax {a: VecF32x4, b: VecF32x4}

// VecF32x4 Compare Operations
pub(super) struct VecF32x4Eq {a: VecF32x4, b: VecF32x4}
pub(super) struct VecF32x4Ne {a: VecF32x4, b: VecF32x4}
pub(super) struct VecF32x4Lt {a: VecF32x4, b: VecF32x4}
pub(super) struct VecF32x4Gt {a: VecF32x4, b: VecF32x4}
pub(super) struct VecF32x4Le {a: VecF32x4, b: VecF32x4}
pub(super) struct VecF32x4Ge {a: VecF32x4, b: VecF32x4}

// VecF32x4 Convert Operations
pub(super) struct VecF32x4ConvertI32x4U {a: VecI32x4}
pub(super) struct VecF32x4ConvertI32x4S {a: VecI32x4}
pub(super) struct VecF32x4DemoteF64x2Zero {a: VecF64x2}

// VecF64x2 Instructions
pub(super) struct VecF64x2Splat {a: u128}
pub(super) struct VecF64x2ExtractLane {a: VecF64x2, b: LaneIdx}
pub(super) struct VecF64x2ReplaceLane {a: VecF64x2, b: LaneIdx}

// VecF64x2 Unary Operations
pub(super) struct VecF64x2Abs {a: VecF64x2}
pub(super) struct VecF64x2Neg {a: VecF64x2}
pub(super) struct VecF64x2Sqrt {a: VecF64x2}
pub(super) struct VecF64x2Ceil {a: VecF64x2}
pub(super) struct VecF64x2Floor {a: VecF64x2}
pub(super) struct VecF64x2Trunc {a: VecF64x2}
pub(super) struct VecF64x2Nearest {a: VecF64x2}

// VecF64x2 Binary Operations
pub(super) struct VecF64x2Add {a: VecF64x2, b: VecF64x2}
pub(super) struct VecF64x2Sub {a: VecF64x2, b: VecF64x2}
pub(super) struct VecF64x2Mul {a: VecF64x2, b: VecF64x2}
pub(super) struct VecF64x2Div {a: VecF64x2, b: VecF64x2}
pub(super) struct VecF64x2Min {a: VecF64x2, b: VecF64x2}
pub(super) struct VecF64x2Max {a: VecF64x2, b: VecF64x2}
pub(super) struct VecF64x2PMin {a: VecF64x2, b: VecF64x2}
pub(super) struct VecF64x2PMax {a: VecF64x2, b: VecF64x2}

// VecF64x2 Compare Operations
pub(super) struct VecF64x2Eq {a: VecF64x2, b: VecF64x2}
pub(super) struct VecF64x2Ne {a: VecF64x2, b: VecF64x2}
pub(super) struct VecF64x2Lt {a: VecF64x2, b: VecF64x2}
pub(super) struct VecF64x2Gt {a: VecF64x2, b: VecF64x2}
pub(super) struct VecF64x2Le {a: VecF64x2, b: VecF64x2}
pub(super) struct VecF64x2Ge {a: VecF64x2, b: VecF64x2}

// VecF64x2 Convert Operations
pub(super) struct VecF64x2ConvertLowI32x4U {a: VecI32x4}
pub(super) struct VecF64x2ConvertLowI32x4S {a: VecI32x4}
pub(super) struct VecF64x2PromotLowF32x4 {a: VecF32x4}

//
// Reference Instructions
//
pub(super) struct FuncRefNull {}
pub(super) struct FuncRefIsNull {a: bool}
pub(super) struct FuncRefRef {a: FuncRef}

pub(super) struct ExternRefNull {}
pub(super) struct ExternRefIsNull {a: bool}
pub(super) struct ExternRefRef {a: ExternRef}

//
// Parametric Instructions
//
pub(super) struct ParametricDrop {}
pub(super) struct ParametricSelect<'a>{a: [&'a dyn ValType]}

//
// Variable Instructions
//
pub(super) struct VariableLocalGet {a: LocalIdx}
pub(super) struct VariableLocalSet {a: LocalIdx}
pub(super) struct VariableLocalTee {a: LocalIdx}
pub(super) struct VariableGlobalGet {a: GlobalIdx}
pub(super) struct VariableGlobalSet {a: GlobalIdx}

//
// Table Instructions
//
pub(super) struct TableGet {a: TableIdx}
pub(super) struct TableSet {a: TableIdx}
pub(super) struct TableSize {a: TableIdx}
pub(super) struct TableGrow {a: TableIdx}
pub(super) struct TableFill {a: TableIdx}
pub(super) struct TableCopy {a: TableIdx, b: TableIdx}
pub(super) struct TableInit {a: TableIdx, b: ElemIdx}
pub(super) struct TableElemDrop {a: ElemIdx}

//
// Memory Instructions
//
pub(super) struct MemSize {}
pub(super) struct MemGrow {}
pub(super) struct MemFill {}
pub(super) struct MemCopy {}
pub(super) struct MemInit {a: DataIdx}
pub(super) struct MemDataDrop {a: DataIdx}

pub(super) struct MemI32Load {a: Offset, b: Align}
pub(super) struct MemI64Load {a: Offset, b: Align}
pub(super) struct MemF32Load {a: Offset, b: Align}
pub(super) struct MemF64Load {a: Offset, b: Align}
pub(super) struct MemI32Load8U {a: Offset, b: Align}
pub(super) struct MemI32Load8S {a: Offset, b: Align}
pub(super) struct MemI32Load16U {a: Offset, b: Align}
pub(super) struct MemI32Load16S {a: Offset, b: Align}
pub(super) struct MemI64Load8U {a: Offset, b: Align}
pub(super) struct MemI64Load8S {a: Offset, b: Align}
pub(super) struct MemI64Load16U {a: Offset, b: Align}
pub(super) struct MemI64Load16S {a: Offset, b: Align}
pub(super) struct MemI64Load32U {a: Offset, b: Align}
pub(super) struct MemI64Load32S {a: Offset, b: Align}
pub(super) struct MemI32Store {a: Offset, b: Align}
pub(super) struct MemI64Store {a: Offset, b: Align}
pub(super) struct MemF32Store {a: Offset, b: Align}
pub(super) struct MemF64Store {a: Offset, b: Align}
pub(super) struct MemI32Store8 {a: Offset, b: Align}
pub(super) struct MemI32Store16 {a: Offset, b: Align}
pub(super) struct MemI64Store8 {a: Offset, b: Align}
pub(super) struct MemI64Store16 {a: Offset, b: Align}
pub(super) struct MemI64Store32 {a: Offset, b: Align}

pub(super) struct MemV128Load {a: Offset, b: Align}
pub(super) struct MemV128Load8x8U {a: Offset, b: Align}
pub(super) struct MemV128Load8x8S {a: Offset, b: Align}
pub(super) struct MemV128Load16x4U {a: Offset, b: Align}
pub(super) struct MemV128Load16x4S {a: Offset, b: Align}
pub(super) struct MemV128Load32x2U {a: Offset, b: Align}
pub(super) struct MemV128Load32x2S {a: Offset, b: Align}
pub(super) struct MemV128Load32Zero {a: Offset, b: Align}
pub(super) struct MemV128Load64Zero {a: Offset, b: Align}
pub(super) struct MemV128Load8Splat {a: Offset, b: Align}
pub(super) struct MemV128Load16Splat {a: Offset, b: Align}
pub(super) struct MemV128Load32Splat {a: Offset, b: Align}
pub(super) struct MemV128Load64Splat {a: Offset, b: Align}
pub(super) struct MemV128Load8Lane {a: Offset, b: Align}
pub(super) struct MemV128Load16Lane {a: Offset, b: Align}
pub(super) struct MemV128Load32Lane {a: Offset, b: Align}
pub(super) struct MemV128Load64Lane {a: Offset, b: Align}
pub(super) struct MemV128Store {a: Offset, b: Align}
pub(super) struct MemV128Store8Lane {a: Offset, b: Align}
pub(super) struct MemV128Store16Lane {a: Offset, b: Align}
pub(super) struct MemV128Store32Lane {a: Offset, b: Align}
pub(super) struct MemV128Store64Lane {a: Offset, b: Align}

//
// Control Instructions
//
pub(super) struct ControlNop {}
pub(super) struct ControlUnreachable {}
pub(super) struct ControlBlock<'a> {a: &'a dyn BlockType, b: &'a [&'a dyn Instr]}
pub(super) struct ControlLoop<'a> {a: &'a dyn BlockType, b: &'a [&'a dyn Instr]}
pub(super) struct ControlIf<'a> {a: &'a dyn BlockType, b: &'a [&'a dyn Instr], c: &'a [&'a dyn Instr]}
pub(super) struct ControlBr {a: LabelIdx}
pub(super) struct ControlBrIf {a: LabelIdx}
pub(super) struct ControlBrTable {a: Vec<LabelIdx>, b: LabelIdx}
pub(super) struct ControlReturn {}
pub(super) struct ControlCall {a: FuncIdx}
pub(super) struct ControlCallIndirect {a: TableIdx, b: TypeIdx}

//
// Expr Instructions
//
pub(super) struct Expr<'a>{a: Vec<&'a dyn Instr>}
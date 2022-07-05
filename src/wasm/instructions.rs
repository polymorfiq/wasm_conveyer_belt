use super::types::*;

pub(super) trait Instr {
    fn exec(&self) -> ResultType;
}

//
// Numeric Instructions
//

// I32 Instructions
struct I32Const {a: u32}

// I32 Unary Operations
struct I32Clz {a: I32}
struct I32Ctz {a: I32}
struct I32Popcnt {a: I32}

// I32 Binary Operations
struct I32Add {a: I32, b: I32}
struct I32Sub {a: I32, b: I32}
struct I32Mul {a: I32, b: I32}
struct I32DivU {a: I32, b: I32}
struct I32DivS {a: I32, b: I32}
struct I32RemU {a: I32, b: I32}
struct I32RemS {a: I32, b: I32}

// I32 Test Operations
struct I32Eqz {a: I32}

// I32 Comparison Operations
struct I32Eq {a: I32, b: I32}
struct I32Ne {a: I32, b: I32}
struct I32LtU {a: I32, b: I32}
struct I32LtS {a: I32, b: I32}
struct I32GtU {a: I32, b: I32}
struct I32GtS {a: I32, b: I32}
struct I32LeU {a: I32, b: I32}
struct I32LeS {a: I32, b: I32}
struct I32GeU {a: I32, b: I32}
struct I32GeS {a: I32, b: I32}

// I32 Convert Operations
struct I32Extend8S {a: I32}
struct I32Extend16S {a: I32}
struct I32WrapI64 {a: I64}
struct I32TruncF32U {a: F32}
struct I32TruncF32S {a: F32}
struct I32TruncF64U {a: F64}
struct I32TruncF64S {a: F64}
struct I32TruncSatF32U {a: F32}
struct I32TruncSatF32S {a: F32}
struct I32TruncSatF64U {a: F64}
struct I32TruncSatF64S {a: F64}
struct I32ReinterpretF32 {a: F32}

// I64 Instructions
struct I64Const {a: u64}

// I64 Unary Operations
struct I64Clz {a: I64}
struct I64Ctz {a: I64}
struct I64Popcnt {a: I64}

// I64 Binary Operations
struct I64Add {a: I64, b: I64}
struct I64Sub {a: I64, b: I64}
struct I64Mul {a: I64, b: I64}
struct I64DivU {a: I64, b: I64}
struct I64DivS {a: I64, b: I64}
struct I64RemU {a: I64, b: I64}
struct I64RemS {a: I64, b: I64}

// I64 Test Operations
struct I64Eqz {a: I64}

// I64 Comparison Operations
struct I64Eq {a: I64, b: I64}
struct I64Ne {a: I64, b: I64}
struct I64LtU {a: I64, b: I64}
struct I64LtS {a: I64, b: I64}
struct I64GtU {a: I64, b: I64}
struct I64GtS {a: I64, b: I64}
struct I64LeU {a: I64, b: I64}
struct I64LeS {a: I64, b: I64}
struct I64GeU {a: I64, b: I64}
struct I64GeS {a: I64, b: I64}

// I64 Convert Operations
struct I64Extend8S {a: I8}
struct I64Extend16S {a: I16}
struct I64Extend32S {a: I32}
struct I64ExtendI32U {a: I32}
struct I64ExtendI32S {a: I32}
struct I64TruncF32U {a: F32}
struct I64TruncF32S {a: F32}
struct I64TruncF64U {a: F64}
struct I64TruncF64S {a: F64}
struct I64TruncSatF32U {a: F32}
struct I64TruncSatF32S {a: F32}
struct I64TruncSatF64U {a: F64}
struct I64TruncSatF64S {a: F64}
struct I64ReinterpretF64 {a: F64}

// F32 Instructions
struct F32Const {a: f32}

// F32 Unary Operations
struct F32Abs {a: F32}
struct F32Neg {a: F32}
struct F32Sqrt {a: F32}
struct F32Ceil {a: F32}
struct F32Floor {a: F32}
struct F32Trunc {a: F32}
struct F32Nearest {a: F32}

// F32 Binary Operations
struct F32Add {a: F32, b: F32}
struct F32Sub {a: F32, b: F32}
struct F32Mul {a: F32, b: F32}
struct F32Div {a: F32, b: F32}
struct F32Min {a: F32, b: F32}
struct F32Max {a: F32, b: F32}
struct F32CopySign {a: F32, b: F32}

// F32 Comparison Operations
struct F32Eq {a: F32, b: F32}
struct F32Ne {a: F32, b: F32}
struct F32Lt {a: F32, b: F32}
struct F32Gt {a: F32, b: F32}
struct F32Le {a: F32, b: F32}
struct F32Ge {a: F32, b: F32}

// F32 Convert Operations
struct F32DemoteF64 {a: F64}
struct F32ConvertI32U {a: I32}
struct F32ConvertI32S {a: I32}
struct F32ConvertI64U {a: I64}
struct F32ConvertI64S {a: I64}
struct F32ReinterpretI32 {a: I32}
struct F32ReinterpretI64 {a: I64}

// F64 Instructions
struct F64Const {a: f64}

// F64 Unary Operations
struct F64Abs {a: F64}
struct F64Neg {a: F64}
struct F64Sqrt {a: F64}
struct F64Ceil {a: F64}
struct F64Floor {a: F64}
struct F64Trunc {a: F64}
struct F64Nearest {a: F64}

// F64 Binary Operations
struct F64Add {a: F64, b: F64}
struct F64Sub {a: F64, b: F64}
struct F64Mul {a: F64, b: F64}
struct F64Div {a: F64, b: F64}
struct F64Min {a: F64, b: F64}
struct F64Max {a: F64, b: F64}
struct F64CopySign {a: F64, b: F64}

// F64 Comparison Operations
struct F64Eq {a: F64, b: F64}
struct F64Ne {a: F64, b: F64}
struct F64Lt {a: F64, b: F64}
struct F64Gt {a: F64, b: F64}
struct F64Le {a: F64, b: F64}
struct F64Ge {a: F64, b: F64}

// F64 Convert Operations
struct F64PromoteF32 {a: F32}
struct F64ConvertI32U {a: I32}
struct F64ConvertI32S {a: I32}
struct F64ConvertI64U {a: I64}
struct F64ConvertI64S {a: I64}
struct F64ReinterpretI32 {a: I32}
struct F64ReinterpretI64 {a: I64}

//
// Vector instructions
//

// Vec128 Instructions
struct Vec128Const {a: u128}

// Vec128 Unary Operations
struct Vec128Not {a: Vec128}

// Vec128 Binary Operations
struct Vec128And {a: Vec128, b: Vec128}
struct Vec128AndNot {a: Vec128, b: Vec128}
struct Vec128Or {a: Vec128, b: Vec128}
struct Vec128Xor {a: Vec128, b: Vec128}

// Vec128 Comparison Operations
struct Vec128Bitselect {a: Vec128, b: Vec128, c: Vec128}

// Vec128 Test Operations
struct Vec128AnyTrue {a: Vec128}

// VecI8X16 Instructions
struct VecI8x16Shuffle {a: VecI8X16}
struct VecI8x16Swizzle {a: VecI8X16}
struct VecI8x16Splat {a: VecI8X16}
struct VecI8x16ExtractLaneU {a: VecI8X16, b: LaneIdx}
struct VecI8x16ExtractLaneS {a: VecI8X16, b: LaneIdx}
struct VecI8x16ReplaceLane {a: VecI8X16, b: LaneIdx}
struct VecI8x16Bitmask {a: VecI8X16}
struct VecI8x16NarrowI16x8U {a: VecI16X8}
struct VecI8x16NarrowI16x8S {a: VecI16X8}

// VecI8x16 Unary Operations
struct VecI8x16Abs {a: VecI8X16}
struct VecI8x16Neg {a: VecI8X16}
struct VecI8x16PopCnt {a: VecI8X16}

// VecI8x16 Binary Operations
struct VecI8x16Add {a: VecI8X16, b: VecI8X16}
struct VecI8x16Sub {a: VecI8X16, b: VecI8X16}
struct VecI8x16AvgrU {a: VecI8X16, b: VecI8X16}

// VecI8x16 Compare Operations
struct VecI8x16Eq {a: VecI8X16, b: VecI8X16}
struct VecI8x16Ne {a: VecI8X16, b: VecI8X16}
struct VecI8x16LtU {a: VecI8X16, b: VecI8X16}
struct VecI8x16LtS {a: VecI8X16, b: VecI8X16}
struct VecI8x16GtU {a: VecI8X16, b: VecI8X16}
struct VecI8x16GtS {a: VecI8X16, b: VecI8X16}
struct VecI8x16LeU {a: VecI8X16, b: VecI8X16}
struct VecI8x16LeS {a: VecI8X16, b: VecI8X16}
struct VecI8x16GeU {a: VecI8X16, b: VecI8X16}
struct VecI8x16GeS {a: VecI8X16, b: VecI8X16}

// VecI8x16 Test Operations
struct VecI8x16AllTrue {a: VecI8X16}

// VecI8x16 Shift Operations
struct VecI8x16Shl {a: VecI8X16, b: I32}
struct VecI8x16ShrU {a: VecI8X16, b: I32}
struct VecI8x16ShrS {a: VecI8X16, b: I32}

// VecI8x16 MinMax Operations
struct VecI8x16MinU {a: VecI8X16, b: VecI8X16}
struct VecI8x16MinS {a: VecI8X16, b: VecI8X16}
struct VecI8x16MaxU {a: VecI8X16, b: VecI8X16}
struct VecI8x16MaxS {a: VecI8X16, b: VecI8X16}

// VecI8x16 Sat Binary Operations
struct VecI8x16AddSatU {a: VecI8X16}
struct VecI8x16AddSatS {a: VecI8X16}
struct VecI8x16SubSatU {a: VecI8X16}
struct VecI8x16SubSatS {a: VecI8X16}

// VecI16X8 Instructions
struct VecI16x8Splat {a: I128}
struct VecI16x8ExtractLaneU {a: VecI16X8, b: LaneIdx}
struct VecI16x8ExtractLaneS {a: VecI16X8, b: LaneIdx}
struct VecI16x8ReplaceLane {a: VecI16X8, b: LaneIdx}
struct VecI16x8Bitmask {a: VecI16X8}
struct VecI16x8NarrowI32x4U {a: VecI32X4}
struct VecI16x8NarrowI32x4S {a: VecI32X4}

// VecI16x8 Unary Operations
struct VecI16x8Abs {a: VecI16X8}
struct VecI16x8Neg {a: VecI16X8}

// VecI16x8 Binary Operations
struct VecI16x8Add {a: VecI16X8, b: VecI16X8}
struct VecI16x8Sub {a: VecI16X8, b: VecI16X8}
struct VecI16x8Mul {a: VecI16X8, b: VecI16X8}
struct VecI16x8AvgrU {a: VecI16X8, b: VecI16X8}
struct VecI16x8Q15MulrSatS {a: VecI16X8, b: VecI16X8}

// VecI16x8 Test Operations
struct VecI16x8AllTrue {a: VecI16X8}

// VecI16x8 Compare Operations
struct VecI16x8Eq {a: VecI16X8, b: VecI16X8}
struct VecI16x8Ne {a: VecI16X8, b: VecI16X8}
struct VecI16x8LtU {a: VecI16X8, b: VecI16X8}
struct VecI16x8LtS {a: VecI16X8, b: VecI16X8}
struct VecI16x8GtU {a: VecI16X8, b: VecI16X8}
struct VecI16x8GtS {a: VecI16X8, b: VecI16X8}
struct VecI16x8LeU {a: VecI16X8, b: VecI16X8}
struct VecI16x8LeS {a: VecI16X8, b: VecI16X8}
struct VecI16x8GeU {a: VecI16X8, b: VecI16X8}
struct VecI16x8GeS {a: VecI16X8, b: VecI16X8}

// VecI16x8 Shift Operations
struct VecI16x8Shl {a: VecI16X8, b: I32}
struct VecI16x8ShrU {a: VecI16X8, b: I32}
struct VecI16x8ShrS {a: VecI16X8, b: I32}

// VecI16x8 MinMax Operations
struct VecI16x8MinU {a: VecI16X8, b: VecI16X8}
struct VecI16x8MinS {a: VecI16X8, b: VecI16X8}
struct VecI16x8MaxU {a: VecI16X8, b: VecI16X8}
struct VecI16x8MaxS {a: VecI16X8, b: VecI16X8}

// VecI16x8 Sat Binary Operations
struct VecI16x8AddSatU {a: VecI16X8}
struct VecI16x8AddSatS {a: VecI16X8}
struct VecI16x8SubSatU {a: VecI16X8}
struct VecI16x8SubSatS {a: VecI16X8}

// Vec16x8 Convert Operations
struct VecI16x8ExtendHalfI8x16U {a: VecI8X16}
struct VecI16x8ExtendHalfI8x16S {a: VecI8X16}
struct VecI16x8ExtMulHalfI8x16U {a: VecI8X16}
struct VecI16x8ExtMulHalfI8x16S {a: VecI8X16}
struct VecI16x8ExtAddPairwiseI8x16U {a: VecI8X16}
struct VecI16x8ExtAddPairwiseI8x16S {a: VecI8X16}

// VecI32X4 Instructions
struct VecI32x4Splat {a: I128}
struct VecI32x4ExtractLane {a: VecI32X4, b: LaneIdx}
struct VecI32x4ReplaceLane {a: VecI32X4, b: LaneIdx}
struct VecI32x4Bitmask {a: VecI32X4}
struct VecI32x4DotI16x8S {a: VecI16X8}

// VecI32x4 Unary Operations
struct VecI32x4Abs {a: VecI32X4}
struct VecI32x4Neg {a: VecI32X4}

// VecI32x4 Binary Operations
struct VecI32x4Add {a: VecI32X4, b: VecI32X4}
struct VecI32x4Sub {a: VecI32X4, b: VecI32X4}
struct VecI32x4Mul {a: VecI32X4, b: VecI32X4}

// VecI32x4 Test Operations
struct VecI32x4AllTrue {a: VecI32X4}

// VecI32x4 Compare Operations
struct VecI32x4Eq {a: VecI32X4, b: VecI32X4}
struct VecI32x4Ne {a: VecI32X4, b: VecI32X4}
struct VecI32x4LtU {a: VecI32X4, b: VecI32X4}
struct VecI32x4LtS {a: VecI32X4, b: VecI32X4}
struct VecI32x4GtU {a: VecI32X4, b: VecI32X4}
struct VecI32x4GtS {a: VecI32X4, b: VecI32X4}
struct VecI32x4LeU {a: VecI32X4, b: VecI32X4}
struct VecI32x4LeS {a: VecI32X4, b: VecI32X4}
struct VecI32x4GeU {a: VecI32X4, b: VecI32X4}
struct VecI32x4GeS {a: VecI32X4, b: VecI32X4}

// VecI32x4 Shift Operations
struct VecI32x4Shl {a: VecI32X4, b: I32}
struct VecI32x4ShrU {a: VecI32X4, b: I32}
struct VecI32x4ShrS {a: VecI32X4, b: I32}

// VecI32x4 MinMax Operations
struct VecI32x4MinU {a: VecI32X4, b: VecI32X4}
struct VecI32x4MinS {a: VecI32X4, b: VecI32X4}
struct VecI32x4MaxU {a: VecI32X4, b: VecI32X4}
struct VecI32x4MaxS {a: VecI32X4, b: VecI32X4}

// VecI32x4 Convert Operations
struct VecI32x4ExtendHalfI16x8U {a: VecI16X8}
struct VecI32x4ExtendHalfI16x8S {a: VecI16X8}
struct VecI32x4ExtMulHalfI16x8U {a: VecI16X8}
struct VecI32x4ExtMulHalfI16x8S {a: VecI16X8}
struct VecI32x4ExtAddPairwiseI16x8U {a: VecI16X8}
struct VecI32x4ExtAddPairwiseI16x8S {a: VecI16X8}
struct VecI32x4TruncSatF32x4U {a: VecF32X4}
struct VecI32x4TruncSatF32x4S {a: VecF32X4}
struct VecI32x4TruncSatF64x2UZero {a: VecF64X2}
struct VecI32x4TruncSatF64x2SZero {a: VecF64X2}

// VecI64X2 Instructions
struct VecI64x2Splat {a: I128}
struct VecI64x2ExtractLane {a: VecI64X2, b: LaneIdx}
struct VecI64x2ReplaceLane {a: VecI64X2, b: LaneIdx}
struct VecI64x2Bitmask {a: VecI64X2}

// VecI64x2 Unary Operations
struct VecI64x2Abs {a: VecI64X2}
struct VecI64x2Neg {a: VecI64X2}

// VecI64x2 Binary Operations
struct VecI64x2Add {a: VecI64X2, b: VecI64X2}
struct VecI64x2Sub {a: VecI64X2, b: VecI64X2}
struct VecI64x2Mul {a: VecI64X2, b: VecI64X2}

// VecI64x2 Test Operations
struct VecI64x2AllTrue {a: VecI64X2}

// VecI64x2 Compare Operations
struct VecI64x2Eq {a: VecI64X2, b: VecI64X2}
struct VecI64x2Ne {a: VecI64X2, b: VecI64X2}
struct VecI64x2LtS {a: VecI64X2, b: VecI64X2}
struct VecI64x2GtS {a: VecI64X2, b: VecI64X2}
struct VecI64x2LeS {a: VecI64X2, b: VecI64X2}
struct VecI64x2GeS {a: VecI64X2, b: VecI64X2}

// VecI64x2 Shift Operations
struct VecI64x2Shl {a: VecI64X2, b: I32}
struct VecI64x2ShrU {a: VecI64X2, b: I32}
struct VecI64x2ShrS {a: VecI64X2, b: I32}

// VecI64x2 Convert Operations
struct VecI64x2ExtendHalfI32x4U {a: VecI32X4}
struct VecI64x2ExtendHalfI32x4S {a: VecI32X4}
struct VecI64x2ExtMulHalfI32x4U {a: VecI32X4}
struct VecI64x2ExtMulHalfI32x4S {a: VecI32X4}

// VecF32X4 Instructions
struct VecF32x4Splat {a: I128}
struct VecF32x4ExtractLane {a: VecF32X4, b: LaneIdx}
struct VecF32x4ReplaceLane {a: VecF32X4, b: LaneIdx}

// VecF32x4 Unary Operations
struct VecF32x4Abs {a: VecF32X4}
struct VecF32x4Neg {a: VecF32X4}
struct VecF32x4Sqrt {a: VecF32X4}
struct VecF32x4Ceil {a: VecF32X4}
struct VecF32x4Floor {a: VecF32X4}
struct VecF32x4Trunc {a: VecF32X4}
struct VecF32x4Nearest {a: VecF32X4}

// VecF32x4 Binary Operations
struct VecF32x4Add {a: VecF32X4, b: VecF32X4}
struct VecF32x4Sub {a: VecF32X4, b: VecF32X4}
struct VecF32x4Mul {a: VecF32X4, b: VecF32X4}
struct VecF32x4Div {a: VecF32X4, b: VecF32X4}
struct VecF32x4Min {a: VecF32X4, b: VecF32X4}
struct VecF32x4Max {a: VecF32X4, b: VecF32X4}
struct VecF32x4PMin {a: VecF32X4, b: VecF32X4}
struct VecF32x4PMax {a: VecF32X4, b: VecF32X4}

// VecF32x4 Compare Operations
struct VecF32x4Eq {a: VecF32X4, b: VecF32X4}
struct VecF32x4Ne {a: VecF32X4, b: VecF32X4}
struct VecF32x4Lt {a: VecF32X4, b: VecF32X4}
struct VecF32x4Gt {a: VecF32X4, b: VecF32X4}
struct VecF32x4Le {a: VecF32X4, b: VecF32X4}
struct VecF32x4Ge {a: VecF32X4, b: VecF32X4}

// VecF32x4 Convert Operations
struct VecF32x4ConvertI32x4U {a: VecI32X4}
struct VecF32x4ConvertI32x4S {a: VecI32X4}
struct VecF32x4DemoteF64x2Zero {a: VecF64X2}

// VecF64x2 Instructions
struct VecF64x2Splat {a: I128}
struct VecF64x2ExtractLane {a: VecF64X2, b: LaneIdx}
struct VecF64x2ReplaceLane {a: VecF64X2, b: LaneIdx}

// VecF64x2 Unary Operations
struct VecF64x2Abs {a: VecF64X2}
struct VecF64x2Neg {a: VecF64X2}
struct VecF64x2Sqrt {a: VecF64X2}
struct VecF64x2Ceil {a: VecF64X2}
struct VecF64x2Floor {a: VecF64X2}
struct VecF64x2Trunc {a: VecF64X2}
struct VecF64x2Nearest {a: VecF64X2}

// VecF64x2 Binary Operations
struct VecF64x2Add {a: VecF64X2, b: VecF64X2}
struct VecF64x2Sub {a: VecF64X2, b: VecF64X2}
struct VecF64x2Mul {a: VecF64X2, b: VecF64X2}
struct VecF64x2Div {a: VecF64X2, b: VecF64X2}
struct VecF64x2Min {a: VecF64X2, b: VecF64X2}
struct VecF64x2Max {a: VecF64X2, b: VecF64X2}
struct VecF64x2PMin {a: VecF64X2, b: VecF64X2}
struct VecF64x2PMax {a: VecF64X2, b: VecF64X2}

// VecF64x2 Compare Operations
struct VecF64x2Eq {a: VecF64X2, b: VecF64X2}
struct VecF64x2Ne {a: VecF64X2, b: VecF64X2}
struct VecF64x2Lt {a: VecF64X2, b: VecF64X2}
struct VecF64x2Gt {a: VecF64X2, b: VecF64X2}
struct VecF64x2Le {a: VecF64X2, b: VecF64X2}
struct VecF64x2Ge {a: VecF64X2, b: VecF64X2}

// VecF64x2 Convert Operations
struct VecF64x2ConvertLowI32x4U {a: VecI32X4}
struct VecF64x2ConvertLowI32x4S {a: VecI32X4}
struct VecF64x2PromotLowF32x4 {a: VecF32X4}

//
// Reference Instructions
//
struct FuncRefNull {}
struct FuncRefIsNull {a: bool}
struct FuncRefRef {a: FuncRef}

struct ExternRefNull {}
struct ExternRefIsNull {a: bool}
struct ExternRefRef {a: ExternRef}

//
// Parametric Instructions
//
struct ParametricDrop {}
struct ParametricSelect{a: Vec<ValType>}

//
// Variable Instructions
//
struct VariableLocalGet {a: LocalIdx}
struct VariableLocalSet {a: LocalIdx}
struct VariableLocalTee {a: LocalIdx}
struct VariableGlobalGet {a: GlobalIdx}
struct VariableGlobalSet {a: GlobalIdx}

//
// Table Instructions
//
struct TableGet {a: TableIdx}
struct TableSet {a: TableIdx}
struct TableSize {a: TableIdx}
struct TableGrow {a: TableIdx}
struct TableFill {a: TableIdx}
struct TableCopy {a: TableIdx, b: TableIdx}
struct TableInit {a: TableIdx, b: ElemIdx}
struct TableElemDrop {a: ElemIdx}

//
// Memory Instructions
//
struct MemSize {}
struct MemGrow {}
struct MemFill {}
struct MemCopy {}
struct MemInit {a: DataIdx}
struct MemDataDrop {a: DataIdx}

struct MemI32Load {a: Offset, b: Align}
struct MemI64Load {a: Offset, b: Align}
struct MemF32Load {a: Offset, b: Align}
struct MemF64Load {a: Offset, b: Align}
struct MemI32Load8U {a: Offset, b: Align}
struct MemI32Load8S {a: Offset, b: Align}
struct MemI32Load16U {a: Offset, b: Align}
struct MemI32Load16S {a: Offset, b: Align}
struct MemI64Load8U {a: Offset, b: Align}
struct MemI64Load8S {a: Offset, b: Align}
struct MemI64Load16U {a: Offset, b: Align}
struct MemI64Load16S {a: Offset, b: Align}
struct MemI64Load32U {a: Offset, b: Align}
struct MemI64Load32S {a: Offset, b: Align}
struct MemI32Store {a: Offset, b: Align}
struct MemI64Store {a: Offset, b: Align}
struct MemF32Store {a: Offset, b: Align}
struct MemF64Store {a: Offset, b: Align}
struct MemI32Store8 {a: Offset, b: Align}
struct MemI32Store16 {a: Offset, b: Align}
struct MemI64Store8 {a: Offset, b: Align}
struct MemI64Store16 {a: Offset, b: Align}
struct MemI64Store32 {a: Offset, b: Align}

struct MemV128Load {a: Offset, b: Align}
struct MemV128Load8x8U {a: Offset, b: Align}
struct MemV128Load8x8S {a: Offset, b: Align}
struct MemV128Load16x4U {a: Offset, b: Align}
struct MemV128Load16x4S {a: Offset, b: Align}
struct MemV128Load32x2U {a: Offset, b: Align}
struct MemV128Load32x2S {a: Offset, b: Align}
struct MemV128Load32Zero {a: Offset, b: Align}
struct MemV128Load64Zero {a: Offset, b: Align}
struct MemV128Load8Splat {a: Offset, b: Align}
struct MemV128Load16Splat {a: Offset, b: Align}
struct MemV128Load32Splat {a: Offset, b: Align}
struct MemV128Load64Splat {a: Offset, b: Align}
struct MemV128Load8Lane {a: Offset, b: Align}
struct MemV128Load16Lane {a: Offset, b: Align}
struct MemV128Load32Lane {a: Offset, b: Align}
struct MemV128Load64Lane {a: Offset, b: Align}
struct MemV128Store {a: Offset, b: Align}
struct MemV128Store8Lane {a: Offset, b: Align}
struct MemV128Store16Lane {a: Offset, b: Align}
struct MemV128Store32Lane {a: Offset, b: Align}
struct MemV128Store64Lane {a: Offset, b: Align}

//
// Control Instructions
//
struct ControlNop {}
struct ControlUnreachable {}
struct ControlBlock<'a> {a: BlockType, b: Vec<&'a dyn Instr>}
struct ControlLoop<'a> {a: BlockType, b: Vec<&'a dyn Instr>}
struct ControlIf<'a> {a: BlockType, b: Vec<&'a dyn Instr>, c: Vec<&'a dyn Instr>}
struct ControlBr {a: LabelIdx}
struct ControlBrIf {a: LabelIdx}
struct ControlBrTable {a: Vec<LabelIdx>, b: LabelIdx}
struct ControlReturn {}
struct ControlCall {a: FuncIdx}
struct ControlCallIndirect {a: TableIdx, b: TypeIdx}

//
// Expr Instructions
//
struct Expr<'a>{a: Vec<&'a dyn Instr>}
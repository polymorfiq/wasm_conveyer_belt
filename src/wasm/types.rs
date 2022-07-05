use super::expressions::Expr;

pub(super) enum ValType {
    NumType(NumType),
    VecType(VecType),
    RefType(RefType),
}

pub(super) enum ResultType {
    VecType(VecType),
}

// Num Types
pub(super) enum NumType {
    I8(I8),
    I16(I16),
    I32(I32),
    I64(I64),
    I128(I128),
    F32(F32),
    F64(F64),
}

pub(super) struct U8 {}
pub(super) struct I8 {}
pub(super) struct I16 {}
pub(super) struct I32 {}
pub(super) struct U32 {}
pub(super) struct I64 {}
pub(super) struct I128 {}
pub(super) struct F32 {}
pub(super) struct F64 {}

pub(super) type Byte = U8;
pub(super) type LaneIdx = U8;
pub(super) type TypeIdx = U32;
pub(super) type FuncIdx = U32;
pub(super) type TableIdx = U32;
pub(super) type MemIdx = U32;
pub(super) type GlobalIdx = U32;
pub(super) type ElemIdx = U32;
pub(super) type DataIdx = U32;
pub(super) type LocalIdx = U32;
pub(super) type LabelIdx = U32;
pub(super) type Offset = U32;
pub(super) type Align = U32;

pub(super) enum BlockType {
    TypeIdx(TypeIdx),
    ValType(ValType)
}

// Reference Types
pub(super) enum RefType {
    FuncRef(FuncRef),
    ExternRef(ExternRef),
}

pub(super) struct FuncRef {}
pub(super) struct ExternRef {}

// Vector Types
pub(super) enum VecType {
    Vec128(Vec128),
    VecI8X16(VecI8X16),
    VecI16X8(VecI16X8),
    VecI32x4(VecI32X4),
    VecI64x2(VecI64X2),
    VecF32X4(VecF32X4),
    VecF64X2(VecF64X2),
}

pub(super) struct Vec128 {}
pub(super) struct VecI8X16 {}
pub(super) struct VecI16X8 {}
pub(super) struct VecI32X4 {}
pub(super) struct VecI64X2 {}
pub(super) struct VecF32X4 {}
pub(super) struct VecF64X2 {}

// Limits
pub(super) struct Limits {
    min: u32,
    max: Option<u32>,
}

// Memory Types
pub(super) struct MemType {
    limits: Limits,
}

pub(super) struct Mem {
    mem_type: MemType,
}

// Function Types
pub(super) struct Start {
    pub func: FuncIdx
}

pub(super) enum FuncType {
    Func(Vec<ValType>, Vec<ValType>)
}

pub(super) struct Func<'a> {
    pub func_type: TypeIdx,
    pub locals: Vec<ValType>,
    pub body: Expr<'a>,
}

// Table Types
pub(super) struct TableType {limits: Limits, reftype: RefType}
pub(super) struct Table {table_type: TableType}

pub(super) struct Elem<'a> {elem_type: RefType, init: Vec<Expr<'a>>, mode: ElemMode}
pub(super) enum ElemMode {
    Passive,
    Active(TableIdx, Offset),
    Declarative
}

// Data Types
pub(super) struct Data {
    init: Vec<Byte>,
    mode: DataMode
}

pub(super) enum DataMode {
    Passive,
    Active(MemIdx, Offset),
}

// Global Types
pub(super) enum GlobalType {
    VarGlobal(ValType),
    ConstGlobal(ValType),
}

// External Types
pub(super) enum ExternType {
    Func(FuncType),
    Table(TableType),
    MemType(MemType),
    GlobalType(GlobalType),
}

// Import Types
pub(super) struct Import<'a> {
    module: &'a str,
    name: &'a str,
    desc: ImportDesc
}

pub(super) enum ImportDesc {
    Func(TableIdx),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

// Export Types
pub(super) struct Export<'a> {
    name: &'a str,
    desc: ExportDesc
}

pub(super) enum ExportDesc {
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx)
}

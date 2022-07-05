use super::expressions::Expr;

pub(super) trait ValType {}
impl ValType for dyn NumType {}
impl ValType for dyn VecType {}
impl ValType for dyn RefType {}

pub(super) trait ResultType {}
impl ResultType for [& dyn ValType] {}

// Num Types
pub(super) trait NumType {}
impl NumType for U8 {}
impl NumType for I8 {}
impl NumType for I16 {}
impl NumType for I32 {}
impl NumType for U32 {}
impl NumType for I64 {}
impl NumType for I128 {}
impl NumType for F32 {}
impl NumType for F64 {}

pub(super) struct U8 {data: u8}
pub(super) struct I8 {data: u8}
pub(super) struct I16 {data: u16}
pub(super) struct I32 {data: u32}
pub(super) struct U32 {data: u32}
pub(super) struct I64 {data: u64}
pub(super) struct I128 {data: u128}
pub(super) struct F32 {data: f32}
pub(super) struct F64 {data: f64}

// Reference Types
pub(super) trait RefType {}
impl RefType for FuncRef {}
impl RefType for ExternRef {}

pub(super) struct FuncRef {idx: TableIdx}
pub(super) struct ExternRef {idx: TableIdx}

// Vector Types
pub(super) trait VecType {
    fn bits(&self) -> u128;
}

impl VecType for Vec128 {
    fn bits(&self) -> u128 { self.data }
}

impl VecType for VecI8x16 {
    fn bits(&self) -> u128 {
        ((self.data[0] as u128) << 120) +
        ((self.data[1] as u128) << 112) +
        ((self.data[2] as u128) << 104) +
        ((self.data[3] as u128) << 96) +
        ((self.data[4] as u128) << 88) +
        ((self.data[5] as u128) << 80) +
        ((self.data[6] as u128) << 72) +
        ((self.data[7] as u128) << 64) +
        ((self.data[8] as u128) << 56) +
        ((self.data[9] as u128) << 48) +
        ((self.data[10] as u128) << 40) +
        ((self.data[11] as u128) << 32) +
        ((self.data[12] as u128) << 24) +
        ((self.data[13] as u128) << 16) +
        ((self.data[14] as u128) << 8) +
        ((self.data[15] as u128) << 0)
    }
}

impl VecType for VecI16x8 {
    fn bits(&self) -> u128 {
        ((self.data[0] as u128) << 112) +
        ((self.data[1] as u128) << 96) +
        ((self.data[2] as u128) << 80) +
        ((self.data[3] as u128) << 64) +
        ((self.data[4] as u128) << 48) +
        ((self.data[5] as u128) << 32) +
        ((self.data[6] as u128) << 16) +
        ((self.data[7] as u128) << 0)
    }
}

impl VecType for VecI32x4 {
    fn bits(&self) -> u128 {
        ((self.data[0] as u128) << 96) +
        ((self.data[1] as u128) << 64) +
        ((self.data[2] as u128) << 32) +
        ((self.data[3] as u128) << 0)
    }
}

impl VecType for VecI64x2 {
    fn bits(&self) -> u128 {
        ((self.data[0] as u128) << 64) +
        ((self.data[1] as u128) << 0)
    }
}

impl VecType for VecF32x4 {
    fn bits(&self) -> u128 {
        ((self.data[0] as u128) << 96) +
        ((self.data[1] as u128) << 64) +
        ((self.data[2] as u128) << 32) +
        ((self.data[3] as u128) << 0)
    }
}

impl VecType for VecF64x2 {
    fn bits(&self) -> u128 {
        ((self.data[0] as u128) << 64) +
        ((self.data[1] as u128) << 0)
    }
}

pub(super) struct Vec128 {data: u128}
pub(super) struct VecI8x16 {data: [u8; 16]}
pub(super) struct VecI16x8 {data: [u16; 8]}
pub(super) struct VecI32x4 {data: [u32; 4]}
pub(super) struct VecI64x2 {data: [u64; 2]}
pub(super) struct VecF32x4 {data: [f32; 4]}
pub(super) struct VecF64x2 {data: [f64; 2]}

// Index Types
pub(super) struct Byte {data: U8}
pub(super) struct LaneIdx {data: U8}
pub(super) struct TypeIdx {data: U32}
pub(super) struct FuncIdx {data: U32}
pub(super) struct TableIdx {data: U32}
pub(super) struct MemIdx {data: U32}
pub(super) struct GlobalIdx {data: U32}
pub(super) struct ElemIdx {data: U32}
pub(super) struct DataIdx {data: U32}
pub(super) struct LocalIdx {data: U32}
pub(super) struct LabelIdx {data: U32}
pub(super) struct Offset {data: U32}
pub(super) struct Align {data: U32}

pub(super) trait BlockType {}
impl BlockType for Option<& dyn ValType> {}
impl BlockType for TypeIdx {}

// Limits
pub(super) struct Limits {
    min: u32,
    max: Option<u32>,
}

// Memory Types
pub(super) trait MemType {
    fn limits(&self) -> &Limits;
}

pub(super) struct Mem {limits: Limits}
impl MemType for Mem {
    fn limits(&self) -> &Limits { &self.limits }
}

// Function Types
pub(super) struct Start {
    pub func: FuncIdx
}

pub(super) trait FuncType {
    fn inputs(&self) -> & [& dyn ValType];
    fn returns(&self) -> & [& dyn ValType];
}

pub(super) struct Func<'a> {
    func_type: TypeIdx,
    locals: &'a [&'a dyn ValType],
    body: Expr<'a>,
    inputs: &'a [&'a dyn ValType],
    returns: &'a [&'a dyn ValType]
}
impl FuncType for Func<'_> {
    fn inputs(&self) -> & [&'_ dyn ValType] { self.inputs }
    fn returns(&self) -> & [&'_ dyn ValType] { self.returns }
}

// Table Types
pub(super) trait TableType {
    fn limits(&self) -> &Limits;
    fn reftype(&self) -> &dyn RefType;
}

impl TableType for Table {
    fn limits(&self) -> &Limits { &self.limits }
    fn reftype(&self) -> &dyn RefType { &self.reftype }
}

pub(super) struct Table {limits: Limits, reftype: dyn RefType}

pub(super) struct Elem<'a> {elem_type: &'a dyn RefType, init: &'a [Expr<'a>], mode: ElemMode}
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
pub(super) struct ConstGlobal<'a> {val: &'a mut dyn ValType}
pub(super) struct VarGlobal {val: dyn ValType}

pub(super) trait GlobalType {}
impl GlobalType for ConstGlobal<'_> {}
impl GlobalType for VarGlobal {}

// External Types
pub(super) trait ExternType {}
impl ExternType for dyn FuncType {}
impl ExternType for dyn TableType {}
impl ExternType for dyn MemType {}
impl ExternType for dyn GlobalType {}

// Import Types
pub(super) struct Import<'a> {
    module: &'a str,
    name: &'a str,
    desc: dyn ImportDesc
}

pub(super) trait ImportDesc {}
impl ImportDesc for dyn FuncType {}
impl ImportDesc for dyn TableType {}
impl ImportDesc for dyn MemType {}
impl ImportDesc for dyn GlobalType {}

// Export Types
pub(super) struct Export<'a> {
    name: &'a str,
    desc: dyn ExportDesc
}

pub(super) trait ExportDesc {}
impl ExportDesc for FuncIdx {}
impl ExportDesc for TableIdx {}
impl ExportDesc for MemIdx {}
impl ExportDesc for GlobalIdx {}

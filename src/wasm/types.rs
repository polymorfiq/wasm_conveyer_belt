use super::expressions::Expr;

#[derive(Clone, Copy)]
pub(super) enum ValTypes {
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    V128,
    AnyNum,
    FuncRef,
    ExternRef,
    AnyRef,
    Vec128,
    VecI8x16,
    VecI16x8,
    VecI32x4,
    VecI64x2,
    VecF32x4,
    VecF64x2,
    AnyVec,
    Unknown
}
pub(super) trait ValType {
    fn is_type(&self, t: ValTypes) -> bool;
}

impl ValType for dyn NumType {
    fn is_type(&self, t: ValTypes) -> bool {
        match t {
            ValTypes::AnyNum => true,
            ValTypes::Unknown => true,
            _ => self.is_type(t)
        }
    }
}
impl ValType for dyn VecType {
    fn is_type(&self, t: ValTypes) -> bool {
        match t {
            ValTypes::AnyVec => true,
            ValTypes::Unknown => true,
            _ => self.is_type(t)
        }
    }
}
impl ValType for dyn RefType {
    fn is_type(&self, t: ValTypes) -> bool {
        match t {
            ValTypes::AnyRef => true,
            ValTypes::Unknown => true,
            _ => self.is_type(t)
        }
    }
}

pub(super) trait ResultType {}
impl ResultType for [ValTypes] {}

// Num Types
pub(super) trait NumType {
    fn is_type(&self, t: ValTypes) -> bool;
}
impl NumType for I8 {
    fn is_type(&self, t: ValTypes) -> bool { matches!(t, ValTypes::I8) }
}
impl NumType for I16 {
    fn is_type(&self, t: ValTypes) -> bool { matches!(t, ValTypes::I16) }
}
impl NumType for I32 {
    fn is_type(&self, t: ValTypes) -> bool { matches!(t, ValTypes::I32) }
}
impl NumType for I64 {
    fn is_type(&self, t: ValTypes) -> bool { matches!(t, ValTypes::I64) }
}
impl NumType for F32 {
    fn is_type(&self, t: ValTypes) -> bool { matches!(t, ValTypes::F32) }
}
impl NumType for F64 {
    fn is_type(&self, t: ValTypes) -> bool { matches!(t, ValTypes::F64) }
}

pub(super) struct I8 {data: u8}
pub(super) struct I16 {data: u16}
pub(super) struct I32 {data: u32}
pub(super) struct I64 {data: u64}
pub(super) struct F32 {data: f32}
pub(super) struct F64 {data: f64}

// Reference Types
pub(super) trait RefType {
    fn is_type(&self, t: ValTypes) -> bool;
}
impl RefType for FuncRef {
    fn is_type(&self, t: ValTypes) -> bool { matches!(t, ValTypes::FuncRef) }
}
impl RefType for ExternRef {
    fn is_type(&self, t: ValTypes) -> bool { matches!(t, ValTypes::ExternRef) }
}

pub(super) struct FuncRef {idx: TableIdx}
pub(super) struct ExternRef {idx: TableIdx}

// Vector Types
pub(super) trait VecType {
    fn is_type(&self, t: ValTypes) -> bool;
    fn bits(&self) -> u128;
}

impl VecType for Vec128 {
    fn is_type(&self, t: ValTypes) -> bool { matches!(t, ValTypes::Vec128) }
    fn bits(&self) -> u128 { self.data }
}

impl VecType for VecI8x16 {
    fn is_type(&self, t: ValTypes) -> bool { matches!(t, ValTypes::VecI8x16) }
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
    fn is_type(&self, t: ValTypes) -> bool { matches!(t, ValTypes::VecI16x8) }
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
    fn is_type(&self, t: ValTypes) -> bool { matches!(t, ValTypes::VecI32x4) }
    fn bits(&self) -> u128 {
        ((self.data[0] as u128) << 96) +
        ((self.data[1] as u128) << 64) +
        ((self.data[2] as u128) << 32) +
        ((self.data[3] as u128) << 0)
    }
}

impl VecType for VecI64x2 {
    fn is_type(&self, t: ValTypes) -> bool { matches!(t, ValTypes::VecI64x2) }
    fn bits(&self) -> u128 {
        ((self.data[0] as u128) << 64) +
        ((self.data[1] as u128) << 0)
    }
}

impl VecType for VecF32x4 {
    fn is_type(&self, t: ValTypes) -> bool { matches!(t, ValTypes::VecF32x4) }
    fn bits(&self) -> u128 {
        ((self.data[0] as u128) << 96) +
        ((self.data[1] as u128) << 64) +
        ((self.data[2] as u128) << 32) +
        ((self.data[3] as u128) << 0)
    }
}

impl VecType for VecF64x2 {
    fn is_type(&self, t: ValTypes) -> bool { matches!(t, ValTypes::VecF64x2) }
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
pub(super) struct Byte {data: u8}
pub(super) struct LaneIdx {data: u8}
pub(super) struct TypeIdx {data: u32}
pub(super) struct FuncIdx {data: u32}
pub(super) struct TableIdx {data: u32}
pub(super) struct MemIdx {data: u32}
pub(super) struct GlobalIdx {data: u32}
pub(super) struct ElemIdx {data: u32}
pub(super) struct DataIdx {data: u32}
pub(super) struct LocalIdx {data: u32}
pub(super) struct LabelIdx {data: u32}
pub(super) struct Offset {data: u32}
pub(super) struct Align {data: u32}

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

// OpCode Types
#[derive(Clone, Copy)]
pub(super) enum OpCode {
    Loop
}
#[derive(Debug, PartialEq)]
pub struct Module {
    pub types: Vec<Type>,
    pub funcs: Vec<Func>,
//    pub tables: Vec<Table>,
//    pub mems: Vec<Mem>,
//    pub globals: Vec<Global>,
//    pub elem: Vec<Elem>,
//    pub data: Vec<Data>,
//    pub start: Option<Start>,
//    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
}

// ValueType ::= NumberType | VectorType | ReferenceType
#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum ValueType {
    NumberType(NumberType),
    VectorType(VectorType),
    ReferenceType(ReferenceType),
}

// NumberType ::= i32 | i64 | f32 | f64
#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum NumberType {
    I32,
    I64,
    F32,
    F64,
}

// VectorType ::= v128
#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum VectorType {
    V128,
}

// ReferenceType ::= funcref | externref
#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum ReferenceType {
    FuncRef,
    ExternRef,
}

// ResultType ::= [vec(ValueType)]
pub type ResultType = Vec<ValueType>;
pub type FuncType = (ResultType, ResultType);
pub type Type = FuncType;

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Instr {
    LocalGet(usize),
    I32Add,
}

// Func ::= {type typeidx, locals vec(ValType), body Expr}
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Func {
    pub f_type: i32,
    pub locals: Vec<ValueType>,
    pub body: Vec<Instr>,
}

// ExportDesc ::= Func(funcidx) | Table(tableidx) | Mem(memidx) | Global(globalidx)
#[derive(Debug, PartialEq, Clone, Eq)]
pub enum ExportDesc {
    Func(usize),
    Table(usize),
    Mem(usize),
    Global(usize),
}

// Export ::= {name name, desc ExportDesc}
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Export {
    pub name: String,
    pub desc: ExportDesc,
}
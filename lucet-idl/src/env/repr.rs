use crate::env::atoms::{AbiType, AtomType};
use cranelift_entity::{entity_impl, PrimaryMap};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ModuleIx(u32);
entity_impl!(ModuleIx);

#[derive(Debug, Clone)]
pub struct PackageRepr {
    pub names: PrimaryMap<ModuleIx, String>,
    pub modules: PrimaryMap<ModuleIx, ModuleRepr>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DatatypeIx(u32);
entity_impl!(DatatypeIx);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DatatypeIdent {
    pub module: ModuleIx,
    pub datatype: DatatypeIx,
}

impl DatatypeIdent {
    pub fn new(module: ModuleIx, datatype: DatatypeIx) -> Self {
        DatatypeIdent { module, datatype }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FuncIx(u32);
entity_impl!(FuncIx);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncIdent(ModuleIx, FuncIx);

#[derive(Debug, Clone)]
pub struct ModuleRepr {
    pub datatypes: ModuleDatatypesRepr,
    pub funcs: ModuleFuncsRepr,
}

#[derive(Debug, Clone)]
pub struct ModuleDatatypesRepr {
    pub names: PrimaryMap<DatatypeIx, String>,
    pub datatypes: PrimaryMap<DatatypeIx, DatatypeRepr>,
    pub topological_order: Vec<DatatypeIx>,
}

#[derive(Debug, Clone)]
pub struct ModuleFuncsRepr {
    pub names: PrimaryMap<FuncIx, String>,
    pub funcs: PrimaryMap<FuncIx, FuncRepr>,
}

#[derive(Debug, Clone)]
pub struct DatatypeRepr {
    pub variant: DatatypeVariantRepr,
    pub mem_size: usize,
    pub mem_align: usize,
}

#[derive(Debug, Clone)]
pub enum DatatypeVariantRepr {
    Atom(AtomType),
    Struct(StructDatatypeRepr),
    Enum(EnumDatatypeRepr),
    Alias(AliasDatatypeRepr),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StructMemberRepr {
    pub type_: DatatypeIdent,
    pub name: String,
    pub offset: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StructDatatypeRepr {
    pub members: Vec<StructMemberRepr>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EnumMember {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EnumDatatypeRepr {
    pub members: Vec<EnumMember>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AliasDatatypeRepr {
    pub to: DatatypeIdent,
}

#[derive(Debug, Clone)]
pub struct FuncRepr {
    pub args: PrimaryMap<ArgIx, ParamRepr>,
    pub rets: PrimaryMap<RetIx, ParamRepr>,
    pub bindings: PrimaryMap<BindingIx, BindingRepr>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ArgIx(u32);
entity_impl!(ArgIx);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RetIx(u32);
entity_impl!(RetIx);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ParamIx {
    Arg(ArgIx),
    Ret(RetIx),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BindingIx(u32);
entity_impl!(BindingIx);

#[derive(Debug, Clone)]
pub struct ParamRepr {
    pub name: String,
    pub type_: AbiType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingRepr {
    pub name: String,
    pub type_: DatatypeIdent,
    pub direction: BindingDirection,
    pub from: BindingFromRepr,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BindingDirection {
    In,
    InOut,
    Out,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BindingFromRepr {
    Ptr(ParamIx),
    Slice(ParamIx, ParamIx),
    Value(ParamIx),
}
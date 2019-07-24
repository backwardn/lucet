use std::fmt;

pub trait MemArea {
    fn repr_size(&self) -> usize;
    fn align(&self) -> usize;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AtomType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
}

impl MemArea for AtomType {
    fn repr_size(&self) -> usize {
        match self {
            AtomType::Bool => 1,
            AtomType::U8 | AtomType::I8 => 1,
            AtomType::U16 | AtomType::I16 => 2,
            AtomType::U32 | AtomType::I32 | AtomType::F32 => 4,
            AtomType::U64 | AtomType::I64 | AtomType::F64 => 8,
        }
    }
    fn align(&self) -> usize {
        self.repr_size()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AbiType {
    I32,
    I64,
    F32,
    F64,
}

impl AbiType {
    pub fn repr_size(&self) -> usize {
        match self {
            AbiType::I32 | AbiType::F32 => 4,
            AbiType::I64 | AbiType::F64 => 8,
        }
    }

    pub fn from_atom(a: &AtomType) -> Self {
        match a {
            AtomType::Bool
            | AtomType::U8
            | AtomType::I8
            | AtomType::U16
            | AtomType::I16
            | AtomType::U32
            | AtomType::I32 => AbiType::I32,
            AtomType::I64 | AtomType::U64 => AbiType::I64,
            AtomType::F32 => AbiType::F32,
            AtomType::F64 => AbiType::F64,
        }
    }

    pub fn of_atom(a: AtomType) -> Option<Self> {
        match a {
            AtomType::I32 => Some(AbiType::I32),
            AtomType::I64 => Some(AbiType::I64),
            AtomType::F32 => Some(AbiType::F32),
            AtomType::F64 => Some(AbiType::F64),
            _ => None,
        }
    }
}

impl From<AbiType> for AtomType {
    fn from(abi: AbiType) -> AtomType {
        match abi {
            AbiType::I32 => AtomType::I32,
            AbiType::I64 => AtomType::I64,
            AbiType::F32 => AtomType::F32,
            AbiType::F64 => AtomType::F64,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Ident(pub usize);

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DataTypeRef {
    Defined(Ident),
    Atom(AtomType),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StructMember {
    pub type_: DataTypeRef,
    pub name: String,
    pub offset: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StructDataType {
    pub members: Vec<StructMember>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EnumMember {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EnumDataType {
    pub members: Vec<EnumMember>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AliasDataType {
    pub to: DataTypeRef,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DataTypeVariant {
    Struct(StructDataType),
    Enum(EnumDataType),
    Alias(AliasDataType),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DataType {
    pub variant: DataTypeVariant,
    pub repr_size: usize,
    pub align: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Name {
    pub name: String,
    pub location: Location,
}

impl MemArea for DataType {
    fn repr_size(&self) -> usize {
        self.repr_size
    }
    fn align(&self) -> usize {
        self.align
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// A convenient structure holding a data type, its name and
/// its internal IDL representation
#[derive(Debug, Clone)]
pub struct Named<'t, E> {
    pub id: Ident,
    pub name: &'t Name,
    pub entity: &'t E,
}

impl<'a, T> Named<'a, T> {
    pub fn using_name<U>(&self, other: &'a U) -> Named<'a, U> {
        Named {
            id: self.id,
            name: self.name,
            entity: other,
        }
    }
}

impl<'a> Named<'a, DataType> {
    pub fn datatype_ref(&self) -> DataTypeRef {
        DataTypeRef::Defined(self.id)
    }
}

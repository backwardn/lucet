use crate::env::atoms::AtomType;
use crate::env::repr::{
    AliasDatatypeRepr, DatatypeIdent, DatatypeIx, DatatypeRepr, DatatypeVariantRepr,
    EnumDatatypeRepr, EnumMember, ModuleIx, ModuleRepr, PackageRepr, StructDatatypeRepr,
    StructMemberRepr,
};
use crate::env::MemArea;

#[derive(Debug, Clone)]
pub struct Package<'a> {
    repr: &'a PackageRepr,
}

impl<'a> Package<'a> {
    pub fn module(&self, name: &str) -> Option<Module<'a>> {
        self.repr
            .names
            .iter()
            .find(|(_, n)| *n == name)
            .and_then(|(ix, _)| self.module_by_ix(ix))
    }

    pub(crate) fn module_by_ix(&self, ix: ModuleIx) -> Option<Module<'a>> {
        if self.repr.modules.is_valid(ix) {
            Some(Module {
                pkg: &self.repr,
                ix,
            })
        } else {
            None
        }
    }

    pub(crate) fn datatype_by_id(&self, id: DatatypeIdent) -> Option<Datatype<'a>> {
        self.module_by_ix(id.module)
            .and_then(|m| m.datatype_by_ix(id.datatype))
    }
}

#[derive(Debug, Clone)]
pub struct Module<'a> {
    pkg: &'a PackageRepr,
    ix: ModuleIx,
}

impl<'a> Module<'a> {
    fn repr(&self) -> &'a ModuleRepr {
        self.pkg.modules.get(self.ix).expect("i exist")
    }

    pub fn name(&self) -> &str {
        self.pkg.names.get(self.ix).expect("i exist")
    }

    pub fn datatype(&self, name: &str) -> Option<Datatype<'a>> {
        self.repr()
            .datatypes
            .names
            .iter()
            .find(|(_, n)| *n == name)
            .and_then(|(ix, _)| self.datatype_by_ix(ix))
    }

    pub(crate) fn datatype_by_ix(&self, ix: DatatypeIx) -> Option<Datatype<'a>> {
        if self.repr().datatypes.datatypes.is_valid(ix) {
            Some(Datatype {
                pkg: self.pkg,
                id: DatatypeIdent::new(self.ix, ix),
            })
        } else {
            None
        }
    }

    pub fn datatypes(&self) -> impl Iterator<Item = Datatype<'a>> {
        let pkg = self.pkg;
        let mix = self.ix;
        self.repr()
            .datatypes
            .datatypes
            .keys()
            .map(move |ix| Datatype {
                pkg,
                id: DatatypeIdent::new(mix, ix),
            })
    }
}

#[derive(Debug, Clone)]
pub struct Datatype<'a> {
    pkg: &'a PackageRepr,
    id: DatatypeIdent,
}

impl<'a> Datatype<'a> {
    fn repr(&self) -> &'a DatatypeRepr {
        self.pkg
            .modules
            .get(self.id.module)
            .expect("my mod exists")
            .datatypes
            .datatypes
            .get(self.id.datatype)
            .expect("i exist")
    }

    pub fn name(&self) -> &'a str {
        self.pkg
            .modules
            .get(self.id.module)
            .expect("my mod exists")
            .datatypes
            .names
            .get(self.id.datatype)
            .expect("i exist")
    }

    pub fn variant(&'a self) -> DatatypeVariant<'a> {
        match self.repr().variant {
            DatatypeVariantRepr::Atom(a) => DatatypeVariant::Atom(a),
            DatatypeVariantRepr::Struct(ref repr) => DatatypeVariant::Struct(StructDatatype {
                pkg: self.pkg,
                repr: &repr,
                id: self.id,
            }),
            DatatypeVariantRepr::Enum(ref repr) => DatatypeVariant::Enum(EnumDatatype {
                pkg: self.pkg,
                repr: &repr,
                id: self.id,
            }),
            DatatypeVariantRepr::Alias(ref repr) => DatatypeVariant::Alias(AliasDatatype {
                pkg: self.pkg,
                repr: &repr,
                id: self.id,
            }),
        }
    }
}

impl<'a> MemArea for Datatype<'a> {
    fn mem_size(&self) -> usize {
        self.repr().mem_size
    }
    fn mem_align(&self) -> usize {
        self.repr().mem_align
    }
}

#[derive(Debug, Clone)]
pub enum DatatypeVariant<'a> {
    Atom(AtomType),
    Struct(StructDatatype<'a>),
    Enum(EnumDatatype<'a>),
    Alias(AliasDatatype<'a>),
}

#[derive(Debug, Clone)]
pub struct StructDatatype<'a> {
    pkg: &'a PackageRepr,
    repr: &'a StructDatatypeRepr,
    id: DatatypeIdent,
}

impl<'a> StructDatatype<'a> {
    pub fn name(&self) -> &str {
        Datatype {
            pkg: self.pkg,
            id: self.id,
        }
        .name()
    }
    pub fn members(&self) -> impl Iterator<Item = StructMember<'a>> {
        let pkg = self.pkg;
        self.repr
            .members
            .iter()
            .map(move |repr| StructMember { pkg, repr })
    }
}

impl<'a> From<StructDatatype<'a>> for Datatype<'a> {
    fn from(s: StructDatatype<'a>) -> Datatype<'a> {
        Datatype {
            pkg: s.pkg,
            id: s.id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StructMember<'a> {
    pkg: &'a PackageRepr,
    repr: &'a StructMemberRepr,
}

impl<'a> StructMember<'a> {
    pub fn name(&self) -> &str {
        &self.repr.name
    }
    pub fn offset(&self) -> usize {
        self.repr.offset
    }
    pub fn type_(&self) -> Datatype<'a> {
        Datatype {
            pkg: self.pkg,
            id: self.repr.type_,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnumDatatype<'a> {
    pkg: &'a PackageRepr,
    repr: &'a EnumDatatypeRepr,
    id: DatatypeIdent,
}

impl<'a> EnumDatatype<'a> {
    pub fn name(&self) -> &str {
        Datatype {
            pkg: self.pkg,
            id: self.id,
        }
        .name()
    }
    pub fn members(&self) -> &'a [EnumMember] {
        &self.repr.members
    }
}
impl<'a> From<EnumDatatype<'a>> for Datatype<'a> {
    fn from(e: EnumDatatype<'a>) -> Datatype<'a> {
        Datatype {
            pkg: e.pkg,
            id: e.id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AliasDatatype<'a> {
    pkg: &'a PackageRepr,
    repr: &'a AliasDatatypeRepr,
    id: DatatypeIdent,
}

impl<'a> AliasDatatype<'a> {
    pub fn name(&self) -> &str {
        Datatype {
            pkg: self.pkg,
            id: self.id,
        }
        .name()
    }
    pub fn to(&self) -> Datatype<'a> {
        Datatype {
            pkg: self.pkg,
            id: self.repr.to,
        }
    }
}

impl<'a> From<AliasDatatype<'a>> for Datatype<'a> {
    fn from(a: AliasDatatype<'a>) -> Datatype<'a> {
        Datatype {
            pkg: a.pkg,
            id: a.id,
        }
    }
}
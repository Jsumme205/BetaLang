use std::{error::Error, fmt::Display};

use super::yarn::{self, Yarn};

pub(crate) struct Attribute<'a> {
    name: yarn::Yarn<'a>,
    value: yarn::Yarn<'a>,
    is_valid: bool
}

impl Attribute<'_> {

    pub fn immortalize(self) -> Attribute<'static> {
        Attribute::<'static> {
            name: self.name.immortalize(),
            value: self.value.immortalize(),
            is_valid: self.is_valid
        }
    }
}

pub(crate) struct DefunDescriptor<'a> {
    name: yarn::Yarn<'a>,
    qualified: yarn::Yarn<'a>,
    attrs: Vec<Attribute<'a>>,
    args: Vec<Box<VarDeclaration<'a>>>,
    return_type: Box<Type<'a>>,
    in_scope: bool
}

impl DefunDescriptor<'_> {

    pub fn immortalize(self) -> DefunDescriptor<'static> {
        DefunDescriptor::<'static> {
            name: self.name.immortalize(),
            qualified: self.qualified.immortalize(),
            attrs: self.attrs.into_iter().map(|at| at.immortalize()).collect(),
            args: self.args.into_iter().map(|ar| Box::new(ar.immortalize())).collect(),
            return_type: Box::new(self.return_type.immortalize()),
            in_scope: self.in_scope
        }
    }

}

pub(crate) struct TraitDescriptor<'a> {
    functions: DefunDescriptor<'a>,
    asociated_aliases: Vec<Box<VarDeclaration<'a>>>,
    in_scope: bool,
    super_traits: Vec<Box<TraitDescriptor<'a>>>
}

impl TraitDescriptor<'_> {

    pub fn immortalize(self) -> TraitDescriptor<'static> {
        TraitDescriptor::<'static> {
            functions: self.functions.immortalize(),
            asociated_aliases: self.asociated_aliases.into_iter().map(|aa| Box::new(aa.immortalize())).collect(),
            in_scope: self.in_scope,
            super_traits: self.super_traits.into_iter().map(|st| Box::new(st.immortalize())).collect()
        }
    }
}

type Traits<'a> = Vec<Box<TraitDescriptor<'a>>>;

pub(crate) struct ObjDescriptor<'a> {
    name: yarn::Yarn<'a>,
    fields: Vec<Box<VarDeclaration<'a>>>,
    attrs: Vec<Attribute<'a>>,
    in_scope: bool,
    traits: Vec<Box<TraitDescriptor<'a>>>,
    functions: Vec<Box<DefunDescriptor<'a>>>
}

impl ObjDescriptor<'_> {

    pub fn immortalize(self) -> ObjDescriptor<'static> {

        ObjDescriptor::<'static> {
            name: self.name.immortalize(),
            fields: self.fields.into_iter().map(|f| Box::new(f.immortalize())).collect(),
            attrs: self.attrs.into_iter().map(|at| at.immortalize()).collect(),
            in_scope: self.in_scope,
            traits: self.traits.into_iter().map(|t| Box::new(t.immortalize())).collect(),
            functions: self.functions.into_iter().map(|f| Box::new(f.immortalize())).collect(),
        }

    }

}

pub(crate) struct CompDescriptor<'a> {
    name: yarn::Yarn<'a>,
    fields: Vec<Box<VarDeclaration<'a>>>,
    attrs: Vec<Attribute<'a>>,
    in_scope: bool
}

impl CompDescriptor<'_> {

    pub fn immortalize(self) -> CompDescriptor<'static> {
        CompDescriptor::<'static> {
            name: self.name.immortalize(),
            fields: self.fields.into_iter().map(|f| Box::new(f.immortalize())).collect(),
            attrs: self.attrs.into_iter().map(|a| a.immortalize()).collect(),
            in_scope: self.in_scope
        }
    }

}

#[derive(Debug)]
pub(crate) enum DeclError {
    LetAbsent,
    MissingColon,
    NoSemicolon,
    NoValidType
}

impl Display for DeclError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LetAbsent => f.write_str("LetAbsent"),
            Self::MissingColon => f.write_str("MissingColon"),
            Self::NoSemicolon => f.write_str("NoSemicolon"),
            Self::NoValidType => f.write_str("NoValidType")
        }
    }
}

#[derive(Debug)]
pub(crate) struct VariableError {
    decl: DeclError,
    line: usize
}

impl VariableError {
    pub fn new(
        tpe: DeclError,
        line: usize
    ) -> Self {
        Self {
            decl: tpe,
            line
        }
    }
}

impl Display for VariableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("VariableError: Type: {}, Line: {}", self.decl, self.line))
    }
}

impl Error for VariableError {}

pub(crate) enum Type<'a> {
    Int8,
    Int16,
    Int32,
    Int64,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Float8,
    Float16,
    Float32,
    Float64,
    Boolean,
    Str,
    UnsafePtr(Box<Type<'a>>),
    SafePtr(Box<Type<'a>>),
    Array(Box<Type<'a>>),
    Slice(Box<Type<'a>>),
    Object(Box<ObjDescriptor<'a>>),
    Composition(Box<CompDescriptor<'a>>),
    Trait(Box<TraitDescriptor<'a>>)
}

impl Type<'_> {

    pub fn immortalize(mut self) -> Type<'static> {
        match self {
            Type::UnsafePtr(ty) => Type::<'static>::UnsafePtr(Box::new(ty.immortalize())),
            Type::SafePtr(ty) => Type::<'static>::UnsafePtr(Box::new(ty.immortalize())),
            Type::Array(ty) => Type::<'static>::Array(Box::new(ty.immortalize())),
            Type::Slice(ty) => Type::<'static>::Array(Box::new(ty.immortalize())),
            Type::Object(ty) => Type::<'static>::Object(Box::new(ty.immortalize())),
            Type::Composition(ty) => Type::<'static>::Composition(Box::new(ty.immortalize())),
            Type::Trait(ty) => Type::<'static>::Trait(Box::new(ty.immortalize())),
            Type::Int8 => Type::<'static>::Int8,
            Type::Int16 => Type::<'static>::Int16,
            Type::Int32 => Type::<'static>::Int32,
            _ => todo!()
        }
    }
}

pub(crate) enum VarDeclaration<'a> {
    Int8 {
        active_traits: Traits<'a>,
        name: Option<Yarn<'a>>
    },
    Int16 {
        active_traits: Traits<'a>,
        name: Option<Yarn<'a>>
    },
    Int32 {
        active_traits: Traits<'a>,
        name: Option<Yarn<'a>>
    },
    Int64{
        active_traits: Traits<'a>,
        name: Option<Yarn<'a>>
    },
    Uint8{
        active_traits: Traits<'a>,
        name: Option<Yarn<'a>>
    },
    Uint16{
        active_traits: Traits<'a>,
        name: Option<Yarn<'a>>
    },
    Uint32{
        active_traits: Traits<'a>,
        name: Option<Yarn<'a>>
    },
    Uint64{
        active_traits: Traits<'a>,
        name: Option<Yarn<'a>>
    },
    Float8{
        active_traits: Traits<'a>,
        name: Option<Yarn<'a>>
    },
    Float16{
        active_traits: Traits<'a>,
        name: Option<Yarn<'a>>
    },
    Float32{
        active_traits: Traits<'a>,
        name: Option<Yarn<'a>>
    },
    Float64{
        active_traits: Traits<'a>,
        name: Option<Yarn<'a>>
    },
    Boolean{
        active_traits: Traits<'a>,
        name: Option<Yarn<'a>>
    },
    Str {
        active_traits: Traits<'a>,
        name: Option<Yarn<'a>>
    },
    UnsafePtr{
        active_traits: Traits<'a>,
        name: Option<Yarn<'a>>,
        ptr_type: Box<Type<'a>>
    },
    SafePtr {
        active_traits: Traits<'a>,
        name: Option<Yarn<'a>>,
        ptr_type: Box<Type<'a>>,
        ptr_delagate: Box<ObjDescriptor<'a>>
    },
    Array {
        active_traits: Traits<'a>,
        name: Option<Yarn<'a>>,
        arr_type: Box<Type<'a>>,
        number: usize
    },
    Slice {
        active_traits: Traits<'a>,
        name: Option<Yarn<'a>>,
        slice_type: Box<Type<'a>>,
        len: usize
    },
    Object {
        name: Option<Yarn<'a>>,
        inner: Box<ObjDescriptor<'a>> 
    },
    Composition {
        name: Option<Yarn<'a>>,
        inner: Box<CompDescriptor<'a>>
    },
    Trait {
        name: Option<Yarn<'a>>,
        inner: Box<TraitDescriptor<'a>>
    },
}

impl<'a> VarDeclaration<'a> {
    
    // obj Foo {}
    
    // TODO: Better Error Handling
    pub fn from_yarn(string: &'a Yarn<'a>) -> Result<Self, VariableError> {
        let parsed = string.spilt(' ');
        let mut last = parsed[1].clone();

        for i in 2..parsed.len() {
            match parsed[i].as_slice() {
                "Int8" => {
                    if !last.as_slice().ends_with(':') {
                        return Err(VariableError::new(
                            DeclError::MissingColon,
                            line!() as usize
                        ));
                    }

                    if parsed[i - 2].clone().as_slice() != "let" {
                        return Err(VariableError::new(
                            DeclError::LetAbsent,
                            line!() as usize
                        ));
                    }

                    let name: Box<str> = last.as_slice()[..last.len() - 1].into();

                    return Ok(Self::Int8 {
                        active_traits: Vec::new(),
                        name: Some(Yarn::owned(name))
                    });
                },
                "Int16" => {
                    if !last.as_slice().ends_with(':') {
                        return Err(VariableError::new(
                            DeclError::MissingColon,
                            line!() as usize
                        ));
                    }

                    if parsed[i - 2].clone().as_slice() != "let" {
                        return Err(VariableError::new(
                            DeclError::LetAbsent,
                            line!() as usize
                        ));
                    }

                    let name: Box<str> = last.as_slice()[..last.len() - 1].into();

                    return Ok(Self::Int16 {
                        active_traits: Vec::new(),
                        name: Some(Yarn::owned(name))
                    });
                },
                "Int32" => {
                    if !last.as_slice().ends_with(':') {
                        return Err(VariableError::new(
                            DeclError::MissingColon,
                            line!() as usize
                        ));
                    }

                    if parsed[i - 2].clone().as_slice() != "let" {
                        return Err(VariableError::new(
                            DeclError::MissingColon,
                            line!() as usize
                        ));
                    }

                    let name: Box<str> = last.as_slice()[..last.len() - 1].into();

                    return Ok(Self::Int32{
                        active_traits: Vec::new(),
                        name: Some(Yarn::owned(name))
                    });
                },
                "Int64" => {

                },
                "Uint8" => {

                },
                "Uint16" => {

                },
                "Uint32" => {

                },
                "Uint64" => {

                },
                "Float8" => {

                },
                "Float16" => {

                },
                "Float32" => {

                },
                "Float64" => {

                },
                "Boolean" => {

                },
                "Str" => {

                },
                "*unsafe" => {
                    
                },
                "obj" => {},
                "comp" => {},
                "trait" => {},
                "extend" => {},
                _ => {
                    //TODO: Add Support for Arrays, Slices, and Safe pointers in here

                    if i == parsed.len() {
                        return Err(VariableError::new(
                            DeclError::NoValidType,
                            line!() as usize
                        ));
                    }
                    continue;
                }
            }
            last = parsed[i].clone();
        }

        todo!()
    }

    pub fn immortalize(mut self) -> VarDeclaration<'static> {
        match self {
            Self::Int8 { active_traits, name } => {
                todo!()
            },
            _ =>  todo!()
        }
    }

} 

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum BinOp {
    Add,
    AddAssign,
    Subtract,
    SubAssign,
    Divide,
    DivAssign,
    Multiply,
    MulAssign,
    Modulus,
    ModAssign,
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanEq,
    LessThan,
    LessThanEq,
    LogOr,
    LogAnd
}

impl BinOp {
    pub fn parse_op<'a>(yarn: &Yarn<'a>) -> Result<Self, ()> {
        match yarn.as_slice() {
            "+" => Ok(Self::Add),
            "+=" => Ok(Self::AddAssign),
            "-" => Ok(Self::Subtract),
            "-=" => Ok(Self::SubAssign),
            "/" => Ok(Self::Divide),
            "/=" => Ok(Self::DivAssign),
            "*" => Ok(Self::Multiply),
            "*=" => Ok(Self::MulAssign),
            "%" => Ok(Self::Modulus),
            "%=" => Ok(Self::ModAssign),
            "==" => Ok(Self::Equals),
            "!=" => Ok(Self::NotEquals),
            ">" => Ok(Self::GreaterThan),
            ">=" => Ok(Self::GreaterThanEq),
            "<" => Ok(Self::LessThan),
            "<=" => Ok(Self::LessThanEq),
            "||" => Ok(Self::LogOr),
            "&&" => Ok(Self::LogAnd),
            _ => Err(())
        }
    }
}

pub(crate) enum UniOp {
    Increment,
    Decrement,
    Negative,
    LogNot,
    BitNot
}

pub(crate) enum Bodies<'a> {
    Object(ObjDescriptor<'a>),
    Composition(CompDescriptor<'a>),
    Trait(TraitDescriptor<'a>),
    Defun(DefunDescriptor<'a>),
}

pub enum Node<'a> {
    Head {
        next: Box<Node<'a>>
    },
    BinaryOp {
        lhs: Box<Node<'a>>,
        rhs: Box<Node<'a>>,  
        op: BinOp
    },
    UnaryOp {
        lhs: Box<Node<'a>>,
        op: UniOp
    },
    Body {
        discriptor: Box<Bodies<'a>>,
        body: Vec<Box<Node<'a>>>
    },
    Value {
       ret: VarDeclaration<'a> 
    },
    Call {
        func: Box<DefunDescriptor<'a>>,
    },
    Chain {
        chained: Vec<Box<Node<'a>>>
    },
    ObjCall {
        obj: Box<ObjDescriptor<'a>>,
        func: Box<DefunDescriptor<'a>>
    }
}

impl<'a> Node<'a> {

    pub fn is_head(&self) -> bool {
        match self {
            Self::Head { .. } => true,
            _ => false
        }
    }

    pub fn is_tail(&self) -> bool {
        match self {
            Self::Value { .. } => true,
            _ => false
        }
    }

    pub fn extract_value(&self) -> Result<&VarDeclaration<'a>, ()> {
        match self {
            Self::Value { ret } => Ok(ret),
            _ => Err(())
        }
    }
}

pub(crate) trait ToNodes {
    fn eval(&self) -> Result<Node<'_>, ()>;
}

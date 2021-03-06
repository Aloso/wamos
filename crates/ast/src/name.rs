use std::fmt;

use tinystring::TinyString;

/// An identifier. It has to fulfill the following criteria:
///
/// * ASCII-only
/// * It can only contain letters (`a-z`, `A-Z`), digits (`0-9`), underscores
///   (`_`) and operators (`+-*/%~<>=?!`)
/// * It must start with a lowercase letter (`a-z`)
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Ident(TinyString);

/// An operator. It has to fulfill the following criteria:
///
/// * ASCII-only
/// * It can only contain letters (`a-z`, `A-Z`), underscores (`_`) and
///   operators (`+-*/%~<>=?!`)
/// * It must start with an operator (`+-*/%~<>=?!`)
/// * The single equality sign (`=`) is NOT a valid operator
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Operator(TinyString);

/// An type name. It has to fulfill the following criteria:
///
/// * ASCII-only
/// * It can only contain letters (`a-z`, `A-Z`), digits (`0-9`), underscores
///   (`_`) and operators (`+-*/%~<>=?!`)
/// * It must start with an uppercase letter (`A-Z`)
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct UpperIdent(TinyString);


impl Ident {
    pub fn new(string: impl Into<TinyString>) -> Self { Self(string.into()) }

    pub fn get(&self) -> &str { &*self.0 }

    pub fn inner(&self) -> TinyString { self.0.clone() }
}

impl Operator {
    pub fn new(string: impl Into<TinyString>) -> Self { Self(string.into()) }

    pub fn get(&self) -> &str { &*self.0 }

    pub fn inner(&self) -> TinyString { self.0.clone() }
}

impl UpperIdent {
    pub fn new(string: impl Into<TinyString>) -> Self { Self(string.into()) }

    pub fn get(&self) -> &str { &*self.0 }

    pub fn inner(&self) -> TinyString { self.0.clone() }
}

impl fmt::Debug for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ident {}", &self.0)
    }
}

impl fmt::Debug for UpperIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UpperIdent {}", &self.0)
    }
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Operator {}", &self.0)
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl fmt::Display for UpperIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[cfg(feature = "fuzz")]
impl arbitrary::Arbitrary for UpperIdent {
    fn arbitrary(_: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        Ok(UpperIdent(TinyString::from("T")))
    }
}

#[cfg(feature = "fuzz")]
impl arbitrary::Arbitrary for Ident {
    fn arbitrary(_: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        Ok(Ident(TinyString::from("v")))
    }
}

#[cfg(feature = "fuzz")]
impl arbitrary::Arbitrary for Operator {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        #[repr(u8)]
        #[derive(arbitrary::Arbitrary)]
        enum OpChar {
            Add = b'+',
            Mul = b'*',
        }

        let b = u.arbitrary::<OpChar>()? as u8 as char;
        let mut buf = [0; 4];
        let s = b.encode_utf8(&mut buf);
        Ok(Operator(TinyString::from(&*s)))
    }
}

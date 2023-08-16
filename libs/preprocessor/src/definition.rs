use peekmore::{PeekMore, PeekMoreIterator};

use crate::token::Token;

#[derive(Clone, Debug, PartialEq, Eq)]
/// A macro definition
pub enum Definition {
    /// A [`FunctionDefinition`] that takes parameters
    Function(FunctionDefinition),
    /// A value that is a list of [`Token`]s to be added at the call site
    Value(Vec<Token>),
    /// A flag that can be checked with `#ifdef`
    Unit,
}

impl Definition {
    #[must_use]
    /// Check if the definition is a [`FunctionDefinition`]
    pub const fn is_function(&self) -> bool {
        matches!(self, Self::Function(_))
    }

    #[must_use]
    /// Check if the definition is a value
    pub const fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }

    #[must_use]
    /// Check if the definition is a flag
    pub const fn is_unit(&self) -> bool {
        matches!(self, Self::Unit)
    }

    #[must_use]
    /// Get the [`FunctionDefinition`] if it is one
    pub const fn as_function(&self) -> Option<&FunctionDefinition> {
        match self {
            Self::Function(f) => Some(f),
            _ => None,
        }
    }

    #[must_use]
    /// Get the value [`Token`]s if it is a value
    pub fn as_value(&self) -> Option<&[Token]> {
        match self {
            Self::Value(v) => Some(v),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A function definition
///
/// # Examples
///
/// ```cpp
/// #define QUOTE(x) #x
/// #define FOO(a, b) QUOTE(a + b)
/// my_value = FOO(1, 2);
/// ```
pub struct FunctionDefinition {
    args: Vec<Token>,
    body: Vec<Token>,
}

impl FunctionDefinition {
    #[must_use]
    /// Create a new [`FunctionDefinition`]
    pub fn new(args: Vec<Token>, body: Vec<Token>) -> Self {
        Self { args, body }
    }

    #[must_use]
    /// Get the parameter [`Token`]s
    pub fn args(&self) -> &[Token] {
        &self.args
    }

    #[must_use]
    /// Get the body [`Token`]s
    pub fn body(&self) -> &[Token] {
        &self.body
    }

    #[must_use]
    /// Get the body as a stream
    pub fn stream(&self) -> PeekMoreIterator<impl Iterator<Item = Token>> {
        self.body.clone().into_iter().peekmore()
    }
}

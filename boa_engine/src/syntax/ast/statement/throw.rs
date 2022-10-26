use crate::syntax::ast::{statement::Statement, ContainsSymbol, Expression};
use boa_interner::{Interner, ToInternedString};

/// The `throw` statement throws a user-defined exception.
///
/// Syntax: `throw expression;`
///
/// Execution of the current function will stop (the statements after throw won't be executed),
/// and control will be passed to the first catch block in the call stack. If no catch block
/// exists among caller functions, the program will terminate.
///
/// More information:
///  - [ECMAScript reference][spec]
///  - [MDN documentation][mdn]
///
/// [spec]: https://tc39.es/ecma262/#prod-ThrowStatement
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/throw
#[cfg_attr(feature = "deser", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Throw {
    target: Expression,
}

impl Throw {
    /// Gets the target expression of this `Throw` statement.
    pub fn target(&self) -> &Expression {
        &self.target
    }

    /// Creates a `Throw` AST node.
    pub fn new(target: Expression) -> Self {
        Self { target }
    }

    #[inline]
    pub(crate) fn contains_arguments(&self) -> bool {
        self.target.contains_arguments()
    }

    #[inline]
    pub(crate) fn contains(&self, symbol: ContainsSymbol) -> bool {
        self.target.contains(symbol)
    }
}

impl ToInternedString for Throw {
    fn to_interned_string(&self, interner: &Interner) -> String {
        format!("throw {}", self.target.to_interned_string(interner))
    }
}

impl From<Throw> for Statement {
    fn from(trw: Throw) -> Self {
        Self::Throw(trw)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fmt() {
        crate::syntax::ast::test_formatting(
            r#"
        try {
            throw "hello";
        } catch(e) {
            console.log(e);
        }
        "#,
        );
    }
}

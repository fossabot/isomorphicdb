// Copyright 2020 - present Alex Dukhno
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::{self, Display, Formatter};
use types::SqlTypeFamily;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BiArithmetic {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Comparison {
    NotEq,
    Eq,
    LtEq,
    GtEq,
    Lt,
    Gt,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Bitwise {
    ShiftRight,
    ShiftLeft,
    Xor,
    And,
    Or,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BiLogical {
    Or,
    And,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PatternMatching {
    Like,
    NotLike,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum StringOp {
    Concat,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BiOperation {
    Arithmetic(BiArithmetic),
    Comparison(Comparison),
    Bitwise(Bitwise),
    Logical(BiLogical),
    PatternMatching(PatternMatching),
    StringOp(StringOp),
}

impl BiOperation {
    pub fn resulted_types(&self) -> Vec<SqlTypeFamily> {
        match self {
            BiOperation::Arithmetic(_) => vec![SqlTypeFamily::Integer, SqlTypeFamily::Real],
            BiOperation::Comparison(_) => vec![SqlTypeFamily::Bool],
            BiOperation::Bitwise(_) => vec![SqlTypeFamily::Integer],
            BiOperation::Logical(_) => vec![SqlTypeFamily::Bool],
            BiOperation::PatternMatching(_) => vec![SqlTypeFamily::Bool],
            BiOperation::StringOp(_) => vec![SqlTypeFamily::Bool],
        }
    }

    pub fn supported_type_family(&self, left: Option<SqlTypeFamily>, right: Option<SqlTypeFamily>) -> bool {
        match self {
            BiOperation::Arithmetic(_) => {
                left == Some(SqlTypeFamily::Integer) && right == Some(SqlTypeFamily::Integer)
                    || left == Some(SqlTypeFamily::Real) && right == Some(SqlTypeFamily::Integer)
                    || left == Some(SqlTypeFamily::Integer) && right == Some(SqlTypeFamily::Real)
                    || left == Some(SqlTypeFamily::Real) && right == Some(SqlTypeFamily::Real)
            }
            BiOperation::Comparison(_) => left.is_some() && left == right,
            BiOperation::Bitwise(_) => left == Some(SqlTypeFamily::Integer) && right == Some(SqlTypeFamily::Integer),
            BiOperation::Logical(_) => left == Some(SqlTypeFamily::Bool) && right == Some(SqlTypeFamily::Bool),
            BiOperation::PatternMatching(_) => {
                left == Some(SqlTypeFamily::String) && right == Some(SqlTypeFamily::String)
            }
            BiOperation::StringOp(_) => left == Some(SqlTypeFamily::String) && right == Some(SqlTypeFamily::String),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnOperator {
    Arithmetic(UnArithmetic),
    LogicalNot,
    BitwiseNot,
}

impl Display for UnOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            UnOperator::Arithmetic(op) => write!(f, "{}", op),
            UnOperator::LogicalNot => write!(f, "NOT"),
            UnOperator::BitwiseNot => write!(f, "~"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnArithmetic {
    Neg,
    Pos,
    SquareRoot,
    CubeRoot,
    Factorial,
    Abs,
}

impl Display for UnArithmetic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            UnArithmetic::Neg => write!(f, "-"),
            UnArithmetic::Pos => write!(f, "+"),
            UnArithmetic::SquareRoot => write!(f, "|/"),
            UnArithmetic::CubeRoot => write!(f, "||/"),
            UnArithmetic::Factorial => write!(f, "!"),
            UnArithmetic::Abs => write!(f, "@"),
        }
    }
}

#[cfg(test)]
mod tests;

// Copyright 2020 - 2021 Alex Dukhno
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

use crate::AnalysisError;
use bigdecimal::BigDecimal;
use data_manipulation_operators::{BiOperator, UnOperator};
use data_manipulation_untyped_tree::{StaticUntypedItem, StaticUntypedTree, UntypedValue};
use query_ast::{BinaryOperator, Expr, Value};
use std::str::FromStr;
use types::{Bool, SqlType};

pub(crate) struct StaticTreeBuilder;

impl StaticTreeBuilder {
    pub(crate) fn build_from(root_expr: Expr) -> Result<StaticUntypedTree, AnalysisError> {
        Self::inner_build(root_expr)
    }

    fn inner_build(root_expr: Expr) -> Result<StaticUntypedTree, AnalysisError> {
        match root_expr {
            Expr::Value(value) => Ok(Self::value(value)),
            Expr::Column(name) => Err(AnalysisError::column_cant_be_referenced(name)),
            Expr::BinaryOp { left, op, right } => Self::binary_op(op, *left, *right),
            Expr::Cast { expr, data_type } => Ok(StaticUntypedTree::UnOp {
                op: UnOperator::Cast(SqlType::from(data_type).family()),
                item: Box::new(Self::inner_build(*expr)?),
            }),
            Expr::UnaryOp { op, expr } => Ok(StaticUntypedTree::UnOp {
                op: UnOperator::from(op),
                item: Box::new(Self::inner_build(*expr)?),
            }),
            Expr::Param(index) => Ok(StaticUntypedTree::Item(StaticUntypedItem::Param((index - 1) as usize))),
        }
    }

    fn binary_op(operator: BinaryOperator, left: Expr, right: Expr) -> Result<StaticUntypedTree, AnalysisError> {
        let left = Self::inner_build(left)?;
        let right = Self::inner_build(right)?;
        Ok(StaticUntypedTree::BiOp {
            left: Box::new(left),
            op: BiOperator::from(operator),
            right: Box::new(right),
        })
    }

    fn value(value: Value) -> StaticUntypedTree {
        match value {
            Value::Int(num) => {
                StaticUntypedTree::Item(StaticUntypedItem::Const(UntypedValue::Number(BigDecimal::from(num))))
            }
            Value::String(string) => StaticUntypedTree::Item(StaticUntypedItem::Const(UntypedValue::String(string))),
            Value::Boolean(boolean) => {
                StaticUntypedTree::Item(StaticUntypedItem::Const(UntypedValue::Bool(Bool(boolean))))
            }
            Value::Null => StaticUntypedTree::Item(StaticUntypedItem::Const(UntypedValue::Null)),
            Value::Number(num) => StaticUntypedTree::Item(StaticUntypedItem::Const(UntypedValue::Number(
                BigDecimal::from_str(&num).unwrap(),
            ))),
        }
    }
}

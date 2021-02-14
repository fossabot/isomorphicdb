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

use catalog::Database;
use data_manipulation_query_plan::{
    ConstraintValidator, DeleteQueryPlan, DynamicValues, FullTableScan, InsertQueryPlan, QueryPlan, Repeater,
    SelectQueryPlan, StaticExpressionEval, StaticValues, TableRecordKeys, UpdateQueryPlan,
};
use data_manipulation_typed_queries::TypedQuery;
use data_manipulation_typed_tree::{DynamicTypedItem, DynamicTypedTree};
use std::sync::Arc;

pub struct QueryPlanner<D> {
    database: Arc<D>,
}

impl<D: Database> QueryPlanner<D> {
    pub fn new(database: Arc<D>) -> QueryPlanner<D> {
        QueryPlanner { database }
    }

    pub fn plan(&self, query: TypedQuery) -> QueryPlan {
        match query {
            TypedQuery::Insert(insert) => {
                let table = self.database.table(&insert.full_table_name);
                QueryPlan::Insert(InsertQueryPlan::new(
                    ConstraintValidator::new(
                        StaticExpressionEval::new(StaticValues::from(insert.values)),
                        table.columns(),
                    ),
                    table,
                ))
            }
            TypedQuery::Delete(delete) => {
                let table = self.database.table(&delete.full_table_name);
                QueryPlan::Delete(DeleteQueryPlan::new(
                    TableRecordKeys::new(FullTableScan::new(&*table)),
                    table,
                ))
            }
            TypedQuery::Update(update) => {
                let table = self.database.table(&update.full_table_name);
                QueryPlan::Update(UpdateQueryPlan::new(
                    ConstraintValidator::new(DynamicValues::new(Repeater::new(update.assignments)), table.columns()),
                    FullTableScan::new(&*table),
                    table,
                ))
            }
            TypedQuery::Select(select) => {
                let table = self.database.table(&select.full_table_name);
                QueryPlan::Select(SelectQueryPlan::new(
                    FullTableScan::new(&*table),
                    select
                        .projection_items
                        .into_iter()
                        .map(|item| match item {
                            DynamicTypedTree::Item(DynamicTypedItem::Column(name)) => name,
                            _ => unimplemented!(),
                        })
                        .collect(),
                    table.columns_short(),
                ))
            }
        }
    }
}

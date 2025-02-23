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

use data_manipulation_typed_tree::{DynamicTypedTree, StaticTypedTree};
use types::SqlType;

pub struct TypeCoercion;

impl TypeCoercion {
    pub fn coerce_dynamic(&self, tree: DynamicTypedTree) -> DynamicTypedTree {
        tree
    }

    pub fn coerce_static(&self, tree: StaticTypedTree, _column_type: SqlType) -> StaticTypedTree {
        tree
    }
}

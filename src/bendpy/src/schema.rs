// Copyright 2021 Datafuse Labs
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

use databend_common_expression::DataSchemaRef;
use pyo3::prelude::*;

#[pyclass(name = "Schema", module = "databend", subclass)]

pub struct PySchema {
    pub(crate) schema: DataSchemaRef,
}

#[pymethods]
impl PySchema {
    fn __repr__(&self, _py: Python) -> PyResult<String> {
        Ok(format!("{:?}", self.schema))
    }
}

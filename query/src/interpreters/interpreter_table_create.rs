// Copyright 2020 Datafuse Labs.
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

use std::sync::Arc;

use common_exception::Result;
use common_planners::CreateTablePlan;
use common_streams::DataBlockStream;
use common_streams::SendableDataBlockStream;

use crate::catalogs::Catalog;
use crate::interpreters::Interpreter;
use crate::interpreters::InterpreterPtr;
use crate::sessions::DatabendQueryContextRef;

pub struct CreateTableInterpreter {
    ctx: DatabendQueryContextRef,
    plan: CreateTablePlan,
}

impl CreateTableInterpreter {
    pub fn try_create(
        ctx: DatabendQueryContextRef,
        plan: CreateTablePlan,
    ) -> Result<InterpreterPtr> {
        Ok(Arc::new(CreateTableInterpreter { ctx, plan }))
    }
}

#[async_trait::async_trait]
impl Interpreter for CreateTableInterpreter {
    fn name(&self) -> &str {
        "CreateTableInterpreter"
    }

    async fn execute(
        &self,
        _input_stream: Option<SendableDataBlockStream>,
    ) -> Result<SendableDataBlockStream> {
        let catalog = self.ctx.get_catalog();
        catalog.create_table(self.plan.clone()).await?;

        Ok(Box::pin(DataBlockStream::create(
            self.plan.schema(),
            None,
            vec![],
        )))
    }
}

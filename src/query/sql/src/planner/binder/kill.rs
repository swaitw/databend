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

use databend_common_ast::ast::KillTarget;
use databend_common_exception::Result;

use crate::planner::binder::BindContext;
use crate::planner::binder::Binder;
use crate::plans::KillPlan;
use crate::plans::Plan;

impl Binder {
    #[async_backtrace::framed]
    pub(super) async fn bind_kill_stmt(
        &mut self,
        _bind_context: &BindContext,
        kill_target: &KillTarget,
        object_id: &str,
    ) -> Result<Plan> {
        let kill_connection = matches!(kill_target, KillTarget::Connection);
        let plan = Box::new(KillPlan {
            id: object_id.to_string(),
            kill_connection,
        });

        Ok(Plan::Kill(plan))
    }
}

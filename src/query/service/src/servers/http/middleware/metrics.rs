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

use std::time::Instant;

use databend_common_metrics::http::metrics_incr_http_request_count;
use databend_common_metrics::http::metrics_incr_http_slow_request_count;
use databend_common_metrics::http::metrics_observe_http_response_duration;
use poem::Endpoint;
use poem::IntoResponse;
use poem::Middleware;
use poem::Request;
use poem::Response;

pub struct MetricsMiddleware {
    api: String,
}

impl MetricsMiddleware {
    pub fn new(api: impl Into<String>) -> Self {
        Self { api: api.into() }
    }
}

impl<E: Endpoint> Middleware<E> for MetricsMiddleware {
    type Output = MetricsMiddlewareEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        MetricsMiddlewareEndpoint {
            ep,
            api: self.api.clone(),
        }
    }
}

pub struct MetricsMiddlewareEndpoint<E> {
    api: String,
    ep: E,
}

impl<E: Endpoint> Endpoint for MetricsMiddlewareEndpoint<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> poem::error::Result<Self::Output> {
        let start_time = Instant::now();
        let method = req.method().to_string();
        let output = self.ep.call(req).await?;
        let resp = output.into_response();
        let status_code = resp.status().to_string();
        let duration = start_time.elapsed();
        metrics_incr_http_request_count(method.clone(), self.api.clone(), status_code.clone());
        metrics_observe_http_response_duration(method.clone(), self.api.clone(), duration);
        if duration.as_secs_f64() > 60.0 {
            // TODO: replace this into histogram
            metrics_incr_http_slow_request_count(method, self.api.clone(), status_code);
        }
        Ok(resp)
    }
}

// Copyright 2020 Oxide Computer Company

use dropshot::endpoint;
use dropshot::HttpError;
use dropshot::HttpResponseOk;
use dropshot::RequestContext;
use std::sync::Arc;

#[endpoint {
    method = GET,
    path = "/test",
}]
async fn bad_endpoint(
    _rqctx: Arc<RequestContext>,
    param: String,
) -> Result<HttpResponseOk<()>, HttpError> {
    Ok(HttpResponseOk(()))
}

fn main() {}

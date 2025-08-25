use crate::backend::platforms::AniItemResult;
use crate::backend::ApiResponse;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// CmdFn 表示：接收 String 参数（arg/url），返回一个 boxed future，输出为 Result<ApiResponse<AniItemResult>, String>
pub type CmdFn = Arc<
    dyn Fn(String) -> Pin<Box<dyn Future<Output = Result<ApiResponse<AniItemResult>, String>> + Send>>
    + Send
    + Sync,
>;


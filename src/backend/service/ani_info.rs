use crate::backend::db::postgresql::dao::list_all_ani_info;
use crate::backend::po::Ani;
use crate::client::components::AniData;
use dioxus::prelude::{ServerFnError, server};

#[server(endpoint = "list_ani_info")]
pub async fn list_ani_info(title: String) -> Result<Vec<AniData>, ServerFnError> {
    let query = sqlx::query_as::<_, crate::backend::po::Ani>(
        r#"
                SELECT id,
                    title,
                    update_count,
                    update_info,
                    image_url,
                    detail_url,
                    update_time,
                    platform
                FROM ani_info
                WHERE
                  title LIKE '%' || $1 || '%'
                ORDER BY update_time DESC
            ;"#,
    )
    .bind(title)
    .fetch_all(pool)
    .await
    .map_err(|e| anyhow::anyhow!("query error: {:?}", e))?;
    // 调用通用的 run_query
    let list = crate::backend::db::postgresql::common::run_query(
        crate::backend::db::postgresql::pool::get_pg_pool(),
        query,
    )
    .await?;
    Ok(list)
}

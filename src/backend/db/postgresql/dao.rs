use crate::backend::db::postgresql::common::run_query;
use crate::backend::db::postgresql::get_pg_pool;
use crate::backend::platforms::AniItem;
use crate::backend::po::Ani;
use anyhow::{Context, Result};

/// 动漫信息插入新记录
pub async fn upsert_ani_info(item: &AniItem) -> Result<()> {
    let _ = sqlx::query(
        r#"
        INSERT INTO ani_info (
            title,
            update_count,
            update_info,
            image_url,
            detail_url,
            platform
        ) VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (title, platform, update_count) DO UPDATE SET
            update_info = EXCLUDED.update_info,
            image_url = EXCLUDED.image_url,
            detail_url = EXCLUDED.detail_url
        "#,
    )
    .bind(&item.title)
    .bind(&item.update_count)
    .bind(&item.update_info)
    .bind(&item.image_url)
    .bind(&item.detail_url)
    .bind(&item.platform)
    .execute(get_pg_pool())
    .await
    .map_err(|e| anyhow::anyhow!("插入或更新 ani_info {:?} 失败: {}", item, e))?;

    Ok(())
}

/// 根据 id 查询单条
pub async fn get_ani_info_by_id(id: i64) -> Result<Ani> {
    let rec = sqlx::query_as::<_, Ani>(
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
                  id = ?
            ;"#,
    )
    .bind(id)
    .fetch_optional(get_pg_pool())
    .await?
    .context(format!("查询番剧 ID={} 不存在", id))?;
    Ok(rec)
}

/// 查询所有动漫信息
pub async fn list_all_ani_info() -> Result<Vec<Ani>> {
    // 构造带绑定参数的 QueryAs
    let query = sqlx::query_as::<_, Ani>(
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
                "#,
    );
    // 调用通用的 run_query
    let list = run_query(get_pg_pool(), query).await?;
    Ok(list)
}

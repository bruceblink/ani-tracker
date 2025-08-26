use crate::backend::po::Ani;
use dioxus::prelude::{ServerFnError, server};

#[server(endpoint = "list_ani_info")]
pub async fn list_ani_info() -> Result<Vec<(usize, String)>, ServerFnError> {
    let _sql = sqlx::query_as::<_, Ani>(
        r#"
                SELECT ai.id,
                       ai.title,
                       ai.update_count,
                       ai.update_info,
                       ai.image_url,
                       ai.detail_url,
                       ai.update_time,
                       ai.platform
                FROM ani_info ai
                WHERE ai.update_time >= ?
           ;"#,
    );

    let ani_info_list = vec![
        (1, "Naruto".to_string()),
        (2, "One Piece".to_string()),
        (3, "Attack on Titan".to_string()),
    ];
    Ok(ani_info_list)
}

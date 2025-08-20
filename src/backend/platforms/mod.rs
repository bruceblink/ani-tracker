use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

mod bilibili;
mod agedm;
mod iqiyi;
mod mikanani;
mod tencent;
mod youku;

pub type AniItemResult = HashMap<String, Vec<AniItem>>;

#[derive(Debug, Clone, FromRow, Deserialize, Serialize)]
pub struct AniItem {
    pub title: String,
    pub update_count: String,
    pub update_info: String,
    pub image_url: String,
    pub detail_url: String,
    pub update_time: String,
    pub platform: String,
}
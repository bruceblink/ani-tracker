use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod agedm;
pub mod bilibili;
pub mod iqiyi;
pub mod mikanani;
pub mod tencent;
pub mod youku;

pub type AniItemResult = HashMap<String, Vec<AniItem>>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AniItem {
    pub title: String,
    pub update_count: String,
    pub update_info: String,
    pub image_url: String,
    pub detail_url: String,
    pub update_time: String,
    pub platform: String,
}

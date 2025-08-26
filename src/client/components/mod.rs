//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.

pub mod ani_list;

mod ani_item;
pub mod search;
pub mod theme;

pub use search::Search;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AniData {
    pub title: String,
    pub update_count: String,
    pub update_info: String,
    pub image_url: String,
    pub detail_url: String,
    pub update_time: String,
    pub platform: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct ApiResponse<T = serde_json::Value> {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            status: "ok".into(),
            message: None,
            data: Some(data),
        }
    }

    pub fn err<E: ToString>(msg: E) -> Self {
        Self {
            status: "error".into(),
            message: Some(msg.to_string()),
            data: None,
        }
    }
}

/// 分页数据结构
#[derive(Serialize, Debug)]
pub struct PageData<T> {
    pub items: Vec<T>,  // 当前页的数据
    pub total: usize,   // 总条数
    pub page: i64,      // 当前页码（1开始）
    pub page_size: i64, // 每页数量
}

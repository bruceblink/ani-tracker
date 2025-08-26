//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.

pub mod ani_list;

pub mod search;
mod ani_item;
pub mod theme;

use serde::{Deserialize, Serialize};
pub use search::Search;

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
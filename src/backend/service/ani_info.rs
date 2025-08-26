use crate::backend::db::postgresql::dao::list_all_ani_info;
use crate::backend::po::Ani;
use dioxus::prelude::{ServerFnError, server};

#[server(endpoint = "list_ani_info")]
pub async fn list_ani_info(title: String) -> Result<Vec<Ani>, ServerFnError> {
    let _ani_list = list_all_ani_info(title);

    Ok(_ani_list)
}

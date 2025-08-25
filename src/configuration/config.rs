use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use dioxus::logger::tracing::info;

#[derive(Debug, Deserialize)]
pub struct DataSource {
    pub name: String,
    pub url: String,
    pub cmd: String,
    pub cron_expr: String,
    pub retry_times: u8,
}

// 不再需要 DataSourceCategory 结构体
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub datasource: HashMap<String, Vec<DataSource>>, // 直接映射到 Vec<DataSource>
}

// 读取配置文件
pub fn load_configuration(config_path: PathBuf) -> Result<AppConfig, config::ConfigError> {
    // 读取配置文件目录
    let configuration_directory = config_path;
    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("config.yaml".to_string()),
        ))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;
    settings.try_deserialize::<AppConfig>()
}


/// 初始化应用配置
pub fn init_config(app_path: PathBuf) -> std::io::Result<PathBuf> {

    let config_path = app_path.join("conf".to_string());
    // 配置文件的目标路径
    let target_config_path = config_path.join("config.yaml".to_string());

    // 检查配置文件是否已存在，如果不存在则复制
    if !target_config_path.exists() {
        let config_file_in_resources = config_path.join("./conf/config.yaml".to_string());

        // 复制配置文件到目标目录
        if !config_file_in_resources.exists() {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Config file not found in resources"));
        }

        // 复制文件
        fs::create_dir_all(config_path.clone())?; // 如果目标目录不存在则创建
        fs::copy(config_file_in_resources, target_config_path)?;
        info!("配置文件已复制到目标目录");
    } else {
        info!("配置文件已存在");
    }

    Ok(config_path)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configuration_folder() {
        let base_path = std::env::current_dir().expect("Failed to determine the current directory");
        assert!(base_path.ends_with("ani-tracker".to_string()));
    }

    #[test]
    fn test_configuration_content() {
        let current_dir = std::env::current_dir().expect("Failed to determine the current directory");
        let config_path = init_config(current_dir).expect("Failed to init conf.");
        let configuration = load_configuration(config_path.clone()).expect("Failed to read conf.");
        println!("{:?}", configuration);
        assert_eq!(configuration.datasource.len(), 2);

        let configuration = load_configuration(config_path).expect("Failed to read conf.");
        println!("{:#?}", configuration);

        // 验证 anime 分类
        let anime_sources = configuration
            .datasource
            .get("anime")
            .expect("Missing anime category");
        assert_eq!(anime_sources.len(), 6);
        assert_eq!(anime_sources[0].name, "哔哩哔哩国创".to_string());
        assert_eq!(
            anime_sources[0].url,
            "https://api.bilibili.com/pgc/web/timeline?types=4&before=6&after=6".to_string()
        );
        assert_eq!(anime_sources[0].cmd, "fetch_bilibili_ani_data".to_string());

        // 验证 drama 分类
        let drama_sources = configuration
            .datasource
            .get("drama")
            .expect("Missing drama category");
        assert_eq!(drama_sources.len(), 1);
        assert_eq!(drama_sources[0].name, "腾讯视频".to_string());
    }
}

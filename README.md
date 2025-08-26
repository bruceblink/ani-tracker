# 追播 (ani-tracker) - 你的个人追番助手 🎬

## 项目简介

该项目为：[ani-todo-app](https://github.com/bruceblink/ani-todo-app) 项目使用Rust的Dioxus重新实现的全栈版本(
包含Web端和桌面端)[ani-tracker](https://ani-tracker.fly.dev/)，旨在提供一个高效、易用的个人追番助手。它可以帮助用户追踪各大视频平台的动漫更新信息，并提供便捷的查询和管理功能。

## 功能特点

- 🔄 定时更新：自动抓取各大视频平台的最新更新信息
- 📺 多平台支持：目前支持蜜柑计划、腾讯视频、哔哩哔哩、爱奇艺和优酷等 后续将支持更多视频平台
- 💾 数据存储：所有信息都存储在服务端，便于方便查询、管理和同步

## 系统要求

- Rust 1.88
- 请查看 `Cargo.toml` 了解所需依赖包

## 安装步骤

1. 克隆仓库到本地：
    ```bash
        git clone https://github.com/bruceblink/ani-tracker
        cd ani-tracker
    ```

2. 安装 dioxus-cli：
    ```bash
       cargo install cargo-binstall
       cargo binstall dioxus-cli
    ```
3. 开发环境运行项目：
   运行以下命令启动开发服务器(Web端应用)：
    ```bash
       dx serve --platform web
   ```
   运行其他平台, 使用 `--platform platform` 标志. 例如运行桌面端应用：
   ```bash
      dx serve --platform desktop
   ```
4. 构建生产版本：
    ```bash
       dx build --platform web --release 
   ```
   构建其他平台, 使用 `--platform platform` 标志. 例如构建桌面端应用：
   ```bash
      dx build --platform desktop --release
   ```

5. 部署到 Fly.io：
   使用Github账号登录到[Fly.io](https://fly.io/dashboard/likanug/new)选择你要部署代码仓库即可，一键部署应用

## 项目文件说明

```txt
ani-tracker/
├── .github/                      存放通用工具和基础模块
│   └── workflows                 Github Actions工作流配置
│       └── fly-deploy.yml        Fly.io部署工作流
├── assets/                       存放静态资源
│   ├── styling                   样式文件夹
│   │   └── main.css              主样式文件
│   │   └── ..... 
│   └── favico.ico                网站图标
├── src/                          源代码目录
│   └── main.rs                   主程序入口文件
├── Dockerfile                    用于构建项目Docker镜像的配置文件
├── index.html                    网站首页HTML模板文件
├── README.md                     项目说明文档
├── Cargo.toml                    Rust项目的配置文件，定义依赖和元数据
├── Dioxus.toml                   Dioxus项目配置文件，定义Dioxus相关设置
├── fly.toml                      Fly.io部署配置文件
├── LICENSE                       项目许可证文件
```

## 贡献指南

欢迎对项目做出贡献！如果你有任何建议或发现了bug，请：

1. Fork 本仓库
2. 创建新的分支
3. 提交你的修改
4. 发起 Pull Request

## 未来计划

- [ ] 支持更多视频平台
- [ ] 📅 每日更新提醒：及时获取最新剧集更新信息
- [ ] 🎯 个性化追踪：可以根据个人喜好设置关注的节目
- [ ] 添加图形用户界面（GUI）
- [ ] 添加订阅提醒功能
- [ ] 支持自定义过滤器
- [ ] 添加导出功能

## 许可证

本项目采用 MIT 许可证 - 详情请查看 [LICENSE](LICENSE) 文件

## 联系方式

如有任何问题或建议，欢迎通过以下方式联系：

- 提交 [Issue](https://github.com/bruceblink/ani-tracker/issues)
- [发送邮件](mailto:likanug.g@qq.com)

## 致谢

感谢所有为本项目做出贡献的开发者和用户。

---

**注意**：本项目仅用于个人学习和研究使用，请勿用于任何商业用途。在使用过程中请遵守相关网站的使用条款和规定。

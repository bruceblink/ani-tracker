use dioxus::prelude::*;
#[cfg(target_family = "wasm")]
use web_sys::window;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl From<&str> for ThemeMode {
    fn from(s: &str) -> Self {
        match s {
            "dark" => ThemeMode::Dark,
            _ => ThemeMode::Light,
        }
    }
}

// 提取一个函数来返回主题按钮的 SVG
fn get_theme_toggle_svg(theme: ThemeMode) -> &'static str {
    match theme {
        ThemeMode::Light => {
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-moon"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path></svg>"#
        }
        ThemeMode::Dark => {
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-sun"><circle cx="12" cy="12" r="5"></circle><line x1="12" y1="1" x2="12" y2="3"></line><line x1="12" y1="21" x2="12" y2="23"></line><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line><line x1="1" y1="12" x2="3" y2="12"></line><line x1="21" y1="12" x2="23" y2="12"></line><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line></svg>"#
        }
    }
}

#[component]
pub fn ThemeToggle() -> Element {
    let mut theme_mode = use_signal(|| ThemeMode::Light);

    #[cfg(target_family = "wasm")]
    {
        // 初始化从 localStorage 读取主题
        use_effect({
            let mut theme_mode = theme_mode.clone();
            move || {
                if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
                    if let Some(saved_theme) = storage.get_item("theme").ok().flatten() {
                        theme_mode.set(ThemeMode::from(saved_theme.as_str()));
                    }
                }
            }
        });

        // 主题变化时，更新 <html> 的 class 及 localStorage
        use_effect({
            let theme_mode = theme_mode.clone();
            move || {
                let current_theme_str = match *theme_mode.read() {
                    ThemeMode::Light => "light",
                    ThemeMode::Dark => "dark",
                };

                if let Some(document) = window().and_then(|w| w.document()) {
                    let html = document
                        .document_element()
                        .expect("should have html element");
                    let class_list = html.class_list();

                    if current_theme_str == "dark" {
                        let _ = class_list.add_1("dark");
                    } else {
                        let _ = class_list.remove_1("dark");
                    }
                }

                if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
                    let _ = storage.set_item("theme", current_theme_str);
                }
            }
        });
    }

    let on_toggle_theme = move |_| {
        // 1. 先对 theme_mode 进行不可变借用，并获取当前值
        let current_theme = *theme_mode.read();

        // 2. 根据当前值，计算出新的主题模式
        let new_theme = match current_theme {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        };

        // 3. 此时，对 theme_mode 的不可变借用已经结束，可以安全地进行可变借用并更新值
        theme_mode.set(new_theme);
    };

    rsx! {
        button {
            class: "theme-toggle-button",
            onclick: on_toggle_theme,
            // 使用 dangerous_inner_html 渲染 SVG
            dangerous_inner_html: get_theme_toggle_svg(*theme_mode.read()),
        }
    }
}

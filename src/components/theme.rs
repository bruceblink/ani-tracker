use dioxus::prelude::*;
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

// 提取一个函数来返回主题按钮的文本
fn get_theme_toggle_text(theme: ThemeMode) -> &'static str {
    match theme {
        ThemeMode::Light => "🌙",
        ThemeMode::Dark => "☀️",
    }
}

#[component]
pub fn ThemeToggle() -> Element {
    let mut theme_mode = use_signal(|| ThemeMode::Light);

    use_effect(move || {
        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
            if let Some(saved_theme) = storage.get_item("theme").ok().flatten() {
                theme_mode.set(ThemeMode::from(saved_theme.as_str()));
            }
        }
    });

    use_effect(move || {
        let current_theme_str = match *theme_mode.read() {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        };

        if let Some(document) = window().and_then(|w| w.document()) {
            let body = document.body().expect("document should have a body");
            let _ = body.set_attribute("data-theme", current_theme_str);
        }

        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
            let _ = storage.set_item("theme", current_theme_str);
        }
    });

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
            // 修正后的代码：调用函数来获取字符串
            "{get_theme_toggle_text(*theme_mode.read())}"
        }
    }
}
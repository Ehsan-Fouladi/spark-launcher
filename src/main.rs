#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod actions;
mod indexer;
mod prelude;

use crate::prelude::*;

slint::include_modules!();

fn center_window(ui: &LauncherWindow) {
    ui.window().with_winit_window(|winit_window| {
        let monitor = winit_window
            .current_monitor()
            .or_else(|| winit_window.primary_monitor());

        if let Some(monitor) = monitor {
            let monitor_pos = monitor.position();
            let monitor_size = monitor.size();
            let window_size = winit_window.outer_size();

            let x = monitor_pos.x + (monitor_size.width as i32 - window_size.width as i32) / 2;
            let y = monitor_pos.y + (monitor_size.height as i32 - window_size.height as i32) / 2;

            winit_window.set_outer_position(
                i_slint_backend_winit::winit::dpi::PhysicalPosition::new(x, y),
            );
        }
    });
}

fn format_number(v: f64) -> String {
    if v.fract() == 0.0 && v.abs() < 1e15 {
        format!("{}", v as i64)
    } else {
        let s = format!("{:.6}", v);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

fn calc_inline(q: &str) -> String {
    let has_digit = q.chars().any(|c| c.is_ascii_digit());
    let has_op = q.chars().any(|c| "+-*/%^".contains(c));
    if q.is_empty() || !has_digit || !has_op {
        return String::new();
    }

    match meval::eval_str(q) {
        Ok(v) if v.is_finite() => {
            let r = format_number(v);
            if r == q { String::new() } else { r }
        }
        _ => String::new(),
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = LauncherWindow::new()?;
    
    {
        let ui_weak = ui.as_weak();
        slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_weak.upgrade() {
                center_window(&ui);
                ui.window().with_winit_window(|winit_window| {
                    let _ = window_vibrancy::apply_acrylic(winit_window, Some((18, 18, 22, 20)));

                    if let Ok(handle) = winit_window.window_handle() {
                        if let RawWindowHandle::Win32(win32_handle) = handle.as_raw() {
                            let hwnd = HWND(win32_handle.hwnd.get() as _);
                            let preference = DWMWCP_ROUND;
                            unsafe {
                                let _ = DwmSetWindowAttribute(
                                    hwnd,
                                    DWMWA_WINDOW_CORNER_PREFERENCE,
                                    &preference as *const _ as *const _,
                                    std::mem::size_of_val(&preference) as u32,
                                );
                            }
                        }
                    }
                });
            }
        })
        .unwrap();
    }

    let all_items = Rc::new(index_system_items());
    let current_actions: Rc<RefCell<Vec<ActionType>>> = Rc::new(RefCell::new(Vec::new()));
    let matcher = SkimMatcherV2::default();

    let update_results = {
        let ui_weak = ui.as_weak();
        let all_items = all_items.clone();
        let current_actions = current_actions.clone();

        move |query: SharedString| {
            let Some(ui) = ui_weak.upgrade() else { return };
            let q = query.trim();

            ui.set_inline_result(calc_inline(q).into());

            let mut matched: Vec<(i64, SearchEntry)> = Vec::new();

            if q.is_empty() {
                for item in all_items.iter().take(6) {
                    matched.push((0, item.clone()));
                }
            } else {
                for item in all_items.iter() {
                    if let Some(score) = matcher.fuzzy_match(&item.title, q) {
                        matched.push((score, item.clone()));
                    }
                }
            }

            matched.sort_by(|a, b| b.0.cmp(&a.0));
            matched.truncate(6);

            if !q.is_empty() {
                let search_query = q.strip_prefix("g ").unwrap_or(q);
                matched.push((
                    -1,
                    SearchEntry {
                        title: format!("جستجوی «{}» در گوگل", search_query),
                        subtitle: "باز کردن در مرورگر".to_string(),
                        category: "WEB".to_string(),
                        action: ActionType::OpenUrl(format!(
                            "https://www.google.com/search?q={}",
                            search_query.replace(' ', "+")
                        )),
                    },
                ));
            }

            let mut slint_items = Vec::new();
            let mut actions = Vec::new();

            for (_, entry) in matched {
                slint_items.push(ResultItem {
                    title: entry.title.into(),
                    subtitle: entry.subtitle.into(),
                    category: entry.category.into(),
                });
                actions.push(entry.action);
            }

            *current_actions.borrow_mut() = actions;
            ui.set_results(ModelRc::new(VecModel::from(slint_items)));
            ui.set_selected_index(0);
        }
    };

    update_results(SharedString::new());
    ui.on_query_changed(update_results);

    {
        let ui_weak = ui.as_weak();
        let current_actions = current_actions.clone();
        ui.on_execute_selected(move || {
            let Some(ui) = ui_weak.upgrade() else { return };
            let idx = ui.get_selected_index() as usize;
            let inline = ui.get_inline_result();

            if !inline.is_empty() && idx == 0 {
                ActionType::CopyText(inline.to_string()).execute();
            } else if let Some(action) = current_actions.borrow().get(idx) {
                action.execute();
            }

            ui.set_search_text(SharedString::new());
            ui.set_inline_result(SharedString::new());
            let _ = ui.hide();
        });
    }

    {
        let ui_weak = ui.as_weak();
        ui.on_hide_window(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let _ = ui.hide();
            }
        });
    }

    let manager = GlobalHotKeyManager::new().unwrap();
    let hotkey = HotKey::new(Some(Modifiers::ALT | Modifiers::SHIFT), Code::Space);
    manager.register(hotkey).unwrap();

    let timer = slint::Timer::default();
    let ui_weak = ui.as_weak();
    timer.start(
        slint::TimerMode::Repeated,
        std::time::Duration::from_millis(30),
        move || {
            if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
                if event.state == HotKeyState::Pressed {
                    if let Some(ui) = ui_weak.upgrade() {
                        if ui.window().is_visible() {
                            let _ = ui.hide();
                        } else {
                            center_window(&ui);
                            let _ = ui.show();
                            ui.invoke_focus_input();
                        }
                    }
                }
            }
        },
    );

    ui.run()
}

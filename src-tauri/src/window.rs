//! macOS window customization
//! 通过 NSLayoutConstraint 精确控制交通灯（traffic light）按钮的位置，
//! 使其与 TopBar (h-14 = 56px) 垂直居中对齐。

/// 交通灯左边距
#[cfg(target_os = "macos")]
const PAD_X: f64 = 12.0;

/// 交通灯顶部偏移 = (56 - 14) / 2 = 21，使按钮在 56px TopBar 中垂直居中
#[cfg(target_os = "macos")]
const PAD_Y: f64 = 21.0;

/// 按钮中心间距（macOS 标准约 20px）
#[cfg(target_os = "macos")]
const BUTTON_SPACING: f64 = 20.0;

/// 按钮尺寸
#[cfg(target_os = "macos")]
const BUTTON_SIZE: f64 = 14.0;

/// 约束标识符前缀
#[cfg(target_os = "macos")]
const CONSTRAINT_ID: &str = "Intento_TrafficLight_Top";

/// 设置 macOS 交通灯按钮位置
#[cfg(target_os = "macos")]
pub fn setup_traffic_light_position(window: &tauri::WebviewWindow) {
    use objc2_app_kit::{
        NSLayoutAttribute, NSLayoutConstraint, NSLayoutRelation,
        NSView, NSWindow, NSWindowButton,
    };
    use objc2_foundation::{NSArray, NSString};

    // 获取 NSWindow 指针
    let ns_window_ptr = match window.ns_window() {
        Ok(ptr) => ptr,
        Err(e) => {
            eprintln!("Failed to get NSWindow: {}", e);
            return;
        }
    };

    unsafe {
        let ns_window: &NSWindow = &*(ns_window_ptr as *const NSWindow);

        // 收集交通灯按钮
        let mut buttons = Vec::new();
        if let Some(btn) = ns_window.standardWindowButton(NSWindowButton::CloseButton) {
            buttons.push(btn);
        }
        if let Some(btn) = ns_window.standardWindowButton(NSWindowButton::MiniaturizeButton) {
            buttons.push(btn);
        }
        if let Some(btn) = ns_window.standardWindowButton(NSWindowButton::ZoomButton) {
            buttons.push(btn);
        }

        if buttons.is_empty() {
            eprintln!("No traffic light buttons found");
            return;
        }

        // 获取标题栏视图（第一个按钮的父视图）
        let title_bar_view_retained = match buttons[0].superview() {
            Some(view) => view,
            None => {
                eprintln!("Failed to get title bar view");
                return;
            }
        };
        let title_bar_view: &NSView = &title_bar_view_retained;

        // 检查是否已有我们的约束（避免重复添加）
        let existing = title_bar_view.constraints();
        for constraint in existing.iter() {
            if let Some(identifier) = constraint.identifier() {
                if identifier.to_string() == CONSTRAINT_ID {
                    // 约束已存在，更新常量值即可
                    constraint.setConstant(PAD_Y);
                    return;
                }
            }
        }

        // 为每个按钮设置约束
        for (i, button) in buttons.iter().enumerate() {
            // 禁用 autoresizing mask 转换
            button.setTranslatesAutoresizingMaskIntoConstraints(false);

            let btn_x = PAD_X + (i as f64 * BUTTON_SPACING);

            // Left 约束
            let left = NSLayoutConstraint::constraintWithItem_attribute_relatedBy_toItem_attribute_multiplier_constant(
                button,
                NSLayoutAttribute::Left,
                NSLayoutRelation::Equal,
                Some(title_bar_view),
                NSLayoutAttribute::Left,
                1.0,
                btn_x,
            );

            // Top 约束
            let top = NSLayoutConstraint::constraintWithItem_attribute_relatedBy_toItem_attribute_multiplier_constant(
                button,
                NSLayoutAttribute::Top,
                NSLayoutRelation::Equal,
                Some(title_bar_view),
                NSLayoutAttribute::Top,
                1.0,
                PAD_Y,
            );
            // 设置标识符以便后续查找和更新
            top.setIdentifier(Some(&NSString::from_str(CONSTRAINT_ID)));

            // Width 约束
            let width = NSLayoutConstraint::constraintWithItem_attribute_relatedBy_toItem_attribute_multiplier_constant(
                button,
                NSLayoutAttribute::Width,
                NSLayoutRelation::Equal,
                None,
                NSLayoutAttribute::NotAnAttribute,
                1.0,
                BUTTON_SIZE,
            );

            // Height 约束
            let height = NSLayoutConstraint::constraintWithItem_attribute_relatedBy_toItem_attribute_multiplier_constant(
                button,
                NSLayoutAttribute::Height,
                NSLayoutRelation::Equal,
                None,
                NSLayoutAttribute::NotAnAttribute,
                1.0,
                BUTTON_SIZE,
            );

            // 添加约束到标题栏视图
            let constraints = NSArray::from_retained_slice(&[left, top, width, height]);
            title_bar_view.addConstraints(&constraints);
        }

        println!("macOS traffic light buttons positioned successfully");
    }
}

/// 非 macOS 平台空实现
#[cfg(not(target_os = "macos"))]
pub fn setup_traffic_light_position(_window: &tauri::WebviewWindow) {
    // No-op on non-macOS platforms
}

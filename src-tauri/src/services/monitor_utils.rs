//! 多显示器工具模块
//! 提供跨平台的活动屏幕获取功能

/// 显示器信息（macOS 原生坐标，左下角原点）
#[derive(Debug, Clone)]
pub struct MonitorInfo {
    /// 屏幕左下角 X 坐标（macOS 原生坐标）
    pub x: f64,
    /// 屏幕左下角 Y 坐标（macOS 原生坐标）
    pub y: f64,
    /// 屏幕宽度
    pub width: f64,
    /// 屏幕高度
    pub height: f64,
}

/// 获取当前活动屏幕的显示器信息（返回 macOS 原生坐标）
pub fn get_active_monitor() -> Option<MonitorInfo> {
    #[cfg(target_os = "macos")]
    {
        get_active_monitor_macos()
    }

    #[cfg(target_os = "windows")]
    {
        get_active_monitor_windows()
    }

    #[cfg(target_os = "linux")]
    {
        None
    }
}

// ============================================================================
// macOS 实现
// ============================================================================

#[cfg(target_os = "macos")]
fn get_active_monitor_macos() -> Option<MonitorInfo> {
    use cocoa::base::nil;
    use cocoa::foundation::{NSPoint, NSRect, NSUInteger};
    use objc::{class, msg_send, sel, sel_impl};

    unsafe {
        // 获取当前鼠标位置（macOS 坐标系：左下角原点）
        let mouse_location: NSPoint = msg_send![class!(NSEvent), mouseLocation];

        // 获取所有屏幕
        let screens: *mut objc::runtime::Object = msg_send![class!(NSScreen), screens];
        if screens == nil {
            return None;
        }

        let count: NSUInteger = msg_send![screens, count];
        if count == 0 {
            return None;
        }

        // 遍历所有屏幕，找到包含鼠标位置的屏幕
        for i in 0..count {
            let screen: *mut objc::runtime::Object = msg_send![screens, objectAtIndex: i];
            let frame: NSRect = msg_send![screen, frame];

            // 检查鼠标是否在这个屏幕内
            if mouse_location.x >= frame.origin.x
                && mouse_location.x < frame.origin.x + frame.size.width
                && mouse_location.y >= frame.origin.y
                && mouse_location.y < frame.origin.y + frame.size.height
            {
                // 找到了包含鼠标的屏幕，返回原生坐标
                return Some(MonitorInfo {
                    x: frame.origin.x,
                    y: frame.origin.y,
                    width: frame.size.width,
                    height: frame.size.height,
                });
            }
        }

        // 如果没找到，使用主屏幕
        let primary_screen: *mut objc::runtime::Object = msg_send![screens, objectAtIndex: 0usize];
        let frame: NSRect = msg_send![primary_screen, frame];
        Some(MonitorInfo {
            x: frame.origin.x,
            y: frame.origin.y,
            width: frame.size.width,
            height: frame.size.height,
        })
    }
}

/// macOS: 使用原生 API 设置窗口位置并显示（避免闪烁）
#[cfg(target_os = "macos")]
pub fn set_window_position_and_show(
    ns_window: *mut objc::runtime::Object,
    monitor: &MonitorInfo,
    window_width: f64,
    window_height: f64,
) {
    use cocoa::base::{id, nil, NO, YES};
    use cocoa::foundation::{NSPoint, NSRect, NSSize};
    use objc::{msg_send, sel, sel_impl};

    unsafe {
        // 计算窗口在目标屏幕上的居中位置（macOS 原生坐标：左下角原点）
        let x = monitor.x + (monitor.width - window_width) / 2.0;
        // Y 坐标：屏幕底部 + 屏幕高度的 2/3 处（因为窗口原点在左下角）
        let y = monitor.y + (monitor.height - window_height) * 2.0 / 3.0;

        let frame = NSRect {
            origin: NSPoint { x, y },
            size: NSSize {
                width: window_width,
                height: window_height,
            },
        };

        // 使用 setFrame:display:animate: 直接设置窗口位置和大小
        // display: YES 表示立即重绘
        // animate: NO 表示不使用动画
        let _: () = msg_send![ns_window as id, setFrame:frame display:YES animate:NO];

        // 显示窗口并置于最前
        let _: () = msg_send![ns_window as id, makeKeyAndOrderFront:nil];
    }
}

// ============================================================================
// Windows 实现
// ============================================================================

#[cfg(target_os = "windows")]
fn get_active_monitor_windows() -> Option<MonitorInfo> {
    use std::mem::zeroed;
    use winapi::um::winuser::{
        GetForegroundWindow, GetMonitorInfoW, MonitorFromWindow, MONITORINFO,
        MONITOR_DEFAULTTOPRIMARY,
    };

    unsafe {
        // 获取前台窗口句柄
        let foreground_hwnd = GetForegroundWindow();

        // 获取窗口所在的显示器
        let monitor = MonitorFromWindow(foreground_hwnd, MONITOR_DEFAULTTOPRIMARY);
        if monitor.is_null() {
            return None;
        }

        // 获取显示器信息
        let mut monitor_info: MONITORINFO = zeroed();
        monitor_info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;

        if GetMonitorInfoW(monitor, &mut monitor_info) == 0 {
            return None;
        }

        // 使用工作区域（排除任务栏）
        let rc = monitor_info.rcWork;

        Some(MonitorInfo {
            x: rc.left as f64,
            y: rc.top as f64,
            width: (rc.right - rc.left) as f64,
            height: (rc.bottom - rc.top) as f64,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_active_monitor() {
        let monitor = get_active_monitor();
        println!("Active monitor: {:?}", monitor);
    }
}

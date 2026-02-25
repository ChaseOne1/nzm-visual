use windows::Win32::Foundation::{HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, GetClassNameW, GetWindowTextW};
use windows::core::{BOOL};

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, _lparam: LPARAM) -> BOOL {
    // 缓冲区清零以避免未初始化内存风险
    let mut title = [0u16; 256];
    let mut class_name = [0u16; 256];

    // 获取窗口标题，只处理成功且长度 > 0 的情况
    let title_len = unsafe { GetWindowTextW(hwnd, &mut title) };
    let title_str = if title_len > 0 {
        String::from_utf16_lossy(&title[..title_len as usize])
    } else {
        String::new()
    };

    // 获取窗口类名
    let class_len = unsafe { GetClassNameW(hwnd, &mut class_name) };
    let class_str = if class_len > 0 {
        String::from_utf16_lossy(&class_name[..class_len as usize])
    } else {
        String::new()
    };

    if title_str.contains("GTA") || class_str.contains("GTA") {
        println!("Found window:");
        println!("  HWND: {:?}", hwnd);
        println!("  Title: {}", title_str);
        println!("  Class: {}", class_str);
    }

    BOOL(1)
}

fn main() {
    unsafe {
        let _ = EnumWindows(Some(enum_windows_proc), LPARAM(0));
    }
}

use windows::{
    Win32::{
        Foundation::*, Graphics::Gdi::*, System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::*,
    },
    core::w,
};

pub struct Overlayer {
    hwnd: HWND,

    width: i32,
    height: i32,

    screen_dc: HDC,
    mem_dc: HDC,
    bitmap: HBITMAP,
    bits: *mut u8,
}

impl Overlayer {
    pub fn initialize() {
        unsafe {
            let class_name = w!("overlay-window");

            let wc = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                lpfnWndProc: Some(Self::procedure),
                hInstance: GetModuleHandleW(None).unwrap().into(),
                lpszClassName: class_name,
                ..Default::default()
            };

            RegisterClassExW(&wc);
        }
    }

    unsafe extern "system" fn procedure(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
    }

    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        unsafe {
            let class_name = w!("overlay-window");

            let hwnd = CreateWindowExW(
                WS_EX_LAYERED | WS_EX_TOPMOST | WS_EX_NOACTIVATE | WS_EX_TRANSPARENT,
                class_name,
                w!(""),
                WS_POPUP,
                x,
                y,
                width,
                height,
                None,
                None,
                Some(GetModuleHandleW(None).unwrap().into()),
                None,
            )
            .unwrap();

            let screen_dc = GetDC(None);
            let mem_dc = CreateCompatibleDC(Some(screen_dc));

            let mut bmi = BITMAPINFO::default();
            bmi.bmiHeader = BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width,
                biHeight: -height,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            };

            let mut bits: *mut std::ffi::c_void = std::ptr::null_mut();

            let bitmap =
                CreateDIBSection(Some(mem_dc), &bmi, DIB_RGB_COLORS, &mut bits, None, 0).unwrap();

            SelectObject(mem_dc, bitmap.into());

            let _ = ShowWindow(hwnd, SW_SHOW);

            Self {
                hwnd,
                width,
                height,
                screen_dc,
                mem_dc,
                bitmap,
                bits: bits as *mut u8,
            }
        }
    }

    pub fn tick(
        &self,
        current_money: u32,
        current_money_score: u32,
        current_attack: u16,
        current_attack_score: u32,
        future_money: u32,
        future_attack: u16,
        duration: std::time::Duration,
    ) {
        unsafe {
            std::ptr::write_bytes(self.bits, 0, (self.width * self.height * 4) as usize);

            SetBkMode(self.mem_dc, TRANSPARENT);
            SetTextColor(self.mem_dc, COLORREF(0x00FFFFFF));

            let text = format!(
                "当前: 金币:{}({}) 攻击力:{}({})\n\n最终: 金币:{} 攻击力:{}\n\n耗时: {:?}",
                current_money, current_money_score, current_attack, current_attack_score,
                future_money, future_attack, duration
            );
            let mut text_wide: Vec<u16> = text.encode_utf16().collect();

            let mut rect = RECT {
                left: 10,
                top: 10,
                right: self.width - 10,
                bottom: self.height - 10,
            };

            let _ = DrawTextW(
                self.mem_dc,
                &mut text_wide,
                &mut rect,
                DT_LEFT | DT_TOP,
            );

            let blend = BLENDFUNCTION {
                BlendOp: AC_SRC_OVER as u8,
                BlendFlags: 0,
                SourceConstantAlpha: 255,
                AlphaFormat: AC_SRC_ALPHA as u8,
            };

            let size = SIZE {
                cx: self.width,
                cy: self.height,
            };

            let pt_src = POINT { x: 0, y: 0 };

            let _ = UpdateLayeredWindow(
                self.hwnd,
                Some(self.screen_dc),
                None,
                Some(&size),
                Some(self.mem_dc),
                Some(&pt_src),
                COLORREF(0),
                Some(&blend),
                ULW_ALPHA,
            );
        }
    }
}

impl Drop for Overlayer {
    fn drop(&mut self) {
        unsafe {
            let _ = DeleteObject(self.bitmap.into());
            let _ = DeleteDC(self.mem_dc);
            ReleaseDC(None, self.screen_dc);
            let _ = DestroyWindow(self.hwnd);
        }
    }
}

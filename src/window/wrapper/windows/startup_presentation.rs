/// Keep the initial maximized window composed but invisible until its first frame.
/// The guard also restores visibility if initialization exits early.
pub struct StartupPresentation {
    window: std::rc::Rc<winit::window::Window>,
    hwnd: windows::Win32::Foundation::HWND,
}

impl StartupPresentation {
    pub fn prepare(window: std::rc::Rc<winit::window::Window>) -> Option<Self> {
        use raw_window_handle::HasWindowHandle;
        use raw_window_handle::RawWindowHandle;
        use windows::Win32::Graphics::Dwm::DWMWA_CLOAK;
        use windows::Win32::Graphics::Dwm::DWMWA_TRANSITIONS_FORCEDISABLED;
        use windows::Win32::Graphics::Dwm::DwmSetWindowAttribute;

        let RawWindowHandle::Win32(handle) = window.window_handle().ok()?.as_raw() else {
            return None;
        };
        let hwnd = windows::Win32::Foundation::HWND(handle.hwnd.get() as *mut _);
        let enabled = 1_i32;
        let result = unsafe {
            DwmSetWindowAttribute(
                hwnd,
                DWMWA_CLOAK,
                &enabled as *const _ as _,
                std::mem::size_of_val(&enabled) as _,
            )
        };
        if let Err(error) = result {
            log::warn!("startup presentation: cannot cloak window: {error}");
            return None;
        }
        let guard = Self { window, hwnd };
        let result = unsafe {
            DwmSetWindowAttribute(
                hwnd,
                DWMWA_TRANSITIONS_FORCEDISABLED,
                &enabled as *const _ as _,
                std::mem::size_of_val(&enabled) as _,
            )
        };
        if let Err(error) = result {
            log::warn!("startup presentation: cannot disable transitions: {error}");
            return None;
        }
        // Compute the native maximized client size before constructing the renderer.
        guard.window.set_visible(true);
        log::info!(
            "startup presentation: prepared {:?}, maximized={}",
            guard.window.inner_size(),
            guard.window.is_maximized()
        );
        Some(guard)
    }
}

impl Drop for StartupPresentation {
    fn drop(&mut self) {
        use windows::Win32::Graphics::Dwm::DWMWA_CLOAK;
        use windows::Win32::Graphics::Dwm::DWMWA_TRANSITIONS_FORCEDISABLED;
        use windows::Win32::Graphics::Dwm::DwmFlush;
        use windows::Win32::Graphics::Dwm::DwmSetWindowAttribute;
        let disabled = 0_i32;
        unsafe {
            // Complete the submitted frame before exposing the DWM surface.
            let _ = DwmFlush();
            if let Err(error) = DwmSetWindowAttribute(
                self.hwnd,
                DWMWA_CLOAK,
                &disabled as *const _ as _,
                std::mem::size_of_val(&disabled) as _,
            ) {
                log::error!("startup presentation: cannot uncloak window: {error}");
            }
            let _ = DwmFlush();
            let _ = DwmSetWindowAttribute(
                self.hwnd,
                DWMWA_TRANSITIONS_FORCEDISABLED,
                &disabled as *const _ as _,
                std::mem::size_of_val(&disabled) as _,
            );
        }
        self.window.focus_window();
        log::info!(
            "startup presentation: revealed {:?}, maximized={}",
            self.window.inner_size(),
            self.window.is_maximized()
        );
    }
}

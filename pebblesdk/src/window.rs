use pebblesdk_sys::{self as sys, GColor8};
extern crate alloc;

use alloc::boxed::Box;

use crate::graphics::GGCOLOR_WHITE_ARGB8;

fn create_window() -> Box<*mut sys::Window> {
    let window = unsafe {
        let window = sys::window_create();
        if window.is_null() {
            panic!("Failed to create window");
        }
        window
    };
    Box::new(window)
}
pub struct Window {
    window: Box<*mut sys::Window>,
    background_color: GColor8,
    click_config: sys::ClickConfigProvider,
    window_handlers: sys::WindowHandlers,
}
impl Window {
    pub fn new() -> Self
    {
        let window = create_window();
        Self {
            window,
            background_color: GGCOLOR_WHITE_ARGB8,
            click_config: None,
            window_handlers: sys::WindowHandlers {
                load: None,
                appear: None,
                disappear: None,
                unload: None,
            },
        }
    }

    pub fn window_destroy(&self) {
        unsafe {
            sys::window_destroy(*self.window);
        }
    }

    pub fn window_set_click_config_provider(&self, click_config: sys::ClickConfigProvider) {
        unsafe {
            sys::window_set_click_config_provider(*self.window, click_config);
        }
    }

    
}

impl Drop for Window {
    fn drop(&mut self) {
        self.window_destroy();
    }
}

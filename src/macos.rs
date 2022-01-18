use crate::Mode;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

extern "C" {
    static NSAppearanceNameAqua: *const Object;
    static NSAppearanceNameAccessibilityHighContrastAqua: *const Object;
    static NSAppearanceNameDarkAqua: *const Object;
    static NSAppearanceNameAccessibilityHighContrastDarkAqua: *const Object;
}

fn is_dark_mode_enabled() -> bool {
    unsafe {
        let mut appearance: *const Object = msg_send![class!(NSAppearance), currentAppearance];
        if appearance.is_null() {
            appearance = msg_send![class!(NSApp), effectiveAppearance];
        }

        let objects = [
            NSAppearanceNameAqua,
            NSAppearanceNameAccessibilityHighContrastAqua,
            NSAppearanceNameDarkAqua,
            NSAppearanceNameAccessibilityHighContrastDarkAqua,
        ];
        let names: *const Object = msg_send![
            class!(NSArray),
            arrayWithObjects:objects.as_ptr()
            count:objects.len()
        ];

        let style: *const Object = msg_send![
            appearance,
            bestMatchFromAppearancesWithNames:&*names
        ];

        return style == NSAppearanceNameDarkAqua
            || style == NSAppearanceNameAccessibilityHighContrastDarkAqua;
    }
}

pub fn detect() -> crate::Mode {
    Mode::from(is_dark_mode_enabled())
}

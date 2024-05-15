// Dark/light mode detection on macOS.
// Written with help from Ryan McGrath (https://rymc.io/).

use crate::Mode;

use objc2::sel;
use objc2_app_kit::{
    NSAppearance, NSAppearanceNameAccessibilityHighContrastAqua,
    NSAppearanceNameAccessibilityHighContrastDarkAqua, NSAppearanceNameAqua,
    NSAppearanceNameDarkAqua, NSApplication,
};
use objc2_foundation::{MainThreadMarker, NSArray, NSCopying, NSObjectProtocol};

fn is_dark_mode_enabled() -> bool {
    // SAFETY: TODO, only perform this function on the main thread.
    let mtm = unsafe { MainThreadMarker::new_unchecked() };

    unsafe {
        #[allow(deprecated)]
        let appearance = NSAppearance::currentAppearance()
            .unwrap_or_else(|| NSApplication::sharedApplication(mtm).effectiveAppearance());

        let names = NSArray::from_id_slice(&[
            NSAppearanceNameAqua.copy(),
            NSAppearanceNameAccessibilityHighContrastAqua.copy(),
            NSAppearanceNameDarkAqua.copy(),
            NSAppearanceNameAccessibilityHighContrastDarkAqua.copy(),
        ]);

        // `bestMatchFromAppearancesWithNames` is only available in macOS 10.14+.
        // Gracefully handle earlier versions.
        if !appearance.respondsToSelector(sel!(bestMatchFromAppearancesWithNames:)) {
            return false;
        }

        if let Some(style) = appearance.bestMatchFromAppearancesWithNames(&names) {
            *style == *NSAppearanceNameDarkAqua
                || *style == *NSAppearanceNameAccessibilityHighContrastDarkAqua
        } else {
            false
        }
    }
}

pub fn detect() -> crate::Mode {
    Mode::from_bool(is_dark_mode_enabled())
}

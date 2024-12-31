pub mod detect;
pub mod subscribe;

use objc2::{class, msg_send};
use objc2_foundation::{NSObject, NSString};

fn is_dark_mode() -> bool {
    unsafe {
        let user_defaults: *mut NSObject = msg_send![class!(NSUserDefaults), standardUserDefaults];
        let apple_domain = NSString::from_str("Apple Global Domain");
        let dict: *mut NSObject = msg_send![user_defaults, persistentDomainForName:&*apple_domain];

        if !dict.is_null() {
            let style_key = NSString::from_str("AppleInterfaceStyle");
            let style: *mut NSObject = msg_send![dict, objectForKey:&*style_key];

            if !style.is_null() {
                // Compare with "Dark"
                let dark_str = NSString::from_str("Dark");
                let is_dark: bool = msg_send![style, isEqualToString:&*dark_str];
                is_dark
            } else {
                false
            }
        } else {
            false
        }
    }
}

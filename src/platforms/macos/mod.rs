pub mod detect;

pub mod subscribe;

use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use objc::{class, msg_send, sel, sel_impl};

fn is_dark_mode() -> bool {
    unsafe {
        let user_defaults: id = msg_send![class!(NSUserDefaults), standardUserDefaults];
        let dict: id = msg_send![user_defaults, persistentDomainForName:
            NSString::alloc(nil).init_str("Apple Global Domain")];
        let key = NSString::alloc(nil).init_str("AppleInterfaceStyle");
        let style: id = msg_send![dict, objectForKey: key];

        if style != nil {
            let dark_mode = NSString::alloc(nil).init_str("Dark");
            let is_dark: bool = msg_send![style, isEqualToString: dark_mode];
            is_dark
        } else {
            false
        }
    }
}

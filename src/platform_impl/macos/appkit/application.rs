use std::ops::BitOrAssign;
use objc2::foundation::{MainThreadMarker, NSArray, NSInteger, NSObject, NSUInteger};
use objc2::rc::{Id, Shared};
use objc2::runtime::Object;
use objc2::{extern_class, extern_methods, msg_send_id, ClassType};
use objc2::{Encode, Encoding};

use super::{NSAppearance, NSEvent, NSMenu, NSResponder, NSWindow};

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct NSApplication;

    unsafe impl ClassType for NSApplication {
        #[inherits(NSObject)]
        type Super = NSResponder;
    }
);

pub(crate) fn NSApp() -> Id<NSApplication, Shared> {
    // TODO: Only allow access from main thread
    NSApplication::shared(unsafe { MainThreadMarker::new_unchecked() })
}

extern_methods!(
    unsafe impl NSApplication {
        /// This can only be called on the main thread since it may initialize
        /// the application and since it's parameters may be changed by the main
        /// thread at any time (hence it is only safe to access on the main thread).
        pub fn shared(_mtm: MainThreadMarker) -> Id<Self, Shared> {
            let app: Option<_> = unsafe { msg_send_id![Self::class(), sharedApplication] };
            // SAFETY: `sharedApplication` always initializes the app if it isn't already
            unsafe { app.unwrap_unchecked() }
        }

        pub fn currentEvent(&self) -> Option<Id<NSEvent, Shared>> {
            unsafe { msg_send_id![self, currentEvent] }
        }

        #[sel(postEvent:atStart:)]
        pub fn postEvent_atStart(&self, event: &NSEvent, front_of_queue: bool);

        #[sel(presentationOptions)]
        pub fn presentationOptions(&self) -> NSApplicationPresentationOptions;

        pub fn windows(&self) -> Id<NSArray<NSWindow, Shared>, Shared> {
            unsafe { msg_send_id![self, windows] }
        }

        pub fn keyWindow(&self) -> Option<Id<NSWindow, Shared>> {
            unsafe { msg_send_id![self, keyWindow] }
        }

        // TODO: NSApplicationDelegate
        #[sel(setDelegate:)]
        pub fn setDelegate(&self, delegate: &Object);

        #[sel(setPresentationOptions:)]
        pub fn setPresentationOptions(&self, options: NSApplicationPresentationOptions);

        #[sel(hide:)]
        pub fn hide(&self, sender: Option<&Object>);

        #[sel(orderFrontCharacterPalette:)]
        #[allow(dead_code)]
        pub fn orderFrontCharacterPalette(&self, sender: Option<&Object>);

        #[sel(hideOtherApplications:)]
        pub fn hideOtherApplications(&self, sender: Option<&Object>);

        #[sel(stop:)]
        pub fn stop(&self, sender: Option<&Object>);

        #[sel(activateIgnoringOtherApps:)]
        pub fn activateIgnoringOtherApps(&self, ignore: bool);

        #[sel(requestUserAttention:)]
        pub fn requestUserAttention(&self, type_: NSRequestUserAttentionType) -> NSInteger;

        #[sel(setActivationPolicy:)]
        pub fn setActivationPolicy(&self, policy: NSApplicationActivationPolicy) -> bool;

        #[sel(setMainMenu:)]
        pub fn setMainMenu(&self, menu: &NSMenu);

        pub fn effectiveAppearance(&self) -> Id<NSAppearance, Shared> {
            unsafe { msg_send_id![self, effectiveAppearance] }
        }

        #[sel(setAppearance:)]
        pub fn setAppearance(&self, appearance: Option<&NSAppearance>);

        #[sel(run)]
        pub unsafe fn run(&self);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct NSRunningApplication;

    unsafe impl ClassType for NSRunningApplication {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSRunningApplication {

        /// https://developer.apple.com/documentation/appkit/nsrunningapplication/1533604-currentapplication?language=objc
        pub fn currentApplication() -> Id<Self, Shared> {
            // https://github.com/servo/core-foundation-rs/blob/master/cocoa/src/appkit.rs#L649
            unsafe { msg_send_id![Self::class(), currentApplication] }
        }

        /// https://github.com/servo/core-foundation-rs/blob/master/cocoa/src/appkit.rs#L652
        /// https://github.com/servo/core-foundation-rs/blob/master/cocoa/src/appkit.rs#L663
        #[sel(activateWithOptions:)]
        pub fn activateWithOptions(&self, options: NSApplicationActivationOptions) -> bool;

    }
);

#[allow(dead_code)]
#[repr(isize)] // NSInteger
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NSApplicationActivationPolicy {
    NSApplicationActivationPolicyRegular = 0,
    NSApplicationActivationPolicyAccessory = 1,
    NSApplicationActivationPolicyProhibited = 2,
    NSApplicationActivationPolicyERROR = -1,
}

unsafe impl Encode for NSApplicationActivationPolicy {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

bitflags! {
    pub struct NSApplicationPresentationOptions: NSUInteger {
        const NSApplicationPresentationDefault = 0;
        const NSApplicationPresentationAutoHideDock = 1 << 0;
        const NSApplicationPresentationHideDock = 1 << 1;
        const NSApplicationPresentationAutoHideMenuBar = 1 << 2;
        const NSApplicationPresentationHideMenuBar = 1 << 3;
        const NSApplicationPresentationDisableAppleMenu = 1 << 4;
        const NSApplicationPresentationDisableProcessSwitching = 1 << 5;
        const NSApplicationPresentationDisableForceQuit = 1 << 6;
        const NSApplicationPresentationDisableSessionTermination = 1 << 7;
        const NSApplicationPresentationDisableHideApplication = 1 << 8;
        const NSApplicationPresentationDisableMenuBarTransparency = 1 << 9;
        const NSApplicationPresentationFullScreen = 1 << 10;
        const NSApplicationPresentationAutoHideToolbar = 1 << 11;
    }
}

unsafe impl Encode for NSApplicationPresentationOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[repr(usize)] // NSUInteger
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NSRequestUserAttentionType {
    NSCriticalRequest = 0,
    NSInformationalRequest = 10,
}

unsafe impl Encode for NSRequestUserAttentionType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

/// https://github.com/servo/core-foundation-rs/blob/master/cocoa/src/appkit.rs#L173
#[repr(usize)] // NSUInteger see https://github.com/servo/core-foundation-rs/blob/master/cocoa/src/appkit.rs#L664 options as NSUInteger
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NSApplicationActivationOptions {
    NSApplicationActivateAllWindows = 1 << 0, // 1
    NSApplicationActivateIgnoringOtherApps = 1 << 1, // 2
    NSApplicationNo = 0, // 0
    NSApplicationActivateAllWindowsIgnoringOtherApps = (1 << 0) | (1 << 1), // NSApplicationActivateAllWindows | NSApplicationActivateIgnoringOtherApps = 3
    // 因为上面已经有 2 了，enum 不允许重复，只能有有一个，所以注释掉，用相等值的 NSApplicationActivateIgnoringOtherApps 即可
    //NSApplicationActivateNoIgnoringOtherApps = (0) | (1 << 1), // NSApplicationNo | NSApplicationActivateIgnoringOtherApps = NSApplicationActivateIgnoringOtherApps = 2
}

unsafe impl Encode for NSApplicationActivationOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

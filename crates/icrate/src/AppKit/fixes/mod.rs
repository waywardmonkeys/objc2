#![allow(clippy::bool_to_int_with_if)]
use crate::common::*;
use crate::AppKit::NSResponder;
use crate::Foundation::NSObject;

/// (!TARGET_CPU_X86_64 || (TARGET_OS_IPHONE && !TARGET_OS_MACCATALYST))
///
/// https://github.com/xamarin/xamarin-macios/issues/12111
// TODO: Make this work with mac catalyst
const TARGET_ABI_USES_IOS_VALUES: bool =
    !cfg!(any(target_arch = "x86", target_arch = "x86_64")) || cfg!(not(target_os = "macos"));

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSImageResizingMode {
        NSImageResizingModeStretch = if TARGET_ABI_USES_IOS_VALUES { 0 } else { 1 },
        NSImageResizingModeTile = if TARGET_ABI_USES_IOS_VALUES { 1 } else { 0 },
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextAlignment {
        NSTextAlignmentLeft = 0,
        NSTextAlignmentRight = if TARGET_ABI_USES_IOS_VALUES { 2 } else { 1 },
        NSTextAlignmentCenter = if TARGET_ABI_USES_IOS_VALUES { 1 } else { 2 },
        NSTextAlignmentJustified = 3,
        NSTextAlignmentNatural = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPopover;

    unsafe impl ClassType for NSPopover {
        #[inherits(NSObject)]
        type Super = NSResponder;
    }
);

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSLayoutAnchor<AnchorType: Message = Object, AnchorTypeOwnership: Ownership = Shared> {
        _inner0: PhantomData<*mut (AnchorType, AnchorTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    unsafe impl<AnchorType: Message, AnchorTypeOwnership: Ownership> ClassType
        for NSLayoutAnchor<AnchorType, AnchorTypeOwnership>
    {
        type Super = NSObject;
    }
);
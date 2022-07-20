use objc2::rc::{DefaultId, Id, Shared};
use objc2::runtime::Object;
use objc2::{msg_send, msg_send_id};

use crate::{
    extern_class, NSCopying, NSDictionary, NSMutableAttributedString, NSMutableCopying, NSObject,
    NSString,
};

extern_class! {
    /// A string that has associated attributes for portions of its text.
    ///
    /// Examples of attributes could be: Visual style, hyperlinks, or
    /// accessibility data.
    ///
    /// Conceptually, each UTF-16 code unit in an attributed string has its
    /// own collection of attributes - most often though
    ///
    /// Only the most basic functionality is defined here, the `AppKit`
    /// framework contains most of the extension methods.
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nsattributedstring?language=objc).
    #[derive(Debug, PartialEq, Eq, Hash)]
    unsafe pub struct NSAttributedString: NSObject;
}

// SAFETY: `NSAttributedString` is immutable and `NSMutableAttributedString`
// can only be mutated from `&mut` methods.
unsafe impl Sync for NSAttributedString {}
unsafe impl Send for NSAttributedString {}

/// Attributes that you can apply to text in an attributed string.
pub type NSAttributedStringKey = NSString;

/// Creating attributed strings.
impl NSAttributedString {
    /// Construct an empty attributed string.
    pub fn new() -> Id<Self, Shared> {
        unsafe { msg_send_id![Self::class(), new].unwrap() }
    }

    /// Creates a new attributed string from the given string and attributes.
    ///
    /// The attributes are associated with every UTF-16 code unit in the
    /// string.
    #[doc(alias = "initWithString:")]
    pub fn new_with_attributes(
        string: &NSString,
        // TODO: Mutability of the dictionary should be (Shared, Shared)
        attributes: &NSDictionary<NSAttributedStringKey, Object>,
    ) -> Id<Self, Shared> {
        unsafe {
            let obj = msg_send_id![Self::class(), alloc];
            msg_send_id![obj, initWithString: string, attributes: attributes].unwrap()
        }
    }

    /// Creates a new attributed string without any attributes.
    #[doc(alias = "initWithString:")]
    pub fn from_nsstring(string: &NSString) -> Id<Self, Shared> {
        unsafe {
            let obj = msg_send_id![Self::class(), alloc];
            msg_send_id![obj, initWithString: string].unwrap()
        }
    }
}

/// Querying.
impl NSAttributedString {
    // TODO: Lifetimes?
    pub fn string(&self) -> Id<NSString, Shared> {
        unsafe { msg_send_id![self, string].unwrap() }
    }

    /// Alias for `self.string().len_utf16()`.
    #[doc(alias = "length")]
    #[allow(unused)]
    // TODO: Finish this
    fn len_utf16(&self) -> usize {
        unsafe { msg_send![self, length] }
    }

    // /// TODO
    // ///
    // /// See [Apple's documentation on Effective and Maximal Ranges][doc].
    // ///
    // /// [doc]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/AttributedStrings/Tasks/AccessingAttrs.html#//apple_ref/doc/uid/20000161-SW2
    // #[doc(alias = "attributesAtIndex:effectiveRange:")]
    // pub fn attributes_in_effective_range(
    //     &self,
    //     index: usize,
    //     range: Range<usize>,
    // ) -> Id<Self, Shared> {
    //     let range = NSRange::from(range);
    //     todo!()
    // }
    //
    // attributesAtIndex:longestEffectiveRange:inRange:

    // TODO: attributedSubstringFromRange:
    // TODO: enumerateAttributesInRange:options:usingBlock:
}

impl DefaultId for NSAttributedString {
    type Ownership = Shared;

    #[inline]
    fn default_id() -> Id<Self, Self::Ownership> {
        Self::new()
    }
}

unsafe impl NSCopying for NSAttributedString {
    type Ownership = Shared;
    type Output = NSAttributedString;
}

unsafe impl NSMutableCopying for NSAttributedString {
    type Output = NSMutableAttributedString;
}

impl alloc::borrow::ToOwned for NSAttributedString {
    type Owned = Id<NSAttributedString, Shared>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use objc2::rc::autoreleasepool;

    use super::*;

    #[test]
    fn test_new() {
        let s = NSAttributedString::new();
        assert_eq!(&s.string().to_string(), "");
    }

    #[test]
    fn test_string_bound_to_attributed() {
        let attr_s = {
            let source = NSString::from_str("Hello world!");
            NSAttributedString::from_nsstring(&source)
        };
        let s = autoreleasepool(|_| attr_s.string());
        assert_eq!(s.len(), 12);
    }

    #[test]
    fn test_from_nsstring() {
        let s = NSAttributedString::from_nsstring(&NSString::from_str("abc"));
        assert_eq!(&s.string().to_string(), "abc");
    }

    #[test]
    fn test_copy() {
        let s1 = NSAttributedString::from_nsstring(&NSString::from_str("abc"));
        let s2 = s1.copy();
        // NSAttributedString performs this optimization in GNUStep's runtime,
        // but not in Apple's; so we don't test for it!
        // assert_eq!(Id::as_ptr(&s1), Id::as_ptr(&s2));
        assert!(s2.is_kind_of(NSAttributedString::class()));

        let s3 = s1.mutable_copy();
        assert_ne!(Id::as_ptr(&s1), Id::as_ptr(&s3).cast());
        assert!(s3.is_kind_of(NSMutableAttributedString::class()));
    }
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// A module to contain functionality used for generating DM ids which
// are restricted in length and format by devicemapper.

/// Define borrowed and owned versions of string types that guarantee
/// conformance to DM restrictions, such as maximum length.
// This implementation follows the example of Path/PathBuf as closely as
// possible.
macro_rules! str_id {
    ($B:ident, $O:ident, $MAX:ident, $check:ident) => {
        /// The borrowed version of the DM identifier.
        #[derive(Debug, PartialEq, Eq, Hash)]
        pub struct $B {
            inner: str,
        }

        /// The owned version of the DM identifier.
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $O {
            inner: String,
        }

        impl $B {
            /// Create a new borrowed identifier from a `&str`.
            #[allow(clippy::new_ret_no_self)]
            pub fn new(value: &str) -> DmResult<&$B> {
                $check(value, $MAX - 1)?;
                Ok(unsafe { &*(value as *const str as *const $B) })
            }

            /// Get the inner value as bytes
            pub fn as_bytes(&self) -> &[u8] {
                self.inner.as_bytes()
            }
        }

        impl ToOwned for $B {
            type Owned = $O;
            fn to_owned(&self) -> $O {
                $O {
                    inner: self.inner.to_owned(),
                }
            }
        }

        impl fmt::Display for $B {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", &self.inner)
            }
        }

        impl $O {
            /// Construct a new owned identifier.
            #[allow(clippy::new_ret_no_self)]
            pub fn new(value: String) -> DmResult<$O> {
                $check(&value, $MAX - 1)?;
                Ok($O { inner: value })
            }
        }

        impl AsRef<$B> for $O {
            fn as_ref(&self) -> &$B {
                self
            }
        }

        impl Borrow<$B> for $O {
            fn borrow(&self) -> &$B {
                self.deref()
            }
        }

        impl Deref for $O {
            type Target = $B;
            fn deref(&self) -> &$B {
                $B::new(&self.inner).expect("inner satisfies all correctness criteria for $B::new")
            }
        }
    };
}

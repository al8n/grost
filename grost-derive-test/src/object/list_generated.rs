mod list {
    use grost::{Object, flavors::groto::*};
    use std::vec::Vec;
    struct VecCombinations {
        #[grost(tag = 4, optional(list(optional(list(scalar)))))]
        optional_vec_optional_vec: Option<Vec<Option<Vec<u16>>>>,
    }
    /// Field indexer for the struct [`VecCombinations`]
    #[repr(u32)]
    enum VecCombinationsIndex {
        /// The field indexer for the field `optional_vec_optional_vec`
        OptionalVecOptionalVec = 4u32,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for VecCombinationsIndex {
        #[inline]
        fn clone(&self) -> VecCombinationsIndex {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for VecCombinationsIndex {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for VecCombinationsIndex {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for VecCombinationsIndex {
        #[inline]
        fn eq(&self, other: &VecCombinationsIndex) -> bool {
            true
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for VecCombinationsIndex {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for VecCombinationsIndex {
        #[inline]
        fn partial_cmp(
            &self,
            other: &VecCombinationsIndex,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for VecCombinationsIndex {
        #[inline]
        fn cmp(&self, other: &VecCombinationsIndex) -> ::core::cmp::Ordering {
            ::core::cmp::Ordering::Equal
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for VecCombinationsIndex {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for VecCombinationsIndex {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "OptionalVecOptionalVec")
        }
    }
    /// The selection type for [`VecCombinations`]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    struct VecCombinationsSelector {
        optional_vec_optional_vec: <Option<
            Vec<Option<Vec<u16>>>,
        > as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Groto,
        >>::Selector,
    }
    /// An iterator over the selected fields of the [`VecCombinationsSelector`]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    struct VecCombinationsSelectorIter<
        '__grost_lifetime__,
        const __GROST_SELECTED__: ::core::primitive::bool = true,
    > {
        selector: &'__grost_lifetime__ VecCombinationsSelector,
        index: ::core::option::Option<VecCombinationsIndex>,
        num: ::core::primitive::usize,
        yielded: ::core::primitive::usize,
    }
    /// Partial struct for the [`PartialVecCombinations`]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    struct PartialVecCombinations {
        optional_vec_optional_vec: <Option<
            Vec<Option<Vec<u16>>>,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >>::Output,
    }
    /// Partial reference struct for the struct [`VecCombinations`]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    struct PartialVecCombinationsRef<
        '__grost_lifetime__,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    > {
        __grost_unknown_buffer__: ::core::option::Option<__GROST_UNKNOWN_BUFFER__>,
        __grost_read_buffer__: ::core::option::Option<__GROST_READ_BUFFER__>,
        optional_vec_optional_vec: <Option<
            Vec<Option<Vec<u16>>>,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::PartialRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
                <Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Groto,
                >>::Format,
                ::grost::__private::flavors::Groto,
            >,
        >>::Output,
    }
    const _: () = {
        impl VecCombinations {
            /// Returns a reference to the `optional_vec_optional_vec`
            #[inline]
            const fn optional_vec_optional_vec_ref(
                &self,
            ) -> ::core::option::Option<
                &<Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flattened,
                >>::Output,
            > {
                self.optional_vec_optional_vec.as_ref()
            }
            /// Returns a mutable reference to the `optional_vec_optional_vec`
            #[inline]
            const fn optional_vec_optional_vec_mut(
                &mut self,
            ) -> ::core::option::Option<
                &mut <Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flattened,
                >>::Output,
            > {
                self.optional_vec_optional_vec.as_mut()
            }
            /// Set the `optional_vec_optional_vec` to the given value
            #[inline]
            fn set_optional_vec_optional_vec(
                &mut self,
                value: <Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flattened,
                >>::Output,
            ) -> &mut Self {
                self.optional_vec_optional_vec = ::core::option::Option::Some(value);
                self
            }
            /// Set the `optional_vec_optional_vec` to the given value
            #[inline]
            fn with_optional_vec_optional_vec(
                mut self,
                value: <Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flattened,
                >>::Output,
            ) -> Self {
                self.optional_vec_optional_vec = ::core::option::Option::Some(value);
                self
            }
            /// Returns a reference to the `optional_vec_optional_vec` if it is not `None`
            ///
            /// ## Panics
            ///
            /// - Panics if the `optional_vec_optional_vec` is `None`
            #[inline]
            const fn unwrap_optional_vec_optional_vec_ref(
                &self,
            ) -> &<Option<
                Vec<Option<Vec<u16>>>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened,
            >>::Output {
                match self.optional_vec_optional_vec.as_ref() {
                    ::core::option::Option::Some(value) => value,
                    ::core::option::Option::None => {
                        ::core::panicking::panic_fmt(
                            format_args!("`optional_vec_optional_vec` is `None`"),
                        );
                    }
                }
            }
            /// Returns a mutable reference to the `optional_vec_optional_vec` if it is not `None`
            ///
            /// ## Panics
            ///
            /// - Panics if the `optional_vec_optional_vec` is `None`
            #[inline]
            const fn unwrap_optional_vec_optional_vec_mut(
                &mut self,
            ) -> &mut <Option<
                Vec<Option<Vec<u16>>>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened,
            >>::Output {
                match self.optional_vec_optional_vec.as_mut() {
                    ::core::option::Option::Some(value) => value,
                    ::core::option::Option::None => {
                        ::core::panicking::panic_fmt(
                            format_args!("`optional_vec_optional_vec` is `None`"),
                        );
                    }
                }
            }
            /// Takes the value of `optional_vec_optional_vec` out if it is not `None`
            #[inline]
            const fn take_optional_vec_optional_vec(
                &mut self,
            ) -> Option<Vec<Option<Vec<u16>>>> {
                self.optional_vec_optional_vec.take()
            }
            /// Clear the value of `optional_vec_optional_vec`
            #[inline]
            fn clear_optional_vec_optional_vec(&mut self) -> &mut Self {
                self.optional_vec_optional_vec = ::core::option::Option::None;
                self
            }
            /// Update the `optional_vec_optional_vec` to the given value or clear the `optional_vec_optional_vec`
            #[inline]
            fn update_optional_vec_optional_vec(
                &mut self,
                value: Option<Vec<Option<Vec<u16>>>>,
            ) -> &mut Self {
                self.optional_vec_optional_vec = value;
                self
            }
            /// Clear the value of `optional_vec_optional_vec`
            #[inline]
            fn without_optional_vec_optional_vec(mut self) -> Self {
                self.optional_vec_optional_vec = ::core::option::Option::None;
                self
            }
            /// Update the `optional_vec_optional_vec` to the given value or clear the `optional_vec_optional_vec`
            #[inline]
            fn maybe_optional_vec_optional_vec(
                mut self,
                value: Option<Vec<Option<Vec<u16>>>>,
            ) -> Self {
                self.optional_vec_optional_vec = value;
                self
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            __GROST_FLATTEN_STATE__: ?::core::marker::Sized,
        > ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<__GROST_FLATTEN_STATE__>,
        > for VecCombinations {
            type Output = Self;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::convert::State<
            ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        > for VecCombinations {
            type Output = PartialVecCombinations;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::convert::State<
            ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        > for PartialVecCombinations {
            type Output = Self;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > ::grost::__private::convert::State<
            ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >
        for PartialVecCombinationsRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > {
            type Output = Self;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > ::grost::__private::convert::State<
            ::grost::__private::convert::PartialRef<
                '__grost_lifetime__,
                ::grost::__private::flavors::Groto,
                ::grost::__private::flavors::groto::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        > for VecCombinations {
            type Output = PartialVecCombinationsRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > ::grost::__private::convert::State<
            ::grost::__private::convert::PartialRef<
                '__grost_lifetime__,
                ::grost::__private::flavors::Groto,
                ::grost::__private::flavors::groto::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        > for PartialVecCombinations {
            type Output = PartialVecCombinationsRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > ::grost::__private::convert::State<
            ::grost::__private::convert::PartialRef<
                '__grost_lifetime__,
                ::grost::__private::flavors::Groto,
                ::grost::__private::flavors::groto::LengthDelimited,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >
        for PartialVecCombinationsRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > {
            type Output = Self;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::default::Default for PartialVecCombinations {
            fn default() -> Self {
                Self::new()
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            __GROST_FLATTEN_STATE__: ?::core::marker::Sized,
        > ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<__GROST_FLATTEN_STATE__>,
        > for PartialVecCombinations {
            type Output = Self;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl PartialVecCombinations {
            /// Creates an empty partial struct.
            #[inline]
            pub const fn new() -> Self {
                Self {
                    optional_vec_optional_vec: ::core::option::Option::None,
                }
            }
            /// Returns `true` if the partial object is empty.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.optional_vec_optional_vec.is_none()
            }
            /// Returns a reference to the `optional_vec_optional_vec`
            #[inline]
            const fn optional_vec_optional_vec_ref(
                &self,
            ) -> ::core::option::Option<
                &<<Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Partial<
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flattened,
                >>::Output,
            > {
                self.optional_vec_optional_vec.as_ref()
            }
            /// Returns a mutable reference to the `optional_vec_optional_vec`
            #[inline]
            const fn optional_vec_optional_vec_mut(
                &mut self,
            ) -> ::core::option::Option<
                &mut <<Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Partial<
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flattened,
                >>::Output,
            > {
                self.optional_vec_optional_vec.as_mut()
            }
            /// Returns a reference to the `optional_vec_optional_vec` if it is not `None`
            ///
            /// ## Panics
            ///
            /// - Panics if the `optional_vec_optional_vec` is `None`
            #[inline]
            const fn unwrap_optional_vec_optional_vec_ref(
                &self,
            ) -> &<<Option<
                Vec<Option<Vec<u16>>>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
            >>::Output as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened,
            >>::Output {
                match self.optional_vec_optional_vec.as_ref() {
                    ::core::option::Option::Some(value) => value,
                    ::core::option::Option::None => {
                        ::core::panicking::panic_fmt(
                            format_args!("`optional_vec_optional_vec` is `None`"),
                        );
                    }
                }
            }
            /// Returns a mutable reference to the `optional_vec_optional_vec` if it is not `None`
            ///
            /// ## Panics
            ///
            /// - Panics if the `optional_vec_optional_vec` is `None`
            #[inline]
            const fn unwrap_optional_vec_optional_vec_mut(
                &mut self,
            ) -> &mut <<Option<
                Vec<Option<Vec<u16>>>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
            >>::Output as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened,
            >>::Output {
                match self.optional_vec_optional_vec.as_mut() {
                    ::core::option::Option::Some(value) => value,
                    ::core::option::Option::None => {
                        ::core::panicking::panic_fmt(
                            format_args!("`optional_vec_optional_vec` is `None`"),
                        );
                    }
                }
            }
            /// Takes the value of `optional_vec_optional_vec` out if it is not `None`
            #[inline]
            const fn take_optional_vec_optional_vec(
                &mut self,
            ) -> <Option<
                Vec<Option<Vec<u16>>>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
            >>::Output {
                self.optional_vec_optional_vec.take()
            }
            /// Clear the value of `optional_vec_optional_vec`
            #[inline]
            fn clear_optional_vec_optional_vec(&mut self) -> &mut Self {
                self.optional_vec_optional_vec = ::core::option::Option::None;
                self
            }
            /// Set the `optional_vec_optional_vec` to the given value
            #[inline]
            fn set_optional_vec_optional_vec(
                &mut self,
                value: <<Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Partial<
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flattened,
                >>::Output,
            ) -> &mut Self {
                self.optional_vec_optional_vec = ::core::option::Option::Some(value);
                self
            }
            /// Update the `optional_vec_optional_vec` to the given value or clear the `optional_vec_optional_vec`
            #[inline]
            fn update_optional_vec_optional_vec(
                &mut self,
                value: <Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Partial<
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output,
            ) -> &mut Self {
                self.optional_vec_optional_vec = value;
                self
            }
            /// Set the `optional_vec_optional_vec` to the given value
            #[inline]
            fn with_optional_vec_optional_vec(
                mut self,
                value: <<Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Partial<
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flattened,
                >>::Output,
            ) -> Self {
                self.optional_vec_optional_vec = ::core::option::Option::Some(value);
                self
            }
            /// Clear the value of `optional_vec_optional_vec`
            #[inline]
            fn without_optional_vec_optional_vec(mut self) -> Self {
                self.optional_vec_optional_vec = ::core::option::Option::None;
                self
            }
            /// Update the `optional_vec_optional_vec` to the given value or clear the `optional_vec_optional_vec`
            #[inline]
            fn maybe_optional_vec_optional_vec(
                mut self,
                value: <Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Partial<
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output,
            ) -> Self {
                self.optional_vec_optional_vec = value;
                self
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > ::core::default::Default
        for PartialVecCombinationsRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > {
            fn default() -> Self {
                Self::new()
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
            __GROST_FLATTEN_STATE__: ?::core::marker::Sized,
        > ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<__GROST_FLATTEN_STATE__>,
        >
        for PartialVecCombinationsRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > {
            type Output = Self;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > PartialVecCombinationsRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > {
            /// Creates an empty partial struct.
            #[inline]
            pub const fn new() -> Self {
                Self {
                    optional_vec_optional_vec: ::core::option::Option::None,
                    __grost_unknown_buffer__: ::core::option::Option::None,
                    __grost_read_buffer__: ::core::option::Option::None,
                }
            }
            /// Returns `true` if the partial struct is empty, which means all fields are `None`.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.__grost_unknown_buffer__.is_none()
                    && self.optional_vec_optional_vec.is_none()
            }
            /// Returns the original encoded type of the partial decoded object.
            #[inline]
            pub const fn raw(&self) -> ::core::option::Option<&__GROST_READ_BUFFER__> {
                self.__grost_read_buffer__.as_ref()
            }
            /// Returns a reference to the unknown buffer, which holds the unknown data when decoding.
            #[inline]
            pub const fn unknown_buffer(
                &self,
            ) -> ::core::option::Option<&__GROST_UNKNOWN_BUFFER__> {
                self.__grost_unknown_buffer__.as_ref()
            }
            /// Returns a mutable reference to the unknown buffer, which holds the unknown data when decoding.
            #[inline]
            pub const fn unknown_buffer_mut(
                &mut self,
            ) -> ::core::option::Option<&mut __GROST_UNKNOWN_BUFFER__> {
                self.__grost_unknown_buffer__.as_mut()
            }
            /// Takes the unknown buffer out if the unknown buffer is not `None`.
            #[inline]
            pub const fn take_unknown_buffer(
                &mut self,
            ) -> ::core::option::Option<__GROST_UNKNOWN_BUFFER__> {
                self.__grost_unknown_buffer__.take()
            }
            /// Set the value of unknown buffer
            #[inline]
            pub fn set_unknown_buffer(
                &mut self,
                buffer: __GROST_UNKNOWN_BUFFER__,
            ) -> &mut Self {
                self.__grost_unknown_buffer__ = ::core::option::Option::Some(buffer);
                self
            }
            /// Clears the unknown buffer.
            #[inline]
            pub fn clear_unknown_buffer(&mut self) -> &mut Self {
                self.__grost_unknown_buffer__ = ::core::option::Option::None;
                self
            }
            /// Set the value of unknown buffer
            #[inline]
            pub fn with_unknown_buffer(
                mut self,
                buffer: __GROST_UNKNOWN_BUFFER__,
            ) -> Self {
                self.__grost_unknown_buffer__ = ::core::option::Option::Some(buffer);
                self
            }
            /// Clears the unknown buffer.
            #[inline]
            pub fn without_unknown_buffer(mut self) -> Self {
                self.__grost_unknown_buffer__ = ::core::option::Option::None;
                self
            }
            /// Returns a reference to the `optional_vec_optional_vec`
            #[inline]
            const fn optional_vec_optional_vec_ref(
                &self,
            ) -> ::core::option::Option<
                &<<Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::PartialRef<
                        '__grost_lifetime__,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                        <Option<
                            Vec<Option<Vec<u16>>>,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Groto,
                        >>::Format,
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flattened,
                >>::Output,
            > {
                self.optional_vec_optional_vec.as_ref()
            }
            /// Returns a mutable reference to the `optional_vec_optional_vec`
            #[inline]
            const fn optional_vec_optional_vec_mut(
                &mut self,
            ) -> ::core::option::Option<
                &mut <<Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::PartialRef<
                        '__grost_lifetime__,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                        <Option<
                            Vec<Option<Vec<u16>>>,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Groto,
                        >>::Format,
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flattened,
                >>::Output,
            > {
                self.optional_vec_optional_vec.as_mut()
            }
            /// Returns a reference to the `optional_vec_optional_vec` if it is not `None`
            ///
            /// ## Panics
            ///
            /// - Panics if the `optional_vec_optional_vec` is `None`
            #[inline]
            const fn unwrap_optional_vec_optional_vec_ref(
                &self,
            ) -> &<<Option<
                Vec<Option<Vec<u16>>>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::PartialRef<
                    '__grost_lifetime__,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                    <Option<
                        Vec<Option<Vec<u16>>>,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Groto,
                    >>::Format,
                    ::grost::__private::flavors::Groto,
                >,
            >>::Output as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened,
            >>::Output {
                match self.optional_vec_optional_vec.as_ref() {
                    ::core::option::Option::Some(value) => value,
                    ::core::option::Option::None => {
                        ::core::panicking::panic_fmt(
                            format_args!("`optional_vec_optional_vec` is `None`"),
                        );
                    }
                }
            }
            /// Returns a mutable reference to the `optional_vec_optional_vec` if it is not `None`
            ///
            /// ## Panics
            ///
            /// - Panics if the `optional_vec_optional_vec` is `None`
            #[inline]
            const fn unwrap_optional_vec_optional_vec_mut(
                &mut self,
            ) -> &mut <<Option<
                Vec<Option<Vec<u16>>>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::PartialRef<
                    '__grost_lifetime__,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                    <Option<
                        Vec<Option<Vec<u16>>>,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Groto,
                    >>::Format,
                    ::grost::__private::flavors::Groto,
                >,
            >>::Output as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened,
            >>::Output {
                match self.optional_vec_optional_vec.as_mut() {
                    ::core::option::Option::Some(value) => value,
                    ::core::option::Option::None => {
                        ::core::panicking::panic_fmt(
                            format_args!("`optional_vec_optional_vec` is `None`"),
                        );
                    }
                }
            }
            /// Takes the value of `optional_vec_optional_vec` out if it is not `None`
            #[inline]
            const fn take_optional_vec_optional_vec(
                &mut self,
            ) -> <Option<
                Vec<Option<Vec<u16>>>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::PartialRef<
                    '__grost_lifetime__,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                    <Option<
                        Vec<Option<Vec<u16>>>,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Groto,
                    >>::Format,
                    ::grost::__private::flavors::Groto,
                >,
            >>::Output {
                self.optional_vec_optional_vec.take()
            }
            /// Clear the value of `optional_vec_optional_vec`
            #[inline]
            fn clear_optional_vec_optional_vec(&mut self) -> &mut Self {
                self.optional_vec_optional_vec = ::core::option::Option::None;
                self
            }
            /// Set the `optional_vec_optional_vec` to the given value
            #[inline]
            fn set_optional_vec_optional_vec(
                &mut self,
                value: <<Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::PartialRef<
                        '__grost_lifetime__,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                        <Option<
                            Vec<Option<Vec<u16>>>,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Groto,
                        >>::Format,
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flattened,
                >>::Output,
            ) -> &mut Self {
                self.optional_vec_optional_vec = ::core::option::Option::Some(value);
                self
            }
            /// Update the `optional_vec_optional_vec` to the given value or clear the `optional_vec_optional_vec`
            #[inline]
            fn update_optional_vec_optional_vec(
                &mut self,
                value: <Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::PartialRef<
                        '__grost_lifetime__,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                        <Option<
                            Vec<Option<Vec<u16>>>,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Groto,
                        >>::Format,
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output,
            ) -> &mut Self {
                self.optional_vec_optional_vec = value;
                self
            }
            /// Set the `optional_vec_optional_vec` to the given value
            #[inline]
            fn with_optional_vec_optional_vec(
                mut self,
                value: <<Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::PartialRef<
                        '__grost_lifetime__,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                        <Option<
                            Vec<Option<Vec<u16>>>,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Groto,
                        >>::Format,
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flattened,
                >>::Output,
            ) -> Self {
                self.optional_vec_optional_vec = ::core::option::Option::Some(value);
                self
            }
            /// Clear the value of `optional_vec_optional_vec`
            #[inline]
            fn without_optional_vec_optional_vec(mut self) -> Self {
                self.optional_vec_optional_vec = ::core::option::Option::None;
                self
            }
            /// Update the `optional_vec_optional_vec` to the given value or clear the `optional_vec_optional_vec`
            #[inline]
            fn maybe_optional_vec_optional_vec(
                mut self,
                value: <Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::PartialRef<
                        '__grost_lifetime__,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                        <Option<
                            Vec<Option<Vec<u16>>>,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Groto,
                        >>::Format,
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output,
            ) -> Self {
                self.optional_vec_optional_vec = value;
                self
            }
        }
        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl ::grost::__private::reflection::Reflectable<VecCombinations>
        for ::grost::__private::reflection::ObjectFieldReflection<
            VecCombinations,
            ::grost::__private::reflection::ObjectField,
            ::grost::__private::flavors::Groto,
            4u32,
        > {
            type Reflection = ::grost::__private::reflection::ObjectField;
            const REFLECTION: &'static Self::Reflection = &{
                ::grost::__private::reflection::ObjectFieldBuilder {
                    name: "optional_vec_optional_vec",
                    description: "",
                    ty: <::grost::__private::reflection::SchemaTypeReflection<
                        Option<Vec<Option<Vec<u16>>>>,
                    > as ::grost::__private::reflection::Reflectable<
                        Option<Vec<Option<Vec<u16>>>>,
                    >>::REFLECTION,
                }
                    .build()
            };
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::reflection::Reflectable<VecCombinations>
        for ::grost::__private::reflection::WireFormatReflection<
            VecCombinations,
            ::grost::__private::flavors::Groto,
            4u32,
        > {
            type Reflection = <Option<
                Vec<Option<Vec<u16>>>,
            > as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Groto,
            >>::Format;
            const REFLECTION: &'static Self::Reflection = &{
                <<Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Groto,
                >>::Format as ::grost::__private::flavors::WireFormat<
                    ::grost::__private::flavors::Groto,
                >>::SELF
            };
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::reflection::Reflectable<VecCombinations>
        for ::grost::__private::reflection::IdentifierReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
                VecCombinations,
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
                ::grost::__private::flavors::Groto,
                4u32,
            >,
        > {
            type Reflection = <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier;
            const REFLECTION: &Self::Reflection = &{
                (::grost::__private::flavors::groto::Identifier::new)(
                    <<Option<
                        Vec<Option<Vec<u16>>>,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Groto,
                    >>::Format as ::grost::__private::flavors::WireFormat<
                        ::grost::__private::flavors::Groto,
                    >>::WIRE_TYPE,
                    (::grost::__private::flavors::groto::Tag::new)(4u32),
                )
            };
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::reflection::Reflectable<VecCombinations>
        for ::grost::__private::reflection::EncodeReflection<
            ::grost::__private::reflection::IdentifierReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    VecCombinations,
                    <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
                    ::grost::__private::flavors::Groto,
                    4u32,
                >,
            >,
        > {
            type Reflection = [::core::primitive::u8];
            const REFLECTION: &Self::Reflection = {
                (::grost::__private::flavors::groto::Identifier::encode)(
                        <::grost::__private::reflection::IdentifierReflection<
                            ::grost::__private::reflection::ObjectFieldReflection<
                                VecCombinations,
                                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
                                ::grost::__private::flavors::Groto,
                                4u32,
                            >,
                        > as ::grost::__private::reflection::Reflectable<
                            VecCombinations,
                        >>::REFLECTION,
                    )
                    .as_slice()
            };
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::reflection::Reflectable<VecCombinations>
        for ::grost::__private::reflection::Len<
            ::grost::__private::reflection::EncodeReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        VecCombinations,
                        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
                        ::grost::__private::flavors::Groto,
                        4u32,
                    >,
                >,
            >,
        > {
            type Reflection = ::core::primitive::usize;
            const REFLECTION: &Self::Reflection = {
                &<::grost::__private::reflection::EncodeReflection<
                    ::grost::__private::reflection::IdentifierReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            VecCombinations,
                            <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
                            ::grost::__private::flavors::Groto,
                            4u32,
                        >,
                    >,
                > as ::grost::__private::reflection::Reflectable<
                    VecCombinations,
                >>::REFLECTION
                    .len()
            };
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::reflection::Reflectable<VecCombinations>
        for ::grost::__private::reflection::TagReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
                VecCombinations,
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag,
                ::grost::__private::flavors::Groto,
                4u32,
            >,
        > {
            type Reflection = <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag;
            const REFLECTION: &Self::Reflection = &{
                (::grost::__private::flavors::groto::Tag::new)(4u32)
            };
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::reflection::Reflectable<VecCombinations>
        for ::grost::__private::reflection::EncodeReflection<
            ::grost::__private::reflection::TagReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    VecCombinations,
                    <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag,
                    ::grost::__private::flavors::Groto,
                    4u32,
                >,
            >,
        > {
            type Reflection = [::core::primitive::u8];
            const REFLECTION: &Self::Reflection = {
                (::grost::__private::flavors::groto::Tag::encode)(
                        <::grost::__private::reflection::TagReflection<
                            ::grost::__private::reflection::ObjectFieldReflection<
                                VecCombinations,
                                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag,
                                ::grost::__private::flavors::Groto,
                                4u32,
                            >,
                        > as ::grost::__private::reflection::Reflectable<
                            VecCombinations,
                        >>::REFLECTION,
                    )
                    .as_slice()
            };
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::reflection::Reflectable<VecCombinations>
        for ::grost::__private::reflection::Len<
            ::grost::__private::reflection::EncodeReflection<
                ::grost::__private::reflection::TagReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        VecCombinations,
                        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag,
                        ::grost::__private::flavors::Groto,
                        4u32,
                    >,
                >,
            >,
        > {
            type Reflection = ::core::primitive::usize;
            const REFLECTION: &Self::Reflection = {
                &<::grost::__private::reflection::EncodeReflection<
                    ::grost::__private::reflection::TagReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            VecCombinations,
                            <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag,
                            ::grost::__private::flavors::Groto,
                            4u32,
                        >,
                    >,
                > as ::grost::__private::reflection::Reflectable<
                    VecCombinations,
                >>::REFLECTION
                    .len()
            };
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::reflection::Reflectable<VecCombinations>
        for ::grost::__private::reflection::WireSchemaTypeReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
                VecCombinations,
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::WireType,
                ::grost::__private::flavors::Groto,
                4u32,
            >,
        > {
            type Reflection = <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::WireType;
            const REFLECTION: &Self::Reflection = &{
                <<Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Groto,
                >>::Format as ::grost::__private::flavors::WireFormat<
                    ::grost::__private::flavors::Groto,
                >>::WIRE_TYPE
            };
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::reflection::Reflectable<VecCombinations>
        for VecCombinations {
            type Reflection = ::grost::__private::reflection::SchemaType;
            const REFLECTION: &'static Self::Reflection = &{
                ::grost::__private::reflection::SchemaType::Object(
                    &::grost::__private::reflection::ObjectBuilder {
                        name: "VecCombinations",
                        description: "",
                        fields: &[
                            &::grost::__private::reflection::ObjectFieldBuilder {
                                name: "optional_vec_optional_vec",
                                description: "",
                                ty: <::grost::__private::reflection::SchemaTypeReflection<
                                    Option<Vec<Option<Vec<u16>>>>,
                                > as ::grost::__private::reflection::Reflectable<
                                    Option<Vec<Option<Vec<u16>>>>,
                                >>::REFLECTION,
                            }
                                .build(),
                        ],
                    }
                        .build(),
                )
            };
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::reflection::Reflectable<VecCombinations>
        for ::grost::__private::reflection::Reflection<
            VecCombinations,
            ::grost::__private::reflection::Object,
            ::grost::__private::flavors::Groto,
        > {
            type Reflection = ::grost::__private::reflection::Object;
            const REFLECTION: &'static Self::Reflection = &{
                ::grost::__private::reflection::ObjectBuilder {
                    name: "VecCombinations",
                    description: "",
                    fields: &[
                        &::grost::__private::reflection::ObjectFieldBuilder {
                            name: "optional_vec_optional_vec",
                            description: "",
                            ty: <::grost::__private::reflection::SchemaTypeReflection<
                                Option<Vec<Option<Vec<u16>>>>,
                            > as ::grost::__private::reflection::Reflectable<
                                Option<Vec<Option<Vec<u16>>>>,
                            >>::REFLECTION,
                        }
                            .build(),
                    ],
                }
                    .build()
            };
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl VecCombinations {
            /// Returns the reflection of the struct.
            #[inline]
            pub const fn reflection() -> ::grost::__private::reflection::Reflection<
                Self,
                ::grost::__private::reflection::Object,
                ::grost::__private::flavors::Groto,
            >
            where
                ::grost::__private::flavors::Groto: ::grost::__private::flavors::Flavor,
            {
                ::grost::__private::reflection::Reflection::new()
            }
            /// Returns the field reflection of the field `VecCombinations.optional_vec_optional_vec`.
            #[inline]
            const fn optional_vec_optional_vec_reflection() -> ::grost::__private::reflection::ObjectFieldReflection<
                VecCombinations,
                ::grost::__private::reflection::ObjectField,
                ::grost::__private::flavors::Groto,
                4u32,
            >
            where
                ::grost::__private::flavors::Groto: ::grost::__private::flavors::Flavor,
            {
                ::grost::__private::reflection::ObjectFieldReflection::new()
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::grost::__private::indexer::Indexable<::grost::__private::flavors::Groto>
        for VecCombinations {
            type Indexer = VecCombinationsIndex;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl VecCombinationsIndex {
            /// Returns the field reflection of the corresponding field.
            #[allow(non_camel_case_types, clippy::type_complexity)]
            #[inline]
            pub const fn reflection(
                &self,
            ) -> &'static ::grost::__private::reflection::ObjectField {
                match self {
                    Self::OptionalVecOptionalVec => {
                        <::grost::__private::reflection::ObjectFieldReflection<
                            VecCombinations,
                            ::grost::__private::reflection::ObjectField,
                            ::grost::__private::flavors::Groto,
                            4u32,
                        > as ::grost::__private::reflection::Reflectable<
                            VecCombinations,
                        >>::REFLECTION
                    }
                }
            }
        }
        #[automatically_derived]
        impl VecCombinationsIndex {
            /// The number of variants of this field indexer.
            pub const VARIANTS: ::core::primitive::usize = 1usize;
            /// The first field indexer.
            pub const FIRST: Self = Self::OptionalVecOptionalVec;
            /// The last field indexer.
            pub const LAST: Self = Self::OptionalVecOptionalVec;
            /// Returns the next field indexer.
            ///
            /// Returns `None` if there are no more fields.
            #[inline]
            pub const fn next(&self) -> ::core::option::Option<Self> {
                match self {
                    Self::OptionalVecOptionalVec => ::core::option::Option::None,
                }
            }
            /// Returns the previous field indexer.
            ///
            /// Returns `None` if there are no previous fields.
            #[inline]
            pub const fn prev(&self) -> ::core::option::Option<Self> {
                match self {
                    Self::OptionalVecOptionalVec => ::core::option::Option::None,
                }
            }
            /// Returns the remaining number of fields.
            #[inline]
            pub const fn remaining(&self) -> ::core::primitive::usize {
                Self::LAST.index() - self.index()
            }
            const fn index(&self) -> ::core::primitive::usize {
                match self {
                    Self::OptionalVecOptionalVec => 0usize,
                }
            }
        }
        #[automatically_derived]
        impl ::core::iter::Iterator for VecCombinationsIndex {
            type Item = Self;
            fn next(&mut self) -> ::core::option::Option<Self> {
                Self::next(self)
            }
            fn size_hint(
                &self,
            ) -> (
                ::core::primitive::usize,
                ::core::option::Option<::core::primitive::usize>,
            ) {
                let remaining = self.remaining();
                (remaining, ::core::option::Option::Some(remaining))
            }
        }
        #[automatically_derived]
        impl ::core::iter::DoubleEndedIterator for VecCombinationsIndex {
            fn next_back(&mut self) -> ::core::option::Option<Self> {
                Self::prev(self)
            }
        }
        #[automatically_derived]
        impl ::core::iter::FusedIterator for VecCombinationsIndex {}
        #[automatically_derived]
        impl ::core::iter::ExactSizeIterator for VecCombinationsIndex {
            fn len(&self) -> ::core::primitive::usize {
                self.remaining()
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Groto,
        > for VecCombinations {
            type Selector = VecCombinationsSelector;
            fn is_empty(&self) -> ::core::primitive::bool {
                false
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Groto,
        > for PartialVecCombinations {
            type Selector = VecCombinationsSelector;
            fn is_empty(&self) -> ::core::primitive::bool {
                Self::is_empty(self)
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>
        for PartialVecCombinationsRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > {
            type Selector = VecCombinationsSelector;
            fn is_empty(&self) -> ::core::primitive::bool {
                Self::is_empty(self)
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::fmt::Debug for VecCombinationsSelector {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                f.debug_struct("VecCombinationsSelector")
                    .field("optional_vec_optional_vec", &self.optional_vec_optional_vec)
                    .finish()
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::cmp::PartialEq for VecCombinationsSelector {
            fn eq(&self, other: &Self) -> ::core::primitive::bool {
                self.optional_vec_optional_vec == other.optional_vec_optional_vec
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::cmp::Eq for VecCombinationsSelector {}
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::hash::Hash for VecCombinationsSelector {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                self.optional_vec_optional_vec.hash(state);
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for VecCombinationsSelector {
            fn clone(&self) -> Self {
                Self {
                    optional_vec_optional_vec: ::core::clone::Clone::clone(
                        &self.optional_vec_optional_vec,
                    ),
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::marker::Copy for VecCombinationsSelector
        where
            <Option<
                Vec<Option<Vec<u16>>>,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Groto,
            >>::Selector: ::core::marker::Copy,
        {}
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::grost::__private::selection::Selector<::grost::__private::flavors::Groto>
        for VecCombinationsSelector {
            const ALL: Self = Self::all();
            const DEFAULT: Self = Self::new();
            const NONE: Self = Self::empty();
            fn selected(&self) -> ::core::primitive::usize {
                Self::selected(self)
            }
            fn unselected(&self) -> ::core::primitive::usize {
                Self::unselected(self)
            }
            fn flip(&mut self) -> &mut Self {
                <<Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::flip(&mut self.optional_vec_optional_vec);
                self
            }
            fn merge(&mut self, other: Self) -> &mut Self {
                <<Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::merge(
                    &mut self.optional_vec_optional_vec,
                    other.optional_vec_optional_vec,
                );
                self
            }
        }
        #[automatically_derived]
        impl VecCombinationsSelector {
            /// Returns a selector with the default values.
            #[inline]
            pub const fn new() -> Self {
                Self {
                    optional_vec_optional_vec: <<Option<
                        Vec<Option<Vec<u16>>>,
                    > as ::grost::__private::selection::Selectable<
                        ::grost::__private::flavors::Groto,
                    >>::Selector as ::grost::__private::selection::Selector<
                        ::grost::__private::flavors::Groto,
                    >>::DEFAULT,
                }
            }
            /// Returns a selector which selects nothing.
            #[inline]
            pub const fn empty() -> Self {
                Self {
                    optional_vec_optional_vec: <<Option<
                        Vec<Option<Vec<u16>>>,
                    > as ::grost::__private::selection::Selectable<
                        ::grost::__private::flavors::Groto,
                    >>::Selector as ::grost::__private::selection::Selector<
                        ::grost::__private::flavors::Groto,
                    >>::NONE,
                }
            }
            /// Returns a selector which selects all.
            #[inline]
            pub const fn all() -> Self {
                Self {
                    optional_vec_optional_vec: <<Option<
                        Vec<Option<Vec<u16>>>,
                    > as ::grost::__private::selection::Selectable<
                        ::grost::__private::flavors::Groto,
                    >>::Selector as ::grost::__private::selection::Selector<
                        ::grost::__private::flavors::Groto,
                    >>::ALL,
                }
            }
            /// Returns `true` if the selector selects nothing.
            #[inline]
            pub fn is_empty(&self) -> ::core::primitive::bool {
                <<Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::is_empty(&self.optional_vec_optional_vec)
            }
            /// Returns `true` if the selector selects all.
            #[inline]
            pub fn is_all(&self) -> ::core::primitive::bool {
                <<Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::is_all(&self.optional_vec_optional_vec)
            }
            /// Returns the number of selected fields.
            #[inline]
            pub fn selected(&self) -> ::core::primitive::usize {
                let mut num = 0;
                if self.is_optional_vec_optional_vec_selected() {
                    num += 1;
                }
                num
            }
            /// Returns the number of unselected fields.
            #[inline]
            pub fn unselected(&self) -> ::core::primitive::usize {
                let mut num = 0;
                if self.is_optional_vec_optional_vec_unselected() {
                    num += 1;
                }
                num
            }
            /// Returns an iterator over the selected fields.
            #[inline]
            pub fn iter_selected<'__grost_lifetime__>(
                &'__grost_lifetime__ self,
            ) -> VecCombinationsSelectorIter<'__grost_lifetime__, true> {
                VecCombinationsSelectorIter::new(self, self.selected())
            }
            /// Returns an iterator over the unselected fields.
            #[inline]
            pub fn iter_unselected<'__grost_lifetime__>(
                &'__grost_lifetime__ self,
            ) -> VecCombinationsSelectorIter<'__grost_lifetime__, false> {
                VecCombinationsSelectorIter::new(self, self.unselected())
            }
            /// Returns `true` if such field is selected.
            #[inline]
            pub fn is_selected(
                &self,
                idx: VecCombinationsIndex,
            ) -> ::core::primitive::bool {
                match idx {
                    VecCombinationsIndex::OptionalVecOptionalVec => {
                        self.is_optional_vec_optional_vec_selected()
                    }
                }
            }
            /// Returns `true` if such field is unselected.
            #[inline]
            pub fn is_unselected(
                &self,
                idx: VecCombinationsIndex,
            ) -> ::core::primitive::bool {
                match idx {
                    VecCombinationsIndex::OptionalVecOptionalVec => {
                        self.is_optional_vec_optional_vec_unselected()
                    }
                }
            }
            /// Select the `VecCombinations.optional_vec_optional_vec` field
            #[inline]
            pub fn select_optional_vec_optional_vec(&mut self) -> &mut Self {
                self
                    .optional_vec_optional_vec = <<Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::DEFAULT;
                self
            }
            /// Unselect the `VecCombinations.optional_vec_optional_vec` field
            #[inline]
            pub fn unselect_optional_vec_optional_vec(&mut self) -> &mut Self {
                self
                    .optional_vec_optional_vec = <<Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::NONE;
                self
            }
            /// Update the `VecCombinations.optional_vec_optional_vec` field
            #[inline]
            pub fn update_optional_vec_optional_vec(
                &mut self,
                value: <Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector,
            ) -> &mut Self {
                self.optional_vec_optional_vec = value;
                self
            }
            /// Set or unset the `VecCombinations.optional_vec_optional_vec` field
            #[inline]
            pub fn maybe_optional_vec_optional_vec(
                mut self,
                val: <Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector,
            ) -> Self {
                self.optional_vec_optional_vec = val;
                self
            }
            /// Get a reference to the selector of `VecCombinations.optional_vec_optional_vec` field
            #[inline]
            pub const fn optional_vec_optional_vec_ref(
                &self,
            ) -> &<Option<
                Vec<Option<Vec<u16>>>,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Groto,
            >>::Selector {
                &self.optional_vec_optional_vec
            }
            /// Get a mutable reference to the selector of `VecCombinations.optional_vec_optional_vec` field
            #[inline]
            pub const fn optional_vec_optional_vec_mut(
                &mut self,
            ) -> &mut <Option<
                Vec<Option<Vec<u16>>>,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Groto,
            >>::Selector {
                &mut self.optional_vec_optional_vec
            }
            /// Set the `VecCombinations.optional_vec_optional_vec` field
            #[inline]
            pub fn with_optional_vec_optional_vec(mut self) -> Self {
                self
                    .optional_vec_optional_vec = <<Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::DEFAULT;
                self
            }
            /// Unset the `VecCombinations.optional_vec_optional_vec` field
            #[inline]
            pub fn without_optional_vec_optional_vec(mut self) -> Self {
                self
                    .optional_vec_optional_vec = <<Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::NONE;
                self
            }
            /// Returns `true` if the `VecCombinations.optional_vec_optional_vec` field is selected
            #[inline]
            pub fn is_optional_vec_optional_vec_selected(
                &self,
            ) -> ::core::primitive::bool {
                !<<Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::is_empty(&self.optional_vec_optional_vec)
            }
            /// Returns `true` if the `VecCombinations.optional_vec_optional_vec` field is unselected
            #[inline]
            pub fn is_optional_vec_optional_vec_unselected(
                &self,
            ) -> ::core::primitive::bool {
                <<Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::is_empty(&self.optional_vec_optional_vec)
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            const __GROST_SELECTED__: ::core::primitive::bool,
        > VecCombinationsSelectorIter<'__grost_lifetime__, __GROST_SELECTED__> {
            #[inline]
            const fn new(
                selector: &'__grost_lifetime__ VecCombinationsSelector,
                num: ::core::primitive::usize,
            ) -> Self {
                Self {
                    selector,
                    index: ::core::option::Option::Some(VecCombinationsIndex::FIRST),
                    num,
                    yielded: 0,
                }
            }
            /// Returns the exact remaining length of the iterator.
            #[inline]
            pub const fn remaining(&self) -> ::core::primitive::usize {
                self.num - self.yielded
            }
            /// Returns `true` if the iterator is empty.
            #[inline]
            pub const fn is_empty(&self) -> ::core::primitive::bool {
                self.remaining() == 0
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            const __GROST_SELECTED__: ::core::primitive::bool,
        > ::core::iter::Iterator
        for VecCombinationsSelectorIter<'__grost_lifetime__, __GROST_SELECTED__> {
            type Item = &'static ::grost::__private::reflection::ObjectField;
            fn next(&mut self) -> ::core::option::Option<Self::Item> {
                if self.yielded >= self.num {
                    return ::core::option::Option::None;
                }
                let mut current_index = self.index;
                while let ::core::option::Option::Some(idx) = current_index {
                    if __GROST_SELECTED__ {
                        if self.selector.is_selected(idx) {
                            self.index = idx.next();
                            self.yielded += 1;
                            return ::core::option::Option::Some(idx.reflection());
                        }
                    } else if self.selector.is_unselected(idx) {
                        self.index = idx.next();
                        self.yielded += 1;
                        return ::core::option::Option::Some(idx.reflection());
                    }
                    current_index = idx.next();
                }
                ::core::option::Option::None
            }
            fn size_hint(
                &self,
            ) -> (
                ::core::primitive::usize,
                ::core::option::Option<::core::primitive::usize>,
            ) {
                let remaining = self.remaining();
                (remaining, ::core::option::Option::Some(remaining))
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            const __GROST_SELECTED__: ::core::primitive::bool,
        > ::core::iter::FusedIterator
        for VecCombinationsSelectorIter<'__grost_lifetime__, __GROST_SELECTED__> {}
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            const __GROST_SELECTED__: ::core::primitive::bool,
        > ::core::iter::ExactSizeIterator
        for VecCombinationsSelectorIter<'__grost_lifetime__, __GROST_SELECTED__> {
            fn len(&self) -> ::core::primitive::usize {
                self.remaining()
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::convert::Transform<
            Self,
            Self,
            ::grost::__private::flavors::groto::LengthDelimited,
            ::grost::__private::flavors::Groto,
        > for VecCombinations {
            fn transform(
                input: Self,
            ) -> ::core::result::Result<
                Self,
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
            > {
                ::core::result::Result::Ok(input)
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > ::grost::__private::decode::Decode<
            '__grost_lifetime__,
            ::grost::__private::flavors::Groto,
            ::grost::__private::flavors::groto::LengthDelimited,
            Self,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > for PartialVecCombinations {
            fn decode(
                context: &'__grost_lifetime__ <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Context,
                src: __GROST_READ_BUFFER__,
            ) -> ::core::result::Result<
                (::core::primitive::usize, Self),
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
            >
            where
                Self: ::core::marker::Sized + '__grost_lifetime__,
                __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf
                    + '__grost_lifetime__,
                __GROST_UNKNOWN_BUFFER__: ::grost::__private::buffer::Buffer<
                        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Unknown<
                            __GROST_READ_BUFFER__,
                        >,
                    > + '__grost_lifetime__,
            {
                <PartialVecCombinations as ::grost::__private::decode::Decode<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Groto,
                    ::grost::__private::flavors::groto::LengthDelimited,
                    PartialVecCombinationsRef<
                        '__grost_lifetime__,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >>::decode(context, src)
                    .and_then(|(read, input)| {
                        <PartialVecCombinations as ::grost::__private::convert::Transform<
                            PartialVecCombinationsRef<
                                '__grost_lifetime__,
                                __GROST_READ_BUFFER__,
                                __GROST_UNKNOWN_BUFFER__,
                            >,
                            PartialVecCombinations,
                            ::grost::__private::flavors::groto::LengthDelimited,
                            ::grost::__private::flavors::Groto,
                        >>::transform(input)
                            .map(|input| (read, input))
                    })
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > ::grost::__private::decode::Decode<
            '__grost_lifetime__,
            ::grost::__private::flavors::Groto,
            ::grost::__private::flavors::groto::LengthDelimited,
            Self,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > for VecCombinations {
            fn decode(
                context: &'__grost_lifetime__ <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Context,
                src: __GROST_READ_BUFFER__,
            ) -> ::core::result::Result<
                (::core::primitive::usize, Self),
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
            >
            where
                Self: ::core::marker::Sized + '__grost_lifetime__,
                __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf
                    + '__grost_lifetime__,
                __GROST_UNKNOWN_BUFFER__: ::grost::__private::buffer::Buffer<
                        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Unknown<
                            __GROST_READ_BUFFER__,
                        >,
                    > + '__grost_lifetime__,
            {
                <PartialVecCombinations as ::grost::__private::decode::Decode<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Groto,
                    ::grost::__private::flavors::groto::LengthDelimited,
                    PartialVecCombinations,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >>::decode(context, src)
                    .and_then(|(read, input)| {
                        <VecCombinations as ::grost::__private::convert::Transform<
                            PartialVecCombinations,
                            VecCombinations,
                            ::grost::__private::flavors::groto::LengthDelimited,
                            ::grost::__private::flavors::Groto,
                        >>::transform(input)
                            .map(|input| (read, input))
                    })
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::convert::Transform<
            Self,
            Self,
            ::grost::__private::flavors::groto::LengthDelimited,
            ::grost::__private::flavors::Groto,
        > for PartialVecCombinations {
            fn transform(
                input: Self,
            ) -> ::core::result::Result<
                Self,
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
            > {
                ::core::result::Result::Ok(input)
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::convert::Transform<
            PartialVecCombinations,
            VecCombinations,
            ::grost::__private::flavors::groto::LengthDelimited,
            ::grost::__private::flavors::Groto,
        > for VecCombinations {
            fn transform(
                input: PartialVecCombinations,
            ) -> ::core::result::Result<
                VecCombinations,
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
            > {
                ::core::result::Result::Ok(Self {
                    optional_vec_optional_vec: input.optional_vec_optional_vec,
                })
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::convert::PartialTransform<
            Self,
            ::core::option::Option<Self>,
            ::grost::__private::flavors::groto::LengthDelimited,
            ::grost::__private::flavors::Groto,
        > for PartialVecCombinations {
            fn partial_transform(
                input: PartialVecCombinations,
                selector: &VecCombinationsSelector,
            ) -> ::core::result::Result<
                ::core::option::Option<PartialVecCombinations>,
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
            > {
                let mut this = Self::new();
                if selector.is_optional_vec_optional_vec_selected() {
                    this
                        .optional_vec_optional_vec = <Option<
                        Vec<Option<Vec<u16>>>,
                    > as ::grost::__private::convert::PartialTransform<
                        <Option<
                            Vec<Option<Vec<u16>>>,
                        > as ::grost::__private::convert::State<
                            ::grost::__private::convert::Partial<
                                ::grost::__private::flavors::Groto,
                            >,
                        >>::Output,
                        <Option<
                            Vec<Option<Vec<u16>>>,
                        > as ::grost::__private::convert::State<
                            ::grost::__private::convert::Partial<
                                ::grost::__private::flavors::Groto,
                            >,
                        >>::Output,
                        <Option<
                            Vec<Option<Vec<u16>>>,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Groto,
                        >>::Format,
                        ::grost::__private::flavors::Groto,
                    >>::partial_transform(
                        this.optional_vec_optional_vec,
                        selector.optional_vec_optional_vec_ref(),
                    )?;
                }
                ::core::result::Result::Ok((!this.is_empty()).then_some(this))
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > ::grost::__private::convert::Transform<
            Self,
            Self,
            ::grost::__private::flavors::groto::LengthDelimited,
            ::grost::__private::flavors::Groto,
        >
        for PartialVecCombinationsRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > {
            fn transform(
                input: Self,
            ) -> ::core::result::Result<
                Self,
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
            > {
                ::core::result::Result::Ok(input)
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            '__grost_decode_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > ::grost::__private::decode::Decode<
            '__grost_decode_lifetime__,
            ::grost::__private::flavors::Groto,
            ::grost::__private::flavors::groto::LengthDelimited,
            Self,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        >
        for PartialVecCombinationsRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        >
        where
            '__grost_decode_lifetime__: '__grost_lifetime__,
        {
            fn decode(
                context: &'__grost_decode_lifetime__ <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Context,
                src: __GROST_READ_BUFFER__,
            ) -> ::core::result::Result<
                (::core::primitive::usize, Self),
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
            >
            where
                Self: ::core::marker::Sized + '__grost_decode_lifetime__,
                __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf
                    + '__grost_decode_lifetime__,
                __GROST_UNKNOWN_BUFFER__: ::grost::__private::buffer::Buffer<
                        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Unknown<
                            __GROST_READ_BUFFER__,
                        >,
                    > + '__grost_decode_lifetime__,
            {
                let buf = src.as_bytes();
                let buf_len = buf.len();
                let mut this = Self::new();
                let mut offset = 0;
                while offset < buf_len {
                    let (read, identifier) = <<::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier as ::grost::__private::flavors::Identifier<
                        ::grost::__private::flavors::Groto,
                    >>::decode::<&[::core::primitive::u8]>(&buf[offset..])?;
                    offset += read;
                    match &identifier {
                        <::grost::__private::reflection::IdentifierReflection<
                            ::grost::__private::reflection::ObjectFieldReflection<
                                VecCombinations,
                                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
                                ::grost::__private::flavors::Groto,
                                4u32,
                            >,
                        > as ::grost::__private::reflection::Reflectable<
                            VecCombinations,
                        >>::REFLECTION => {
                            if offset >= buf_len {
                                return ::core::result::Result::Err(
                                    ::core::convert::Into::into(
                                        ::grost::__private::error::Error::buffer_underflow(),
                                    ),
                                );
                            }
                            if this.optional_vec_optional_vec.is_some() {
                                return ::core::result::Result::Err(
                                    ::core::convert::Into::into(
                                        ::grost::__private::error::Error::duplicated_field(
                                            "optional_vec_optional_vec",
                                            ::core::any::type_name::<Option<Vec<Option<Vec<u16>>>>>(),
                                            *<::grost::__private::reflection::IdentifierReflection<
                                                ::grost::__private::reflection::ObjectFieldReflection<
                                                    VecCombinations,
                                                    <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
                                                    ::grost::__private::flavors::Groto,
                                                    4u32,
                                                >,
                                            > as ::grost::__private::reflection::Reflectable<
                                                VecCombinations,
                                            >>::REFLECTION,
                                        ),
                                    ),
                                );
                            }
                            let (read, value) = (<Option<
                                Vec<Option<Vec<u16>>>,
                            > as ::grost::__private::decode::Decode<
                                '__grost_decode_lifetime__,
                                ::grost::__private::flavors::Groto,
                                <Option<
                                    Vec<Option<Vec<u16>>>,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Groto,
                                >>::Format,
                                <Option<
                                    Vec<Option<Vec<u16>>>,
                                > as ::grost::__private::convert::State<
                                    ::grost::__private::convert::PartialRef<
                                        '__grost_lifetime__,
                                        __GROST_READ_BUFFER__,
                                        __GROST_UNKNOWN_BUFFER__,
                                        <Option<
                                            Vec<Option<Vec<u16>>>,
                                        > as ::grost::__private::flavors::DefaultWireFormat<
                                            ::grost::__private::flavors::Groto,
                                        >>::Format,
                                        ::grost::__private::flavors::Groto,
                                    >,
                                >>::Output,
                                __GROST_READ_BUFFER__,
                                __GROST_UNKNOWN_BUFFER__,
                            >>::decode)(context, src.slice(offset..))?;
                            this.optional_vec_optional_vec = value;
                            offset += read;
                        }
                        _ => {
                            if context.err_on_unknown() {
                                return ::core::result::Result::Err(
                                    ::core::convert::Into::into(
                                        ::grost::__private::error::Error::unknown_identifier(
                                            ::core::any::type_name::<VecCombinations>(),
                                            identifier,
                                        ),
                                    ),
                                );
                            }
                            if context.skip_unknown() {
                                if offset >= buf_len {
                                    return ::core::result::Result::Err(
                                        ::core::convert::Into::into(
                                            ::grost::__private::error::Error::buffer_underflow(),
                                        ),
                                    );
                                }
                                offset
                                    += <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::skip(
                                        context,
                                        identifier.wire_type(),
                                        src.slice(offset..),
                                    )?;
                            } else {
                                let encoded_len = <<::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier as ::grost::__private::flavors::Identifier<
                                    ::grost::__private::flavors::Groto,
                                >>::encoded_len(&identifier);
                                let (read, unknown) = <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::decode_unknown(
                                    context,
                                    src.slice(offset - encoded_len..),
                                )?;
                                offset += read;
                                let unknowns_mut = this
                                    .__grost_unknown_buffer__
                                    .get_or_insert_with(|| __GROST_UNKNOWN_BUFFER__::new());
                                if let ::core::option::Option::Some(unknown) = unknowns_mut
                                    .push(unknown)
                                {
                                    let len = ::grost::__private::Buffer::len(unknowns_mut);
                                    return ::core::result::Result::Err(
                                        ::core::convert::Into::into(
                                            ::grost::__private::error::Error::buffer_overflow(
                                                len,
                                                ::core::num::NonZeroUsize::new(len + 1).unwrap(),
                                            ),
                                        ),
                                    );
                                }
                            }
                        }
                    }
                }
                ::core::result::Result::Ok((offset, this))
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            '__grost_decode_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > ::grost::__private::decode::Decode<
            '__grost_decode_lifetime__,
            ::grost::__private::flavors::Groto,
            ::grost::__private::flavors::groto::LengthDelimited,
            PartialVecCombinationsRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > for VecCombinations
        where
            '__grost_decode_lifetime__: '__grost_lifetime__,
        {
            fn decode(
                context: &'__grost_decode_lifetime__ <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Context,
                src: __GROST_READ_BUFFER__,
            ) -> ::core::result::Result<
                (
                    ::core::primitive::usize,
                    PartialVecCombinationsRef<
                        '__grost_lifetime__,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
                ),
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
            >
            where
                PartialVecCombinationsRef<
                    '__grost_lifetime__,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >: ::core::marker::Sized + '__grost_decode_lifetime__,
                __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf
                    + '__grost_decode_lifetime__,
                __GROST_UNKNOWN_BUFFER__: ::grost::__private::buffer::Buffer<
                        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Unknown<
                            __GROST_READ_BUFFER__,
                        >,
                    > + '__grost_decode_lifetime__,
            {
                <PartialVecCombinationsRef<
                    '__grost_lifetime__,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                > as ::grost::__private::decode::Decode<
                    '__grost_decode_lifetime__,
                    ::grost::__private::flavors::Groto,
                    ::grost::__private::flavors::groto::LengthDelimited,
                    PartialVecCombinationsRef<
                        '__grost_lifetime__,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >>::decode(context, src)
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            '__grost_decode_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > ::grost::__private::decode::Decode<
            '__grost_decode_lifetime__,
            ::grost::__private::flavors::Groto,
            ::grost::__private::flavors::groto::LengthDelimited,
            PartialVecCombinationsRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > for PartialVecCombinations
        where
            '__grost_decode_lifetime__: '__grost_lifetime__,
        {
            fn decode(
                context: &'__grost_decode_lifetime__ <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Context,
                src: __GROST_READ_BUFFER__,
            ) -> ::core::result::Result<
                (
                    ::core::primitive::usize,
                    PartialVecCombinationsRef<
                        '__grost_lifetime__,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
                ),
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
            >
            where
                PartialVecCombinationsRef<
                    '__grost_lifetime__,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >: ::core::marker::Sized + '__grost_decode_lifetime__,
                __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf
                    + '__grost_decode_lifetime__,
                __GROST_UNKNOWN_BUFFER__: ::grost::__private::buffer::Buffer<
                        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Unknown<
                            __GROST_READ_BUFFER__,
                        >,
                    > + '__grost_decode_lifetime__,
            {
                <PartialVecCombinationsRef<
                    '__grost_lifetime__,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                > as ::grost::__private::decode::Decode<
                    '__grost_decode_lifetime__,
                    ::grost::__private::flavors::Groto,
                    ::grost::__private::flavors::groto::LengthDelimited,
                    PartialVecCombinationsRef<
                        '__grost_lifetime__,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >>::decode(context, src)
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > ::grost::__private::convert::Transform<
            PartialVecCombinationsRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
            PartialVecCombinations,
            ::grost::__private::flavors::groto::LengthDelimited,
            ::grost::__private::flavors::Groto,
        > for PartialVecCombinations
        where
            __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf
                + '__grost_lifetime__,
            __GROST_UNKNOWN_BUFFER__: ::grost::__private::buffer::Buffer<
                    <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Unknown<
                        __GROST_READ_BUFFER__,
                    >,
                > + '__grost_lifetime__,
        {
            fn transform(
                input: PartialVecCombinationsRef<
                    '__grost_lifetime__,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            ) -> ::core::result::Result<
                Self,
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
            > {
                let mut this = Self::new();
                this
                    .optional_vec_optional_vec = <Option<
                    Vec<Option<Vec<u16>>>,
                > as ::grost::__private::convert::Transform<
                    <Option<
                        Vec<Option<Vec<u16>>>,
                    > as ::grost::__private::convert::State<
                        ::grost::__private::convert::PartialRef<
                            '__grost_lifetime__,
                            __GROST_READ_BUFFER__,
                            __GROST_UNKNOWN_BUFFER__,
                            <Option<
                                Vec<Option<Vec<u16>>>,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Groto,
                            >>::Format,
                            ::grost::__private::flavors::Groto,
                        >,
                    >>::Output,
                    <Option<
                        Vec<Option<Vec<u16>>>,
                    > as ::grost::__private::convert::State<
                        ::grost::__private::convert::Partial<
                            ::grost::__private::flavors::Groto,
                        >,
                    >>::Output,
                    <Option<
                        Vec<Option<Vec<u16>>>,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Groto,
                    >>::Format,
                    ::grost::__private::flavors::Groto,
                >>::transform(input.optional_vec_optional_vec)?;
                ::core::result::Result::Ok(this)
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > ::grost::__private::convert::PartialTransform<
            PartialVecCombinationsRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
            ::core::option::Option<PartialVecCombinations>,
            ::grost::__private::flavors::groto::LengthDelimited,
            ::grost::__private::flavors::Groto,
        > for PartialVecCombinations
        where
            __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf
                + '__grost_lifetime__,
            __GROST_UNKNOWN_BUFFER__: ::grost::__private::buffer::Buffer<
                    <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Unknown<
                        __GROST_READ_BUFFER__,
                    >,
                > + '__grost_lifetime__,
        {
            fn partial_transform(
                input: PartialVecCombinationsRef<
                    '__grost_lifetime__,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
                selector: &VecCombinationsSelector,
            ) -> ::core::result::Result<
                ::core::option::Option<Self>,
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
            > {
                let mut this = Self::new();
                if selector.is_optional_vec_optional_vec_selected() {
                    this
                        .optional_vec_optional_vec = <Option<
                        Vec<Option<Vec<u16>>>,
                    > as ::grost::__private::convert::PartialTransform<
                        <Option<
                            Vec<Option<Vec<u16>>>,
                        > as ::grost::__private::convert::State<
                            ::grost::__private::convert::PartialRef<
                                '__grost_lifetime__,
                                __GROST_READ_BUFFER__,
                                __GROST_UNKNOWN_BUFFER__,
                                <Option<
                                    Vec<Option<Vec<u16>>>,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Groto,
                                >>::Format,
                                ::grost::__private::flavors::Groto,
                            >,
                        >>::Output,
                        <Option<
                            Vec<Option<Vec<u16>>>,
                        > as ::grost::__private::convert::State<
                            ::grost::__private::convert::Partial<
                                ::grost::__private::flavors::Groto,
                            >,
                        >>::Output,
                        <Option<
                            Vec<Option<Vec<u16>>>,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Groto,
                        >>::Format,
                        ::grost::__private::flavors::Groto,
                    >>::partial_transform(
                        input.optional_vec_optional_vec,
                        selector.optional_vec_optional_vec_ref(),
                    )?;
                }
                ::core::result::Result::Ok((!this.is_empty()).then_some(this))
            }
        }
    };
}

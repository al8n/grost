mod optional_bytes {
    #![allow(warnings)]
    use grost::Object;
    struct OptionalBytes {
        #[grost(tag = 1, optional(bytes))]
        string: Option<std::vec::Vec<u8>>,
    }
    /// Field indexer for the struct [`OptionalBytes`]
    #[repr(u32)]
    enum OptionalBytesIndex {
        /// The field indexer for the field `string`
        String = 1u32,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for OptionalBytesIndex {
        #[inline]
        fn clone(&self) -> OptionalBytesIndex {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for OptionalBytesIndex {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for OptionalBytesIndex {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for OptionalBytesIndex {
        #[inline]
        fn eq(&self, other: &OptionalBytesIndex) -> bool {
            true
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for OptionalBytesIndex {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for OptionalBytesIndex {
        #[inline]
        fn partial_cmp(
            &self,
            other: &OptionalBytesIndex,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for OptionalBytesIndex {
        #[inline]
        fn cmp(&self, other: &OptionalBytesIndex) -> ::core::cmp::Ordering {
            ::core::cmp::Ordering::Equal
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for OptionalBytesIndex {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for OptionalBytesIndex {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "String")
        }
    }
    /// The selection type for [`OptionalBytes`]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    struct OptionalBytesSelector {
        string: <Option<
            std::vec::Vec<u8>,
        > as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Groto,
        >>::Selector,
    }
    /// An iterator over the selected fields of the [`OptionalBytesSelector`]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    struct OptionalBytesSelectorIter<
        '__grost_lifetime__,
        const __GROST_SELECTED__: ::core::primitive::bool = true,
    > {
        selector: &'__grost_lifetime__ OptionalBytesSelector,
        index: ::core::option::Option<OptionalBytesIndex>,
        num: ::core::primitive::usize,
        yielded: ::core::primitive::usize,
    }
    /// Partial struct for the [`PartialOptionalBytes`]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    struct PartialOptionalBytes {
        string: <Option<
            std::vec::Vec<u8>,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >>::Output,
    }
    /// Partial reference struct for the struct [`OptionalBytes`]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    struct PartialOptionalBytesRef<
        '__grost_lifetime__,
        __GROST_READ_BUFFER__,
        __GROST_UNKNOWN_BUFFER__,
    > {
        __grost_unknown_buffer__: ::core::option::Option<__GROST_UNKNOWN_BUFFER__>,
        __grost_read_buffer__: ::core::option::Option<__GROST_READ_BUFFER__>,
        string: <Option<
            std::vec::Vec<u8>,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::PartialRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
                <Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Groto,
                >>::Format,
                ::grost::__private::flavors::Groto,
            >,
        >>::Output,
    }
    const _: () = {
        impl OptionalBytes {
            /// Returns a reference to the `string`
            #[inline]
            const fn string_ref(
                &self,
            ) -> ::core::option::Option<
                &<Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flatten,
                >>::Output,
            > {
                self.string.as_ref()
            }
            /// Returns a mutable reference to the `string`
            #[inline]
            const fn string_mut(
                &mut self,
            ) -> ::core::option::Option<
                &mut <Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flatten,
                >>::Output,
            > {
                self.string.as_mut()
            }
            /// Set the `string` to the given value
            #[inline]
            fn set_string(
                &mut self,
                value: <Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flatten,
                >>::Output,
            ) -> &mut Self {
                self.string = ::core::option::Option::Some(value);
                self
            }
            /// Set the `string` to the given value
            #[inline]
            fn with_string(
                mut self,
                value: <Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flatten,
                >>::Output,
            ) -> Self {
                self.string = ::core::option::Option::Some(value);
                self
            }
            /// Returns a reference to the `string` if it is not `None`
            ///
            /// ## Panics
            ///
            /// - Panics if the `string` is `None`
            #[inline]
            const fn unwrap_string_ref(
                &self,
            ) -> &<Option<
                std::vec::Vec<u8>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output {
                match self.string.as_ref() {
                    ::core::option::Option::Some(value) => value,
                    ::core::option::Option::None => {
                        ::core::panicking::panic_fmt(format_args!("`string` is `None`"));
                    }
                }
            }
            /// Returns a mutable reference to the `string` if it is not `None`
            ///
            /// ## Panics
            ///
            /// - Panics if the `string` is `None`
            #[inline]
            const fn unwrap_string_mut(
                &mut self,
            ) -> &mut <Option<
                std::vec::Vec<u8>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output {
                match self.string.as_mut() {
                    ::core::option::Option::Some(value) => value,
                    ::core::option::Option::None => {
                        ::core::panicking::panic_fmt(format_args!("`string` is `None`"));
                    }
                }
            }
            /// Takes the value of `string` out if it is not `None`
            #[inline]
            const fn take_string(&mut self) -> Option<std::vec::Vec<u8>> {
                self.string.take()
            }
            /// Clear the value of `string`
            #[inline]
            fn clear_string(&mut self) -> &mut Self {
                self.string = ::core::option::Option::None;
                self
            }
            /// Update the `string` to the given value or clear the `string`
            #[inline]
            fn update_string(&mut self, value: Option<std::vec::Vec<u8>>) -> &mut Self {
                self.string = value;
                self
            }
            /// Clear the value of `string`
            #[inline]
            fn without_string(mut self) -> Self {
                self.string = ::core::option::Option::None;
                self
            }
            /// Update the `string` to the given value or clear the `string`
            #[inline]
            fn maybe_string(mut self, value: Option<std::vec::Vec<u8>>) -> Self {
                self.string = value;
                self
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            __GROST_FLATTEN_STATE__: ?::core::marker::Sized,
        > ::grost::__private::convert::State<
            ::grost::__private::convert::Flatten<__GROST_FLATTEN_STATE__>,
        > for OptionalBytes {
            type Output = Self;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::convert::State<
            ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        > for OptionalBytes {
            type Output = PartialOptionalBytes;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::convert::State<
            ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        > for PartialOptionalBytes {
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
        for PartialOptionalBytesRef<
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
        > for OptionalBytes {
            type Output = PartialOptionalBytesRef<
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
        > for PartialOptionalBytes {
            type Output = PartialOptionalBytesRef<
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
        for PartialOptionalBytesRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > {
            type Output = Self;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::default::Default for PartialOptionalBytes {
            fn default() -> Self {
                Self::new()
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            __GROST_FLATTEN_STATE__: ?::core::marker::Sized,
        > ::grost::__private::convert::State<
            ::grost::__private::convert::Flatten<__GROST_FLATTEN_STATE__>,
        > for PartialOptionalBytes {
            type Output = Self;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl PartialOptionalBytes {
            /// Creates an empty partial struct.
            #[inline]
            pub const fn new() -> Self {
                Self {
                    string: ::core::option::Option::None,
                }
            }
            /// Returns `true` if the partial object is empty.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.string.is_none()
            }
            /// Returns a reference to the `string`
            #[inline]
            const fn string_ref(
                &self,
            ) -> ::core::option::Option<
                &<<Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Partial<
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flatten,
                >>::Output,
            > {
                self.string.as_ref()
            }
            /// Returns a mutable reference to the `string`
            #[inline]
            const fn string_mut(
                &mut self,
            ) -> ::core::option::Option<
                &mut <<Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Partial<
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flatten,
                >>::Output,
            > {
                self.string.as_mut()
            }
            /// Returns a reference to the `string` if it is not `None`
            ///
            /// ## Panics
            ///
            /// - Panics if the `string` is `None`
            #[inline]
            const fn unwrap_string_ref(
                &self,
            ) -> &<<Option<
                std::vec::Vec<u8>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
            >>::Output as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output {
                match self.string.as_ref() {
                    ::core::option::Option::Some(value) => value,
                    ::core::option::Option::None => {
                        ::core::panicking::panic_fmt(format_args!("`string` is `None`"));
                    }
                }
            }
            /// Returns a mutable reference to the `string` if it is not `None`
            ///
            /// ## Panics
            ///
            /// - Panics if the `string` is `None`
            #[inline]
            const fn unwrap_string_mut(
                &mut self,
            ) -> &mut <<Option<
                std::vec::Vec<u8>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
            >>::Output as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output {
                match self.string.as_mut() {
                    ::core::option::Option::Some(value) => value,
                    ::core::option::Option::None => {
                        ::core::panicking::panic_fmt(format_args!("`string` is `None`"));
                    }
                }
            }
            /// Takes the value of `string` out if it is not `None`
            #[inline]
            const fn take_string(
                &mut self,
            ) -> <Option<
                std::vec::Vec<u8>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
            >>::Output {
                self.string.take()
            }
            /// Clear the value of `string`
            #[inline]
            fn clear_string(&mut self) -> &mut Self {
                self.string = ::core::option::Option::None;
                self
            }
            /// Set the `string` to the given value
            #[inline]
            fn set_string(
                &mut self,
                value: <<Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Partial<
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flatten,
                >>::Output,
            ) -> &mut Self {
                self.string = ::core::option::Option::Some(value);
                self
            }
            /// Update the `string` to the given value or clear the `string`
            #[inline]
            fn update_string(
                &mut self,
                value: <Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Partial<
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output,
            ) -> &mut Self {
                self.string = value;
                self
            }
            /// Set the `string` to the given value
            #[inline]
            fn with_string(
                mut self,
                value: <<Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Partial<
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flatten,
                >>::Output,
            ) -> Self {
                self.string = ::core::option::Option::Some(value);
                self
            }
            /// Clear the value of `string`
            #[inline]
            fn without_string(mut self) -> Self {
                self.string = ::core::option::Option::None;
                self
            }
            /// Update the `string` to the given value or clear the `string`
            #[inline]
            fn maybe_string(
                mut self,
                value: <Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Partial<
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output,
            ) -> Self {
                self.string = value;
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
        for PartialOptionalBytesRef<
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
            ::grost::__private::convert::Flatten<__GROST_FLATTEN_STATE__>,
        >
        for PartialOptionalBytesRef<
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
        > PartialOptionalBytesRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > {
            /// Creates an empty partial struct.
            #[inline]
            pub const fn new() -> Self {
                Self {
                    string: ::core::option::Option::None,
                    __grost_unknown_buffer__: ::core::option::Option::None,
                    __grost_read_buffer__: ::core::option::Option::None,
                }
            }
            /// Returns `true` if the partial struct is empty, which means all fields are `None`.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                self.__grost_unknown_buffer__.is_none() && self.string.is_none()
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
            /// Returns a reference to the `string`
            #[inline]
            const fn string_ref(
                &self,
            ) -> ::core::option::Option<
                &<<Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::PartialRef<
                        '__grost_lifetime__,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                        <Option<
                            std::vec::Vec<u8>,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Groto,
                        >>::Format,
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flatten,
                >>::Output,
            > {
                self.string.as_ref()
            }
            /// Returns a mutable reference to the `string`
            #[inline]
            const fn string_mut(
                &mut self,
            ) -> ::core::option::Option<
                &mut <<Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::PartialRef<
                        '__grost_lifetime__,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                        <Option<
                            std::vec::Vec<u8>,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Groto,
                        >>::Format,
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flatten,
                >>::Output,
            > {
                self.string.as_mut()
            }
            /// Returns a reference to the `string` if it is not `None`
            ///
            /// ## Panics
            ///
            /// - Panics if the `string` is `None`
            #[inline]
            const fn unwrap_string_ref(
                &self,
            ) -> &<<Option<
                std::vec::Vec<u8>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::PartialRef<
                    '__grost_lifetime__,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                    <Option<
                        std::vec::Vec<u8>,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Groto,
                    >>::Format,
                    ::grost::__private::flavors::Groto,
                >,
            >>::Output as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output {
                match self.string.as_ref() {
                    ::core::option::Option::Some(value) => value,
                    ::core::option::Option::None => {
                        ::core::panicking::panic_fmt(format_args!("`string` is `None`"));
                    }
                }
            }
            /// Returns a mutable reference to the `string` if it is not `None`
            ///
            /// ## Panics
            ///
            /// - Panics if the `string` is `None`
            #[inline]
            const fn unwrap_string_mut(
                &mut self,
            ) -> &mut <<Option<
                std::vec::Vec<u8>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::PartialRef<
                    '__grost_lifetime__,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                    <Option<
                        std::vec::Vec<u8>,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Groto,
                    >>::Format,
                    ::grost::__private::flavors::Groto,
                >,
            >>::Output as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output {
                match self.string.as_mut() {
                    ::core::option::Option::Some(value) => value,
                    ::core::option::Option::None => {
                        ::core::panicking::panic_fmt(format_args!("`string` is `None`"));
                    }
                }
            }
            /// Takes the value of `string` out if it is not `None`
            #[inline]
            const fn take_string(
                &mut self,
            ) -> <Option<
                std::vec::Vec<u8>,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::PartialRef<
                    '__grost_lifetime__,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                    <Option<
                        std::vec::Vec<u8>,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Groto,
                    >>::Format,
                    ::grost::__private::flavors::Groto,
                >,
            >>::Output {
                self.string.take()
            }
            /// Clear the value of `string`
            #[inline]
            fn clear_string(&mut self) -> &mut Self {
                self.string = ::core::option::Option::None;
                self
            }
            /// Set the `string` to the given value
            #[inline]
            fn set_string(
                &mut self,
                value: <<Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::PartialRef<
                        '__grost_lifetime__,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                        <Option<
                            std::vec::Vec<u8>,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Groto,
                        >>::Format,
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flatten,
                >>::Output,
            ) -> &mut Self {
                self.string = ::core::option::Option::Some(value);
                self
            }
            /// Update the `string` to the given value or clear the `string`
            #[inline]
            fn update_string(
                &mut self,
                value: <Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::PartialRef<
                        '__grost_lifetime__,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                        <Option<
                            std::vec::Vec<u8>,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Groto,
                        >>::Format,
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output,
            ) -> &mut Self {
                self.string = value;
                self
            }
            /// Set the `string` to the given value
            #[inline]
            fn with_string(
                mut self,
                value: <<Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::PartialRef<
                        '__grost_lifetime__,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                        <Option<
                            std::vec::Vec<u8>,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Groto,
                        >>::Format,
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flatten,
                >>::Output,
            ) -> Self {
                self.string = ::core::option::Option::Some(value);
                self
            }
            /// Clear the value of `string`
            #[inline]
            fn without_string(mut self) -> Self {
                self.string = ::core::option::Option::None;
                self
            }
            /// Update the `string` to the given value or clear the `string`
            #[inline]
            fn maybe_string(
                mut self,
                value: <Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::PartialRef<
                        '__grost_lifetime__,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                        <Option<
                            std::vec::Vec<u8>,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Groto,
                        >>::Format,
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output,
            ) -> Self {
                self.string = value;
                self
            }
        }
        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl ::grost::__private::reflection::Reflectable<OptionalBytes>
        for ::grost::__private::reflection::ObjectFieldReflection<
            OptionalBytes,
            ::grost::__private::reflection::ObjectField,
            ::grost::__private::flavors::Groto,
            1u32,
        > {
            type Reflection = ::grost::__private::reflection::ObjectField;
            const REFLECTION: &'static Self::Reflection = &{
                ::grost::__private::reflection::ObjectFieldBuilder {
                    name: "string",
                    description: "",
                    ty: <::grost::__private::reflection::SchemaTypeReflection<
                        Option<std::vec::Vec<u8>>,
                    > as ::grost::__private::reflection::Reflectable<
                        Option<std::vec::Vec<u8>>,
                    >>::REFLECTION,
                }
                    .build()
            };
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::reflection::Reflectable<OptionalBytes>
        for ::grost::__private::reflection::WireFormatReflection<
            OptionalBytes,
            ::grost::__private::flavors::Groto,
            1u32,
        > {
            type Reflection = <Option<
                std::vec::Vec<u8>,
            > as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Groto,
            >>::Format;
            const REFLECTION: &'static Self::Reflection = &{
                <<Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Groto,
                >>::Format as ::grost::__private::flavors::WireFormat<
                    ::grost::__private::flavors::Groto,
                >>::SELF
            };
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::reflection::Reflectable<OptionalBytes>
        for ::grost::__private::reflection::IdentifierReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
                OptionalBytes,
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
                ::grost::__private::flavors::Groto,
                1u32,
            >,
        > {
            type Reflection = <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier;
            const REFLECTION: &Self::Reflection = &{
                (::grost::__private::flavors::groto::Identifier::new)(
                    <<Option<
                        std::vec::Vec<u8>,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Groto,
                    >>::Format as ::grost::__private::flavors::WireFormat<
                        ::grost::__private::flavors::Groto,
                    >>::WIRE_TYPE,
                    (::grost::__private::flavors::groto::Tag::new)(1u32),
                )
            };
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::reflection::Reflectable<OptionalBytes>
        for ::grost::__private::reflection::EncodeReflection<
            ::grost::__private::reflection::IdentifierReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    OptionalBytes,
                    <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
                    ::grost::__private::flavors::Groto,
                    1u32,
                >,
            >,
        > {
            type Reflection = [::core::primitive::u8];
            const REFLECTION: &Self::Reflection = {
                (::grost::__private::flavors::groto::Identifier::encode)(
                        <::grost::__private::reflection::IdentifierReflection<
                            ::grost::__private::reflection::ObjectFieldReflection<
                                OptionalBytes,
                                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
                                ::grost::__private::flavors::Groto,
                                1u32,
                            >,
                        > as ::grost::__private::reflection::Reflectable<
                            OptionalBytes,
                        >>::REFLECTION,
                    )
                    .as_slice()
            };
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::reflection::Reflectable<OptionalBytes>
        for ::grost::__private::reflection::Len<
            ::grost::__private::reflection::EncodeReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        OptionalBytes,
                        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
                        ::grost::__private::flavors::Groto,
                        1u32,
                    >,
                >,
            >,
        > {
            type Reflection = ::core::primitive::usize;
            const REFLECTION: &Self::Reflection = {
                &<::grost::__private::reflection::EncodeReflection<
                    ::grost::__private::reflection::IdentifierReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            OptionalBytes,
                            <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
                            ::grost::__private::flavors::Groto,
                            1u32,
                        >,
                    >,
                > as ::grost::__private::reflection::Reflectable<
                    OptionalBytes,
                >>::REFLECTION
                    .len()
            };
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::reflection::Reflectable<OptionalBytes>
        for ::grost::__private::reflection::TagReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
                OptionalBytes,
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag,
                ::grost::__private::flavors::Groto,
                1u32,
            >,
        > {
            type Reflection = <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag;
            const REFLECTION: &Self::Reflection = &{
                (::grost::__private::flavors::groto::Tag::new)(1u32)
            };
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::reflection::Reflectable<OptionalBytes>
        for ::grost::__private::reflection::EncodeReflection<
            ::grost::__private::reflection::TagReflection<
                ::grost::__private::reflection::ObjectFieldReflection<
                    OptionalBytes,
                    <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag,
                    ::grost::__private::flavors::Groto,
                    1u32,
                >,
            >,
        > {
            type Reflection = [::core::primitive::u8];
            const REFLECTION: &Self::Reflection = {
                (::grost::__private::flavors::groto::Tag::encode)(
                        <::grost::__private::reflection::TagReflection<
                            ::grost::__private::reflection::ObjectFieldReflection<
                                OptionalBytes,
                                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag,
                                ::grost::__private::flavors::Groto,
                                1u32,
                            >,
                        > as ::grost::__private::reflection::Reflectable<
                            OptionalBytes,
                        >>::REFLECTION,
                    )
                    .as_slice()
            };
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::reflection::Reflectable<OptionalBytes>
        for ::grost::__private::reflection::Len<
            ::grost::__private::reflection::EncodeReflection<
                ::grost::__private::reflection::TagReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        OptionalBytes,
                        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag,
                        ::grost::__private::flavors::Groto,
                        1u32,
                    >,
                >,
            >,
        > {
            type Reflection = ::core::primitive::usize;
            const REFLECTION: &Self::Reflection = {
                &<::grost::__private::reflection::EncodeReflection<
                    ::grost::__private::reflection::TagReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            OptionalBytes,
                            <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag,
                            ::grost::__private::flavors::Groto,
                            1u32,
                        >,
                    >,
                > as ::grost::__private::reflection::Reflectable<
                    OptionalBytes,
                >>::REFLECTION
                    .len()
            };
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::reflection::Reflectable<OptionalBytes>
        for ::grost::__private::reflection::WireSchemaTypeReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
                OptionalBytes,
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::WireType,
                ::grost::__private::flavors::Groto,
                1u32,
            >,
        > {
            type Reflection = <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::WireType;
            const REFLECTION: &Self::Reflection = &{
                <<Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Groto,
                >>::Format as ::grost::__private::flavors::WireFormat<
                    ::grost::__private::flavors::Groto,
                >>::WIRE_TYPE
            };
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::reflection::Reflectable<OptionalBytes>
        for OptionalBytes {
            type Reflection = ::grost::__private::reflection::SchemaType;
            const REFLECTION: &'static Self::Reflection = &{
                ::grost::__private::reflection::SchemaType::Object(
                    &::grost::__private::reflection::ObjectBuilder {
                        name: "OptionalBytes",
                        description: "",
                        fields: &[
                            &::grost::__private::reflection::ObjectFieldBuilder {
                                name: "string",
                                description: "",
                                ty: <::grost::__private::reflection::SchemaTypeReflection<
                                    Option<std::vec::Vec<u8>>,
                                > as ::grost::__private::reflection::Reflectable<
                                    Option<std::vec::Vec<u8>>,
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
        impl ::grost::__private::reflection::Reflectable<OptionalBytes>
        for ::grost::__private::reflection::Reflection<
            OptionalBytes,
            ::grost::__private::reflection::Object,
            ::grost::__private::flavors::Groto,
        > {
            type Reflection = ::grost::__private::reflection::Object;
            const REFLECTION: &'static Self::Reflection = &{
                ::grost::__private::reflection::ObjectBuilder {
                    name: "OptionalBytes",
                    description: "",
                    fields: &[
                        &::grost::__private::reflection::ObjectFieldBuilder {
                            name: "string",
                            description: "",
                            ty: <::grost::__private::reflection::SchemaTypeReflection<
                                Option<std::vec::Vec<u8>>,
                            > as ::grost::__private::reflection::Reflectable<
                                Option<std::vec::Vec<u8>>,
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
        impl OptionalBytes {
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
            /// Returns the field reflection of the field `OptionalBytes.string`.
            #[inline]
            const fn string_reflection() -> ::grost::__private::reflection::ObjectFieldReflection<
                OptionalBytes,
                ::grost::__private::reflection::ObjectField,
                ::grost::__private::flavors::Groto,
                1u32,
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
        for OptionalBytes {
            type Indexer = OptionalBytesIndex;
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl OptionalBytesIndex {
            /// Returns the field reflection of the corresponding field.
            #[allow(non_camel_case_types, clippy::type_complexity)]
            #[inline]
            pub const fn reflection(
                &self,
            ) -> &'static ::grost::__private::reflection::ObjectField {
                match self {
                    Self::String => {
                        <::grost::__private::reflection::ObjectFieldReflection<
                            OptionalBytes,
                            ::grost::__private::reflection::ObjectField,
                            ::grost::__private::flavors::Groto,
                            1u32,
                        > as ::grost::__private::reflection::Reflectable<
                            OptionalBytes,
                        >>::REFLECTION
                    }
                }
            }
        }
        #[automatically_derived]
        impl OptionalBytesIndex {
            /// The number of variants of this field indexer.
            pub const VARIANTS: ::core::primitive::usize = 1usize;
            /// The first field indexer.
            pub const FIRST: Self = Self::String;
            /// The last field indexer.
            pub const LAST: Self = Self::String;
            /// Returns the next field indexer.
            ///
            /// Returns `None` if there are no more fields.
            #[inline]
            pub const fn next(&self) -> ::core::option::Option<Self> {
                match self {
                    Self::String => ::core::option::Option::None,
                }
            }
            /// Returns the previous field indexer.
            ///
            /// Returns `None` if there are no previous fields.
            #[inline]
            pub const fn prev(&self) -> ::core::option::Option<Self> {
                match self {
                    Self::String => ::core::option::Option::None,
                }
            }
            /// Returns the remaining number of fields.
            #[inline]
            pub const fn remaining(&self) -> ::core::primitive::usize {
                Self::LAST.index() - self.index()
            }
            const fn index(&self) -> ::core::primitive::usize {
                match self {
                    Self::String => 0usize,
                }
            }
        }
        #[automatically_derived]
        impl ::core::iter::Iterator for OptionalBytesIndex {
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
        impl ::core::iter::DoubleEndedIterator for OptionalBytesIndex {
            fn next_back(&mut self) -> ::core::option::Option<Self> {
                Self::prev(self)
            }
        }
        #[automatically_derived]
        impl ::core::iter::FusedIterator for OptionalBytesIndex {}
        #[automatically_derived]
        impl ::core::iter::ExactSizeIterator for OptionalBytesIndex {
            fn len(&self) -> ::core::primitive::usize {
                self.remaining()
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Groto,
        > for OptionalBytes {
            type Selector = OptionalBytesSelector;
            fn is_empty(&self) -> ::core::primitive::bool {
                false
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Groto,
        > for PartialOptionalBytes {
            type Selector = OptionalBytesSelector;
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
        for PartialOptionalBytesRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > {
            type Selector = OptionalBytesSelector;
            fn is_empty(&self) -> ::core::primitive::bool {
                Self::is_empty(self)
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::fmt::Debug for OptionalBytesSelector {
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::result::Result<(), ::core::fmt::Error> {
                f.debug_struct("OptionalBytesSelector")
                    .field("string", &self.string)
                    .finish()
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::cmp::PartialEq for OptionalBytesSelector {
            fn eq(&self, other: &Self) -> ::core::primitive::bool {
                self.string == other.string
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::cmp::Eq for OptionalBytesSelector {}
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::hash::Hash for OptionalBytesSelector {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                self.string.hash(state);
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for OptionalBytesSelector {
            fn clone(&self) -> Self {
                Self {
                    string: ::core::clone::Clone::clone(&self.string),
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::marker::Copy for OptionalBytesSelector
        where
            <Option<
                std::vec::Vec<u8>,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Groto,
            >>::Selector: ::core::marker::Copy,
        {}
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::grost::__private::selection::Selector<::grost::__private::flavors::Groto>
        for OptionalBytesSelector {
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
                    std::vec::Vec<u8>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::flip(&mut self.string);
                self
            }
            fn merge(&mut self, other: Self) -> &mut Self {
                <<Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::merge(&mut self.string, other.string);
                self
            }
        }
        #[automatically_derived]
        impl OptionalBytesSelector {
            /// Returns a selector with the default values.
            #[inline]
            pub const fn new() -> Self {
                Self {
                    string: <<Option<
                        std::vec::Vec<u8>,
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
                    string: <<Option<
                        std::vec::Vec<u8>,
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
                    string: <<Option<
                        std::vec::Vec<u8>,
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
                    std::vec::Vec<u8>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::is_empty(&self.string)
            }
            /// Returns `true` if the selector selects all.
            #[inline]
            pub fn is_all(&self) -> ::core::primitive::bool {
                <<Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::is_all(&self.string)
            }
            /// Returns the number of selected fields.
            #[inline]
            pub fn selected(&self) -> ::core::primitive::usize {
                let mut num = 0;
                if self.is_string_selected() {
                    num += 1;
                }
                num
            }
            /// Returns the number of unselected fields.
            #[inline]
            pub fn unselected(&self) -> ::core::primitive::usize {
                let mut num = 0;
                if self.is_string_unselected() {
                    num += 1;
                }
                num
            }
            /// Returns an iterator over the selected fields.
            #[inline]
            pub fn iter_selected<'__grost_lifetime__>(
                &'__grost_lifetime__ self,
            ) -> OptionalBytesSelectorIter<'__grost_lifetime__, true> {
                OptionalBytesSelectorIter::new(self, self.selected())
            }
            /// Returns an iterator over the unselected fields.
            #[inline]
            pub fn iter_unselected<'__grost_lifetime__>(
                &'__grost_lifetime__ self,
            ) -> OptionalBytesSelectorIter<'__grost_lifetime__, false> {
                OptionalBytesSelectorIter::new(self, self.unselected())
            }
            /// Returns `true` if such field is selected.
            #[inline]
            pub fn is_selected(
                &self,
                idx: OptionalBytesIndex,
            ) -> ::core::primitive::bool {
                match idx {
                    OptionalBytesIndex::String => self.is_string_selected(),
                }
            }
            /// Returns `true` if such field is unselected.
            #[inline]
            pub fn is_unselected(
                &self,
                idx: OptionalBytesIndex,
            ) -> ::core::primitive::bool {
                match idx {
                    OptionalBytesIndex::String => self.is_string_unselected(),
                }
            }
            /// Select the `OptionalBytes.string` field
            #[inline]
            pub fn select_string(&mut self) -> &mut Self {
                self
                    .string = <<Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::DEFAULT;
                self
            }
            /// Unselect the `OptionalBytes.string` field
            #[inline]
            pub fn unselect_string(&mut self) -> &mut Self {
                self
                    .string = <<Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::NONE;
                self
            }
            /// Update the `OptionalBytes.string` field
            #[inline]
            pub fn update_string(
                &mut self,
                value: <Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector,
            ) -> &mut Self {
                self.string = value;
                self
            }
            /// Set or unset the `OptionalBytes.string` field
            #[inline]
            pub fn maybe_string(
                mut self,
                val: <Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector,
            ) -> Self {
                self.string = val;
                self
            }
            /// Get a reference to the selector of `OptionalBytes.string` field
            #[inline]
            pub const fn string_ref(
                &self,
            ) -> &<Option<
                std::vec::Vec<u8>,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Groto,
            >>::Selector {
                &self.string
            }
            /// Get a mutable reference to the selector of `OptionalBytes.string` field
            #[inline]
            pub const fn string_mut(
                &mut self,
            ) -> &mut <Option<
                std::vec::Vec<u8>,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Groto,
            >>::Selector {
                &mut self.string
            }
            /// Set the `OptionalBytes.string` field
            #[inline]
            pub fn with_string(mut self) -> Self {
                self
                    .string = <<Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::DEFAULT;
                self
            }
            /// Unset the `OptionalBytes.string` field
            #[inline]
            pub fn without_string(mut self) -> Self {
                self
                    .string = <<Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::NONE;
                self
            }
            /// Returns `true` if the `OptionalBytes.string` field is selected
            #[inline]
            pub fn is_string_selected(&self) -> ::core::primitive::bool {
                !<<Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::is_empty(&self.string)
            }
            /// Returns `true` if the `OptionalBytes.string` field is unselected
            #[inline]
            pub fn is_string_unselected(&self) -> ::core::primitive::bool {
                <<Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::is_empty(&self.string)
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            const __GROST_SELECTED__: ::core::primitive::bool,
        > OptionalBytesSelectorIter<'__grost_lifetime__, __GROST_SELECTED__> {
            #[inline]
            const fn new(
                selector: &'__grost_lifetime__ OptionalBytesSelector,
                num: ::core::primitive::usize,
            ) -> Self {
                Self {
                    selector,
                    index: ::core::option::Option::Some(OptionalBytesIndex::FIRST),
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
        for OptionalBytesSelectorIter<'__grost_lifetime__, __GROST_SELECTED__> {
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
        for OptionalBytesSelectorIter<'__grost_lifetime__, __GROST_SELECTED__> {}
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<
            '__grost_lifetime__,
            const __GROST_SELECTED__: ::core::primitive::bool,
        > ::core::iter::ExactSizeIterator
        for OptionalBytesSelectorIter<'__grost_lifetime__, __GROST_SELECTED__> {
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
        > for OptionalBytes {
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
        > for PartialOptionalBytes {
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
                <PartialOptionalBytes as ::grost::__private::decode::Decode<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Groto,
                    ::grost::__private::flavors::groto::LengthDelimited,
                    PartialOptionalBytesRef<
                        '__grost_lifetime__,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >>::decode(context, src)
                    .and_then(|(read, input)| {
                        <PartialOptionalBytes as ::grost::__private::convert::Transform<
                            PartialOptionalBytesRef<
                                '__grost_lifetime__,
                                __GROST_READ_BUFFER__,
                                __GROST_UNKNOWN_BUFFER__,
                            >,
                            PartialOptionalBytes,
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
        > for OptionalBytes {
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
                <PartialOptionalBytes as ::grost::__private::decode::Decode<
                    '__grost_lifetime__,
                    ::grost::__private::flavors::Groto,
                    ::grost::__private::flavors::groto::LengthDelimited,
                    PartialOptionalBytes,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >>::decode(context, src)
                    .and_then(|(read, input)| {
                        <OptionalBytes as ::grost::__private::convert::Transform<
                            PartialOptionalBytes,
                            OptionalBytes,
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
        > for PartialOptionalBytes {
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
            PartialOptionalBytes,
            OptionalBytes,
            ::grost::__private::flavors::groto::LengthDelimited,
            ::grost::__private::flavors::Groto,
        > for OptionalBytes {
            fn transform(
                input: PartialOptionalBytes,
            ) -> ::core::result::Result<
                OptionalBytes,
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
            > {
                ::core::result::Result::Ok(Self { string: input.string })
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl ::grost::__private::convert::PartialTransform<
            Self,
            ::core::option::Option<Self>,
            ::grost::__private::flavors::groto::LengthDelimited,
            ::grost::__private::flavors::Groto,
        > for PartialOptionalBytes {
            fn partial_transform(
                input: PartialOptionalBytes,
                selector: &OptionalBytesSelector,
            ) -> ::core::result::Result<
                ::core::option::Option<PartialOptionalBytes>,
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
            > {
                let mut this = Self::new();
                if selector.is_string_selected() {
                    this
                        .string = <<Option<
                        std::vec::Vec<u8>,
                    > as ::grost::__private::convert::State<
                        ::grost::__private::convert::Partial<
                            ::grost::__private::flavors::Groto,
                        >,
                    >>::Output as ::grost::__private::convert::PartialTransform<
                        <Option<
                            std::vec::Vec<u8>,
                        > as ::grost::__private::convert::State<
                            ::grost::__private::convert::Partial<
                                ::grost::__private::flavors::Groto,
                            >,
                        >>::Output,
                        <Option<
                            std::vec::Vec<u8>,
                        > as ::grost::__private::convert::State<
                            ::grost::__private::convert::Partial<
                                ::grost::__private::flavors::Groto,
                            >,
                        >>::Output,
                        <Option<
                            std::vec::Vec<u8>,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Groto,
                        >>::Format,
                        ::grost::__private::flavors::Groto,
                    >>::partial_transform(this.string, selector.string_ref())?;
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
        for PartialOptionalBytesRef<
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
        for PartialOptionalBytesRef<
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
                                OptionalBytes,
                                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
                                ::grost::__private::flavors::Groto,
                                1u32,
                            >,
                        > as ::grost::__private::reflection::Reflectable<
                            OptionalBytes,
                        >>::REFLECTION => {
                            if offset >= buf_len {
                                return ::core::result::Result::Err(
                                    ::core::convert::Into::into(
                                        ::grost::__private::error::Error::buffer_underflow(),
                                    ),
                                );
                            }
                            if this.string.is_some() {
                                return ::core::result::Result::Err(
                                    ::core::convert::Into::into(
                                        ::grost::__private::error::Error::duplicated_field(
                                            "string",
                                            ::core::any::type_name::<Option<std::vec::Vec<u8>>>(),
                                            *<::grost::__private::reflection::IdentifierReflection<
                                                ::grost::__private::reflection::ObjectFieldReflection<
                                                    OptionalBytes,
                                                    <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
                                                    ::grost::__private::flavors::Groto,
                                                    1u32,
                                                >,
                                            > as ::grost::__private::reflection::Reflectable<
                                                OptionalBytes,
                                            >>::REFLECTION,
                                        ),
                                    ),
                                );
                            }
                            let (read, value) = (<Option<
                                std::vec::Vec<u8>,
                            > as ::grost::__private::decode::Decode<
                                '__grost_decode_lifetime__,
                                ::grost::__private::flavors::Groto,
                                <Option<
                                    std::vec::Vec<u8>,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Groto,
                                >>::Format,
                                <Option<
                                    std::vec::Vec<u8>,
                                > as ::grost::__private::convert::State<
                                    ::grost::__private::convert::PartialRef<
                                        '__grost_lifetime__,
                                        __GROST_READ_BUFFER__,
                                        __GROST_UNKNOWN_BUFFER__,
                                        <Option<
                                            std::vec::Vec<u8>,
                                        > as ::grost::__private::flavors::DefaultWireFormat<
                                            ::grost::__private::flavors::Groto,
                                        >>::Format,
                                        ::grost::__private::flavors::Groto,
                                    >,
                                >>::Output,
                                __GROST_READ_BUFFER__,
                                __GROST_UNKNOWN_BUFFER__,
                            >>::decode)(context, src.slice(offset..))?;
                            this.string = value;
                            offset += read;
                        }
                        _ => {
                            if context.err_on_unknown() {
                                return ::core::result::Result::Err(
                                    ::core::convert::Into::into(
                                        ::grost::__private::error::Error::unknown_identifier(
                                            ::core::any::type_name::<OptionalBytes>(),
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
            PartialOptionalBytesRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > for OptionalBytes
        where
            '__grost_decode_lifetime__: '__grost_lifetime__,
        {
            fn decode(
                context: &'__grost_decode_lifetime__ <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Context,
                src: __GROST_READ_BUFFER__,
            ) -> ::core::result::Result<
                (
                    ::core::primitive::usize,
                    PartialOptionalBytesRef<
                        '__grost_lifetime__,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
                ),
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
            >
            where
                PartialOptionalBytesRef<
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
                <PartialOptionalBytesRef<
                    '__grost_lifetime__,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                > as ::grost::__private::decode::Decode<
                    '__grost_decode_lifetime__,
                    ::grost::__private::flavors::Groto,
                    ::grost::__private::flavors::groto::LengthDelimited,
                    PartialOptionalBytesRef<
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
            PartialOptionalBytesRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
            __GROST_READ_BUFFER__,
            __GROST_UNKNOWN_BUFFER__,
        > for PartialOptionalBytes
        where
            '__grost_decode_lifetime__: '__grost_lifetime__,
        {
            fn decode(
                context: &'__grost_decode_lifetime__ <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Context,
                src: __GROST_READ_BUFFER__,
            ) -> ::core::result::Result<
                (
                    ::core::primitive::usize,
                    PartialOptionalBytesRef<
                        '__grost_lifetime__,
                        __GROST_READ_BUFFER__,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
                ),
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
            >
            where
                PartialOptionalBytesRef<
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
                <PartialOptionalBytesRef<
                    '__grost_lifetime__,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                > as ::grost::__private::decode::Decode<
                    '__grost_decode_lifetime__,
                    ::grost::__private::flavors::Groto,
                    ::grost::__private::flavors::groto::LengthDelimited,
                    PartialOptionalBytesRef<
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
            PartialOptionalBytesRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
            PartialOptionalBytes,
            ::grost::__private::flavors::groto::LengthDelimited,
            ::grost::__private::flavors::Groto,
        > for PartialOptionalBytes
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
                input: PartialOptionalBytesRef<
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
                    .string = <<Option<
                    std::vec::Vec<u8>,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Partial<
                        ::grost::__private::flavors::Groto,
                    >,
                >>::Output as ::grost::__private::convert::Transform<
                    <Option<
                        std::vec::Vec<u8>,
                    > as ::grost::__private::convert::State<
                        ::grost::__private::convert::PartialRef<
                            '__grost_lifetime__,
                            __GROST_READ_BUFFER__,
                            __GROST_UNKNOWN_BUFFER__,
                            <Option<
                                std::vec::Vec<u8>,
                            > as ::grost::__private::flavors::DefaultWireFormat<
                                ::grost::__private::flavors::Groto,
                            >>::Format,
                            ::grost::__private::flavors::Groto,
                        >,
                    >>::Output,
                    <Option<
                        std::vec::Vec<u8>,
                    > as ::grost::__private::convert::State<
                        ::grost::__private::convert::Partial<
                            ::grost::__private::flavors::Groto,
                        >,
                    >>::Output,
                    <Option<
                        std::vec::Vec<u8>,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Groto,
                    >>::Format,
                    ::grost::__private::flavors::Groto,
                >>::transform(input.string)?;
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
            PartialOptionalBytesRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_UNKNOWN_BUFFER__,
            >,
            ::core::option::Option<PartialOptionalBytes>,
            ::grost::__private::flavors::groto::LengthDelimited,
            ::grost::__private::flavors::Groto,
        > for PartialOptionalBytes
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
                input: PartialOptionalBytesRef<
                    '__grost_lifetime__,
                    __GROST_READ_BUFFER__,
                    __GROST_UNKNOWN_BUFFER__,
                >,
                selector: &OptionalBytesSelector,
            ) -> ::core::result::Result<
                ::core::option::Option<Self>,
                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
            > {
                let mut this = Self::new();
                if selector.is_string_selected() {
                    this
                        .string = <<Option<
                        std::vec::Vec<u8>,
                    > as ::grost::__private::convert::State<
                        ::grost::__private::convert::Partial<
                            ::grost::__private::flavors::Groto,
                        >,
                    >>::Output as ::grost::__private::convert::PartialTransform<
                        <Option<
                            std::vec::Vec<u8>,
                        > as ::grost::__private::convert::State<
                            ::grost::__private::convert::PartialRef<
                                '__grost_lifetime__,
                                __GROST_READ_BUFFER__,
                                __GROST_UNKNOWN_BUFFER__,
                                <Option<
                                    std::vec::Vec<u8>,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Groto,
                                >>::Format,
                                ::grost::__private::flavors::Groto,
                            >,
                        >>::Output,
                        <Option<
                            std::vec::Vec<u8>,
                        > as ::grost::__private::convert::State<
                            ::grost::__private::convert::Partial<
                                ::grost::__private::flavors::Groto,
                            >,
                        >>::Output,
                        <Option<
                            std::vec::Vec<u8>,
                        > as ::grost::__private::flavors::DefaultWireFormat<
                            ::grost::__private::flavors::Groto,
                        >>::Format,
                        ::grost::__private::flavors::Groto,
                    >>::partial_transform(input.string, selector.string_ref())?;
                }
                ::core::result::Result::Ok((!this.is_empty()).then_some(this))
            }
        }
    };
}

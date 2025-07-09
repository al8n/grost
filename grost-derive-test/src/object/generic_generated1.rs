use grost::flavors::groto::LengthDelimited;

mod generic {
    use core::marker::PhantomData;
    use grost::{Object, flavors::groto::LengthDelimited, marker::BytesMarker};
    struct GenericWithWireFormatVec<I> {
        id: Vec<I>,
    }
    /// The selection type for [`GenericWithWireFormatVec`]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    struct GenericWithWireFormatVecSelector<I>
    where
        ::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::WireFormatMarker<
                <Vec<
                    I,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flattened<
                        ::grost::__private::convert::Inner,
                    >,
                >>::Output,
                LengthDelimited,
            >,
        >: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
        >,
        Vec<
            I,
        >: ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>,
    {
        id: <Vec<
            I,
        > as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Groto,
        >>::Selector,
    }

    /// Partial struct for the [`PartialGenericWithWireFormatVec`]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    struct PartialGenericWithWireFormatVec<I>
    where
        Vec<
            I,
        >: ::grost::__private::convert::State<
            ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >,
        <Vec<
            I,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >>::Output: ::core::marker::Sized,
    {
        id: ::core::option::Option<
            <Vec<
                I,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
            >>::Output,
        >,
    }
    /// Partial reference struct for the struct [`GenericWithWireFormatVec`]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    struct PartialGenericWithWireFormatVecRef<
        '__grost_lifetime__,
        I,
        __GROST_READ_BUFFER__,
        __GROST_BUFFER__,
    >
    where
        ::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::WireFormatMarker<
                I,
                LengthDelimited,
            >,
        >: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
        >,
        ::grost::__private::marker::WireFormatMarker<
            <Vec<
                I,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<
                    ::grost::__private::convert::Inner,
                >,
            >>::Output,
            LengthDelimited,
        >: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
        >,
        Vec<
            I,
        >: ::grost::__private::convert::State<
            ::grost::__private::convert::PartialRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_BUFFER__,
                <::grost::__private::marker::ListMarker<
                    Vec<I>,
                    ::grost::__private::marker::WireFormatMarker<
                        I,
                        LengthDelimited,
                    >,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Groto,
                >>::Format,
                ::grost::__private::flavors::Groto,
            >,
        >,
        <Vec<
            I,
        > as ::grost::__private::convert::State<
            ::grost::__private::convert::PartialRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_BUFFER__,
                <::grost::__private::marker::ListMarker<
                    Vec<I>,
                    ::grost::__private::marker::WireFormatMarker<
                        <Vec<
                            I,
                        > as ::grost::__private::convert::State<
                            ::grost::__private::convert::Flattened<
                                ::grost::__private::convert::Inner,
                            >,
                        >>::Output,
                        LengthDelimited,
                    >,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Groto,
                >>::Format,
                ::grost::__private::flavors::Groto,
            >,
        >>::Output: ::core::marker::Sized,
    {
        __grost_buffer__: ::core::option::Option<__GROST_BUFFER__>,
        __grost_read_buffer__: ::core::option::Option<__GROST_READ_BUFFER__>,
        id: ::core::option::Option<
            <Vec<
                I,
            > as ::grost::__private::convert::State<
                ::grost::__private::convert::PartialRef<
                    '__grost_lifetime__,
                    __GROST_READ_BUFFER__,
                    __GROST_BUFFER__,
                    <::grost::__private::marker::ListMarker<
                        Vec<I>,
                        ::grost::__private::marker::WireFormatMarker<
                            I,
                            LengthDelimited,
                        >,
                    > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Groto,
                    >>::Format,
                    ::grost::__private::flavors::Groto,
                >,
            >>::Output,
        >,
    }

}

const _: () = {
        let a: <::grost::__private::marker::ListMarker<
            Vec<Vec<u8>>,
            ::grost::__private::marker::WireFormatMarker<
                Vec<u8>,
                LengthDelimited,
            >,
        > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
        >>::Format = grost::flavors::Packed::<LengthDelimited>;
};
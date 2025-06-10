mod object {
    use darling::{FromField, FromMeta};
    use grost_mir::utils::Output;
    pub struct Field {
        ident: ::core::option::Option<::grost_mir::__private::syn::Ident>,
        vis: ::grost_mir::__private::syn::Visibility,
        ty: ::grost_mir::__private::syn::Type,
        attrs: ::std::vec::Vec<::grost_mir::__private::syn::Attribute>,
        __meta__: ::grost_mir::__private::object::FieldAttribute,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Field {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "Field",
                "ident",
                &self.ident,
                "vis",
                &self.vis,
                "ty",
                &self.ty,
                "attrs",
                &self.attrs,
                "__meta__",
                &&self.__meta__,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Field {
        #[inline]
        fn clone(&self) -> Field {
            Field {
                ident: ::core::clone::Clone::clone(&self.ident),
                vis: ::core::clone::Clone::clone(&self.vis),
                ty: ::core::clone::Clone::clone(&self.ty),
                attrs: ::core::clone::Clone::clone(&self.attrs),
                __meta__: ::core::clone::Clone::clone(&self.__meta__),
            }
        }
    }
    const _: () = {
        use ::grost_mir::__private::{darling, syn};
        #[allow(clippy::large_enum_variant)]
        enum __FieldNestedMeta__ {
            Label(::grost_mir::__private::object::Label),
            Nested(::grost_mir::__private::darling::ast::NestedMeta),
        }
        impl ::grost_mir::__private::syn::parse::Parse for __FieldNestedMeta__ {
            fn parse(
                input: ::grost_mir::__private::syn::parse::ParseStream,
            ) -> ::grost_mir::__private::syn::Result<Self> {
                if ::grost_mir::__private::object::Label::peek(&input)? {
                    let label: ::grost_mir::__private::object::Label = input.parse()?;
                    ::core::result::Result::Ok(Self::Label(label))
                } else {
                    ::grost_mir::__private::darling::ast::NestedMeta::parse(input)
                        .map(Self::Nested)
                }
            }
        }
        struct __FieldCustomMeta__ {}
        #[automatically_derived]
        impl ::core::fmt::Debug for __FieldCustomMeta__ {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "__FieldCustomMeta__")
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for __FieldCustomMeta__ {
            #[inline]
            fn clone(&self) -> __FieldCustomMeta__ {
                __FieldCustomMeta__ {}
            }
        }
        #[automatically_derived]
        #[allow(clippy::manual_unwrap_or_default)]
        impl ::darling::FromMeta for __FieldCustomMeta__ {
            fn from_list(
                __items: &[::darling::export::NestedMeta],
            ) -> ::darling::Result<Self> {
                let mut __errors = ::darling::Error::accumulator();
                for __item in __items {
                    match *__item {
                        ::darling::export::NestedMeta::Meta(ref __inner) => {
                            let __name = ::darling::util::path_to_string(__inner.path());
                            match __name.as_str() {
                                __other => {
                                    __errors
                                        .push(
                                            ::darling::Error::unknown_field(__other).with_span(__inner),
                                        );
                                }
                            }
                        }
                        ::darling::export::NestedMeta::Lit(ref __inner) => {
                            __errors
                                .push(
                                    ::darling::Error::unsupported_format("literal")
                                        .with_span(__inner),
                                );
                        }
                    }
                }
                __errors.finish()?;
                ::darling::export::Ok(Self {})
            }
        }
        struct __FieldFieldMetaWithoutLabel__ {
            #[darling(default)]
            schema: ::grost_mir::__private::utils::SchemaFromMeta,
            #[darling(default)]
            default: ::core::option::Option<::grost_mir::__private::syn::Path>,
            #[darling(default)]
            tag: ::core::option::Option<::core::num::NonZeroU32>,
            #[darling(default)]
            flavor: ::grost_mir::__private::object::meta::FieldFlavorFromMeta,
            #[darling(default)]
            convert: ::grost_mir::__private::object::meta::ConvertFromMeta,
            #[darling(default)]
            partial: ::grost_mir::__private::object::meta::PartialFieldFromMeta,
            #[darling(default)]
            partial_decoded: ::grost_mir::__private::object::meta::PartialDecodedFieldFromMeta,
            #[darling(default)]
            selector: ::grost_mir::__private::object::meta::SelectorFieldFromMeta,
            #[darling(default)]
            copy: ::core::primitive::bool,
            #[darling(default)]
            skip: ::core::primitive::bool,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for __FieldFieldMetaWithoutLabel__ {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "schema",
                    "default",
                    "tag",
                    "flavor",
                    "convert",
                    "partial",
                    "partial_decoded",
                    "selector",
                    "copy",
                    "skip",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.schema,
                    &self.default,
                    &self.tag,
                    &self.flavor,
                    &self.convert,
                    &self.partial,
                    &self.partial_decoded,
                    &self.selector,
                    &self.copy,
                    &&self.skip,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "__FieldFieldMetaWithoutLabel__",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for __FieldFieldMetaWithoutLabel__ {
            #[inline]
            fn clone(&self) -> __FieldFieldMetaWithoutLabel__ {
                __FieldFieldMetaWithoutLabel__ {
                    schema: ::core::clone::Clone::clone(&self.schema),
                    default: ::core::clone::Clone::clone(&self.default),
                    tag: ::core::clone::Clone::clone(&self.tag),
                    flavor: ::core::clone::Clone::clone(&self.flavor),
                    convert: ::core::clone::Clone::clone(&self.convert),
                    partial: ::core::clone::Clone::clone(&self.partial),
                    partial_decoded: ::core::clone::Clone::clone(&self.partial_decoded),
                    selector: ::core::clone::Clone::clone(&self.selector),
                    copy: ::core::clone::Clone::clone(&self.copy),
                    skip: ::core::clone::Clone::clone(&self.skip),
                }
            }
        }
        #[automatically_derived]
        #[allow(clippy::manual_unwrap_or_default)]
        impl ::darling::FromMeta for __FieldFieldMetaWithoutLabel__ {
            fn from_list(
                __items: &[::darling::export::NestedMeta],
            ) -> ::darling::Result<Self> {
                let mut schema: (
                    bool,
                    ::darling::export::Option<
                        ::grost_mir::__private::utils::SchemaFromMeta,
                    >,
                ) = (false, None);
                let mut default: (
                    bool,
                    ::darling::export::Option<
                        ::core::option::Option<::grost_mir::__private::syn::Path>,
                    >,
                ) = (false, None);
                let mut tag: (
                    bool,
                    ::darling::export::Option<
                        ::core::option::Option<::core::num::NonZeroU32>,
                    >,
                ) = (false, None);
                let mut flavor: (
                    bool,
                    ::darling::export::Option<
                        ::grost_mir::__private::object::meta::FieldFlavorFromMeta,
                    >,
                ) = (false, None);
                let mut convert: (
                    bool,
                    ::darling::export::Option<
                        ::grost_mir::__private::object::meta::ConvertFromMeta,
                    >,
                ) = (false, None);
                let mut partial: (
                    bool,
                    ::darling::export::Option<
                        ::grost_mir::__private::object::meta::PartialFieldFromMeta,
                    >,
                ) = (false, None);
                let mut partial_decoded: (
                    bool,
                    ::darling::export::Option<
                        ::grost_mir::__private::object::meta::PartialDecodedFieldFromMeta,
                    >,
                ) = (false, None);
                let mut selector: (
                    bool,
                    ::darling::export::Option<
                        ::grost_mir::__private::object::meta::SelectorFieldFromMeta,
                    >,
                ) = (false, None);
                let mut copy: (
                    bool,
                    ::darling::export::Option<::core::primitive::bool>,
                ) = (false, None);
                let mut skip: (
                    bool,
                    ::darling::export::Option<::core::primitive::bool>,
                ) = (false, None);
                let mut __errors = ::darling::Error::accumulator();
                for __item in __items {
                    match *__item {
                        ::darling::export::NestedMeta::Meta(ref __inner) => {
                            let __name = ::darling::util::path_to_string(__inner.path());
                            match __name.as_str() {
                                "schema" => {
                                    if !schema.0 {
                                        schema = (
                                            true,
                                            __errors
                                                .handle(
                                                    ::darling::export::identity::<
                                                        fn(&::syn::Meta) -> ::darling::Result<_>,
                                                    >(::darling::FromMeta::from_meta)(__inner)
                                                        .map_err(|e| e.with_span(&__inner).at("schema")),
                                                ),
                                        );
                                    } else {
                                        __errors
                                            .push(
                                                ::darling::Error::duplicate_field("schema")
                                                    .with_span(&__inner),
                                            );
                                    }
                                }
                                "default" => {
                                    if !default.0 {
                                        default = (
                                            true,
                                            __errors
                                                .handle(
                                                    ::darling::export::identity::<
                                                        fn(&::syn::Meta) -> ::darling::Result<_>,
                                                    >(::darling::FromMeta::from_meta)(__inner)
                                                        .map_err(|e| e.with_span(&__inner).at("default")),
                                                ),
                                        );
                                    } else {
                                        __errors
                                            .push(
                                                ::darling::Error::duplicate_field("default")
                                                    .with_span(&__inner),
                                            );
                                    }
                                }
                                "tag" => {
                                    if !tag.0 {
                                        tag = (
                                            true,
                                            __errors
                                                .handle(
                                                    ::darling::export::identity::<
                                                        fn(&::syn::Meta) -> ::darling::Result<_>,
                                                    >(::darling::FromMeta::from_meta)(__inner)
                                                        .map_err(|e| e.with_span(&__inner).at("tag")),
                                                ),
                                        );
                                    } else {
                                        __errors
                                            .push(
                                                ::darling::Error::duplicate_field("tag").with_span(&__inner),
                                            );
                                    }
                                }
                                "flavor" => {
                                    if !flavor.0 {
                                        flavor = (
                                            true,
                                            __errors
                                                .handle(
                                                    ::darling::export::identity::<
                                                        fn(&::syn::Meta) -> ::darling::Result<_>,
                                                    >(::darling::FromMeta::from_meta)(__inner)
                                                        .map_err(|e| e.with_span(&__inner).at("flavor")),
                                                ),
                                        );
                                    } else {
                                        __errors
                                            .push(
                                                ::darling::Error::duplicate_field("flavor")
                                                    .with_span(&__inner),
                                            );
                                    }
                                }
                                "convert" => {
                                    if !convert.0 {
                                        convert = (
                                            true,
                                            __errors
                                                .handle(
                                                    ::darling::export::identity::<
                                                        fn(&::syn::Meta) -> ::darling::Result<_>,
                                                    >(::darling::FromMeta::from_meta)(__inner)
                                                        .map_err(|e| e.with_span(&__inner).at("convert")),
                                                ),
                                        );
                                    } else {
                                        __errors
                                            .push(
                                                ::darling::Error::duplicate_field("convert")
                                                    .with_span(&__inner),
                                            );
                                    }
                                }
                                "partial" => {
                                    if !partial.0 {
                                        partial = (
                                            true,
                                            __errors
                                                .handle(
                                                    ::darling::export::identity::<
                                                        fn(&::syn::Meta) -> ::darling::Result<_>,
                                                    >(::darling::FromMeta::from_meta)(__inner)
                                                        .map_err(|e| e.with_span(&__inner).at("partial")),
                                                ),
                                        );
                                    } else {
                                        __errors
                                            .push(
                                                ::darling::Error::duplicate_field("partial")
                                                    .with_span(&__inner),
                                            );
                                    }
                                }
                                "partial_decoded" => {
                                    if !partial_decoded.0 {
                                        partial_decoded = (
                                            true,
                                            __errors
                                                .handle(
                                                    ::darling::export::identity::<
                                                        fn(&::syn::Meta) -> ::darling::Result<_>,
                                                    >(::darling::FromMeta::from_meta)(__inner)
                                                        .map_err(|e| e.with_span(&__inner).at("partial_decoded")),
                                                ),
                                        );
                                    } else {
                                        __errors
                                            .push(
                                                ::darling::Error::duplicate_field("partial_decoded")
                                                    .with_span(&__inner),
                                            );
                                    }
                                }
                                "selector" => {
                                    if !selector.0 {
                                        selector = (
                                            true,
                                            __errors
                                                .handle(
                                                    ::darling::export::identity::<
                                                        fn(&::syn::Meta) -> ::darling::Result<_>,
                                                    >(::darling::FromMeta::from_meta)(__inner)
                                                        .map_err(|e| e.with_span(&__inner).at("selector")),
                                                ),
                                        );
                                    } else {
                                        __errors
                                            .push(
                                                ::darling::Error::duplicate_field("selector")
                                                    .with_span(&__inner),
                                            );
                                    }
                                }
                                "copy" => {
                                    if !copy.0 {
                                        copy = (
                                            true,
                                            __errors
                                                .handle(
                                                    ::darling::export::identity::<
                                                        fn(&::syn::Meta) -> ::darling::Result<_>,
                                                    >(::darling::FromMeta::from_meta)(__inner)
                                                        .map_err(|e| e.with_span(&__inner).at("copy")),
                                                ),
                                        );
                                    } else {
                                        __errors
                                            .push(
                                                ::darling::Error::duplicate_field("copy")
                                                    .with_span(&__inner),
                                            );
                                    }
                                }
                                "skip" => {
                                    if !skip.0 {
                                        skip = (
                                            true,
                                            __errors
                                                .handle(
                                                    ::darling::export::identity::<
                                                        fn(&::syn::Meta) -> ::darling::Result<_>,
                                                    >(::darling::FromMeta::from_meta)(__inner)
                                                        .map_err(|e| e.with_span(&__inner).at("skip")),
                                                ),
                                        );
                                    } else {
                                        __errors
                                            .push(
                                                ::darling::Error::duplicate_field("skip")
                                                    .with_span(&__inner),
                                            );
                                    }
                                }
                                __other => {
                                    __errors
                                        .push(
                                            ::darling::Error::unknown_field_with_alts(
                                                    __other,
                                                    &[
                                                        "schema",
                                                        "default",
                                                        "tag",
                                                        "flavor",
                                                        "convert",
                                                        "partial",
                                                        "partial_decoded",
                                                        "selector",
                                                        "copy",
                                                        "skip",
                                                    ],
                                                )
                                                .with_span(__inner),
                                        );
                                }
                            }
                        }
                        ::darling::export::NestedMeta::Lit(ref __inner) => {
                            __errors
                                .push(
                                    ::darling::Error::unsupported_format("literal")
                                        .with_span(__inner),
                                );
                        }
                    }
                }
                __errors.finish()?;
                ::darling::export::Ok(Self {
                    schema: if let Some(__val) = schema.1 {
                        __val
                    } else {
                        ::darling::export::Default::default()
                    },
                    default: if let Some(__val) = default.1 {
                        __val
                    } else {
                        ::darling::export::Default::default()
                    },
                    tag: if let Some(__val) = tag.1 {
                        __val
                    } else {
                        ::darling::export::Default::default()
                    },
                    flavor: if let Some(__val) = flavor.1 {
                        __val
                    } else {
                        ::darling::export::Default::default()
                    },
                    convert: if let Some(__val) = convert.1 {
                        __val
                    } else {
                        ::darling::export::Default::default()
                    },
                    partial: if let Some(__val) = partial.1 {
                        __val
                    } else {
                        ::darling::export::Default::default()
                    },
                    partial_decoded: if let Some(__val) = partial_decoded.1 {
                        __val
                    } else {
                        ::darling::export::Default::default()
                    },
                    selector: if let Some(__val) = selector.1 {
                        __val
                    } else {
                        ::darling::export::Default::default()
                    },
                    copy: if let Some(__val) = copy.1 {
                        __val
                    } else {
                        ::darling::export::Default::default()
                    },
                    skip: if let Some(__val) = skip.1 {
                        __val
                    } else {
                        ::darling::export::Default::default()
                    },
                })
            }
        }
        struct __FieldFieldMeta__ {
            label: ::grost_mir::__private::object::Label,
            schema: ::grost_mir::__private::utils::SchemaFromMeta,
            default: ::core::option::Option<::grost_mir::__private::syn::Path>,
            tag: ::core::option::Option<::core::num::NonZeroU32>,
            flavor: ::grost_mir::__private::object::meta::FieldFlavorFromMeta,
            convert: ::grost_mir::__private::object::meta::ConvertFromMeta,
            partial: ::grost_mir::__private::object::meta::PartialFieldFromMeta,
            partial_decoded: ::grost_mir::__private::object::meta::PartialDecodedFieldFromMeta,
            selector: ::grost_mir::__private::object::meta::SelectorFieldFromMeta,
            copy: ::core::primitive::bool,
            skip: ::core::primitive::bool,
        }
        impl ::grost_mir::__private::darling::FromMeta for __FieldFieldMeta__ {
            fn from_meta(
                item: &::grost_mir::__private::syn::Meta,
            ) -> ::grost_mir::__private::darling::Result<Self> {
                (match item {
                    ::grost_mir::__private::syn::Meta::Path(_) => Self::from_word(),
                    ::grost_mir::__private::syn::Meta::NameValue(value) => {
                        Self::from_expr(&value.value)
                    }
                    ::grost_mir::__private::syn::Meta::List(value) => {
                        use ::grost_mir::__private::syn::parse::Parser;
                        let punctuated = ::grost_mir::__private::syn::punctuated::Punctuated::<
                            __FieldNestedMeta__,
                            ::syn::token::Comma,
                        >::parse_terminated
                            .parse2(value.tokens.clone())?;
                        let mut nested_meta = ::std::vec::Vec::new();
                        let mut label: ::core::option::Option<
                            ::grost_mir::__private::object::Label,
                        > = ::core::option::Option::None;
                        for item in punctuated {
                            match item {
                                __FieldNestedMeta__::Label(l) => {
                                    if let ::core::option::Option::Some(ref label) = label {
                                        return ::core::result::Result::Err(
                                            ::grost_mir::__private::darling::Error::custom(
                                                ::alloc::__export::must_use({
                                                    let res = ::alloc::fmt::format(
                                                        format_args!(
                                                            "Cannot specify both `{0}` and `{1}` at the same time.",
                                                            label,
                                                            l,
                                                        ),
                                                    );
                                                    res
                                                }),
                                            ),
                                        );
                                    }
                                    label = ::core::option::Option::Some(l);
                                }
                                __FieldNestedMeta__::Nested(value) => {
                                    nested_meta.push(value);
                                }
                            }
                        }
                        let __FieldFieldMetaWithoutLabel__ {
                            schema,
                            default,
                            tag,
                            flavor,
                            convert,
                            partial,
                            partial_decoded,
                            selector,
                            copy,
                            skip,
                        } = <__FieldFieldMetaWithoutLabel__ as ::grost_mir::__private::darling::FromMeta>::from_list(
                            &nested_meta,
                        )?;
                        ::core::result::Result::Ok(Self {
                            label: label
                                .ok_or_else(|| ::grost_mir::__private::darling::Error::custom(
                                    "Expected one of [scalar, bytes, string, object, enum, union, interface, map, set, list, optional] to be specified for a field",
                                ))?,
                            schema,
                            default,
                            tag,
                            flavor,
                            convert,
                            partial,
                            partial_decoded,
                            selector,
                            copy,
                            skip,
                        })
                    }
                })
                    .map_err(|e| e.with_span(item))
            }
        }
        struct __FieldInput__ {
            ident: ::core::option::Option<::grost_mir::__private::syn::Ident>,
            vis: ::grost_mir::__private::syn::Visibility,
            ty: ::grost_mir::__private::syn::Type,
            attrs: ::std::vec::Vec<::grost_mir::__private::syn::Attribute>,
            __meta__: ::grost_mir::__private::object::meta::FieldFromMeta,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for __FieldInput__ {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "__FieldInput__",
                    "ident",
                    &self.ident,
                    "vis",
                    &self.vis,
                    "ty",
                    &self.ty,
                    "attrs",
                    &self.attrs,
                    "__meta__",
                    &&self.__meta__,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for __FieldInput__ {
            #[inline]
            fn clone(&self) -> __FieldInput__ {
                __FieldInput__ {
                    ident: ::core::clone::Clone::clone(&self.ident),
                    vis: ::core::clone::Clone::clone(&self.vis),
                    ty: ::core::clone::Clone::clone(&self.ty),
                    attrs: ::core::clone::Clone::clone(&self.attrs),
                    __meta__: ::core::clone::Clone::clone(&self.__meta__),
                }
            }
        }
        impl ::grost_mir::__private::darling::FromField for __FieldInput__ {
            fn from_field(
                field: &::grost_mir::__private::syn::Field,
            ) -> ::grost_mir::__private::darling::Result<Self> {
                use ::grost_mir::__private::quote::ToTokens;
                let mut errors = ::grost_mir::__private::darling::Error::accumulator();
                let mut fwd_attrs: ::std::vec::Vec<
                    ::grost_mir::__private::syn::Attribute,
                > = ::std::vec::Vec::new();
                let mut meta: ::core::option::Option<__FieldFieldMeta__> = ::core::option::Option::None;
                for attr in &field.attrs {
                    match ::darling::export::ToString::to_string(
                            &attr.path().clone().into_token_stream(),
                        )
                        .as_str()
                    {
                        "grost" => {
                            if meta.is_some() {
                                errors
                                    .push(
                                        ::grost_mir::__private::darling::Error::custom(
                                                ::alloc::__export::must_use({
                                                    let res = ::alloc::fmt::format(
                                                        format_args!(
                                                            "Cannot specify `{0}` attribute multiple times on the same field.",
                                                            "grost",
                                                        ),
                                                    );
                                                    res
                                                }),
                                            )
                                            .with_span(attr),
                                    );
                                continue;
                            }
                            match <__FieldFieldMeta__ as ::grost_mir::__private::darling::FromMeta>::from_meta(
                                &attr.meta,
                            ) {
                                ::core::result::Result::Ok(val) => {
                                    meta = ::core::option::Option::Some(val);
                                }
                                ::core::result::Result::Err(e) => {
                                    errors.push(e);
                                }
                            }
                        }
                        _ => fwd_attrs.push(attr.clone()),
                    }
                }
                match meta {
                    ::core::option::Option::None => {
                        errors
                            .push(
                                ::grost_mir::__private::darling::Error::missing_field(
                                    "grost",
                                ),
                            );
                        errors.finish()
                    }
                    ::core::option::Option::Some(meta) => {
                        errors.finish()?;
                        let __FieldFieldMeta__ {
                            label,
                            schema,
                            default,
                            tag,
                            flavor,
                            convert,
                            partial,
                            partial_decoded,
                            selector,
                            copy,
                            skip,
                        } = meta;
                        ::core::result::Result::Ok(Self {
                            ident: field.ident.clone(),
                            ty: field.ty.clone(),
                            vis: field.vis.clone(),
                            attrs: fwd_attrs,
                            __meta__: ::grost_mir::__private::object::meta::FieldFromMeta {
                                label,
                                schema,
                                default,
                                tag,
                                flavor,
                                convert,
                                partial,
                                partial_decoded,
                                selector,
                                copy,
                                skip,
                            },
                        })
                    }
                }
            }
        }
        impl ::darling::usage::UsesTypeParams for Field {
            fn uses_type_params<'gen>(
                &self,
                options: &::darling::usage::Options,
                type_set: &'gen ::darling::usage::IdentSet,
            ) -> ::darling::usage::IdentRefSet<'gen> {
                self.ty.uses_type_params(options, type_set)
            }
        }
        impl ::darling::usage::UsesLifetimes for Field {
            fn uses_lifetimes<'gen>(
                &self,
                options: &::darling::usage::Options,
                type_set: &'gen ::darling::usage::LifetimeSet,
            ) -> ::darling::usage::LifetimeRefSet<'gen> {
                self.ty.uses_lifetimes(options, type_set)
            }
        }
        impl ::core::convert::TryFrom<__FieldInput__> for Field {
            type Error = ::grost_mir::__private::syn::Error;
            #[inline]
            fn try_from(
                input: __FieldInput__,
            ) -> ::core::result::Result<Self, Self::Error> {
                ::core::result::Result::Ok(Self {
                    ident: input.ident,
                    vis: input.vis,
                    ty: input.ty,
                    attrs: input.attrs,
                    __meta__: input.__meta__.finalize()?,
                })
            }
        }
        impl ::grost_mir::__private::darling::FromField for Field {
            fn from_field(
                field: &::grost_mir::__private::syn::Field,
            ) -> ::grost_mir::__private::darling::Result<Self> {
                <__FieldInput__ as ::grost_mir::__private::darling::FromField>::from_field(
                        field,
                    )
                    .and_then(|field| {
                        ::core::convert::TryInto::try_into(field)
                            .map_err(::grost_mir::__private::darling::Error::from)
                    })
            }
        }
        impl ::grost_mir::__private::object::RawField for Field {
            type Meta = ();
            #[inline]
            fn name(&self) -> &::grost_mir::__private::syn::Ident {
                self.ident
                    .as_ref()
                    .expect("the field of the named struct must have a name")
            }
            #[inline]
            fn ty(&self) -> &::grost_mir::__private::syn::Type {
                &self.ty
            }
            #[inline]
            fn vis(&self) -> &::grost_mir::__private::syn::Visibility {
                &self.vis
            }
            #[inline]
            fn attrs(&self) -> &[::grost_mir::__private::syn::Attribute] {
                &self.attrs
            }
            fn tag(&self) -> ::core::option::Option<::core::num::NonZeroU32> {
                self.__meta__.tag()
            }
            fn flavors(
                &self,
            ) -> &[::grost_mir::__private::object::FieldFlavorAttribute] {
                self.__meta__.flavors()
            }
            fn convert(&self) -> &::grost_mir::__private::object::ConvertAttribute {
                &self.__meta__.convert()
            }
            fn partial(&self) -> &::grost_mir::__private::object::PartialFieldAttribute {
                self.__meta__.partial()
            }
            fn partial_decoded(
                &self,
            ) -> &::grost_mir::__private::object::PartialDecodedFieldAttribute {
                self.__meta__.partial_decoded()
            }
            fn copy(&self) -> ::core::primitive::bool {
                self.__meta__.copy()
            }
            fn skip(&self) -> ::core::primitive::bool {
                self.__meta__.skip()
            }
            fn selector(
                &self,
            ) -> &::grost_mir::__private::object::SelectorFieldAttribute {
                &self.__meta__.selector()
            }
            fn label(&self) -> &::grost_mir::__private::object::Label {
                self.__meta__.label()
            }
            fn schema(&self) -> &::grost_mir::__private::utils::SchemaAttribute {
                &self.__meta__.schema()
            }
            fn default(&self) -> ::core::option::Option<&syn::Path> {
                self.__meta__.default()
            }
            fn meta(&self) -> &Self::Meta {
                &()
            }
        }
    };
    pub struct ObjectMeta {
        pub output: Option<Output>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ObjectMeta {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ObjectMeta",
                "output",
                &&self.output,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ObjectMeta {
        #[inline]
        fn clone(&self) -> ObjectMeta {
            ObjectMeta {
                output: ::core::clone::Clone::clone(&self.output),
            }
        }
    }
    impl ObjectMeta {
        #[inline]
        pub const fn output(&self) -> &Option<Output> {
            &self.output
        }
    }
    pub struct Object {
        object: ::grost_mir::__private::object::Object<
            ObjectMeta,
            <Field as ::grost_mir::__private::object::RawField>::Meta,
        >,
        derived: ::core::primitive::bool,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Object {
        #[inline]
        fn clone(&self) -> Object {
            Object {
                object: ::core::clone::Clone::clone(&self.object),
                derived: ::core::clone::Clone::clone(&self.derived),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Object {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Object",
                "object",
                &self.object,
                "derived",
                &&self.derived,
            )
        }
    }
    const _: () = {
        use ::grost_mir::__private::{
            darling, syn, quote::{quote, ToTokens},
            proc_macro2,
        };
        struct __ObjectInput__ {
            ident: ::grost_mir::__private::syn::Ident,
            vis: ::grost_mir::__private::syn::Visibility,
            generics: ::grost_mir::__private::syn::Generics,
            attrs: ::std::vec::Vec<::grost_mir::__private::syn::Attribute>,
            data: ::grost_mir::__private::darling::ast::Data<
                ::grost_mir::__private::darling::util::Ignored,
                Field,
            >,
            #[doc(hidden)]
            __args__: __ObjectMeta__,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for __ObjectInput__ {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "ident",
                    "vis",
                    "generics",
                    "attrs",
                    "data",
                    "__args__",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.ident,
                    &self.vis,
                    &self.generics,
                    &self.attrs,
                    &self.data,
                    &&self.__args__,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "__ObjectInput__",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for __ObjectInput__ {
            #[inline]
            fn clone(&self) -> __ObjectInput__ {
                __ObjectInput__ {
                    ident: ::core::clone::Clone::clone(&self.ident),
                    vis: ::core::clone::Clone::clone(&self.vis),
                    generics: ::core::clone::Clone::clone(&self.generics),
                    attrs: ::core::clone::Clone::clone(&self.attrs),
                    data: ::core::clone::Clone::clone(&self.data),
                    __args__: ::core::clone::Clone::clone(&self.__args__),
                }
            }
        }
        #[allow(warnings)]
        #[doc(hidden)]
        struct __ObjectMeta__ {
            #[doc(hidden)]
            __custom_meta__: ObjectMeta,
            #[doc(hidden)]
            __meta__: ::grost_mir::__private::object::ObjectAttribute,
        }
        #[automatically_derived]
        #[allow(warnings)]
        impl ::core::fmt::Debug for __ObjectMeta__ {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "__ObjectMeta__",
                    "__custom_meta__",
                    &self.__custom_meta__,
                    "__meta__",
                    &&self.__meta__,
                )
            }
        }
        #[automatically_derived]
        #[allow(warnings)]
        impl ::core::clone::Clone for __ObjectMeta__ {
            #[inline]
            fn clone(&self) -> __ObjectMeta__ {
                __ObjectMeta__ {
                    __custom_meta__: ::core::clone::Clone::clone(&self.__custom_meta__),
                    __meta__: ::core::clone::Clone::clone(&self.__meta__),
                }
            }
        }
        #[allow(warnings)]
        #[doc(hidden)]
        struct __ObjectDarlingDeriveMeta__ {
            #[darling(default)]
            pub output: Option<Output>,
            #[darling(
                rename = "crate",
                default = ":: grost_mir :: __private :: default_grost_path"
            )]
            #[doc(hidden)]
            __path_to_crate__: ::grost_mir::__private::syn::Path,
            #[darling(flatten)]
            #[doc(hidden)]
            __meta__: ::grost_mir::__private::object::meta::ObjectFromMeta,
        }
        #[automatically_derived]
        #[allow(warnings)]
        impl ::core::fmt::Debug for __ObjectDarlingDeriveMeta__ {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "__ObjectDarlingDeriveMeta__",
                    "output",
                    &self.output,
                    "__path_to_crate__",
                    &self.__path_to_crate__,
                    "__meta__",
                    &&self.__meta__,
                )
            }
        }
        #[automatically_derived]
        #[allow(warnings)]
        impl ::core::clone::Clone for __ObjectDarlingDeriveMeta__ {
            #[inline]
            fn clone(&self) -> __ObjectDarlingDeriveMeta__ {
                __ObjectDarlingDeriveMeta__ {
                    output: ::core::clone::Clone::clone(&self.output),
                    __path_to_crate__: ::core::clone::Clone::clone(
                        &self.__path_to_crate__,
                    ),
                    __meta__: ::core::clone::Clone::clone(&self.__meta__),
                }
            }
        }
        #[automatically_derived]
        #[allow(clippy::manual_unwrap_or_default)]
        impl ::darling::FromMeta for __ObjectDarlingDeriveMeta__ {
            fn from_list(
                __items: &[::darling::export::NestedMeta],
            ) -> ::darling::Result<Self> {
                let mut output: (bool, ::darling::export::Option<Option<Output>>) = (
                    false,
                    None,
                );
                let mut __path_to_crate__: (
                    bool,
                    ::darling::export::Option<::grost_mir::__private::syn::Path>,
                ) = (false, None);
                let mut __meta__: (
                    bool,
                    ::darling::export::Option<
                        ::grost_mir::__private::object::meta::ObjectFromMeta,
                    >,
                ) = (false, None);
                let mut __flatten: Vec<::darling::ast::NestedMeta> = ::alloc::vec::Vec::new();
                let mut __errors = ::darling::Error::accumulator();
                for __item in __items {
                    match *__item {
                        ::darling::export::NestedMeta::Meta(ref __inner) => {
                            let __name = ::darling::util::path_to_string(__inner.path());
                            match __name.as_str() {
                                "output" => {
                                    if !output.0 {
                                        output = (
                                            true,
                                            __errors
                                                .handle(
                                                    ::darling::export::identity::<
                                                        fn(&::syn::Meta) -> ::darling::Result<_>,
                                                    >(::darling::FromMeta::from_meta)(__inner)
                                                        .map_err(|e| e.with_span(&__inner).at("output")),
                                                ),
                                        );
                                    } else {
                                        __errors
                                            .push(
                                                ::darling::Error::duplicate_field("output")
                                                    .with_span(&__inner),
                                            );
                                    }
                                }
                                "crate" => {
                                    if !__path_to_crate__.0 {
                                        __path_to_crate__ = (
                                            true,
                                            __errors
                                                .handle(
                                                    ::darling::export::identity::<
                                                        fn(&::syn::Meta) -> ::darling::Result<_>,
                                                    >(::darling::FromMeta::from_meta)(__inner)
                                                        .map_err(|e| e.with_span(&__inner).at("crate")),
                                                ),
                                        );
                                    } else {
                                        __errors
                                            .push(
                                                ::darling::Error::duplicate_field("crate")
                                                    .with_span(&__inner),
                                            );
                                    }
                                }
                                __other => {
                                    __flatten
                                        .push(::darling::ast::NestedMeta::Meta(__inner.clone()));
                                }
                            }
                        }
                        ::darling::export::NestedMeta::Lit(ref __inner) => {
                            __errors
                                .push(
                                    ::darling::Error::unsupported_format("literal")
                                        .with_span(__inner),
                                );
                        }
                    }
                }
                __meta__ = (
                    true,
                    __errors
                        .handle(
                            ::darling::FromMeta::from_list(&__flatten)
                                .map_err(|e| {
                                    e.add_sibling_alts_for_unknown_field(&["output", "crate"])
                                }),
                        ),
                );
                if !__meta__.0 {
                    match <::grost_mir::__private::object::meta::ObjectFromMeta as ::darling::FromMeta>::from_none() {
                        ::darling::export::Some(__type_fallback) => {
                            __meta__.1 = ::darling::export::Some(__type_fallback);
                        }
                        ::darling::export::None => {
                            __errors.push(::darling::Error::missing_field("__meta__"))
                        }
                    }
                }
                __errors.finish()?;
                ::darling::export::Ok(Self {
                    output: if let Some(__val) = output.1 {
                        __val
                    } else {
                        ::darling::export::Default::default()
                    },
                    __path_to_crate__: if let Some(__val) = __path_to_crate__.1 {
                        __val
                    } else {
                        ::grost_mir::__private::default_grost_path()
                    },
                    __meta__: __meta__
                        .1
                        .expect(
                            "Uninitialized fields without defaults were already checked",
                        ),
                })
            }
        }
        #[allow(warnings)]
        #[doc(hidden)]
        struct __ObjectDarlingAttributeMeta__ {
            #[darling(default)]
            pub output: Option<Output>,
            #[darling(
                rename = "crate",
                default = ":: grost_mir :: __private :: default_grost_path"
            )]
            #[doc(hidden)]
            __path_to_crate__: ::grost_mir::__private::syn::Path,
            #[darling(flatten)]
            #[doc(hidden)]
            __meta__: ::grost_mir::__private::object::meta::ObjectFromMeta,
        }
        #[automatically_derived]
        #[allow(warnings)]
        impl ::core::fmt::Debug for __ObjectDarlingAttributeMeta__ {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "__ObjectDarlingAttributeMeta__",
                    "output",
                    &self.output,
                    "__path_to_crate__",
                    &self.__path_to_crate__,
                    "__meta__",
                    &&self.__meta__,
                )
            }
        }
        #[automatically_derived]
        #[allow(warnings)]
        impl ::core::clone::Clone for __ObjectDarlingAttributeMeta__ {
            #[inline]
            fn clone(&self) -> __ObjectDarlingAttributeMeta__ {
                __ObjectDarlingAttributeMeta__ {
                    output: ::core::clone::Clone::clone(&self.output),
                    __path_to_crate__: ::core::clone::Clone::clone(
                        &self.__path_to_crate__,
                    ),
                    __meta__: ::core::clone::Clone::clone(&self.__meta__),
                }
            }
        }
        #[automatically_derived]
        #[allow(clippy::manual_unwrap_or_default)]
        impl ::darling::FromMeta for __ObjectDarlingAttributeMeta__ {
            fn from_list(
                __items: &[::darling::export::NestedMeta],
            ) -> ::darling::Result<Self> {
                let mut output: (bool, ::darling::export::Option<Option<Output>>) = (
                    false,
                    None,
                );
                let mut __path_to_crate__: (
                    bool,
                    ::darling::export::Option<::grost_mir::__private::syn::Path>,
                ) = (false, None);
                let mut __meta__: (
                    bool,
                    ::darling::export::Option<
                        ::grost_mir::__private::object::meta::ObjectFromMeta,
                    >,
                ) = (false, None);
                let mut __flatten: Vec<::darling::ast::NestedMeta> = ::alloc::vec::Vec::new();
                let mut __errors = ::darling::Error::accumulator();
                for __item in __items {
                    match *__item {
                        ::darling::export::NestedMeta::Meta(ref __inner) => {
                            let __name = ::darling::util::path_to_string(__inner.path());
                            match __name.as_str() {
                                "output" => {
                                    if !output.0 {
                                        output = (
                                            true,
                                            __errors
                                                .handle(
                                                    ::darling::export::identity::<
                                                        fn(&::syn::Meta) -> ::darling::Result<_>,
                                                    >(::darling::FromMeta::from_meta)(__inner)
                                                        .map_err(|e| e.with_span(&__inner).at("output")),
                                                ),
                                        );
                                    } else {
                                        __errors
                                            .push(
                                                ::darling::Error::duplicate_field("output")
                                                    .with_span(&__inner),
                                            );
                                    }
                                }
                                "crate" => {
                                    if !__path_to_crate__.0 {
                                        __path_to_crate__ = (
                                            true,
                                            __errors
                                                .handle(
                                                    ::darling::export::identity::<
                                                        fn(&::syn::Meta) -> ::darling::Result<_>,
                                                    >(::darling::FromMeta::from_meta)(__inner)
                                                        .map_err(|e| e.with_span(&__inner).at("crate")),
                                                ),
                                        );
                                    } else {
                                        __errors
                                            .push(
                                                ::darling::Error::duplicate_field("crate")
                                                    .with_span(&__inner),
                                            );
                                    }
                                }
                                __other => {
                                    __flatten
                                        .push(::darling::ast::NestedMeta::Meta(__inner.clone()));
                                }
                            }
                        }
                        ::darling::export::NestedMeta::Lit(ref __inner) => {
                            __errors
                                .push(
                                    ::darling::Error::unsupported_format("literal")
                                        .with_span(__inner),
                                );
                        }
                    }
                }
                __meta__ = (
                    true,
                    __errors
                        .handle(
                            ::darling::FromMeta::from_list(&__flatten)
                                .map_err(|e| {
                                    e.add_sibling_alts_for_unknown_field(&["output", "crate"])
                                }),
                        ),
                );
                if !__meta__.0 {
                    match <::grost_mir::__private::object::meta::ObjectFromMeta as ::darling::FromMeta>::from_none() {
                        ::darling::export::Some(__type_fallback) => {
                            __meta__.1 = ::darling::export::Some(__type_fallback);
                        }
                        ::darling::export::None => {
                            __errors.push(::darling::Error::missing_field("__meta__"))
                        }
                    }
                }
                __errors.finish()?;
                ::darling::export::Ok(Self {
                    output: if let Some(__val) = output.1 {
                        __val
                    } else {
                        ::darling::export::Default::default()
                    },
                    __path_to_crate__: if let Some(__val) = __path_to_crate__.1 {
                        __val
                    } else {
                        ::grost_mir::__private::default_grost_path()
                    },
                    __meta__: __meta__
                        .1
                        .expect(
                            "Uninitialized fields without defaults were already checked",
                        ),
                })
            }
        }
        #[allow(warnings)]
        #[doc(hidden)]
        #[darling(attributes(grost), forward_attrs, supports(struct_named))]
        struct __ObjectDeriveInput__ {
            ident: ::grost_mir::__private::syn::Ident,
            vis: ::grost_mir::__private::syn::Visibility,
            generics: ::grost_mir::__private::syn::Generics,
            attrs: ::std::vec::Vec<::grost_mir::__private::syn::Attribute>,
            data: ::grost_mir::__private::darling::ast::Data<
                ::grost_mir::__private::darling::util::Ignored,
                Field,
            >,
            #[darling(flatten)]
            #[doc(hidden)]
            __args__: __ObjectDarlingDeriveMeta__,
        }
        #[automatically_derived]
        #[allow(warnings)]
        impl ::core::fmt::Debug for __ObjectDeriveInput__ {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "ident",
                    "vis",
                    "generics",
                    "attrs",
                    "data",
                    "__args__",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.ident,
                    &self.vis,
                    &self.generics,
                    &self.attrs,
                    &self.data,
                    &&self.__args__,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "__ObjectDeriveInput__",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        #[allow(warnings)]
        impl ::core::clone::Clone for __ObjectDeriveInput__ {
            #[inline]
            fn clone(&self) -> __ObjectDeriveInput__ {
                __ObjectDeriveInput__ {
                    ident: ::core::clone::Clone::clone(&self.ident),
                    vis: ::core::clone::Clone::clone(&self.vis),
                    generics: ::core::clone::Clone::clone(&self.generics),
                    attrs: ::core::clone::Clone::clone(&self.attrs),
                    data: ::core::clone::Clone::clone(&self.data),
                    __args__: ::core::clone::Clone::clone(&self.__args__),
                }
            }
        }
        #[automatically_derived]
        #[allow(clippy::manual_unwrap_or_default)]
        impl ::darling::FromDeriveInput for __ObjectDeriveInput__ {
            fn from_derive_input(
                __di: &::darling::export::syn::DeriveInput,
            ) -> ::darling::Result<Self> {
                let mut __errors = ::darling::Error::accumulator();
                let mut __args__: (
                    bool,
                    ::darling::export::Option<__ObjectDarlingDeriveMeta__>,
                ) = (false, None);
                let mut __flatten: Vec<::darling::ast::NestedMeta> = ::alloc::vec::Vec::new();
                let mut __fwd_attrs: ::darling::export::Vec<
                    ::darling::export::syn::Attribute,
                > = ::alloc::vec::Vec::new();
                let mut attrs: ::darling::export::Option<_> = None;
                use ::darling::ToTokens;
                for __attr in &__di.attrs {
                    match ::darling::export::ToString::to_string(
                            &__attr.path().clone().into_token_stream(),
                        )
                        .as_str()
                    {
                        "grost" => {
                            match ::darling::util::parse_attribute_to_meta_list(__attr) {
                                ::darling::export::Ok(__data) => {
                                    match ::darling::export::NestedMeta::parse_meta_list(
                                        __data.tokens,
                                    ) {
                                        ::darling::export::Ok(ref __items) => {
                                            if __items.is_empty() {
                                                continue;
                                            }
                                            for __item in __items {
                                                match *__item {
                                                    ::darling::export::NestedMeta::Meta(ref __inner) => {
                                                        let __name = ::darling::util::path_to_string(
                                                            __inner.path(),
                                                        );
                                                        match __name.as_str() {
                                                            __other => {
                                                                __flatten
                                                                    .push(::darling::ast::NestedMeta::Meta(__inner.clone()));
                                                            }
                                                        }
                                                    }
                                                    ::darling::export::NestedMeta::Lit(ref __inner) => {
                                                        __errors
                                                            .push(
                                                                ::darling::Error::unsupported_format("literal")
                                                                    .with_span(__inner),
                                                            );
                                                    }
                                                }
                                            }
                                        }
                                        ::darling::export::Err(__err) => {
                                            __errors.push(__err.into());
                                        }
                                    }
                                }
                                ::darling::export::Err(__err) => {
                                    __errors.push(__err);
                                }
                            }
                        }
                        _ => __fwd_attrs.push(__attr.clone()),
                    }
                }
                attrs = ::darling::export::Some(__fwd_attrs);
                #[allow(unused_variables)]
                fn __validate_body(
                    __body: &::darling::export::syn::Data,
                ) -> ::darling::Result<()> {
                    {
                        let struct_check = ::darling::util::ShapeSet::new(
                            <[_]>::into_vec(
                                ::alloc::boxed::box_new([::darling::util::Shape::Named]),
                            ),
                        );
                        let enum_check = ::darling::util::ShapeSet::new(
                            ::alloc::vec::Vec::new(),
                        );
                        match *__body {
                            ::darling::export::syn::Data::Enum(ref data) => {
                                if enum_check.is_empty() {
                                    return ::darling::export::Err(
                                        ::darling::Error::unsupported_shape_with_expected(
                                            "enum",
                                            &::alloc::__export::must_use({
                                                let res = ::alloc::fmt::format(
                                                    format_args!("struct with {0}", struct_check),
                                                );
                                                res
                                            }),
                                        ),
                                    );
                                }
                                let mut variant_errors = ::darling::Error::accumulator();
                                for variant in &data.variants {
                                    variant_errors.handle(enum_check.check(variant));
                                }
                                variant_errors.finish()
                            }
                            ::darling::export::syn::Data::Struct(ref struct_data) => {
                                if struct_check.is_empty() {
                                    return ::darling::export::Err(
                                        ::darling::Error::unsupported_shape_with_expected(
                                            "struct",
                                            &::alloc::__export::must_use({
                                                let res = ::alloc::fmt::format(
                                                    format_args!("enum with {0}", enum_check),
                                                );
                                                res
                                            }),
                                        ),
                                    );
                                }
                                struct_check.check(struct_data)
                            }
                            ::darling::export::syn::Data::Union(_) => {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                        }
                    }
                }
                __errors.handle(__validate_body(&__di.data));
                __args__ = (
                    true,
                    __errors.handle(::darling::FromMeta::from_list(&__flatten)),
                );
                if !__args__.0 {
                    match <__ObjectDarlingDeriveMeta__ as ::darling::FromMeta>::from_none() {
                        ::darling::export::Some(__type_fallback) => {
                            __args__.1 = ::darling::export::Some(__type_fallback);
                        }
                        ::darling::export::None => {
                            __errors.push(::darling::Error::missing_field("__args__"))
                        }
                    }
                }
                __errors.finish()?;
                ::darling::export::Ok(__ObjectDeriveInput__ {
                    ident: __di.ident.clone(),
                    generics: ::darling::FromGenerics::from_generics(&__di.generics)?,
                    vis: __di.vis.clone(),
                    attrs: attrs.expect("Errors were already checked"),
                    data: ::darling::ast::Data::try_from(&__di.data)?,
                    __args__: __args__
                        .1
                        .expect(
                            "Uninitialized fields without defaults were already checked",
                        ),
                })
            }
        }
        #[allow(warnings)]
        #[doc(hidden)]
        #[darling(forward_attrs, supports(struct_named))]
        struct __ObjectAttributeInput__ {
            ident: ::grost_mir::__private::syn::Ident,
            vis: ::grost_mir::__private::syn::Visibility,
            generics: ::grost_mir::__private::syn::Generics,
            attrs: ::std::vec::Vec<::grost_mir::__private::syn::Attribute>,
            data: ::grost_mir::__private::darling::ast::Data<
                ::grost_mir::__private::darling::util::Ignored,
                Field,
            >,
        }
        #[automatically_derived]
        #[allow(warnings)]
        impl ::core::fmt::Debug for __ObjectAttributeInput__ {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "__ObjectAttributeInput__",
                    "ident",
                    &self.ident,
                    "vis",
                    &self.vis,
                    "generics",
                    &self.generics,
                    "attrs",
                    &self.attrs,
                    "data",
                    &&self.data,
                )
            }
        }
        #[automatically_derived]
        #[allow(warnings)]
        impl ::core::clone::Clone for __ObjectAttributeInput__ {
            #[inline]
            fn clone(&self) -> __ObjectAttributeInput__ {
                __ObjectAttributeInput__ {
                    ident: ::core::clone::Clone::clone(&self.ident),
                    vis: ::core::clone::Clone::clone(&self.vis),
                    generics: ::core::clone::Clone::clone(&self.generics),
                    attrs: ::core::clone::Clone::clone(&self.attrs),
                    data: ::core::clone::Clone::clone(&self.data),
                }
            }
        }
        #[automatically_derived]
        #[allow(clippy::manual_unwrap_or_default)]
        impl ::darling::FromDeriveInput for __ObjectAttributeInput__ {
            fn from_derive_input(
                __di: &::darling::export::syn::DeriveInput,
            ) -> ::darling::Result<Self> {
                let mut __errors = ::darling::Error::accumulator();
                let mut __fwd_attrs: ::darling::export::Vec<
                    ::darling::export::syn::Attribute,
                > = ::alloc::vec::Vec::new();
                let mut attrs: ::darling::export::Option<_> = None;
                use ::darling::ToTokens;
                for __attr in &__di.attrs {
                    match ::darling::export::ToString::to_string(
                            &__attr.path().clone().into_token_stream(),
                        )
                        .as_str()
                    {
                        _ => __fwd_attrs.push(__attr.clone()),
                    }
                }
                attrs = ::darling::export::Some(__fwd_attrs);
                #[allow(unused_variables)]
                fn __validate_body(
                    __body: &::darling::export::syn::Data,
                ) -> ::darling::Result<()> {
                    {
                        let struct_check = ::darling::util::ShapeSet::new(
                            <[_]>::into_vec(
                                ::alloc::boxed::box_new([::darling::util::Shape::Named]),
                            ),
                        );
                        let enum_check = ::darling::util::ShapeSet::new(
                            ::alloc::vec::Vec::new(),
                        );
                        match *__body {
                            ::darling::export::syn::Data::Enum(ref data) => {
                                if enum_check.is_empty() {
                                    return ::darling::export::Err(
                                        ::darling::Error::unsupported_shape_with_expected(
                                            "enum",
                                            &::alloc::__export::must_use({
                                                let res = ::alloc::fmt::format(
                                                    format_args!("struct with {0}", struct_check),
                                                );
                                                res
                                            }),
                                        ),
                                    );
                                }
                                let mut variant_errors = ::darling::Error::accumulator();
                                for variant in &data.variants {
                                    variant_errors.handle(enum_check.check(variant));
                                }
                                variant_errors.finish()
                            }
                            ::darling::export::syn::Data::Struct(ref struct_data) => {
                                if struct_check.is_empty() {
                                    return ::darling::export::Err(
                                        ::darling::Error::unsupported_shape_with_expected(
                                            "struct",
                                            &::alloc::__export::must_use({
                                                let res = ::alloc::fmt::format(
                                                    format_args!("enum with {0}", enum_check),
                                                );
                                                res
                                            }),
                                        ),
                                    );
                                }
                                struct_check.check(struct_data)
                            }
                            ::darling::export::syn::Data::Union(_) => {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                        }
                    }
                }
                __errors.handle(__validate_body(&__di.data));
                __errors.finish()?;
                ::darling::export::Ok(__ObjectAttributeInput__ {
                    ident: __di.ident.clone(),
                    generics: ::darling::FromGenerics::from_generics(&__di.generics)?,
                    vis: __di.vis.clone(),
                    attrs: attrs.expect("Errors were already checked"),
                    data: ::darling::ast::Data::try_from(&__di.data)?,
                })
            }
        }
        impl __ObjectInput__ {
            /// Parse the input from the derive macro.
            ///
            /// **Note:** This function is only used for the derive macro input, and it will not
            /// work correctly if you use it for the attribute macro. For the attribute macro,
            /// use [`from_attribute_input`](Self::from_attribute_input) instead.
            pub fn from_derive_input(
                input: ::grost_mir::__private::proc_macro2::TokenStream,
            ) -> ::grost_mir::__private::darling::Result<
                ::grost_mir::__private::object::Object<
                    ObjectMeta,
                    <<Self as ::grost_mir::__private::object::RawObject>::Field as ::grost_mir::__private::object::RawField>::Meta,
                >,
            > {
                let input: ::grost_mir::__private::syn::DeriveInput = ::grost_mir::__private::syn::parse2(
                    input,
                )?;
                let input = <__ObjectDeriveInput__ as ::grost_mir::__private::darling::FromDeriveInput>::from_derive_input(
                    &input,
                )?;
                let args = input.__args__;
                let this = Self {
                    ident: input.ident,
                    vis: input.vis,
                    generics: input.generics,
                    attrs: input.attrs,
                    data: input.data,
                    __args__: __ObjectMeta__ {
                        __meta__: args.__meta__.finalize(args.__path_to_crate__)?,
                        __custom_meta__: ObjectMeta { output: args.output },
                    },
                };
                ::grost_mir::__private::object::Object::from_raw(this)
            }
            /// Parse the input from the attribute macro input.
            ///
            /// **Note:** This function is only used for the attribute macro input, and it will not
            /// work correctly if you use it for the derive macro. For the derive macro,
            /// use [`from_derive_input`](Self::from_derive_input) instead.
            pub fn from_attribute_input(
                args: ::grost_mir::__private::proc_macro2::TokenStream,
                input: ::grost_mir::__private::proc_macro2::TokenStream,
            ) -> ::grost_mir::__private::darling::Result<
                ::grost_mir::__private::object::Object<
                    ObjectMeta,
                    <<Self as ::grost_mir::__private::object::RawObject>::Field as ::grost_mir::__private::object::RawField>::Meta,
                >,
            > {
                let input: ::grost_mir::__private::syn::DeriveInput = ::grost_mir::__private::syn::parse2(
                    input,
                )?;
                let input = <__ObjectAttributeInput__ as ::grost_mir::__private::darling::FromDeriveInput>::from_derive_input(
                    &input,
                )?;
                let args = ::grost_mir::__private::darling::ast::NestedMeta::parse_meta_list(
                    args,
                )?;
                let args = <__ObjectDarlingAttributeMeta__ as ::grost_mir::__private::darling::FromMeta>::from_list(
                    &args,
                )?;
                let this = Self {
                    ident: input.ident,
                    vis: input.vis,
                    generics: input.generics,
                    attrs: input.attrs,
                    data: input.data,
                    __args__: __ObjectMeta__ {
                        __meta__: args.__meta__.finalize(args.__path_to_crate__)?,
                        __custom_meta__: ObjectMeta { output: args.output },
                    },
                };
                ::grost_mir::__private::object::Object::from_raw(this)
            }
        }
        impl ::grost_mir::__private::object::RawObject for __ObjectInput__ {
            type Field = Field;
            type Meta = ObjectMeta;
            fn name(&self) -> &::grost_mir::__private::syn::Ident {
                &self.ident
            }
            fn vis(&self) -> &::grost_mir::__private::syn::Visibility {
                &self.vis
            }
            fn generics(&self) -> &::grost_mir::__private::syn::Generics {
                &self.generics
            }
            fn attrs(&self) -> &[::grost_mir::__private::syn::Attribute] {
                &self.attrs
            }
            fn fields(&self) -> ::std::vec::Vec<&Self::Field> {
                self.data
                    .as_ref()
                    .take_struct()
                    .expect("__ObjectDeriveInput__ only supports named structs")
                    .fields
            }
            fn path_to_grost(&self) -> &syn::Path {
                self.__args__.__meta__.path_to_grost()
            }
            fn default(&self) -> ::core::option::Option<&syn::Path> {
                self.__args__.__meta__.default()
            }
            fn schema(&self) -> &::grost_mir::__private::utils::SchemaAttribute {
                self.__args__.__meta__.schema()
            }
            fn partial(
                &self,
            ) -> &::grost_mir::__private::object::PartialObjectAttribute {
                self.__args__.__meta__.partial()
            }
            fn partial_decoded(
                &self,
            ) -> &::grost_mir::__private::object::PartialDecodedObjectAttribute {
                self.__args__.__meta__.partial_decoded()
            }
            fn selector(&self) -> &::grost_mir::__private::object::SelectorAttribute {
                self.__args__.__meta__.selector()
            }
            fn selector_iter(
                &self,
            ) -> &::grost_mir::__private::object::SelectorIterAttribute {
                self.__args__.__meta__.selector_iter()
            }
            fn indexer(&self) -> &::grost_mir::__private::object::IndexerAttribute {
                self.__args__.__meta__.indexer()
            }
            fn copy(&self) -> ::core::primitive::bool {
                self.__args__.__meta__.copy()
            }
            fn flavors(&self) -> &[::grost_mir::__private::flavor::FlavorAttribute] {
                self.__args__.__meta__.flavors()
            }
            fn meta(&self) -> &Self::Meta {
                &self.__args__.__custom_meta__
            }
        }
        impl ::core::cmp::PartialEq for Object {
            fn eq(&self, other: &Self) -> ::core::primitive::bool {
                self.object.name().eq(other.object.name())
            }
        }
        impl ::core::cmp::Eq for Object {}
        impl ::core::hash::Hash for Object {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                self.object.name().hash(state);
            }
        }
        impl ToTokens for Object {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                self.object.to_tokens(tokens);
            }
        }
        impl Object {
            /// Parse the input from the derive macro.
            ///
            /// **Note:** This function is only used for the derive macro input, and it will not
            /// work correctly if you use it for the attribute macro. For the attribute macro,
            /// use [`from_attribute_input`](Self::from_attribute_input) instead.
            pub fn from_derive_input(
                input: ::grost_mir::__private::proc_macro2::TokenStream,
            ) -> ::grost_mir::__private::darling::Result<Self> {
                __ObjectInput__::from_derive_input(input)
                    .map(|object| Self { object, derived: true })
            }
            /// Parse the input from the attribute macro input.
            ///
            /// **Note:** This function is only used for the attribute macro input, and it will not
            /// work correctly if you use it for the derive macro. For the derive macro,
            /// use [`from_derive_input`](Self::from_derive_input) instead.
            pub fn from_attribute_input(
                args: ::grost_mir::__private::proc_macro2::TokenStream,
                input: ::grost_mir::__private::proc_macro2::TokenStream,
            ) -> ::grost_mir::__private::darling::Result<Self> {
                __ObjectInput__::from_attribute_input(args, input)
                    .map(|object| Self { object, derived: false })
            }
            /// Returns the MIR representation of the object.
            #[inline]
            pub const fn mir(
                &self,
            ) -> &::grost_mir::__private::object::Object<
                ObjectMeta,
                <Field as ::grost_mir::__private::object::RawField>::Meta,
            > {
                &self.object
            }
            /// Derives the generated code of the object.
            pub fn derive(&self) -> darling::Result<proc_macro2::TokenStream> {
                self.object
                    .derive()
                    .map(|token_stream| {
                        if self.derived {
                            token_stream
                        } else {
                            let obj = &self.object;
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                ::quote::ToTokens::to_tokens(&obj, &mut _s);
                                ::quote::ToTokens::to_tokens(&token_stream, &mut _s);
                                _s
                            }
                        }
                    })
            }
        }
    };
}

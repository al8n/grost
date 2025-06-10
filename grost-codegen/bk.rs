#[automatically_derived]
        #[allow(clippy::manual_unwrap_or_default)]
        impl ::darling::FromField for __FieldInput__ {
            fn from_field(
                __field: &::darling::export::syn::Field,
            ) -> ::darling::Result<Self> {
                let mut __errors = ::darling::Error::accumulator();
                let mut __meta__: (
                    bool,
                    ::darling::export::Option<
                        ::grost_mir::__private::object::FieldFromMeta,
                    >,
                ) = (false, None);
                let mut __flatten: Vec<::darling::ast::NestedMeta> = ::alloc::vec::Vec::new();
                let mut __fwd_attrs: ::darling::export::Vec<
                    ::darling::export::syn::Attribute,
                > = ::alloc::vec::Vec::new();
                let mut attrs: ::darling::export::Option<_> = None;
                use ::darling::ToTokens;
                for __attr in &__field.attrs {
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
                __meta__ = (
                    true,
                    __errors.handle(::darling::FromMeta::from_list(&__flatten)),
                );
                if !__meta__.0 {
                    match <::grost_mir::__private::object::FieldFromMeta as ::darling::FromMeta>::from_none() {
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
                    ident: __field.ident.clone(),
                    ty: __field.ty.clone(),
                    vis: __field.vis.clone(),
                    attrs: attrs.expect("Errors were already checked"),
                    __meta__: __meta__
                        .1
                        .expect(
                            "Uninitialized fields without defaults were already checked",
                        ),
                })
            }
        }
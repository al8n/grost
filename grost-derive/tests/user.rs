pub struct User<I> {
  id: I,
  name: String,
  age: u8,
  _w: core::marker::PhantomData<*const ()>,
}
#[allow(non_camel_case_types, clippy::type_complexity)]
pub struct PartialUser<I, __GROST_UNKNOWN_BUFFER__>
where
  I: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
  <I as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output:
    ::core::marker::Sized,
{
  __grost_unknown_buffer__: ::core::option::Option<__GROST_UNKNOWN_BUFFER__>,
  id: ::core::option::Option<
    <I as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
  >,
  name: ::core::option::Option<
    <String as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
  >,
  age: ::core::option::Option<
    <u8 as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
  >,
  _w: ::core::option::Option<core::marker::PhantomData<*const ()>>,
}
/// Partial reference struct for the struct [`User`]
#[allow(clippy::type_complexity, non_camel_case_types)]
pub struct PartialRefUser<
    '__grost_lifetime__,
    I,
    __GROST_FLAVOR__: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
    __GROST_UNKNOWN_BUFFER__,
>
where
    ::grost::__private::reflection::WireFormatReflection<
        User<I>,
        __GROST_FLAVOR__,
        1u32,
    >: ::grost::__private::reflection::Reflectable<User<I>>,
    <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        __GROST_FLAVOR__,
        1u32,
    > as ::grost::__private::reflection::Reflectable<
        User<I>,
    >>::Reflection: ::grost::__private::flavors::WireFormat<__GROST_FLAVOR__>,
    I: ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            __GROST_UNKNOWN_BUFFER__,
        >,
    >,
    <I as ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            __GROST_UNKNOWN_BUFFER__,
        >,
    >>::Output: ::core::marker::Sized + ::core::marker::Copy,
    ::grost::__private::reflection::WireFormatReflection<
        User<I>,
        __GROST_FLAVOR__,
        2u32,
    >: ::grost::__private::reflection::Reflectable<User<I>>,
    <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        __GROST_FLAVOR__,
        2u32,
    > as ::grost::__private::reflection::Reflectable<
        User<I>,
    >>::Reflection: ::grost::__private::flavors::WireFormat<__GROST_FLAVOR__>,
    String: ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            __GROST_UNKNOWN_BUFFER__,
        >,
    >,
    <String as ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            __GROST_UNKNOWN_BUFFER__,
        >,
    >>::Output: ::core::marker::Sized + ::core::marker::Copy,
    ::grost::__private::reflection::WireFormatReflection<
        User<I>,
        __GROST_FLAVOR__,
        3u32,
    >: ::grost::__private::reflection::Reflectable<User<I>>,
    <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        __GROST_FLAVOR__,
        3u32,
    > as ::grost::__private::reflection::Reflectable<
        User<I>,
    >>::Reflection: ::grost::__private::flavors::WireFormat<__GROST_FLAVOR__>,
    u8: ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            __GROST_UNKNOWN_BUFFER__,
        >,
    >,
    <u8 as ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
            '__grost_lifetime__,
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            __GROST_UNKNOWN_BUFFER__,
        >,
    >>::Output: ::core::marker::Sized + ::core::marker::Copy,
{
    __grost_unknown_buffer__: ::core::option::Option<__GROST_UNKNOWN_BUFFER__>,
    id: ::core::option::Option<
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output,
    >,
    name: ::core::option::Option<
        <String as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output,
    >,
    age: ::core::option::Option<
        <u8 as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output,
    >,
    _w: ::core::option::Option<core::marker::PhantomData<*const ()>>,
}
/// Field indexer for the struct [`User`]
#[derive(
  ::core::clone::Clone,
  ::core::marker::Copy,
  ::core::cmp::PartialEq,
  ::core::cmp::Eq,
  ::core::cmp::PartialOrd,
  ::core::cmp::Ord,
  ::core::hash::Hash,
  ::core::fmt::Debug,
)]
#[repr(u32)]
pub enum UserIndex {
  /// The field indexer for the field `id`
  Id = 0u32,
  /// The field indexer for the field `name`
  Name = 1u32,
  /// The field indexer for the field `age`
  Age = 2u32,
}
/// The selection type for [`User`]
#[allow(non_camel_case_types, clippy::type_complexity)]
pub struct UserSelector<I, __GROST_FLAVOR__: ?::core::marker::Sized>
where
    ::grost::__private::reflection::WireFormatReflection<
        User<I>,
        __GROST_FLAVOR__,
        1u32,
    >: ::grost::__private::reflection::Reflectable<User<I>>,
    I: ::grost::__private::selection::Selectable<
        __GROST_FLAVOR__,
        <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
    >,
    ::grost::__private::reflection::WireFormatReflection<
        User<I>,
        __GROST_FLAVOR__,
        2u32,
    >: ::grost::__private::reflection::Reflectable<User<I>>,
    String: ::grost::__private::selection::Selectable<
        __GROST_FLAVOR__,
        <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
    >,
    ::grost::__private::reflection::WireFormatReflection<
        User<I>,
        __GROST_FLAVOR__,
        3u32,
    >: ::grost::__private::reflection::Reflectable<User<I>>,
    u8: ::grost::__private::selection::Selectable<
        __GROST_FLAVOR__,
        <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
    >,
{
    id: <I as ::grost::__private::selection::Selectable<
        __GROST_FLAVOR__,
        <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
    >>::Selector,
    name: <String as ::grost::__private::selection::Selectable<
        __GROST_FLAVOR__,
        <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
    >>::Selector,
    age: <u8 as ::grost::__private::selection::Selectable<
        __GROST_FLAVOR__,
        <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
    >>::Selector,
    #[allow(dead_code)]
    ___w__: ::core::marker::PhantomData<core::marker::PhantomData<*const ()>>,
}
/// An iterator over the selected fields of the [`UserSelector`]
#[allow(non_camel_case_types, clippy::type_complexity)]
pub struct UserSelectorIter<
    '__grost_lifetime__,
    I,
    __GROST_FLAVOR__: ?::core::marker::Sized,
    const __GROST_SELECTED__: ::core::primitive::bool = true,
>
where
    ::grost::__private::reflection::WireFormatReflection<
        User<I>,
        __GROST_FLAVOR__,
        1u32,
    >: ::grost::__private::reflection::Reflectable<User<I>>,
    I: ::grost::__private::selection::Selectable<
        __GROST_FLAVOR__,
        <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
    >,
    ::grost::__private::reflection::WireFormatReflection<
        User<I>,
        __GROST_FLAVOR__,
        2u32,
    >: ::grost::__private::reflection::Reflectable<User<I>>,
    String: ::grost::__private::selection::Selectable<
        __GROST_FLAVOR__,
        <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
    >,
    ::grost::__private::reflection::WireFormatReflection<
        User<I>,
        __GROST_FLAVOR__,
        3u32,
    >: ::grost::__private::reflection::Reflectable<User<I>>,
    u8: ::grost::__private::selection::Selectable<
        __GROST_FLAVOR__,
        <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
    >,
{
    selector: &'__grost_lifetime__ UserSelector<I, __GROST_FLAVOR__>,
    index: ::core::option::Option<UserIndex>,
    num: ::core::primitive::usize,
    yielded: ::core::primitive::usize,
}
const _: () = {
  const _: () = {
    #[automatically_derived]
    #[allow(clippy::type_complexity, non_camel_case_types)]
    impl<I, __GROST_FLAVOR__: ?::core::marker::Sized>
      ::grost::__private::reflection::Reflectable<User<I>>
      for ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::reflection::ObjectField,
        __GROST_FLAVOR__,
        1u32,
      >
    where
      ::grost::__private::reflection::SchemaTypeReflection<I>:
        ::grost::__private::reflection::Reflectable<
            I,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
      ::grost::__private::reflection::SchemaTypeReflection<String>:
        ::grost::__private::reflection::Reflectable<
            String,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
      ::grost::__private::reflection::SchemaTypeReflection<u8>:
        ::grost::__private::reflection::Reflectable<
            u8,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
    {
      type Reflection = ::grost::__private::reflection::ObjectField;
      const REFLECTION: &'static Self::Reflection = &{
        ::grost::__private::reflection::ObjectFieldBuilder {
                    name: "id",
                    description: "The id of the user",
                    ty: <::grost::__private::reflection::SchemaTypeReflection<
                        I,
                    > as ::grost::__private::reflection::Reflectable<I>>::REFLECTION,
                }
                    .build()
      };
    }
    #[automatically_derived]
    #[allow(clippy::type_complexity, non_camel_case_types)]
    impl<I, __GROST_FLAVOR__: ?::core::marker::Sized>
      ::grost::__private::reflection::Reflectable<User<I>>
      for ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::reflection::ObjectField,
        __GROST_FLAVOR__,
        2u32,
      >
    where
      ::grost::__private::reflection::SchemaTypeReflection<I>:
        ::grost::__private::reflection::Reflectable<
            I,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
      ::grost::__private::reflection::SchemaTypeReflection<String>:
        ::grost::__private::reflection::Reflectable<
            String,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
      ::grost::__private::reflection::SchemaTypeReflection<u8>:
        ::grost::__private::reflection::Reflectable<
            u8,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
    {
      type Reflection = ::grost::__private::reflection::ObjectField;
      const REFLECTION: &'static Self::Reflection = &{
        ::grost::__private::reflection::ObjectFieldBuilder {
                    name: "name",
                    description: "The nick name of the user",
                    ty: <::grost::__private::reflection::SchemaTypeReflection<
                        String,
                    > as ::grost::__private::reflection::Reflectable<String>>::REFLECTION,
                }
                    .build()
      };
    }
    #[automatically_derived]
    #[allow(clippy::type_complexity, non_camel_case_types)]
    impl<I, __GROST_FLAVOR__: ?::core::marker::Sized>
      ::grost::__private::reflection::Reflectable<User<I>>
      for ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::reflection::ObjectField,
        __GROST_FLAVOR__,
        3u32,
      >
    where
      ::grost::__private::reflection::SchemaTypeReflection<I>:
        ::grost::__private::reflection::Reflectable<
            I,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
      ::grost::__private::reflection::SchemaTypeReflection<String>:
        ::grost::__private::reflection::Reflectable<
            String,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
      ::grost::__private::reflection::SchemaTypeReflection<u8>:
        ::grost::__private::reflection::Reflectable<
            u8,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
    {
      type Reflection = ::grost::__private::reflection::ObjectField;
      const REFLECTION: &'static Self::Reflection = &{
        ::grost::__private::reflection::ObjectFieldBuilder {
                    name: "age",
                    description: "The age of the user",
                    ty: <::grost::__private::reflection::SchemaTypeReflection<
                        u8,
                    > as ::grost::__private::reflection::Reflectable<u8>>::REFLECTION,
                }
                    .build()
      };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I> ::grost::__private::reflection::Reflectable<User<I>>
      for ::grost::__private::reflection::SchemaTypeReflection<User<I>>
    where
      ::grost::__private::reflection::SchemaTypeReflection<I>:
        ::grost::__private::reflection::Reflectable<
            I,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
      ::grost::__private::reflection::SchemaTypeReflection<String>:
        ::grost::__private::reflection::Reflectable<
            String,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
      ::grost::__private::reflection::SchemaTypeReflection<u8>:
        ::grost::__private::reflection::Reflectable<
            u8,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
    {
      type Reflection = ::grost::__private::reflection::SchemaType;
      const REFLECTION: &'static Self::Reflection = &{
        ::grost::__private::reflection::SchemaType::Object(
                    &::grost::__private::reflection::ObjectBuilder {
                        name: "User",
                        description: "",
                        fields: &[
                            &::grost::__private::reflection::ObjectFieldBuilder {
                                name: "id",
                                description: "The id of the user",
                                ty: <::grost::__private::reflection::SchemaTypeReflection<
                                    I,
                                > as ::grost::__private::reflection::Reflectable<
                                    I,
                                >>::REFLECTION,
                            }
                                .build(),
                            &::grost::__private::reflection::ObjectFieldBuilder {
                                name: "name",
                                description: "The nick name of the user",
                                ty: <::grost::__private::reflection::SchemaTypeReflection<
                                    String,
                                > as ::grost::__private::reflection::Reflectable<
                                    String,
                                >>::REFLECTION,
                            }
                                .build(),
                            &::grost::__private::reflection::ObjectFieldBuilder {
                                name: "age",
                                description: "The age of the user",
                                ty: <::grost::__private::reflection::SchemaTypeReflection<
                                    u8,
                                > as ::grost::__private::reflection::Reflectable<
                                    u8,
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
    impl<I, __GROST_FLAVOR__: ?::core::marker::Sized>
      ::grost::__private::reflection::Reflectable<User<I>>
      for ::grost::__private::reflection::ObjectReflection<
        User<I>,
        ::grost::__private::reflection::Object,
        __GROST_FLAVOR__,
      >
    where
      ::grost::__private::reflection::SchemaTypeReflection<I>:
        ::grost::__private::reflection::Reflectable<
            I,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
      ::grost::__private::reflection::SchemaTypeReflection<String>:
        ::grost::__private::reflection::Reflectable<
            String,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
      ::grost::__private::reflection::SchemaTypeReflection<u8>:
        ::grost::__private::reflection::Reflectable<
            u8,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
    {
      type Reflection = ::grost::__private::reflection::Object;
      const REFLECTION: &'static Self::Reflection = &{
        ::grost::__private::reflection::ObjectBuilder {
                    name: "User",
                    description: "",
                    fields: &[
                        &::grost::__private::reflection::ObjectFieldBuilder {
                            name: "id",
                            description: "The id of the user",
                            ty: <::grost::__private::reflection::SchemaTypeReflection<
                                I,
                            > as ::grost::__private::reflection::Reflectable<
                                I,
                            >>::REFLECTION,
                        }
                            .build(),
                        &::grost::__private::reflection::ObjectFieldBuilder {
                            name: "name",
                            description: "The nick name of the user",
                            ty: <::grost::__private::reflection::SchemaTypeReflection<
                                String,
                            > as ::grost::__private::reflection::Reflectable<
                                String,
                            >>::REFLECTION,
                        }
                            .build(),
                        &::grost::__private::reflection::ObjectFieldBuilder {
                            name: "age",
                            description: "The age of the user",
                            ty: <::grost::__private::reflection::SchemaTypeReflection<
                                u8,
                            > as ::grost::__private::reflection::Reflectable<
                                u8,
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
    impl<I> User<I> {
      /// Returns the reflection of the struct.
      #[inline]
      pub const fn reflection<__GROST_FLAVOR__>()
      -> ::grost::__private::reflection::ObjectReflection<
        Self,
        ::grost::__private::reflection::Object,
        __GROST_FLAVOR__,
      >
      where
        __GROST_FLAVOR__: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
      {
        ::grost::__private::reflection::ObjectReflection::new()
      }
      /// Returns the field reflection of the field `User.id`.
      #[inline]
      pub const fn id_reflection<__GROST_FLAVOR__>()
      -> ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::reflection::ObjectField,
        __GROST_FLAVOR__,
        1u32,
      >
      where
        __GROST_FLAVOR__: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
      {
        ::grost::__private::reflection::ObjectFieldReflection::new()
      }
      /// Returns the field reflection of the field `User.name`.
      #[inline]
      pub const fn name_reflection<__GROST_FLAVOR__>()
      -> ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::reflection::ObjectField,
        __GROST_FLAVOR__,
        2u32,
      >
      where
        __GROST_FLAVOR__: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
      {
        ::grost::__private::reflection::ObjectFieldReflection::new()
      }
      /// Returns the field reflection of the field `User.age`.
      #[inline]
      pub const fn age_reflection<__GROST_FLAVOR__>()
      -> ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::reflection::ObjectField,
        __GROST_FLAVOR__,
        3u32,
      >
      where
        __GROST_FLAVOR__: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
      {
        ::grost::__private::reflection::ObjectFieldReflection::new()
      }
    }
  };
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl<I, __GROST_FLAVOR__: ?::core::marker::Sized>
    ::grost::__private::indexer::Indexable<__GROST_FLAVOR__> for User<I>
  {
    type Indexer = UserIndex;
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl UserIndex {
    /// Returns the field reflection of the corresponding field.
    #[allow(non_camel_case_types, clippy::type_complexity)]
    #[inline]
    pub const fn reflection<I, __GROST_FLAVOR__: ?::core::marker::Sized>(
      &self,
    ) -> &'static ::grost::__private::reflection::ObjectField
    where
      ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::reflection::ObjectField,
        __GROST_FLAVOR__,
        1u32,
      >: ::grost::__private::reflection::Reflectable<
          User<I>,
          Reflection = ::grost::__private::reflection::ObjectField,
        >,
      ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::reflection::ObjectField,
        __GROST_FLAVOR__,
        2u32,
      >: ::grost::__private::reflection::Reflectable<
          User<I>,
          Reflection = ::grost::__private::reflection::ObjectField,
        >,
      ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::reflection::ObjectField,
        __GROST_FLAVOR__,
        3u32,
      >: ::grost::__private::reflection::Reflectable<
          User<I>,
          Reflection = ::grost::__private::reflection::ObjectField,
        >,
    {
      match self {
        Self::Id => {
          <::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            ::grost::__private::reflection::ObjectField,
            __GROST_FLAVOR__,
            1u32,
          > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
        }
        Self::Name => {
          <::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            ::grost::__private::reflection::ObjectField,
            __GROST_FLAVOR__,
            2u32,
          > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
        }
        Self::Age => {
          <::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            ::grost::__private::reflection::ObjectField,
            __GROST_FLAVOR__,
            3u32,
          > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
        }
      }
    }
  }
  #[automatically_derived]
  impl UserIndex {
    /// The number of variants of this field indexer.
    pub const VARIANTS: ::core::primitive::usize = 3usize;
    /// The first field indexer.
    pub const FIRST: Self = Self::Id;
    /// The last field indexer.
    pub const LAST: Self = Self::Age;
    /// Returns the next field indexer.
    ///
    /// Returns `None` if there are no more fields.
    #[inline]
    pub const fn next(&self) -> ::core::option::Option<Self> {
      match self {
        Self::Age => ::core::option::Option::None,
        Self::Id => ::core::option::Option::Some(Self::Name),
        Self::Name => ::core::option::Option::Some(Self::Age),
      }
    }
    /// Returns the previous field indexer.
    ///
    /// Returns `None` if there are no previous fields.
    #[inline]
    pub const fn prev(&self) -> ::core::option::Option<Self> {
      match self {
        Self::Id => ::core::option::Option::None,
        Self::Age => ::core::option::Option::Some(Self::Name),
        Self::Name => ::core::option::Option::Some(Self::Id),
      }
    }
    /// Returns the remaining number of fields.
    #[inline]
    pub const fn remaining(&self) -> ::core::primitive::usize {
      Self::LAST as ::core::primitive::usize
        - *self as ::core::primitive::u32 as ::core::primitive::usize
    }
  }
  #[automatically_derived]
  impl ::core::iter::Iterator for UserIndex {
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
  impl ::core::iter::DoubleEndedIterator for UserIndex {
    fn next_back(&mut self) -> ::core::option::Option<Self> {
      Self::prev(self)
    }
  }
  #[automatically_derived]
  impl ::core::iter::FusedIterator for UserIndex {}
  #[automatically_derived]
  impl ::core::iter::ExactSizeIterator for UserIndex {
    fn len(&self) -> ::core::primitive::usize {
      self.remaining()
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl<I, __GROST_FLAVOR__: ?::core::marker::Sized> ::core::fmt::Debug
    for UserSelector<I, __GROST_FLAVOR__>
    where
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        I: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        String: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        u8: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
    {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            f.debug_struct("UserSelector")
                .field("id", &self.id)
                .field("name", &self.name)
                .field("age", &self.age)
                .finish()
        }
    }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl<I, __GROST_FLAVOR__: ?::core::marker::Sized> ::core::cmp::PartialEq
    for UserSelector<I, __GROST_FLAVOR__>
    where
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        I: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        String: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        u8: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
    {
        fn eq(&self, other: &Self) -> ::core::primitive::bool {
            self.id == other.id && self.name == other.name && self.age == other.age
        }
    }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl<I, __GROST_FLAVOR__: ?::core::marker::Sized> ::core::cmp::Eq
    for UserSelector<I, __GROST_FLAVOR__>
    where
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        I: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        String: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        u8: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
    {}
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl<I, __GROST_FLAVOR__: ?::core::marker::Sized> ::core::hash::Hash
    for UserSelector<I, __GROST_FLAVOR__>
    where
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        I: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        String: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        u8: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
    {
        fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
            self.id.hash(state);
            self.name.hash(state);
            self.age.hash(state);
        }
    }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl<I, __GROST_FLAVOR__: ?::core::marker::Sized> ::core::clone::Clone
    for UserSelector<I, __GROST_FLAVOR__>
    where
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        I: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        String: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        u8: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
    {
        fn clone(&self) -> Self {
            Self {
                id: ::core::clone::Clone::clone(&self.id),
                name: ::core::clone::Clone::clone(&self.name),
                age: ::core::clone::Clone::clone(&self.age),
                ___w__: ::core::marker::PhantomData,
            }
        }
    }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl<I, __GROST_FLAVOR__: ?::core::marker::Sized> ::core::marker::Copy
    for UserSelector<I, __GROST_FLAVOR__>
    where
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        I: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        String: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        u8: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        <I as ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >>::Selector: ::core::marker::Copy,
        <String as ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >>::Selector: ::core::marker::Copy,
        <u8 as ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >>::Selector: ::core::marker::Copy,
    {}
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl<
        I,
        __GROST_FLAVOR__: ?::core::marker::Sized,
    > ::grost::__private::selection::Selector<__GROST_FLAVOR__>
    for UserSelector<I, __GROST_FLAVOR__>
    where
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        I: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        String: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        u8: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
    {
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
            <<I as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::flip(&mut self.id);
            <<String as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::flip(&mut self.name);
            <<u8 as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::flip(&mut self.age);
            self
        }
        fn merge(&mut self, other: Self) -> &mut Self {
            <<I as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::merge(&mut self.id, other.id);
            <<String as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::merge(&mut self.name, other.name);
            <<u8 as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::merge(&mut self.age, other.age);
            self
        }
    }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<I, __GROST_FLAVOR__: ?::core::marker::Sized> ::core::default::Default
    for UserSelector<I, __GROST_FLAVOR__>
    where
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        I: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        String: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        u8: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
    {
        fn default() -> Self {
            Self::new()
        }
    }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<I, __GROST_FLAVOR__: ?::core::marker::Sized> UserSelector<I, __GROST_FLAVOR__>
    where
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        I: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        String: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        u8: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
    {
        /// The number of options in this selection type.
        pub const OPTIONS: ::core::primitive::usize = 3usize;
        /// Returns a selector with the default values.
        #[inline]
        pub const fn new() -> Self {
            Self {
                id: <<I as ::grost::__private::selection::Selectable<
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<
                    __GROST_FLAVOR__,
                >>::DEFAULT,
                name: <<String as ::grost::__private::selection::Selectable<
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<
                    __GROST_FLAVOR__,
                >>::DEFAULT,
                age: <<u8 as ::grost::__private::selection::Selectable<
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<
                    __GROST_FLAVOR__,
                >>::DEFAULT,
                ___w__: ::core::marker::PhantomData,
            }
        }
        /// Returns a selector which selects nothing.
        #[inline]
        pub const fn empty() -> Self {
            Self {
                id: <<I as ::grost::__private::selection::Selectable<
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<
                    __GROST_FLAVOR__,
                >>::NONE,
                name: <<String as ::grost::__private::selection::Selectable<
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<
                    __GROST_FLAVOR__,
                >>::NONE,
                age: <<u8 as ::grost::__private::selection::Selectable<
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<
                    __GROST_FLAVOR__,
                >>::NONE,
                ___w__: ::core::marker::PhantomData,
            }
        }
        /// Returns a selector which selects all.
        #[inline]
        pub const fn all() -> Self {
            Self {
                id: <<I as ::grost::__private::selection::Selectable<
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<
                    __GROST_FLAVOR__,
                >>::ALL,
                name: <<String as ::grost::__private::selection::Selectable<
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<
                    __GROST_FLAVOR__,
                >>::ALL,
                age: <<u8 as ::grost::__private::selection::Selectable<
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<
                    __GROST_FLAVOR__,
                >>::ALL,
                ___w__: ::core::marker::PhantomData,
            }
        }
        /// Returns `true` if the selector selects nothing.
        #[inline]
        pub fn is_empty(&self) -> ::core::primitive::bool {
            <<I as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.id)
                && <<String as ::grost::__private::selection::Selectable<
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<
                    __GROST_FLAVOR__,
                >>::is_empty(&self.name)
                && <<u8 as ::grost::__private::selection::Selectable<
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<
                    __GROST_FLAVOR__,
                >>::is_empty(&self.age)
        }
        /// Returns `true` if the selector selects all.
        #[inline]
        pub fn is_all(&self) -> ::core::primitive::bool {
            <<I as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::is_all(&self.id)
                && <<String as ::grost::__private::selection::Selectable<
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<
                    __GROST_FLAVOR__,
                >>::is_all(&self.name)
                && <<u8 as ::grost::__private::selection::Selectable<
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                >>::Selector as ::grost::__private::selection::Selector<
                    __GROST_FLAVOR__,
                >>::is_all(&self.age)
        }
        /// Returns the number of selected fields.
        #[inline]
        pub fn selected(&self) -> ::core::primitive::usize {
            let mut num = 0;
            if self.is_id_selected() {
                num += 1;
            }
            if self.is_name_selected() {
                num += 1;
            }
            if self.is_age_selected() {
                num += 1;
            }
            num
        }
        /// Returns the number of unselected fields.
        #[inline]
        pub fn unselected(&self) -> ::core::primitive::usize {
            let mut num = 0;
            if self.is_id_unselected() {
                num += 1;
            }
            if self.is_name_unselected() {
                num += 1;
            }
            if self.is_age_unselected() {
                num += 1;
            }
            num
        }
        /// Returns an iterator over the selected fields.
        #[inline]
        pub fn iter_selected(&self) -> UserSelectorIter<'_, I, __GROST_FLAVOR__, true> {
            UserSelectorIter::new(self, self.selected())
        }
        /// Returns an iterator over the unselected fields.
        #[inline]
        pub fn iter_unselected(
            &self,
        ) -> UserSelectorIter<'_, I, __GROST_FLAVOR__, false> {
            UserSelectorIter::new(self, self.unselected())
        }
        /// Returns `true` if such field is selected.
        #[inline]
        pub fn is_selected(&self, idx: UserIndex) -> ::core::primitive::bool {
            match idx {
                UserIndex::Id => self.is_id_selected(),
                UserIndex::Name => self.is_name_selected(),
                UserIndex::Age => self.is_age_selected(),
            }
        }
        /// Returns `true` if such field is unselected.
        #[inline]
        pub fn is_unselected(&self, idx: UserIndex) -> ::core::primitive::bool {
            match idx {
                UserIndex::Id => self.is_id_unselected(),
                UserIndex::Name => self.is_name_unselected(),
                UserIndex::Age => self.is_age_unselected(),
            }
        }
        /// Select the `User.id` field
        #[inline]
        pub fn select_id(&mut self) -> &mut Self {
            self.id = <<I as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::DEFAULT;
            self
        }
        /// Unselect the `User.id` field
        #[inline]
        pub fn unselect_id(&mut self) -> &mut Self {
            self.id = <<I as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::NONE;
            self
        }
        /// Update the `User.id` field
        #[inline]
        pub fn update_id(
            &mut self,
            value: <I as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector,
        ) -> &mut Self {
            self.id = value;
            self
        }
        /// Set or unset the `User.id` field
        #[inline]
        pub fn maybe_id(
            mut self,
            val: <I as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector,
        ) -> Self {
            self.id = val;
            self
        }
        /// Get a reference to the selector of `User.id` field
        #[inline]
        pub const fn id_ref(
            &self,
        ) -> &<I as ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >>::Selector {
            &self.id
        }
        /// Get a mutable reference to the selector of `User.id` field
        #[inline]
        pub const fn id_mut(
            &mut self,
        ) -> &mut <I as ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >>::Selector {
            &mut self.id
        }
        /// Set the `User.id` field
        #[inline]
        pub fn with_id(mut self) -> Self {
            self.id = <<I as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::DEFAULT;
            self
        }
        /// Unset the `User.id` field
        #[inline]
        pub fn without_id(mut self) -> Self {
            self.id = <<I as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::NONE;
            self
        }
        /// Returns `true` if the `User.id` field is selected
        #[inline]
        pub fn is_id_selected(&self) -> ::core::primitive::bool {
            !<<I as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.id)
        }
        /// Returns `true` if the `User.id` field is unselected
        #[inline]
        pub fn is_id_unselected(&self) -> ::core::primitive::bool {
            <<I as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.id)
        }
        /// Select the `User.name` field
        #[inline]
        pub fn select_name(&mut self) -> &mut Self {
            self.name = <<String as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::DEFAULT;
            self
        }
        /// Unselect the `User.name` field
        #[inline]
        pub fn unselect_name(&mut self) -> &mut Self {
            self.name = <<String as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::NONE;
            self
        }
        /// Update the `User.name` field
        #[inline]
        pub fn update_name(
            &mut self,
            value: <String as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector,
        ) -> &mut Self {
            self.name = value;
            self
        }
        /// Set or unset the `User.name` field
        #[inline]
        pub fn maybe_name(
            mut self,
            val: <String as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector,
        ) -> Self {
            self.name = val;
            self
        }
        /// Get a reference to the selector of `User.name` field
        #[inline]
        pub const fn name_ref(
            &self,
        ) -> &<String as ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >>::Selector {
            &self.name
        }
        /// Get a mutable reference to the selector of `User.name` field
        #[inline]
        pub const fn name_mut(
            &mut self,
        ) -> &mut <String as ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >>::Selector {
            &mut self.name
        }
        /// Set the `User.name` field
        #[inline]
        pub fn with_name(mut self) -> Self {
            self.name = <<String as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::DEFAULT;
            self
        }
        /// Unset the `User.name` field
        #[inline]
        pub fn without_name(mut self) -> Self {
            self.name = <<String as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::NONE;
            self
        }
        /// Returns `true` if the `User.name` field is selected
        #[inline]
        pub fn is_name_selected(&self) -> ::core::primitive::bool {
            !<<String as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.name)
        }
        /// Returns `true` if the `User.name` field is unselected
        #[inline]
        pub fn is_name_unselected(&self) -> ::core::primitive::bool {
            <<String as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.name)
        }
        /// Select the `User.age` field
        #[inline]
        pub fn select_age(&mut self) -> &mut Self {
            self.age = <<u8 as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::DEFAULT;
            self
        }
        /// Unselect the `User.age` field
        #[inline]
        pub fn unselect_age(&mut self) -> &mut Self {
            self.age = <<u8 as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::NONE;
            self
        }
        /// Update the `User.age` field
        #[inline]
        pub fn update_age(
            &mut self,
            value: <u8 as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector,
        ) -> &mut Self {
            self.age = value;
            self
        }
        /// Set or unset the `User.age` field
        #[inline]
        pub fn maybe_age(
            mut self,
            val: <u8 as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector,
        ) -> Self {
            self.age = val;
            self
        }
        /// Get a reference to the selector of `User.age` field
        #[inline]
        pub const fn age_ref(
            &self,
        ) -> &<u8 as ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >>::Selector {
            &self.age
        }
        /// Get a mutable reference to the selector of `User.age` field
        #[inline]
        pub const fn age_mut(
            &mut self,
        ) -> &mut <u8 as ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >>::Selector {
            &mut self.age
        }
        /// Set the `User.age` field
        #[inline]
        pub fn with_age(mut self) -> Self {
            self.age = <<u8 as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::DEFAULT;
            self
        }
        /// Unset the `User.age` field
        #[inline]
        pub fn without_age(mut self) -> Self {
            self.age = <<u8 as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::NONE;
            self
        }
        /// Returns `true` if the `User.age` field is selected
        #[inline]
        pub fn is_age_selected(&self) -> ::core::primitive::bool {
            !<<u8 as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.age)
        }
        /// Returns `true` if the `User.age` field is unselected
        #[inline]
        pub fn is_age_unselected(&self) -> ::core::primitive::bool {
            <<u8 as ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >>::Selector as ::grost::__private::selection::Selector<
                __GROST_FLAVOR__,
            >>::is_empty(&self.age)
        }
    }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<I> User<I> {
    /// Returns the selector with default selections
    #[inline]
        pub const fn select<__GROST_FLAVOR__: ?::core::marker::Sized>() -> UserSelector<
            I,
            __GROST_FLAVOR__,
        >
        where
            ::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                1u32,
            >: ::grost::__private::reflection::Reflectable<User<I>>,
            I: ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >,
            ::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                2u32,
            >: ::grost::__private::reflection::Reflectable<User<I>>,
            String: ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >,
            ::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                3u32,
            >: ::grost::__private::reflection::Reflectable<User<I>>,
            u8: ::grost::__private::selection::Selectable<
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            >,
        {
      UserSelector::new()
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<
        I,
        __GROST_FLAVOR__: ?::core::marker::Sized,
        __GROST_WIRE_FORMAT__: ?::core::marker::Sized,
    > ::grost::__private::selection::Selectable<__GROST_FLAVOR__, __GROST_WIRE_FORMAT__>
    for User<I>
    where
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        I: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        String: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        u8: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
    {
        type Selector = UserSelector<I, __GROST_FLAVOR__>;
    }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<
        I,
        __GROST_UNKNOWN_BUFFER__,
        __GROST_WIRE_FORMAT__: ?::core::marker::Sized,
        __GROST_FLAVOR__: ?::core::marker::Sized,
    > ::grost::__private::selection::Selectable<__GROST_FLAVOR__, __GROST_WIRE_FORMAT__>
    for PartialUser<I, __GROST_UNKNOWN_BUFFER__>
    where
        I: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Flatten,
        >>::Output: ::core::marker::Sized,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        I: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        String: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        u8: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
    {
        type Selector = UserSelector<I, __GROST_FLAVOR__>;
    }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<
        '__grost_lifetime__,
        I,
        __GROST_FLAVOR__: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
        __GROST_UNKNOWN_BUFFER__,
        __GROST_WIRE_FORMAT__: ?::core::marker::Sized,
    > ::grost::__private::selection::Selectable<__GROST_FLAVOR__, __GROST_WIRE_FORMAT__>
    for PartialRefUser<
        '__grost_lifetime__,
        I,
        __GROST_FLAVOR__,
        __GROST_UNKNOWN_BUFFER__,
    >
    where
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        > as ::grost::__private::reflection::Reflectable<
            User<I>,
        >>::Reflection: ::grost::__private::flavors::WireFormat<__GROST_FLAVOR__>,
        I: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        > as ::grost::__private::reflection::Reflectable<
            User<I>,
        >>::Reflection: ::grost::__private::flavors::WireFormat<__GROST_FLAVOR__>,
        String: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <String as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        > as ::grost::__private::reflection::Reflectable<
            User<I>,
        >>::Reflection: ::grost::__private::flavors::WireFormat<__GROST_FLAVOR__>,
        u8: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <u8 as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        I: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        String: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        u8: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
    {
        type Selector = UserSelector<I, __GROST_FLAVOR__>;
    }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<
        '__grost_lifetime__,
        I,
        __GROST_FLAVOR__: ?::core::marker::Sized,
        const __GROST_SELECTED__: ::core::primitive::bool,
    > UserSelectorIter<'__grost_lifetime__, I, __GROST_FLAVOR__, __GROST_SELECTED__>
    where
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        I: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                1u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        String: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                2u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        u8: ::grost::__private::selection::Selectable<
            __GROST_FLAVOR__,
            <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                __GROST_FLAVOR__,
                3u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        >,
    {
        #[inline]
        const fn new(
            selector: &'__grost_lifetime__ UserSelector<I, __GROST_FLAVOR__>,
            num: ::core::primitive::usize,
        ) -> Self {
            Self {
                selector,
                index: ::core::option::Option::Some(UserIndex::FIRST),
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
        I,
        __GROST_FLAVOR__: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
        __GROST_UNKNOWN_BUFFER__,
    > ::core::default::Default
    for PartialRefUser<
        '__grost_lifetime__,
        I,
        __GROST_FLAVOR__,
        __GROST_UNKNOWN_BUFFER__,
    >
    where
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        > as ::grost::__private::reflection::Reflectable<
            User<I>,
        >>::Reflection: ::grost::__private::flavors::WireFormat<__GROST_FLAVOR__>,
        I: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        > as ::grost::__private::reflection::Reflectable<
            User<I>,
        >>::Reflection: ::grost::__private::flavors::WireFormat<__GROST_FLAVOR__>,
        String: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <String as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        > as ::grost::__private::reflection::Reflectable<
            User<I>,
        >>::Reflection: ::grost::__private::flavors::WireFormat<__GROST_FLAVOR__>,
        u8: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <u8 as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
    {
        fn default() -> Self {
            Self::new()
        }
    }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<
        '__grost_lifetime__,
        I,
        __GROST_FLAVOR__: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
        __GROST_UNKNOWN_BUFFER__,
        __GROST_FLATTEN_STATE__: ?::core::marker::Sized,
    > ::grost::__private::convert::State<
        ::grost::__private::convert::Flatten<__GROST_FLATTEN_STATE__>,
    >
    for PartialRefUser<
        '__grost_lifetime__,
        I,
        __GROST_FLAVOR__,
        __GROST_UNKNOWN_BUFFER__,
    >
    where
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        > as ::grost::__private::reflection::Reflectable<
            User<I>,
        >>::Reflection: ::grost::__private::flavors::WireFormat<__GROST_FLAVOR__>,
        I: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        > as ::grost::__private::reflection::Reflectable<
            User<I>,
        >>::Reflection: ::grost::__private::flavors::WireFormat<__GROST_FLAVOR__>,
        String: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <String as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        > as ::grost::__private::reflection::Reflectable<
            User<I>,
        >>::Reflection: ::grost::__private::flavors::WireFormat<__GROST_FLAVOR__>,
        u8: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <u8 as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
    {
        type Output = Self;
        type Input = Self;
    }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<
        '__grost_lifetime__,
        I,
        __GROST_FLAVOR__: ?::core::marker::Sized + ::grost::__private::flavors::Flavor,
        __GROST_UNKNOWN_BUFFER__,
    > PartialRefUser<
        '__grost_lifetime__,
        I,
        __GROST_FLAVOR__,
        __GROST_UNKNOWN_BUFFER__,
    >
    where
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            1u32,
        > as ::grost::__private::reflection::Reflectable<
            User<I>,
        >>::Reflection: ::grost::__private::flavors::WireFormat<__GROST_FLAVOR__>,
        I: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            2u32,
        > as ::grost::__private::reflection::Reflectable<
            User<I>,
        >>::Reflection: ::grost::__private::flavors::WireFormat<__GROST_FLAVOR__>,
        String: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <String as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
        ::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        >: ::grost::__private::reflection::Reflectable<User<I>>,
        <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            __GROST_FLAVOR__,
            3u32,
        > as ::grost::__private::reflection::Reflectable<
            User<I>,
        >>::Reflection: ::grost::__private::flavors::WireFormat<__GROST_FLAVOR__>,
        u8: ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >,
        <u8 as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output: ::core::marker::Sized + ::core::marker::Copy,
    {
        /// Creates an empty partial struct.
        #[inline]
        pub const fn new() -> Self {
            Self {
                id: ::core::option::Option::None,
                name: ::core::option::Option::None,
                age: ::core::option::Option::None,
                _w: ::core::option::Option::None,
                __grost_unknown_buffer__: ::core::option::Option::None,
            }
        }
        /// Returns `true` if the partial struct is empty, which means all fields are `None`.
        #[inline]
        pub const fn is_empty(&self) -> bool {
            self.__grost_unknown_buffer__.is_none() && self.id.is_none()
                && self.name.is_none() && self.age.is_none()
        }
        /// Returns a reference to the unknown buffer, which holds the unknown data when decoding.
        #[inline]
        pub const fn unknown_buffer(
            &self,
        ) -> ::core::option::Option<&__GROST_UNKNOWN_BUFFER__> {
            self.__grost_unknown_buffer__.as_ref()
        }
        /// Returns a reference to the `id`
        #[inline]
        pub const fn id_ref(
            &self,
        ) -> ::core::option::Option<
            &<I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        > {
            self.id.as_ref()
        }
        /// Returns a mutable reference to the `id`
        #[inline]
        pub const fn id_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        > {
            self.id.as_mut()
        }
        /// Returns a reference to the `id` if it is not `None`
        ///
        /// ## Panics
        ///
        /// - Panics if the `id` is `None`
        #[inline]
        pub const fn unwrap_id_ref(
            &self,
        ) -> &<I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output {
            match self.id.as_ref() {
                ::core::option::Option::Some(value) => value,
                ::core::option::Option::None => panic!("`id` is `None`"),
            }
        }
        /// Returns a mutable reference to the `id` if it is not `None`
        ///
        /// ## Panics
        ///
        /// - Panics if the `id` is `None`
        #[inline]
        pub const fn unwrap_id_mut(
            &mut self,
        ) -> &mut <I as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output {
            match self.id.as_mut() {
                ::core::option::Option::Some(value) => value,
                ::core::option::Option::None => panic!("`id` is `None`"),
            }
        }
        /// Takes the value of `id` out if it is not `None`
        #[inline]
        pub const fn take_id(
            &mut self,
        ) -> ::core::option::Option<
            <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        > {
            self.id.take()
        }
        /// Clear the value of `id`
        #[inline]
        pub const fn clear_id(&mut self) -> &mut Self {
            self.id = ::core::option::Option::None;
            self
        }
        /// Set the `id` to the given value
        #[inline]
        pub const fn set_id(
            &mut self,
            value: <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        ) -> &mut Self {
            self.id = ::core::option::Option::Some(value);
            self
        }
        /// Update the `id` to the given value or clear the `id`
        #[inline]
        pub const fn update_id(
            &mut self,
            value: ::core::option::Option<
                <I as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        __GROST_FLAVOR__,
                        <::grost::__private::reflection::WireFormatReflection<
                            User<I>,
                            __GROST_FLAVOR__,
                            1u32,
                        > as ::grost::__private::reflection::Reflectable<
                            User<I>,
                        >>::Reflection,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
                >>::Output,
            >,
        ) -> &mut Self {
            self.id = value;
            self
        }
        /// Set the `id` to the given value
        #[inline]
        pub const fn with_id(
            mut self,
            value: <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        1u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        ) -> Self {
            self.id = ::core::option::Option::Some(value);
            self
        }
        /// Clear the value of `id`
        #[inline]
        pub const fn without_id(mut self) -> Self {
            self.id = ::core::option::Option::None;
            self
        }
        /// Update the `id` to the given value or clear the `id`
        #[inline]
        pub const fn maybe_id(
            mut self,
            value: ::core::option::Option<
                <I as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        __GROST_FLAVOR__,
                        <::grost::__private::reflection::WireFormatReflection<
                            User<I>,
                            __GROST_FLAVOR__,
                            1u32,
                        > as ::grost::__private::reflection::Reflectable<
                            User<I>,
                        >>::Reflection,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
                >>::Output,
            >,
        ) -> Self {
            self.id = value;
            self
        }
        /// Returns a reference to the `name`
        #[inline]
        pub const fn name_ref(
            &self,
        ) -> ::core::option::Option<
            &<String as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        > {
            self.name.as_ref()
        }
        /// Returns a mutable reference to the `name`
        #[inline]
        pub const fn name_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <String as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        > {
            self.name.as_mut()
        }
        /// Returns a reference to the `name` if it is not `None`
        ///
        /// ## Panics
        ///
        /// - Panics if the `name` is `None`
        #[inline]
        pub const fn unwrap_name_ref(
            &self,
        ) -> &<String as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output {
            match self.name.as_ref() {
                ::core::option::Option::Some(value) => value,
                ::core::option::Option::None => panic!("`name` is `None`"),
            }
        }
        /// Returns a mutable reference to the `name` if it is not `None`
        ///
        /// ## Panics
        ///
        /// - Panics if the `name` is `None`
        #[inline]
        pub const fn unwrap_name_mut(
            &mut self,
        ) -> &mut <String as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output {
            match self.name.as_mut() {
                ::core::option::Option::Some(value) => value,
                ::core::option::Option::None => panic!("`name` is `None`"),
            }
        }
        /// Takes the value of `name` out if it is not `None`
        #[inline]
        pub const fn take_name(
            &mut self,
        ) -> ::core::option::Option<
            <String as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        > {
            self.name.take()
        }
        /// Clear the value of `name`
        #[inline]
        pub const fn clear_name(&mut self) -> &mut Self {
            self.name = ::core::option::Option::None;
            self
        }
        /// Set the `name` to the given value
        #[inline]
        pub const fn set_name(
            &mut self,
            value: <String as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        ) -> &mut Self {
            self.name = ::core::option::Option::Some(value);
            self
        }
        /// Update the `name` to the given value or clear the `name`
        #[inline]
        pub const fn update_name(
            &mut self,
            value: ::core::option::Option<
                <String as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        __GROST_FLAVOR__,
                        <::grost::__private::reflection::WireFormatReflection<
                            User<I>,
                            __GROST_FLAVOR__,
                            2u32,
                        > as ::grost::__private::reflection::Reflectable<
                            User<I>,
                        >>::Reflection,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
                >>::Output,
            >,
        ) -> &mut Self {
            self.name = value;
            self
        }
        /// Set the `name` to the given value
        #[inline]
        pub const fn with_name(
            mut self,
            value: <String as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        2u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        ) -> Self {
            self.name = ::core::option::Option::Some(value);
            self
        }
        /// Clear the value of `name`
        #[inline]
        pub const fn without_name(mut self) -> Self {
            self.name = ::core::option::Option::None;
            self
        }
        /// Update the `name` to the given value or clear the `name`
        #[inline]
        pub const fn maybe_name(
            mut self,
            value: ::core::option::Option<
                <String as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        __GROST_FLAVOR__,
                        <::grost::__private::reflection::WireFormatReflection<
                            User<I>,
                            __GROST_FLAVOR__,
                            2u32,
                        > as ::grost::__private::reflection::Reflectable<
                            User<I>,
                        >>::Reflection,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
                >>::Output,
            >,
        ) -> Self {
            self.name = value;
            self
        }
        /// Returns a reference to the `age`
        #[inline]
        pub const fn age_ref(
            &self,
        ) -> ::core::option::Option<
            &<u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        > {
            self.age.as_ref()
        }
        /// Returns a mutable reference to the `age`
        #[inline]
        pub const fn age_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        > {
            self.age.as_mut()
        }
        /// Returns a reference to the `age` if it is not `None`
        ///
        /// ## Panics
        ///
        /// - Panics if the `age` is `None`
        #[inline]
        pub const fn unwrap_age_ref(
            &self,
        ) -> &<u8 as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output {
            match self.age.as_ref() {
                ::core::option::Option::Some(value) => value,
                ::core::option::Option::None => panic!("`age` is `None`"),
            }
        }
        /// Returns a mutable reference to the `age` if it is not `None`
        ///
        /// ## Panics
        ///
        /// - Panics if the `age` is `None`
        #[inline]
        pub const fn unwrap_age_mut(
            &mut self,
        ) -> &mut <u8 as ::grost::__private::convert::State<
            ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                __GROST_FLAVOR__,
                <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    __GROST_FLAVOR__,
                    3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
            >,
        >>::Output {
            match self.age.as_mut() {
                ::core::option::Option::Some(value) => value,
                ::core::option::Option::None => panic!("`age` is `None`"),
            }
        }
        /// Takes the value of `age` out if it is not `None`
        #[inline]
        pub const fn take_age(
            &mut self,
        ) -> ::core::option::Option<
            <u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        > {
            self.age.take()
        }
        /// Clear the value of `age`
        #[inline]
        pub const fn clear_age(&mut self) -> &mut Self {
            self.age = ::core::option::Option::None;
            self
        }
        /// Set the `age` to the given value
        #[inline]
        pub const fn set_age(
            &mut self,
            value: <u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        ) -> &mut Self {
            self.age = ::core::option::Option::Some(value);
            self
        }
        /// Update the `age` to the given value or clear the `age`
        #[inline]
        pub const fn update_age(
            &mut self,
            value: ::core::option::Option<
                <u8 as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        __GROST_FLAVOR__,
                        <::grost::__private::reflection::WireFormatReflection<
                            User<I>,
                            __GROST_FLAVOR__,
                            3u32,
                        > as ::grost::__private::reflection::Reflectable<
                            User<I>,
                        >>::Reflection,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
                >>::Output,
            >,
        ) -> &mut Self {
            self.age = value;
            self
        }
        /// Set the `age` to the given value
        #[inline]
        pub const fn with_age(
            mut self,
            value: <u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                    '__grost_lifetime__,
                    __GROST_FLAVOR__,
                    <::grost::__private::reflection::WireFormatReflection<
                        User<I>,
                        __GROST_FLAVOR__,
                        3u32,
                    > as ::grost::__private::reflection::Reflectable<
                        User<I>,
                    >>::Reflection,
                    __GROST_UNKNOWN_BUFFER__,
                >,
            >>::Output,
        ) -> Self {
            self.age = ::core::option::Option::Some(value);
            self
        }
        /// Clear the value of `age`
        #[inline]
        pub const fn without_age(mut self) -> Self {
            self.age = ::core::option::Option::None;
            self
        }
        /// Update the `age` to the given value or clear the `age`
        #[inline]
        pub const fn maybe_age(
            mut self,
            value: ::core::option::Option<
                <u8 as ::grost::__private::convert::State<
                    ::grost::__private::convert::Decoded<
                        '__grost_lifetime__,
                        __GROST_FLAVOR__,
                        <::grost::__private::reflection::WireFormatReflection<
                            User<I>,
                            __GROST_FLAVOR__,
                            3u32,
                        > as ::grost::__private::reflection::Reflectable<
                            User<I>,
                        >>::Reflection,
                        __GROST_UNKNOWN_BUFFER__,
                    >,
                >>::Output,
            >,
        ) -> Self {
            self.age = value;
            self
        }
    }
  #[automatically_derived]
  #[allow(non_camel_case_types)]
  impl<I, __GROST_UNKNOWN_BUFFER__> ::core::default::Default
    for PartialUser<I, __GROST_UNKNOWN_BUFFER__>
  where
    I: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
    <I as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output:
      ::core::marker::Sized,
  {
    fn default() -> Self {
      Self::new()
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<I, __GROST_UNKNOWN_BUFFER__, __GROST_FLATTEN_STATE__: ?::core::marker::Sized>
    ::grost::__private::convert::State<
      ::grost::__private::convert::Flatten<__GROST_FLATTEN_STATE__>,
    > for PartialUser<I, __GROST_UNKNOWN_BUFFER__>
  where
    I: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
    <I as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output:
      ::core::marker::Sized,
  {
    type Output = Self;
    type Input = Self;
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<I, __GROST_UNKNOWN_BUFFER__> PartialUser<I, __GROST_UNKNOWN_BUFFER__>
  where
    I: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
    <I as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output:
      ::core::marker::Sized,
  {
    /// Creates an empty partial struct.
    #[inline]
    pub const fn new() -> Self {
      Self {
        __grost_unknown_buffer__: ::core::option::Option::None,
        id: ::core::option::Option::None,
        name: ::core::option::Option::None,
        age: ::core::option::Option::None,
        _w: ::core::option::Option::None,
      }
    }
    /// Returns `true` if the partial object is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
      self.id.is_none() && self.name.is_none() && self.age.is_none()
    }
    /// Returns a reference to the unknown buffer, which holds the unknown data when decoding.
    #[inline]
    pub const fn unknown_buffer(&self) -> ::core::option::Option<&__GROST_UNKNOWN_BUFFER__> {
      self.__grost_unknown_buffer__.as_ref()
    }
    /// Returns a reference to the `id`
    #[inline]
    pub const fn id_ref(
      &self,
    ) -> ::core::option::Option<
      &<I as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
    > {
      self.id.as_ref()
    }
    /// Returns a mutable reference to the `id`
    #[inline]
    pub const fn id_mut(
      &mut self,
    ) -> ::core::option::Option<
      &mut <I as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
    > {
      self.id.as_mut()
    }
    /// Returns a reference to the `id` if it is not `None`
    ///
    /// ## Panics
    ///
    /// - Panics if the `id` is `None`
    #[inline]
    pub const fn unwrap_id_ref(
      &self,
    ) -> &<I as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output
    {
      match self.id.as_ref() {
        ::core::option::Option::Some(value) => value,
        ::core::option::Option::None => panic!("`id` is `None`"),
      }
    }
    /// Returns a mutable reference to the `id` if it is not `None`
    ///
    /// ## Panics
    ///
    /// - Panics if the `id` is `None`
    #[inline]
    pub const fn unwrap_id_mut(
      &mut self,
    ) -> &mut <I as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output
    {
      match self.id.as_mut() {
        ::core::option::Option::Some(value) => value,
        ::core::option::Option::None => panic!("`id` is `None`"),
      }
    }
    /// Takes the value of `id` out if it is not `None`
    #[inline]
    pub const fn take_id(
      &mut self,
    ) -> ::core::option::Option<
      <I as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
    > {
      self.id.take()
    }
    /// Clear the value of `id`
    #[inline]
    pub fn clear_id(&mut self) -> &mut Self {
      self.id = ::core::option::Option::None;
      self
    }
    /// Set the `id` to the given value
    #[inline]
    pub fn set_id(
      &mut self,
      value: <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
    ) -> &mut Self {
      self.id = ::core::option::Option::Some(value);
      self
    }
    /// Update the `id` to the given value or clear the `id`
    #[inline]
    pub fn update_id(
      &mut self,
      value: ::core::option::Option<
        <I as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
      >,
    ) -> &mut Self {
      self.id = value;
      self
    }
    /// Set the `id` to the given value
    #[inline]
    pub fn with_id(
      mut self,
      value: <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
    ) -> Self {
      self.id = ::core::option::Option::Some(value);
      self
    }
    /// Clear the value of `id`
    #[inline]
    pub fn without_id(mut self) -> Self {
      self.id = ::core::option::Option::None;
      self
    }
    /// Update the `id` to the given value or clear the `id`
    #[inline]
    pub fn maybe_id(
      mut self,
      value: ::core::option::Option<
        <I as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
      >,
    ) -> Self {
      self.id = value;
      self
    }
    /// Returns a reference to the `name`
    #[inline]
    pub const fn name_ref(
      &self,
    ) -> ::core::option::Option<
      &<String as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
    > {
      self.name.as_ref()
    }
    /// Returns a mutable reference to the `name`
    #[inline]
        pub const fn name_mut(
            &mut self,
        ) -> ::core::option::Option<
            &mut <String as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
    >{
      self.name.as_mut()
    }
    /// Returns a reference to the `name` if it is not `None`
    ///
    /// ## Panics
    ///
    /// - Panics if the `name` is `None`
    #[inline]
    pub const fn unwrap_name_ref(
      &self,
    ) -> &<String as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output
    {
      match self.name.as_ref() {
        ::core::option::Option::Some(value) => value,
        ::core::option::Option::None => panic!("`name` is `None`"),
      }
    }
    /// Returns a mutable reference to the `name` if it is not `None`
    ///
    /// ## Panics
    ///
    /// - Panics if the `name` is `None`
    #[inline]
        pub const fn unwrap_name_mut(
            &mut self,
        ) -> &mut <String as ::grost::__private::convert::State<
            ::grost::__private::convert::Flatten,
    >>::Output{
      match self.name.as_mut() {
        ::core::option::Option::Some(value) => value,
        ::core::option::Option::None => panic!("`name` is `None`"),
      }
    }
    /// Takes the value of `name` out if it is not `None`
    #[inline]
    pub const fn take_name(
      &mut self,
    ) -> ::core::option::Option<
      <String as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
    > {
      self.name.take()
    }
    /// Clear the value of `name`
    #[inline]
    pub fn clear_name(&mut self) -> &mut Self {
      self.name = ::core::option::Option::None;
      self
    }
    /// Set the `name` to the given value
    #[inline]
    pub fn set_name(
      &mut self,
      value: <String as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
    ) -> &mut Self {
      self.name = ::core::option::Option::Some(value);
      self
    }
    /// Update the `name` to the given value or clear the `name`
    #[inline]
    pub fn update_name(
      &mut self,
      value: ::core::option::Option<
                <String as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flatten,
                >>::Output,
            >,
    ) -> &mut Self {
      self.name = value;
      self
    }
    /// Set the `name` to the given value
    #[inline]
    pub fn with_name(
      mut self,
      value: <String as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
    ) -> Self {
      self.name = ::core::option::Option::Some(value);
      self
    }
    /// Clear the value of `name`
    #[inline]
    pub fn without_name(mut self) -> Self {
      self.name = ::core::option::Option::None;
      self
    }
    /// Update the `name` to the given value or clear the `name`
    #[inline]
    pub fn maybe_name(
      mut self,
      value: ::core::option::Option<
                <String as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flatten,
                >>::Output,
            >,
    ) -> Self {
      self.name = value;
      self
    }
    /// Returns a reference to the `age`
    #[inline]
    pub const fn age_ref(
      &self,
    ) -> ::core::option::Option<
      &<u8 as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
    > {
      self.age.as_ref()
    }
    /// Returns a mutable reference to the `age`
    #[inline]
    pub const fn age_mut(
      &mut self,
    ) -> ::core::option::Option<
      &mut <u8 as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
    > {
      self.age.as_mut()
    }
    /// Returns a reference to the `age` if it is not `None`
    ///
    /// ## Panics
    ///
    /// - Panics if the `age` is `None`
    #[inline]
    pub const fn unwrap_age_ref(
      &self,
    ) -> &<u8 as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output
    {
      match self.age.as_ref() {
        ::core::option::Option::Some(value) => value,
        ::core::option::Option::None => panic!("`age` is `None`"),
      }
    }
    /// Returns a mutable reference to the `age` if it is not `None`
    ///
    /// ## Panics
    ///
    /// - Panics if the `age` is `None`
    #[inline]
    pub const fn unwrap_age_mut(
      &mut self,
    ) -> &mut <u8 as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output
    {
      match self.age.as_mut() {
        ::core::option::Option::Some(value) => value,
        ::core::option::Option::None => panic!("`age` is `None`"),
      }
    }
    /// Takes the value of `age` out if it is not `None`
    #[inline]
    pub const fn take_age(
      &mut self,
    ) -> ::core::option::Option<
      <u8 as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
    > {
      self.age.take()
    }
    /// Clear the value of `age`
    #[inline]
    pub const fn clear_age(&mut self) -> &mut Self {
      self.age = ::core::option::Option::None;
      self
    }
    /// Set the `age` to the given value
    #[inline]
    pub const fn set_age(
      &mut self,
      value: <u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
    ) -> &mut Self {
      self.age = ::core::option::Option::Some(value);
      self
    }
    /// Update the `age` to the given value or clear the `age`
    #[inline]
    pub const fn update_age(
      &mut self,
      value: ::core::option::Option<
        <u8 as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
      >,
    ) -> &mut Self {
      self.age = value;
      self
    }
    /// Set the `age` to the given value
    #[inline]
    pub const fn with_age(
      mut self,
      value: <u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Flatten,
            >>::Output,
    ) -> Self {
      self.age = ::core::option::Option::Some(value);
      self
    }
    /// Clear the value of `age`
    #[inline]
    pub const fn without_age(mut self) -> Self {
      self.age = ::core::option::Option::None;
      self
    }
    /// Update the `age` to the given value or clear the `age`
    #[inline]
    pub const fn maybe_age(
      mut self,
      value: ::core::option::Option<
        <u8 as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output,
      >,
    ) -> Self {
      self.age = value;
      self
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<I> ::core::default::Default for User<I>
  where
    I: ::core::default::Default,
  {
    fn default() -> Self {
      Self::new()
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<I> User<I>
  where
    I: ::core::default::Default,
  {
    /// Creates a new instance with default values.
    #[inline]
    pub fn new() -> Self {
      Self {
        id: ::core::default::Default::default(),
        name: ::core::default::Default::default(),
        age: ::core::default::Default::default(),
        _w: ::core::default::Default::default(),
      }
    }
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<I, __GROST_FLATTEN_STATE__: ?::core::marker::Sized>
    ::grost::__private::convert::State<
      ::grost::__private::convert::Flatten<__GROST_FLATTEN_STATE__>,
    > for User<I>
  {
    type Output = Self;
    type Input = Self;
  }
  #[automatically_derived]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  impl<I> User<I> {
    /// Returns a reference to the `id`
    #[inline]
    pub const fn id_ref(&self) -> &I {
      &self.id
    }
    /// Returns a mutable reference to the `id`
    #[inline]
    pub const fn id_mut(&mut self) -> &mut I {
      &mut self.id
    }
    /// Set the `id` to the given value
    #[inline]
    pub fn set_id(&mut self, value: I) -> &mut Self {
      self.id = value;
      self
    }
    /// Set the `id` to the given value
    #[inline]
    pub fn with_id(mut self, value: I) -> Self {
      self.id = value;
      self
    }
    /// Returns a reference to the `name`
    #[inline]
    pub const fn name_ref(&self) -> &String {
      &self.name
    }
    /// Returns a mutable reference to the `name`
    #[inline]
    pub const fn name_mut(&mut self) -> &mut String {
      &mut self.name
    }
    /// Set the `name` to the given value
    #[inline]
    pub fn set_name(&mut self, value: String) -> &mut Self {
      self.name = value;
      self
    }
    /// Set the `name` to the given value
    #[inline]
    pub fn with_name(mut self, value: String) -> Self {
      self.name = value;
      self
    }
    /// Returns a reference to the `age`
    #[inline]
    pub const fn age_ref(&self) -> &u8 {
      &self.age
    }
    /// Returns a mutable reference to the `age`
    #[inline]
    pub const fn age_mut(&mut self) -> &mut u8 {
      &mut self.age
    }
    /// Set the `age` to the given value
    #[inline]
    pub const fn set_age(&mut self, value: u8) -> &mut Self {
      self.age = value;
      self
    }
    /// Set the `age` to the given value
    #[inline]
    pub const fn with_age(mut self, value: u8) -> Self {
      self.age = value;
      self
    }
  }
};
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::IdentifierReflection<
    ::grost::__private::reflection::ObjectFieldReflection<
      User<I>,
      ::grost::__private::flavors::network::Identifier,
      ::grost::__private::flavors::Network,
      1u32,
    >,
  >
{
  type Reflection = ::grost::__private::flavors::network::Identifier;
  const REFLECTION: &Self::Reflection = &{
    ::grost::__private::flavors::network::Identifier::new(
      <grost::flavors::network::LengthDelimited as ::grost::__private::flavors::WireFormat<
        ::grost::__private::flavors::Network,
      >>::WIRE_TYPE,
      ::grost::__private::flavors::network::Tag::new(1u32),
    )
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::IdentifierReflection<
    ::grost::__private::reflection::ObjectFieldReflection<
      User<I>,
      ::grost::__private::flavors::network::Identifier,
      ::grost::__private::flavors::Network,
      2u32,
    >,
  >
{
  type Reflection = ::grost::__private::flavors::network::Identifier;
  const REFLECTION: &Self::Reflection = &{
    ::grost::__private::flavors::network::Identifier::new(
      <<String as ::grost::__private::flavors::DefaultWireFormat<
        ::grost::__private::flavors::Network,
      >>::Format as ::grost::__private::flavors::WireFormat<
        ::grost::__private::flavors::Network,
      >>::WIRE_TYPE,
      ::grost::__private::flavors::network::Tag::new(2u32),
    )
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::IdentifierReflection<
    ::grost::__private::reflection::ObjectFieldReflection<
      User<I>,
      ::grost::__private::flavors::network::Identifier,
      ::grost::__private::flavors::Network,
      3u32,
    >,
  >
{
  type Reflection = ::grost::__private::flavors::network::Identifier;
  const REFLECTION: &Self::Reflection = &{
    ::grost::__private::flavors::network::Identifier::new(
      <<u8 as ::grost::__private::flavors::DefaultWireFormat<
        ::grost::__private::flavors::Network,
      >>::Format as ::grost::__private::flavors::WireFormat<
        ::grost::__private::flavors::Network,
      >>::WIRE_TYPE,
      ::grost::__private::flavors::network::Tag::new(3u32),
    )
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::EncodeReflection<
    ::grost::__private::reflection::IdentifierReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::flavors::network::Identifier,
        ::grost::__private::flavors::Network,
        1u32,
      >,
    >,
  >
{
  type Reflection = [::core::primitive::u8];
  const REFLECTION: &Self::Reflection = {
    <::grost::__private::reflection::IdentifierReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::flavors::network::Identifier,
        ::grost::__private::flavors::Network,
        1u32,
      >,
    > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
      .encode()
      .as_slice()
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::EncodeReflection<
    ::grost::__private::reflection::IdentifierReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::flavors::network::Identifier,
        ::grost::__private::flavors::Network,
        2u32,
      >,
    >,
  >
{
  type Reflection = [::core::primitive::u8];
  const REFLECTION: &Self::Reflection = {
    <::grost::__private::reflection::IdentifierReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::flavors::network::Identifier,
        ::grost::__private::flavors::Network,
        2u32,
      >,
    > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
      .encode()
      .as_slice()
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::EncodeReflection<
    ::grost::__private::reflection::IdentifierReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::flavors::network::Identifier,
        ::grost::__private::flavors::Network,
        3u32,
      >,
    >,
  >
{
  type Reflection = [::core::primitive::u8];
  const REFLECTION: &Self::Reflection = {
    <::grost::__private::reflection::IdentifierReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::flavors::network::Identifier,
        ::grost::__private::flavors::Network,
        3u32,
      >,
    > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
      .encode()
      .as_slice()
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::Len<
    ::grost::__private::reflection::EncodeReflection<
      ::grost::__private::reflection::IdentifierReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
          User<I>,
          ::grost::__private::flavors::network::Identifier,
          ::grost::__private::flavors::Network,
          1u32,
        >,
      >,
    >,
  >
{
  type Reflection = ::core::primitive::usize;
  const REFLECTION: &Self::Reflection = &{
    <::grost::__private::reflection::EncodeReflection<
      ::grost::__private::reflection::IdentifierReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
          User<I>,
          ::grost::__private::flavors::network::Identifier,
          ::grost::__private::flavors::Network,
          1u32,
        >,
      >,
    > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
      .len()
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::Len<
    ::grost::__private::reflection::EncodeReflection<
      ::grost::__private::reflection::IdentifierReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
          User<I>,
          ::grost::__private::flavors::network::Identifier,
          ::grost::__private::flavors::Network,
          2u32,
        >,
      >,
    >,
  >
{
  type Reflection = ::core::primitive::usize;
  const REFLECTION: &Self::Reflection = &{
    <::grost::__private::reflection::EncodeReflection<
      ::grost::__private::reflection::IdentifierReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
          User<I>,
          ::grost::__private::flavors::network::Identifier,
          ::grost::__private::flavors::Network,
          2u32,
        >,
      >,
    > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
      .len()
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::Len<
    ::grost::__private::reflection::EncodeReflection<
      ::grost::__private::reflection::IdentifierReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
          User<I>,
          ::grost::__private::flavors::network::Identifier,
          ::grost::__private::flavors::Network,
          3u32,
        >,
      >,
    >,
  >
{
  type Reflection = ::core::primitive::usize;
  const REFLECTION: &Self::Reflection = &{
    <::grost::__private::reflection::EncodeReflection<
      ::grost::__private::reflection::IdentifierReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
          User<I>,
          ::grost::__private::flavors::network::Identifier,
          ::grost::__private::flavors::Network,
          3u32,
        >,
      >,
    > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
      .len()
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::TagReflection<
    ::grost::__private::reflection::ObjectFieldReflection<
      User<I>,
      ::grost::__private::flavors::network::Tag,
      ::grost::__private::flavors::Network,
      1u32,
    >,
  >
{
  type Reflection = ::grost::__private::flavors::network::Tag;
  const REFLECTION: &Self::Reflection = &{
    <::grost::__private::reflection::IdentifierReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::flavors::network::Identifier,
        ::grost::__private::flavors::Network,
        1u32,
      >,
    > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
      .tag()
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::TagReflection<
    ::grost::__private::reflection::ObjectFieldReflection<
      User<I>,
      ::grost::__private::flavors::network::Tag,
      ::grost::__private::flavors::Network,
      2u32,
    >,
  >
{
  type Reflection = ::grost::__private::flavors::network::Tag;
  const REFLECTION: &Self::Reflection = &{
    <::grost::__private::reflection::IdentifierReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::flavors::network::Identifier,
        ::grost::__private::flavors::Network,
        2u32,
      >,
    > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
      .tag()
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::TagReflection<
    ::grost::__private::reflection::ObjectFieldReflection<
      User<I>,
      ::grost::__private::flavors::network::Tag,
      ::grost::__private::flavors::Network,
      3u32,
    >,
  >
{
  type Reflection = ::grost::__private::flavors::network::Tag;
  const REFLECTION: &Self::Reflection = &{
    <::grost::__private::reflection::IdentifierReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::flavors::network::Identifier,
        ::grost::__private::flavors::Network,
        3u32,
      >,
    > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
      .tag()
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::EncodeReflection<
    ::grost::__private::reflection::TagReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::flavors::network::Tag,
        ::grost::__private::flavors::Network,
        1u32,
      >,
    >,
  >
{
  type Reflection = [::core::primitive::u8];
  const REFLECTION: &Self::Reflection = {
    <::grost::__private::reflection::TagReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::flavors::network::Tag,
        ::grost::__private::flavors::Network,
        1u32,
      >,
    > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
      .encode()
      .as_slice()
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::EncodeReflection<
    ::grost::__private::reflection::TagReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::flavors::network::Tag,
        ::grost::__private::flavors::Network,
        2u32,
      >,
    >,
  >
{
  type Reflection = [::core::primitive::u8];
  const REFLECTION: &Self::Reflection = {
    <::grost::__private::reflection::TagReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::flavors::network::Tag,
        ::grost::__private::flavors::Network,
        2u32,
      >,
    > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
      .encode()
      .as_slice()
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::EncodeReflection<
    ::grost::__private::reflection::TagReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::flavors::network::Tag,
        ::grost::__private::flavors::Network,
        3u32,
      >,
    >,
  >
{
  type Reflection = [::core::primitive::u8];
  const REFLECTION: &Self::Reflection = {
    <::grost::__private::reflection::TagReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::flavors::network::Tag,
        ::grost::__private::flavors::Network,
        3u32,
      >,
    > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
      .encode()
      .as_slice()
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::Len<
    ::grost::__private::reflection::EncodeReflection<
      ::grost::__private::reflection::TagReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
          User<I>,
          ::grost::__private::flavors::network::Tag,
          ::grost::__private::flavors::Network,
          1u32,
        >,
      >,
    >,
  >
{
  type Reflection = ::core::primitive::usize;
  const REFLECTION: &Self::Reflection = &{
    <::grost::__private::reflection::EncodeReflection<
      ::grost::__private::reflection::TagReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
          User<I>,
          ::grost::__private::flavors::network::Tag,
          ::grost::__private::flavors::Network,
          1u32,
        >,
      >,
    > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
      .len()
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::Len<
    ::grost::__private::reflection::EncodeReflection<
      ::grost::__private::reflection::TagReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
          User<I>,
          ::grost::__private::flavors::network::Tag,
          ::grost::__private::flavors::Network,
          2u32,
        >,
      >,
    >,
  >
{
  type Reflection = ::core::primitive::usize;
  const REFLECTION: &Self::Reflection = &{
    <::grost::__private::reflection::EncodeReflection<
      ::grost::__private::reflection::TagReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
          User<I>,
          ::grost::__private::flavors::network::Tag,
          ::grost::__private::flavors::Network,
          2u32,
        >,
      >,
    > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
      .len()
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::Len<
    ::grost::__private::reflection::EncodeReflection<
      ::grost::__private::reflection::TagReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
          User<I>,
          ::grost::__private::flavors::network::Tag,
          ::grost::__private::flavors::Network,
          3u32,
        >,
      >,
    >,
  >
{
  type Reflection = ::core::primitive::usize;
  const REFLECTION: &Self::Reflection = &{
    <::grost::__private::reflection::EncodeReflection<
      ::grost::__private::reflection::TagReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
          User<I>,
          ::grost::__private::flavors::network::Tag,
          ::grost::__private::flavors::Network,
          3u32,
        >,
      >,
    > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
      .len()
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::WireSchemaTypeReflection<
    ::grost::__private::reflection::ObjectFieldReflection<
      User<I>,
      ::grost::__private::flavors::network::WireType,
      ::grost::__private::flavors::Network,
      1u32,
    >,
  >
{
  type Reflection = ::grost::__private::flavors::network::WireType;
  const REFLECTION: &Self::Reflection = &{
    <::grost::__private::reflection::IdentifierReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::flavors::network::Identifier,
        ::grost::__private::flavors::Network,
        1u32,
      >,
    > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
      .wire_type()
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::WireSchemaTypeReflection<
    ::grost::__private::reflection::ObjectFieldReflection<
      User<I>,
      ::grost::__private::flavors::network::WireType,
      ::grost::__private::flavors::Network,
      2u32,
    >,
  >
{
  type Reflection = ::grost::__private::flavors::network::WireType;
  const REFLECTION: &Self::Reflection = &{
    <::grost::__private::reflection::IdentifierReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::flavors::network::Identifier,
        ::grost::__private::flavors::Network,
        2u32,
      >,
    > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
      .wire_type()
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::WireSchemaTypeReflection<
    ::grost::__private::reflection::ObjectFieldReflection<
      User<I>,
      ::grost::__private::flavors::network::WireType,
      ::grost::__private::flavors::Network,
      3u32,
    >,
  >
{
  type Reflection = ::grost::__private::flavors::network::WireType;
  const REFLECTION: &Self::Reflection = &{
    <::grost::__private::reflection::IdentifierReflection<
      ::grost::__private::reflection::ObjectFieldReflection<
        User<I>,
        ::grost::__private::flavors::network::Identifier,
        ::grost::__private::flavors::Network,
        3u32,
      >,
    > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION
      .wire_type()
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    1u32,
  >
{
  type Reflection = grost::flavors::network::LengthDelimited;
  const REFLECTION: &'static Self::Reflection = &{
    <grost::flavors::network::LengthDelimited as ::grost::__private::flavors::WireFormat<
      ::grost::__private::flavors::Network,
    >>::SELF
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    2u32,
  >
{
  type Reflection = <String as ::grost::__private::flavors::DefaultWireFormat<
    ::grost::__private::flavors::Network,
  >>::Format;
  const REFLECTION: &'static Self::Reflection = &{
    <<String as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format as ::grost::__private::flavors::WireFormat<
            ::grost::__private::flavors::Network,
        >>::SELF
  };
}
#[automatically_derived]
#[allow(non_camel_case_types, clippy::type_complexity)]
impl<I> ::grost::__private::reflection::Reflectable<User<I>>
  for ::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    3u32,
  >
{
  type Reflection = <u8 as ::grost::__private::flavors::DefaultWireFormat<
    ::grost::__private::flavors::Network,
  >>::Format;
  const REFLECTION: &'static Self::Reflection = &{
    <<u8 as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Network,
        >>::Format as ::grost::__private::flavors::WireFormat<
            ::grost::__private::flavors::Network,
        >>::SELF
  };
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl<'__grost_lifetime__, I, __GROST_UNKNOWN_BUFFER__>
  ::grost::__private::convert::State<
    ::grost::__private::convert::Decoded<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      ::grost::__private::flavors::network::LengthDelimited,
      __GROST_UNKNOWN_BUFFER__,
    >,
  > for User<I>
where
  ::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    1u32,
  >: ::grost::__private::reflection::Reflectable<User<I>>,
  <::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    1u32,
  > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection:
    ::grost::__private::flavors::WireFormat<::grost::__private::flavors::Network>,
  I: ::grost::__private::convert::State<
      ::grost::__private::convert::Decoded<
        '__grost_lifetime__,
        ::grost::__private::flavors::Network,
        <::grost::__private::reflection::WireFormatReflection<
          User<I>,
          ::grost::__private::flavors::Network,
          1u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        __GROST_UNKNOWN_BUFFER__,
      >,
    >,
  <I as ::grost::__private::convert::State<
    ::grost::__private::convert::Decoded<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        1u32,
      > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
      __GROST_UNKNOWN_BUFFER__,
    >,
  >>::Output: ::core::marker::Sized + ::core::marker::Copy,
  ::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    2u32,
  >: ::grost::__private::reflection::Reflectable<User<I>>,
  <::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    2u32,
  > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection:
    ::grost::__private::flavors::WireFormat<::grost::__private::flavors::Network>,
  String: ::grost::__private::convert::State<
      ::grost::__private::convert::Decoded<
        '__grost_lifetime__,
        ::grost::__private::flavors::Network,
        <::grost::__private::reflection::WireFormatReflection<
          User<I>,
          ::grost::__private::flavors::Network,
          2u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        __GROST_UNKNOWN_BUFFER__,
      >,
    >,
  <String as ::grost::__private::convert::State<
    ::grost::__private::convert::Decoded<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        2u32,
      > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
      __GROST_UNKNOWN_BUFFER__,
    >,
  >>::Output: ::core::marker::Sized + ::core::marker::Copy,
  ::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    3u32,
  >: ::grost::__private::reflection::Reflectable<User<I>>,
  <::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    3u32,
  > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection:
    ::grost::__private::flavors::WireFormat<::grost::__private::flavors::Network>,
  u8: ::grost::__private::convert::State<
      ::grost::__private::convert::Decoded<
        '__grost_lifetime__,
        ::grost::__private::flavors::Network,
        <::grost::__private::reflection::WireFormatReflection<
          User<I>,
          ::grost::__private::flavors::Network,
          3u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        __GROST_UNKNOWN_BUFFER__,
      >,
    >,
  <u8 as ::grost::__private::convert::State<
    ::grost::__private::convert::Decoded<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        3u32,
      > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
      __GROST_UNKNOWN_BUFFER__,
    >,
  >>::Output: ::core::marker::Sized + ::core::marker::Copy,
  Self: ::grost::__private::selection::Selectable<
      ::grost::__private::flavors::Network,
      ::grost::__private::flavors::network::LengthDelimited,
    >,
  __GROST_UNKNOWN_BUFFER__: '__grost_lifetime__,
  I: '__grost_lifetime__,
{
  type Input = &'__grost_lifetime__ [::core::primitive::u8];
  type Output = PartialRefUser<
    '__grost_lifetime__,
    I,
    ::grost::__private::flavors::Network,
    __GROST_UNKNOWN_BUFFER__,
  >;
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl<'__grost_lifetime__, I, __GROST_UNKNOWN_BUFFER__>
  ::grost::__private::convert::State<
    ::grost::__private::convert::Decoded<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      ::grost::__private::flavors::network::LengthDelimited,
      __GROST_UNKNOWN_BUFFER__,
    >,
  > for PartialUser<I, __GROST_UNKNOWN_BUFFER__>
where
  ::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    1u32,
  >: ::grost::__private::reflection::Reflectable<User<I>>,
  <::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    1u32,
  > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection:
    ::grost::__private::flavors::WireFormat<::grost::__private::flavors::Network>,
  I: ::grost::__private::convert::State<
      ::grost::__private::convert::Decoded<
        '__grost_lifetime__,
        ::grost::__private::flavors::Network,
        <::grost::__private::reflection::WireFormatReflection<
          User<I>,
          ::grost::__private::flavors::Network,
          1u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        __GROST_UNKNOWN_BUFFER__,
      >,
    >,
  <I as ::grost::__private::convert::State<
    ::grost::__private::convert::Decoded<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        1u32,
      > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
      __GROST_UNKNOWN_BUFFER__,
    >,
  >>::Output: ::core::marker::Sized + ::core::marker::Copy,
  ::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    2u32,
  >: ::grost::__private::reflection::Reflectable<User<I>>,
  <::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    2u32,
  > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection:
    ::grost::__private::flavors::WireFormat<::grost::__private::flavors::Network>,
  String: ::grost::__private::convert::State<
      ::grost::__private::convert::Decoded<
        '__grost_lifetime__,
        ::grost::__private::flavors::Network,
        <::grost::__private::reflection::WireFormatReflection<
          User<I>,
          ::grost::__private::flavors::Network,
          2u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        __GROST_UNKNOWN_BUFFER__,
      >,
    >,
  <String as ::grost::__private::convert::State<
    ::grost::__private::convert::Decoded<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        2u32,
      > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
      __GROST_UNKNOWN_BUFFER__,
    >,
  >>::Output: ::core::marker::Sized + ::core::marker::Copy,
  ::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    3u32,
  >: ::grost::__private::reflection::Reflectable<User<I>>,
  <::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    3u32,
  > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection:
    ::grost::__private::flavors::WireFormat<::grost::__private::flavors::Network>,
  u8: ::grost::__private::convert::State<
      ::grost::__private::convert::Decoded<
        '__grost_lifetime__,
        ::grost::__private::flavors::Network,
        <::grost::__private::reflection::WireFormatReflection<
          User<I>,
          ::grost::__private::flavors::Network,
          3u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        __GROST_UNKNOWN_BUFFER__,
      >,
    >,
  <u8 as ::grost::__private::convert::State<
    ::grost::__private::convert::Decoded<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        3u32,
      > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
      __GROST_UNKNOWN_BUFFER__,
    >,
  >>::Output: ::core::marker::Sized + ::core::marker::Copy,
  Self: ::grost::__private::selection::Selectable<
      ::grost::__private::flavors::Network,
      ::grost::__private::flavors::network::LengthDelimited,
    >,
  __GROST_UNKNOWN_BUFFER__: '__grost_lifetime__,
  I: '__grost_lifetime__,
  I: ::grost::__private::convert::State<::grost::__private::convert::Flatten>,
  <I as ::grost::__private::convert::State<::grost::__private::convert::Flatten>>::Output:
    ::core::marker::Sized,
{
  type Input = &'__grost_lifetime__ [::core::primitive::u8];
  type Output = PartialRefUser<
    '__grost_lifetime__,
    I,
    ::grost::__private::flavors::Network,
    __GROST_UNKNOWN_BUFFER__,
  >;
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl<'__grost_lifetime__, I, __GROST_UNKNOWN_BUFFER__>
  ::grost::__private::convert::State<
    ::grost::__private::convert::Decoded<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      ::grost::__private::flavors::network::LengthDelimited,
      __GROST_UNKNOWN_BUFFER__,
    >,
  >
  for PartialRefUser<
    '__grost_lifetime__,
    I,
    ::grost::__private::flavors::Network,
    __GROST_UNKNOWN_BUFFER__,
  >
where
  ::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    1u32,
  >: ::grost::__private::reflection::Reflectable<User<I>>,
  <::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    1u32,
  > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection:
    ::grost::__private::flavors::WireFormat<::grost::__private::flavors::Network>,
  I: ::grost::__private::convert::State<
      ::grost::__private::convert::Decoded<
        '__grost_lifetime__,
        ::grost::__private::flavors::Network,
        <::grost::__private::reflection::WireFormatReflection<
          User<I>,
          ::grost::__private::flavors::Network,
          1u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        __GROST_UNKNOWN_BUFFER__,
      >,
    >,
  <I as ::grost::__private::convert::State<
    ::grost::__private::convert::Decoded<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        1u32,
      > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
      __GROST_UNKNOWN_BUFFER__,
    >,
  >>::Output: ::core::marker::Sized + ::core::marker::Copy,
  ::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    2u32,
  >: ::grost::__private::reflection::Reflectable<User<I>>,
  <::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    2u32,
  > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection:
    ::grost::__private::flavors::WireFormat<::grost::__private::flavors::Network>,
  String: ::grost::__private::convert::State<
      ::grost::__private::convert::Decoded<
        '__grost_lifetime__,
        ::grost::__private::flavors::Network,
        <::grost::__private::reflection::WireFormatReflection<
          User<I>,
          ::grost::__private::flavors::Network,
          2u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        __GROST_UNKNOWN_BUFFER__,
      >,
    >,
  <String as ::grost::__private::convert::State<
    ::grost::__private::convert::Decoded<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        2u32,
      > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
      __GROST_UNKNOWN_BUFFER__,
    >,
  >>::Output: ::core::marker::Sized + ::core::marker::Copy,
  ::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    3u32,
  >: ::grost::__private::reflection::Reflectable<User<I>>,
  <::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    3u32,
  > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection:
    ::grost::__private::flavors::WireFormat<::grost::__private::flavors::Network>,
  u8: ::grost::__private::convert::State<
      ::grost::__private::convert::Decoded<
        '__grost_lifetime__,
        ::grost::__private::flavors::Network,
        <::grost::__private::reflection::WireFormatReflection<
          User<I>,
          ::grost::__private::flavors::Network,
          3u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        __GROST_UNKNOWN_BUFFER__,
      >,
    >,
  <u8 as ::grost::__private::convert::State<
    ::grost::__private::convert::Decoded<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        3u32,
      > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
      __GROST_UNKNOWN_BUFFER__,
    >,
  >>::Output: ::core::marker::Sized + ::core::marker::Copy,
  Self: ::grost::__private::selection::Selectable<
      ::grost::__private::flavors::Network,
      ::grost::__private::flavors::network::LengthDelimited,
    >,
  __GROST_UNKNOWN_BUFFER__: '__grost_lifetime__,
  I: '__grost_lifetime__,
{
  type Input = &'__grost_lifetime__ [::core::primitive::u8];
  type Output = Self;
}
#[automatically_derived]
#[allow(non_camel_case_types)]
impl<'__grost_lifetime__, I, __GROST_UNKNOWN_BUFFER__>
  ::grost::__private::Decode<
    '__grost_lifetime__,
    ::grost::__private::flavors::Network,
    ::grost::__private::flavors::network::LengthDelimited,
    Self,
    __GROST_UNKNOWN_BUFFER__,
  >
  for PartialRefUser<
    '__grost_lifetime__,
    I,
    ::grost::__private::flavors::Network,
    __GROST_UNKNOWN_BUFFER__,
  >
where
  ::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    1u32,
  >: ::grost::__private::reflection::Reflectable<User<I>>,
  <::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    1u32,
  > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection:
    ::grost::__private::flavors::WireFormat<::grost::__private::flavors::Network>,
  I: ::grost::__private::convert::State<
      ::grost::__private::convert::Decoded<
        '__grost_lifetime__,
        ::grost::__private::flavors::Network,
        <::grost::__private::reflection::WireFormatReflection<
          User<I>,
          ::grost::__private::flavors::Network,
          1u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        __GROST_UNKNOWN_BUFFER__,
      >,
    >,
  <I as ::grost::__private::convert::State<
    ::grost::__private::convert::Decoded<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        1u32,
      > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
      __GROST_UNKNOWN_BUFFER__,
    >,
  >>::Output: ::core::marker::Sized + ::core::marker::Copy,
  ::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    2u32,
  >: ::grost::__private::reflection::Reflectable<User<I>>,
  <::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    2u32,
  > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection:
    ::grost::__private::flavors::WireFormat<::grost::__private::flavors::Network>,
  String: ::grost::__private::convert::State<
      ::grost::__private::convert::Decoded<
        '__grost_lifetime__,
        ::grost::__private::flavors::Network,
        <::grost::__private::reflection::WireFormatReflection<
          User<I>,
          ::grost::__private::flavors::Network,
          2u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        __GROST_UNKNOWN_BUFFER__,
      >,
    >,
  <String as ::grost::__private::convert::State<
    ::grost::__private::convert::Decoded<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        2u32,
      > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
      __GROST_UNKNOWN_BUFFER__,
    >,
  >>::Output: ::core::marker::Sized + ::core::marker::Copy,
  ::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    3u32,
  >: ::grost::__private::reflection::Reflectable<User<I>>,
  <::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    3u32,
  > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection:
    ::grost::__private::flavors::WireFormat<::grost::__private::flavors::Network>,
  u8: ::grost::__private::convert::State<
      ::grost::__private::convert::Decoded<
        '__grost_lifetime__,
        ::grost::__private::flavors::Network,
        <::grost::__private::reflection::WireFormatReflection<
          User<I>,
          ::grost::__private::flavors::Network,
          3u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        __GROST_UNKNOWN_BUFFER__,
      >,
    >,
  <u8 as ::grost::__private::convert::State<
    ::grost::__private::convert::Decoded<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        3u32,
      > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
      __GROST_UNKNOWN_BUFFER__,
    >,
  >>::Output: ::core::marker::Sized + ::core::marker::Copy,
  Self: ::grost::__private::selection::Selectable<
      ::grost::__private::flavors::Network,
      ::grost::__private::flavors::network::LengthDelimited,
    >,
  __GROST_UNKNOWN_BUFFER__: '__grost_lifetime__,
  I: '__grost_lifetime__,
  I: ::grost::__private::Decode<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        1u32,
      > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
      <I as ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
          '__grost_lifetime__,
          ::grost::__private::flavors::Network,
          <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            ::grost::__private::flavors::Network,
            1u32,
          > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
          __GROST_UNKNOWN_BUFFER__,
        >,
      >>::Output,
      __GROST_UNKNOWN_BUFFER__,
    >,
  String: ::grost::__private::Decode<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        2u32,
      > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
      <String as ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
          '__grost_lifetime__,
          ::grost::__private::flavors::Network,
          <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            ::grost::__private::flavors::Network,
            2u32,
          > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
          __GROST_UNKNOWN_BUFFER__,
        >,
      >>::Output,
      __GROST_UNKNOWN_BUFFER__,
    >,
  u8: ::grost::__private::Decode<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        3u32,
      > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
      <u8 as ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
          '__grost_lifetime__,
          ::grost::__private::flavors::Network,
          <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            ::grost::__private::flavors::Network,
            3u32,
          > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
          __GROST_UNKNOWN_BUFFER__,
        >,
      >>::Output,
      __GROST_UNKNOWN_BUFFER__,
    >,
{
  fn decode<__GROST_BUF__>(
    ctx: &::grost::__private::flavors::network::Context,
    buf: __GROST_BUF__,
  ) -> ::core::result::Result<
    (::core::primitive::usize, Self),
    ::grost::__private::flavors::network::Error,
  >
  where
    Self: ::core::marker::Sized,
    __GROST_BUF__: ::grost::__private::ReadBuf<'__grost_lifetime__>,
    __GROST_UNKNOWN_BUFFER__:
      ::grost::__private::Buffer<::grost::__private::flavors::network::Unknown<__GROST_BUF__>>,
  {
    let bytes = buf.as_bytes();
    let mut this = Self::new();
    let mut offset = 0;
    let buf_len = bytes.len();
    while offset < buf_len {
      let (encoded_identifier_len, identifier) =
        ::grost::__private::flavors::network::Identifier::decode(&bytes[offset..])?;
      offset += encoded_identifier_len;
      match &identifier {
        <::grost::__private::reflection::IdentifierReflection<
          ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            ::grost::__private::flavors::network::Identifier,
            ::grost::__private::flavors::Network,
            1u32,
          >,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION => {
          if this.id.is_some() {
            return ::core::result::Result::Err(
              ::grost::__private::flavors::network::Error::duplicate_field(
                "User", "id", identifier,
              ),
            );
          }
          if offset >= buf_len {
            return ::core::result::Result::Err(
              ::grost::__private::flavors::network::Error::buffer_underflow(),
            );
          }
          let (read, val) = <I as ::grost::__private::Decode<
            '__grost_lifetime__,
            ::grost::__private::flavors::Network,
            <::grost::__private::reflection::WireFormatReflection<
              User<I>,
              ::grost::__private::flavors::Network,
              1u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            <I as ::grost::__private::convert::State<
              ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                <::grost::__private::reflection::WireFormatReflection<
                  User<I>,
                  ::grost::__private::flavors::Network,
                  1u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
              >,
            >>::Output,
            __GROST_UNKNOWN_BUFFER__,
          >>::decode::<__GROST_BUF__>(ctx, buf.slice(offset..))?;
          offset += read;
          this.id = ::core::option::Option::Some(val);
        }
        <::grost::__private::reflection::IdentifierReflection<
          ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            ::grost::__private::flavors::network::Identifier,
            ::grost::__private::flavors::Network,
            2u32,
          >,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION => {
          if this.name.is_some() {
            return ::core::result::Result::Err(
              ::grost::__private::flavors::network::Error::duplicate_field(
                "User", "name", identifier,
              ),
            );
          }
          if offset >= buf_len {
            return ::core::result::Result::Err(
              ::grost::__private::flavors::network::Error::buffer_underflow(),
            );
          }
          let (read, val) = <String as ::grost::__private::Decode<
            '__grost_lifetime__,
            ::grost::__private::flavors::Network,
            <::grost::__private::reflection::WireFormatReflection<
              User<I>,
              ::grost::__private::flavors::Network,
              2u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            <String as ::grost::__private::convert::State<
              ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                <::grost::__private::reflection::WireFormatReflection<
                  User<I>,
                  ::grost::__private::flavors::Network,
                  2u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
              >,
            >>::Output,
            __GROST_UNKNOWN_BUFFER__,
          >>::decode::<__GROST_BUF__>(ctx, buf.slice(offset..))?;
          offset += read;
          this.name = ::core::option::Option::Some(val);
        }
        <::grost::__private::reflection::IdentifierReflection<
          ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            ::grost::__private::flavors::network::Identifier,
            ::grost::__private::flavors::Network,
            3u32,
          >,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION => {
          if this.age.is_some() {
            return ::core::result::Result::Err(
              ::grost::__private::flavors::network::Error::duplicate_field(
                "User", "age", identifier,
              ),
            );
          }
          if offset >= buf_len {
            return ::core::result::Result::Err(
              ::grost::__private::flavors::network::Error::buffer_underflow(),
            );
          }
          let (read, val) = <u8 as ::grost::__private::Decode<
            '__grost_lifetime__,
            ::grost::__private::flavors::Network,
            <::grost::__private::reflection::WireFormatReflection<
              User<I>,
              ::grost::__private::flavors::Network,
              3u32,
            > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
            <u8 as ::grost::__private::convert::State<
              ::grost::__private::convert::Decoded<
                '__grost_lifetime__,
                ::grost::__private::flavors::Network,
                <::grost::__private::reflection::WireFormatReflection<
                  User<I>,
                  ::grost::__private::flavors::Network,
                  3u32,
                > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                __GROST_UNKNOWN_BUFFER__,
              >,
            >>::Output,
            __GROST_UNKNOWN_BUFFER__,
          >>::decode::<__GROST_BUF__>(ctx, buf.slice(offset..))?;
          offset += read;
          this.age = ::core::option::Option::Some(val);
        }
        _ => {
          if ctx.err_on_unknown() {
            return ::core::result::Result::Err(
              ::grost::__private::flavors::network::Error::unknown_identifier("User", identifier),
            );
          }
          if ctx.skip_unknown() {
            if offset >= buf_len {
              return ::core::result::Result::Err(
                ::grost::__private::flavors::network::Error::buffer_underflow(),
              );
            }
            offset +=
              <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::skip(
                ctx,
                identifier.wire_type(),
                buf.slice(offset..),
              )?;
          } else {
            let (read, unknown) = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::decode_unknown(
                            ctx,
                            buf.slice(offset - encoded_identifier_len..),
                        )?;
            offset += read;
            let unknowns_mut = this
              .__grost_unknown_buffer__
              .get_or_insert_with(|| __GROST_UNKNOWN_BUFFER__::new());
            if let ::core::option::Option::Some(unknown) = unknowns_mut.push(unknown) {
              let len = ::grost::__private::Buffer::len(unknowns_mut);
              return ::core::result::Result::Err(
                ::grost::__private::flavors::network::Error::buffer_overflow(
                  len,
                  ::core::num::NonZeroUsize::new(len + 1).unwrap(),
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
#[allow(non_camel_case_types)]
impl<'__grost_lifetime__, I, __GROST_UNKNOWN_BUFFER__>
  ::grost::__private::PartialDecode<
    '__grost_lifetime__,
    ::grost::__private::flavors::Network,
    ::grost::__private::flavors::network::LengthDelimited,
    Self,
    __GROST_UNKNOWN_BUFFER__,
  >
  for PartialRefUser<
    '__grost_lifetime__,
    I,
    ::grost::__private::flavors::Network,
    __GROST_UNKNOWN_BUFFER__,
  >
where
  ::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    1u32,
  >: ::grost::__private::reflection::Reflectable<User<I>>,
  <::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    1u32,
  > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection:
    ::grost::__private::flavors::WireFormat<::grost::__private::flavors::Network>,
  I: ::grost::__private::convert::State<
      ::grost::__private::convert::Decoded<
        '__grost_lifetime__,
        ::grost::__private::flavors::Network,
        <::grost::__private::reflection::WireFormatReflection<
          User<I>,
          ::grost::__private::flavors::Network,
          1u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        __GROST_UNKNOWN_BUFFER__,
      >,
    >,
  <I as ::grost::__private::convert::State<
    ::grost::__private::convert::Decoded<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        1u32,
      > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
      __GROST_UNKNOWN_BUFFER__,
    >,
  >>::Output: ::core::marker::Sized + ::core::marker::Copy,
  ::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    2u32,
  >: ::grost::__private::reflection::Reflectable<User<I>>,
  <::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    2u32,
  > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection:
    ::grost::__private::flavors::WireFormat<::grost::__private::flavors::Network>,
  String: ::grost::__private::convert::State<
      ::grost::__private::convert::Decoded<
        '__grost_lifetime__,
        ::grost::__private::flavors::Network,
        <::grost::__private::reflection::WireFormatReflection<
          User<I>,
          ::grost::__private::flavors::Network,
          2u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        __GROST_UNKNOWN_BUFFER__,
      >,
    >,
  <String as ::grost::__private::convert::State<
    ::grost::__private::convert::Decoded<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        2u32,
      > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
      __GROST_UNKNOWN_BUFFER__,
    >,
  >>::Output: ::core::marker::Sized + ::core::marker::Copy,
  ::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    3u32,
  >: ::grost::__private::reflection::Reflectable<User<I>>,
  <::grost::__private::reflection::WireFormatReflection<
    User<I>,
    ::grost::__private::flavors::Network,
    3u32,
  > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection:
    ::grost::__private::flavors::WireFormat<::grost::__private::flavors::Network>,
  u8: ::grost::__private::convert::State<
      ::grost::__private::convert::Decoded<
        '__grost_lifetime__,
        ::grost::__private::flavors::Network,
        <::grost::__private::reflection::WireFormatReflection<
          User<I>,
          ::grost::__private::flavors::Network,
          3u32,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
        __GROST_UNKNOWN_BUFFER__,
      >,
    >,
  <u8 as ::grost::__private::convert::State<
    ::grost::__private::convert::Decoded<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        3u32,
      > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
      __GROST_UNKNOWN_BUFFER__,
    >,
  >>::Output: ::core::marker::Sized + ::core::marker::Copy,
  Self: ::grost::__private::selection::Selectable<
      ::grost::__private::flavors::Network,
      ::grost::__private::flavors::network::LengthDelimited,
    >,
  __GROST_UNKNOWN_BUFFER__: '__grost_lifetime__,
  I: '__grost_lifetime__,
  I: ::grost::__private::PartialDecode<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        1u32,
      > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
      <I as ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
          '__grost_lifetime__,
          ::grost::__private::flavors::Network,
          <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            ::grost::__private::flavors::Network,
            1u32,
          > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
          __GROST_UNKNOWN_BUFFER__,
        >,
      >>::Output,
      __GROST_UNKNOWN_BUFFER__,
    >,
  String: ::grost::__private::PartialDecode<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        2u32,
      > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
      <String as ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
          '__grost_lifetime__,
          ::grost::__private::flavors::Network,
          <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            ::grost::__private::flavors::Network,
            2u32,
          > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
          __GROST_UNKNOWN_BUFFER__,
        >,
      >>::Output,
      __GROST_UNKNOWN_BUFFER__,
    >,
  u8: ::grost::__private::PartialDecode<
      '__grost_lifetime__,
      ::grost::__private::flavors::Network,
      <::grost::__private::reflection::WireFormatReflection<
        User<I>,
        ::grost::__private::flavors::Network,
        3u32,
      > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
      <u8 as ::grost::__private::convert::State<
        ::grost::__private::convert::Decoded<
          '__grost_lifetime__,
          ::grost::__private::flavors::Network,
          <::grost::__private::reflection::WireFormatReflection<
            User<I>,
            ::grost::__private::flavors::Network,
            3u32,
          > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
          __GROST_UNKNOWN_BUFFER__,
        >,
      >>::Output,
      __GROST_UNKNOWN_BUFFER__,
    >,
  Self: ::grost::__private::selection::Selectable<
      ::grost::__private::flavors::Network,
      ::grost::__private::flavors::network::LengthDelimited,
      Selector = UserSelector<I, ::grost::__private::flavors::Network>,
    >,
{
  fn partial_decode<__GROST_BUF__>(
    ctx: &::grost::__private::flavors::network::Context,
    buf: __GROST_BUF__,
    selector: &Self::Selector,
  ) -> ::core::result::Result<
    (::core::primitive::usize, ::core::option::Option<Self>),
    ::grost::__private::flavors::network::Error,
  >
  where
    Self: ::core::marker::Sized,
    __GROST_BUF__: ::grost::__private::ReadBuf<'__grost_lifetime__>,
    __GROST_UNKNOWN_BUFFER__:
      ::grost::__private::Buffer<::grost::__private::flavors::network::Unknown<__GROST_BUF__>>,
  {
    let bytes = buf.as_bytes();
    let mut this = Self::new();
    let mut offset = 0;
    let buf_len = bytes.len();
    while offset < buf_len {
      let (encoded_identifier_len, identifier) =
        ::grost::__private::flavors::network::Identifier::decode(&bytes[offset..])?;
      offset += encoded_identifier_len;
      match &identifier {
        <::grost::__private::reflection::IdentifierReflection<
          ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            ::grost::__private::flavors::network::Identifier,
            ::grost::__private::flavors::Network,
            1u32,
          >,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION => {
          if offset >= buf_len {
            return ::core::result::Result::Err(
              ::grost::__private::flavors::network::Error::buffer_underflow(),
            );
          }
          if selector.is_id_selected() {
            let (read, val) = <I as ::grost::__private::PartialDecode<
              '_,
              ::grost::__private::flavors::Network,
              <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                ::grost::__private::flavors::Network,
                1u32,
              > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
              <I as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                  '__grost_lifetime__,
                  ::grost::__private::flavors::Network,
                  <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    ::grost::__private::flavors::Network,
                    1u32,
                  > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                  __GROST_UNKNOWN_BUFFER__,
                >,
              >>::Output,
              __GROST_UNKNOWN_BUFFER__,
            >>::partial_decode::<__GROST_BUF__>(
              ctx, buf.slice(offset..), selector.id_ref()
            )?;
            offset += read;
            this.id = val;
          } else {
            offset +=
              <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::skip(
                ctx,
                identifier.wire_type(),
                buf.slice(offset..),
              )?;
          }
        }
        <::grost::__private::reflection::IdentifierReflection<
          ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            ::grost::__private::flavors::network::Identifier,
            ::grost::__private::flavors::Network,
            2u32,
          >,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION => {
          if offset >= buf_len {
            return ::core::result::Result::Err(
              ::grost::__private::flavors::network::Error::buffer_underflow(),
            );
          }
          if selector.is_name_selected() {
            let (read, val) = <String as ::grost::__private::PartialDecode<
              '_,
              ::grost::__private::flavors::Network,
              <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                ::grost::__private::flavors::Network,
                2u32,
              > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
              <String as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                  '__grost_lifetime__,
                  ::grost::__private::flavors::Network,
                  <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    ::grost::__private::flavors::Network,
                    2u32,
                  > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                  __GROST_UNKNOWN_BUFFER__,
                >,
              >>::Output,
              __GROST_UNKNOWN_BUFFER__,
            >>::partial_decode::<__GROST_BUF__>(
              ctx, buf.slice(offset..), selector.name_ref()
            )?;
            offset += read;
            this.name = val;
          } else {
            offset +=
              <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::skip(
                ctx,
                identifier.wire_type(),
                buf.slice(offset..),
              )?;
          }
        }
        <::grost::__private::reflection::IdentifierReflection<
          ::grost::__private::reflection::ObjectFieldReflection<
            User<I>,
            ::grost::__private::flavors::network::Identifier,
            ::grost::__private::flavors::Network,
            3u32,
          >,
        > as ::grost::__private::reflection::Reflectable<User<I>>>::REFLECTION => {
          if offset >= buf_len {
            return ::core::result::Result::Err(
              ::grost::__private::flavors::network::Error::buffer_underflow(),
            );
          }
          if selector.is_age_selected() {
            let (read, val) = <u8 as ::grost::__private::PartialDecode<
              '_,
              ::grost::__private::flavors::Network,
              <::grost::__private::reflection::WireFormatReflection<
                User<I>,
                ::grost::__private::flavors::Network,
                3u32,
              > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
              <u8 as ::grost::__private::convert::State<
                ::grost::__private::convert::Decoded<
                  '__grost_lifetime__,
                  ::grost::__private::flavors::Network,
                  <::grost::__private::reflection::WireFormatReflection<
                    User<I>,
                    ::grost::__private::flavors::Network,
                    3u32,
                  > as ::grost::__private::reflection::Reflectable<User<I>>>::Reflection,
                  __GROST_UNKNOWN_BUFFER__,
                >,
              >>::Output,
              __GROST_UNKNOWN_BUFFER__,
            >>::partial_decode::<__GROST_BUF__>(
              ctx, buf.slice(offset..), selector.age_ref()
            )?;
            offset += read;
            this.age = val;
          } else {
            offset +=
              <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::skip(
                ctx,
                identifier.wire_type(),
                buf.slice(offset..),
              )?;
          }
        }
        _ => {
          if ctx.err_on_unknown() {
            return ::core::result::Result::Err(
              ::grost::__private::flavors::network::Error::unknown_identifier("User", identifier),
            );
          }
          if ctx.skip_unknown() {
            if offset >= buf_len {
              return ::core::result::Result::Err(
                ::grost::__private::flavors::network::Error::buffer_underflow(),
              );
            }
            offset +=
              <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::skip(
                ctx,
                identifier.wire_type(),
                buf.slice(offset..),
              )?;
          } else {
            let (read, unknown) = <::grost::__private::flavors::Network as ::grost::__private::flavors::Flavor>::decode_unknown(
                            ctx,
                            buf.slice(offset - encoded_identifier_len..),
                        )?;
            offset += read;
            let unknowns_mut = this
              .__grost_unknown_buffer__
              .get_or_insert_with(|| __GROST_UNKNOWN_BUFFER__::new());
            if let ::core::option::Option::Some(unknown) = unknowns_mut.push(unknown) {
              let len = ::grost::__private::Buffer::len(unknowns_mut);
              return ::core::result::Result::Err(
                ::grost::__private::flavors::network::Error::buffer_overflow(
                  len,
                  ::core::num::NonZeroUsize::new(len + 1).unwrap(),
                ),
              );
            }
          }
        }
      }
    }
    if this.is_empty() {
      return ::core::result::Result::Ok((offset, ::core::option::Option::None));
    }
    ::core::result::Result::Ok((offset, ::core::option::Option::Some(this)))
  }
}

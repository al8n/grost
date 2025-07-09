mod generic {
  use core::marker::PhantomData;
  use grost::{Object, flavors::groto::LengthDelimited};
  struct GenericWithMarkerVec<I, M> {
    #[grost(tag = 1, list(generic(marker = "M")))]
    id: Vec<I>,
    #[grost(skip)]
    _m: PhantomData<M>,
  }
  /// Field indexer for the struct [`GenericWithMarkerVec`]
  #[repr(u32)]
  enum GenericWithMarkerVecIndex {
    /// The field indexer for the field `id`
    Id = 1u32,
  }
  #[automatically_derived]
  impl ::core::clone::Clone for GenericWithMarkerVecIndex {
    #[inline]
    fn clone(&self) -> GenericWithMarkerVecIndex {
      *self
    }
  }
  #[automatically_derived]
  impl ::core::marker::Copy for GenericWithMarkerVecIndex {}
  #[automatically_derived]
  impl ::core::marker::StructuralPartialEq for GenericWithMarkerVecIndex {}
  #[automatically_derived]
  impl ::core::cmp::PartialEq for GenericWithMarkerVecIndex {
    #[inline]
    fn eq(&self, other: &GenericWithMarkerVecIndex) -> bool {
      true
    }
  }
  #[automatically_derived]
  impl ::core::cmp::Eq for GenericWithMarkerVecIndex {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
  }
  #[automatically_derived]
  impl ::core::cmp::PartialOrd for GenericWithMarkerVecIndex {
    #[inline]
    fn partial_cmp(
      &self,
      other: &GenericWithMarkerVecIndex,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
      ::core::option::Option::Some(::core::cmp::Ordering::Equal)
    }
  }
  #[automatically_derived]
  impl ::core::cmp::Ord for GenericWithMarkerVecIndex {
    #[inline]
    fn cmp(&self, other: &GenericWithMarkerVecIndex) -> ::core::cmp::Ordering {
      ::core::cmp::Ordering::Equal
    }
  }
  #[automatically_derived]
  impl ::core::hash::Hash for GenericWithMarkerVecIndex {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
  }
  #[automatically_derived]
  impl ::core::fmt::Debug for GenericWithMarkerVecIndex {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      ::core::fmt::Formatter::write_str(f, "Id")
    }
  }
  /// The selection type for [`GenericWithMarkerVec`]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  struct GenericWithMarkerVecSelector<I, M>
    where
        ::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
                <Vec<
                    I,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flattened<
                        ::grost::__private::convert::Inner,
                    >,
                >>::Output,
                M,
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
        _m: ::core::marker::PhantomData<PhantomData<M>>,
    }
  /// An iterator over the selected fields of the [`GenericWithMarkerVecSelector`]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  struct GenericWithMarkerVecSelectorIter<
    '__grost_lifetime__,
    I,
    M,
    const __GROST_SELECTED__: ::core::primitive::bool = true,
  >
  where
    ::grost::__private::marker::ListMarker<
      Vec<I>,
      ::grost::__private::marker::GenericMarker<
        <Vec<I> as ::grost::__private::convert::State<
          ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
        >>::Output,
        M,
      >,
    >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
    Vec<I>: ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>,
  {
    selector: &'__grost_lifetime__ GenericWithMarkerVecSelector<I, M>,
    index: ::core::option::Option<GenericWithMarkerVecIndex>,
    num: ::core::primitive::usize,
    yielded: ::core::primitive::usize,
  }
  /// Partial struct for the [`PartialGenericWithMarkerVec`]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  struct PartialGenericWithMarkerVec<I, M>
  where
    Vec<I>: ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >,
    <Vec<I> as ::grost::__private::convert::State<
      ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
    >>::Output: ::core::marker::Sized,
  {
    id: ::core::option::Option<
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output,
    >,
    _m: ::core::marker::PhantomData<PhantomData<M>>,
  }
  /// Partial reference struct for the struct [`GenericWithMarkerVec`]
  #[allow(non_camel_case_types, clippy::type_complexity)]
  struct PartialGenericWithMarkerVecRef<
        '__grost_lifetime__,
        I,
        M,
        __GROST_READ_BUFFER__,
        __GROST_BUFFER__,
    >
    where
        ::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
                I,
                M,
            >,
        >: ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
        >,
        M: ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Groto,
            >
            + ::grost::__private::marker::Marker<
                Marked = I,
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
            ::grost::__private::marker::GenericMarker<
                I,
                M,
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
            ::grost::__private::marker::GenericMarker<
                I,
                M,
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
            ::grost::__private::marker::GenericMarker<
                <Vec<
                    I,
                > as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flattened<
                        ::grost::__private::convert::Inner,
                    >,
                >>::Output,
                M,
            >,
        > as ::grost::__private::flavors::DefaultWireFormat<
                        ::grost::__private::flavors::Groto,
                    >>::Format,
                    ::grost::__private::flavors::Groto,
                >,
            >>::Output,
        >,
        _m: ::core::marker::PhantomData<PhantomData<M>>,
    }
  const _: () = {
    impl<I, M> GenericWithMarkerVec<I, M> {
      /// Returns a reference to the `id`
      #[inline]
      const fn id_ref(&self) -> &Vec<I> {
        &self.id
      }
      /// Returns a mutable reference to the `id`
      #[inline]
      const fn id_mut(&mut self) -> &mut Vec<I> {
        &mut self.id
      }
      /// Set the `id` to the given value
      #[inline]
      fn set_id(&mut self, value: Vec<I>) -> &mut Self {
        self.id = value;
        self
      }
      /// Set the `id` to the given value
      #[inline]
      fn with_id(mut self, value: Vec<I>) -> Self {
        self.id = value;
        self
      }
    }
    impl<I, M>
      ::grost::__private::flavors::DefaultObjectWireFormat<::grost::__private::flavors::Groto>
      for GenericWithMarkerVec<I, M>
    {
      type Format = ::grost::__private::flavors::groto::LengthDelimited;
    }
    impl<I, M>
      ::grost::__private::flavors::DefaultObjectWireFormat<::grost::__private::flavors::Groto>
      for PartialGenericWithMarkerVec<I, M>
    where
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output: ::core::marker::Sized,
    {
      type Format = ::grost::__private::flavors::groto::LengthDelimited;
    }
    impl<'__grost_lifetime__, I, M, __GROST_READ_BUFFER__, __GROST_BUFFER__>
      ::grost::__private::flavors::DefaultObjectWireFormat<::grost::__private::flavors::Groto>
      for PartialGenericWithMarkerVecRef<
        '__grost_lifetime__,
        I,
        M,
        __GROST_READ_BUFFER__,
        __GROST_BUFFER__,
      >
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          <::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
          > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output: ::core::marker::Sized,
    {
      type Format = ::grost::__private::flavors::groto::LengthDelimited;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I, M, __GROST_FLATTEN_STATE__: ?::core::marker::Sized>
      ::grost::__private::convert::State<
        ::grost::__private::convert::Flattened<__GROST_FLATTEN_STATE__>,
      > for GenericWithMarkerVec<I, M>
    {
      type Output = Self;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I, M>
      ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      > for GenericWithMarkerVec<I, M>
    where
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output: ::core::marker::Sized,
    {
      type Output = PartialGenericWithMarkerVec<I, M>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I, M>
      ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      > for PartialGenericWithMarkerVec<I, M>
    where
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output: ::core::marker::Sized,
    {
      type Output = Self;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<'__grost_lifetime__, I, M, __GROST_READ_BUFFER__, __GROST_BUFFER__>
      ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >
      for PartialGenericWithMarkerVecRef<
        '__grost_lifetime__,
        I,
        M,
        __GROST_READ_BUFFER__,
        __GROST_BUFFER__,
      >
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          <::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
          > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output: ::core::marker::Sized,
    {
      type Output = Self;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<'__grost_lifetime__, I, M, __GROST_READ_BUFFER__, __GROST_BUFFER__>
      ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          ::grost::__private::flavors::Groto,
          ::grost::__private::flavors::groto::LengthDelimited,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
        >,
      > for GenericWithMarkerVec<I, M>
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          <::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
          > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output: ::core::marker::Sized,
    {
      type Output = PartialGenericWithMarkerVecRef<
        '__grost_lifetime__,
        I,
        M,
        __GROST_READ_BUFFER__,
        __GROST_BUFFER__,
      >;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<'__grost_lifetime__, I, M, __GROST_READ_BUFFER__, __GROST_BUFFER__>
      ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          ::grost::__private::flavors::Groto,
          ::grost::__private::flavors::groto::LengthDelimited,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
        >,
      > for PartialGenericWithMarkerVec<I, M>
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          <::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
          > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output: ::core::marker::Sized,
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output: ::core::marker::Sized,
    {
      type Output = PartialGenericWithMarkerVecRef<
        '__grost_lifetime__,
        I,
        M,
        __GROST_READ_BUFFER__,
        __GROST_BUFFER__,
      >;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<'__grost_lifetime__, I, M, __GROST_READ_BUFFER__, __GROST_BUFFER__>
      ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          ::grost::__private::flavors::Groto,
          ::grost::__private::flavors::groto::LengthDelimited,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
        >,
      >
      for PartialGenericWithMarkerVecRef<
        '__grost_lifetime__,
        I,
        M,
        __GROST_READ_BUFFER__,
        __GROST_BUFFER__,
      >
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          <::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
          > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output: ::core::marker::Sized,
    {
      type Output = Self;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<I, M> ::core::default::Default for PartialGenericWithMarkerVec<I, M>
    where
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output: ::core::marker::Sized,
    {
      fn default() -> Self {
        Self::new()
      }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I, M, __GROST_FLATTEN_STATE__: ?::core::marker::Sized>
      ::grost::__private::convert::State<
        ::grost::__private::convert::Flattened<__GROST_FLATTEN_STATE__>,
      > for PartialGenericWithMarkerVec<I, M>
    where
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output: ::core::marker::Sized,
    {
      type Output = Self;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I, M> PartialGenericWithMarkerVec<I, M>
    where
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output: ::core::marker::Sized,
    {
      /// Creates an empty partial struct.
      #[inline]
      pub const fn new() -> Self {
        Self {
          id: ::core::option::Option::None,
          _m: ::core::marker::PhantomData,
        }
      }
      /// Returns `true` if the partial object is empty.
      #[inline]
      pub const fn is_empty(&self) -> bool {
        self.id.is_none()
      }
      /// Returns a reference to the `id`
      #[inline]
      const fn id_ref(
        &self,
      ) -> ::core::option::Option<
        &<Vec<I> as ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >>::Output,
      > {
        self.id.as_ref()
      }
      /// Returns a mutable reference to the `id`
      #[inline]
      const fn id_mut(
        &mut self,
      ) -> ::core::option::Option<
        &mut <Vec<I> as ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
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
      const fn unwrap_id_ref(
        &self,
      ) -> &<Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output {
        match self.id.as_ref() {
          ::core::option::Option::Some(value) => value,
          ::core::option::Option::None => {
            ::core::panicking::panic_fmt(format_args!("`id` is `None`"));
          }
        }
      }
      /// Returns a mutable reference to the `id` if it is not `None`
      ///
      /// ## Panics
      ///
      /// - Panics if the `id` is `None`
      #[inline]
      const fn unwrap_id_mut(
        &mut self,
      ) -> &mut <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output {
        match self.id.as_mut() {
          ::core::option::Option::Some(value) => value,
          ::core::option::Option::None => {
            ::core::panicking::panic_fmt(format_args!("`id` is `None`"));
          }
        }
      }
      /// Takes the value of `id` out if it is not `None`
      #[inline]
      const fn take_id(
        &mut self,
      ) -> ::core::option::Option<
        <Vec<I> as ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >>::Output,
      > {
        self.id.take()
      }
      /// Clear the value of `id`
      #[inline]
      fn clear_id(&mut self) -> &mut Self {
        self.id = ::core::option::Option::None;
        self
      }
      /// Set the `id` to the given value
      #[inline]
      fn set_id(
        &mut self,
        value: <Vec<I> as ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >>::Output,
      ) -> &mut Self {
        self.id = ::core::option::Option::Some(value);
        self
      }
      /// Update the `id` to the given value or clear the `id`
      #[inline]
      fn update_id(
        &mut self,
        value: ::core::option::Option<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
          >>::Output,
        >,
      ) -> &mut Self {
        self.id = value;
        self
      }
      /// Set the `id` to the given value
      #[inline]
      fn with_id(
        mut self,
        value: <Vec<I> as ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >>::Output,
      ) -> Self {
        self.id = ::core::option::Option::Some(value);
        self
      }
      /// Clear the value of `id`
      #[inline]
      fn without_id(mut self) -> Self {
        self.id = ::core::option::Option::None;
        self
      }
      /// Update the `id` to the given value or clear the `id`
      #[inline]
      fn maybe_id(
        mut self,
        value: ::core::option::Option<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
          >>::Output,
        >,
      ) -> Self {
        self.id = value;
        self
      }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<'__grost_lifetime__, I, M, __GROST_READ_BUFFER__, __GROST_BUFFER__>
      ::core::default::Default
      for PartialGenericWithMarkerVecRef<
        '__grost_lifetime__,
        I,
        M,
        __GROST_READ_BUFFER__,
        __GROST_BUFFER__,
      >
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          <::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
          > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output: ::core::marker::Sized,
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
      M,
      __GROST_READ_BUFFER__,
      __GROST_BUFFER__,
      __GROST_FLATTEN_STATE__: ?::core::marker::Sized,
    >
      ::grost::__private::convert::State<
        ::grost::__private::convert::Flattened<__GROST_FLATTEN_STATE__>,
      >
      for PartialGenericWithMarkerVecRef<
        '__grost_lifetime__,
        I,
        M,
        __GROST_READ_BUFFER__,
        __GROST_BUFFER__,
      >
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          <::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
          > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output: ::core::marker::Sized,
    {
      type Output = Self;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<'__grost_lifetime__, I, M, __GROST_READ_BUFFER__, __GROST_BUFFER__>
      PartialGenericWithMarkerVecRef<
        '__grost_lifetime__,
        I,
        M,
        __GROST_READ_BUFFER__,
        __GROST_BUFFER__,
      >
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          <::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
          > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output: ::core::marker::Sized,
    {
      /// Creates an empty partial struct.
      #[inline]
      pub const fn new() -> Self {
        Self {
          id: ::core::option::Option::None,
          _m: ::core::marker::PhantomData,
          __grost_buffer__: ::core::option::Option::None,
          __grost_read_buffer__: ::core::option::Option::None,
        }
      }
      /// Returns `true` if the partial struct is empty, which means all fields are `None`.
      #[inline]
      pub const fn is_empty(&self) -> bool {
        self.__grost_buffer__.is_none() && self.id.is_none()
      }
      /// Returns the original encoded type of the partial decoded object.
      #[inline]
      pub const fn raw(&self) -> ::core::option::Option<&__GROST_READ_BUFFER__> {
        self.__grost_read_buffer__.as_ref()
      }
      /// Returns a reference to the unknown buffer, which holds the unknown data when decoding.
      #[inline]
      pub const fn buffer(&self) -> ::core::option::Option<&__GROST_BUFFER__> {
        self.__grost_buffer__.as_ref()
      }
      /// Returns a mutable reference to the unknown buffer, which holds the unknown data when decoding.
      #[inline]
      pub const fn buffer_mut(&mut self) -> ::core::option::Option<&mut __GROST_BUFFER__> {
        self.__grost_buffer__.as_mut()
      }
      /// Takes the unknown buffer out if the unknown buffer is not `None`.
      #[inline]
      pub const fn take_buffer(&mut self) -> ::core::option::Option<__GROST_BUFFER__> {
        self.__grost_buffer__.take()
      }
      /// Set the value of unknown buffer
      #[inline]
      pub fn set_buffer(&mut self, buffer: __GROST_BUFFER__) -> &mut Self {
        self.__grost_buffer__ = ::core::option::Option::Some(buffer);
        self
      }
      /// Clears the unknown buffer.
      #[inline]
      pub fn clear_buffer(&mut self) -> &mut Self {
        self.__grost_buffer__ = ::core::option::Option::None;
        self
      }
      /// Set the value of unknown buffer
      #[inline]
      pub fn with_buffer(mut self, buffer: __GROST_BUFFER__) -> Self {
        self.__grost_buffer__ = ::core::option::Option::Some(buffer);
        self
      }
      /// Clears the unknown buffer.
      #[inline]
      pub fn without_buffer(mut self) -> Self {
        self.__grost_buffer__ = ::core::option::Option::None;
        self
      }
      /// Returns a reference to the `id`
      #[inline]
      const fn id_ref(
        &self,
      ) -> ::core::option::Option<
        &<Vec<I> as ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >>::Output,
      > {
        self.id.as_ref()
      }
      /// Returns a mutable reference to the `id`
      #[inline]
      const fn id_mut(
        &mut self,
      ) -> ::core::option::Option<
        &mut <Vec<I> as ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
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
      const fn unwrap_id_ref(
        &self,
      ) -> &<Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          <::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
          > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output {
        match self.id.as_ref() {
          ::core::option::Option::Some(value) => value,
          ::core::option::Option::None => {
            ::core::panicking::panic_fmt(format_args!("`id` is `None`"));
          }
        }
      }
      /// Returns a mutable reference to the `id` if it is not `None`
      ///
      /// ## Panics
      ///
      /// - Panics if the `id` is `None`
      #[inline]
      const fn unwrap_id_mut(
        &mut self,
      ) -> &mut <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          <::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
          > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output {
        match self.id.as_mut() {
          ::core::option::Option::Some(value) => value,
          ::core::option::Option::None => {
            ::core::panicking::panic_fmt(format_args!("`id` is `None`"));
          }
        }
      }
      /// Takes the value of `id` out if it is not `None`
      #[inline]
      const fn take_id(
        &mut self,
      ) -> ::core::option::Option<
        <Vec<I> as ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >>::Output,
      > {
        self.id.take()
      }
      /// Clear the value of `id`
      #[inline]
      fn clear_id(&mut self) -> &mut Self {
        self.id = ::core::option::Option::None;
        self
      }
      /// Set the `id` to the given value
      #[inline]
      fn set_id(
        &mut self,
        value: <Vec<I> as ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >>::Output,
      ) -> &mut Self {
        self.id = ::core::option::Option::Some(value);
        self
      }
      /// Update the `id` to the given value or clear the `id`
      #[inline]
      fn update_id(
        &mut self,
        value: ::core::option::Option<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::PartialRef<
              '__grost_lifetime__,
              __GROST_READ_BUFFER__,
              __GROST_BUFFER__,
              <::grost::__private::marker::ListMarker<
                Vec<I>,
                ::grost::__private::marker::GenericMarker<
                  <Vec<I> as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                  >>::Output,
                  M,
                >,
              > as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Groto,
              >>::Format,
              ::grost::__private::flavors::Groto,
            >,
          >>::Output,
        >,
      ) -> &mut Self {
        self.id = value;
        self
      }
      /// Set the `id` to the given value
      #[inline]
      fn with_id(
        mut self,
        value: <Vec<I> as ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >>::Output,
      ) -> Self {
        self.id = ::core::option::Option::Some(value);
        self
      }
      /// Clear the value of `id`
      #[inline]
      fn without_id(mut self) -> Self {
        self.id = ::core::option::Option::None;
        self
      }
      /// Update the `id` to the given value or clear the `id`
      #[inline]
      fn maybe_id(
        mut self,
        value: ::core::option::Option<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::PartialRef<
              '__grost_lifetime__,
              __GROST_READ_BUFFER__,
              __GROST_BUFFER__,
              <::grost::__private::marker::ListMarker<
                Vec<I>,
                ::grost::__private::marker::GenericMarker<
                  <Vec<I> as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                  >>::Output,
                  M,
                >,
              > as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Groto,
              >>::Format,
              ::grost::__private::flavors::Groto,
            >,
          >>::Output,
        >,
      ) -> Self {
        self.id = value;
        self
      }
    }
    #[automatically_derived]
    #[allow(clippy::type_complexity, non_camel_case_types)]
    impl<I, M> ::grost::__private::reflection::Reflectable<GenericWithMarkerVec<I, M>>
      for ::grost::__private::reflection::ObjectFieldReflection<
        GenericWithMarkerVec<I, M>,
        ::grost::__private::reflection::ObjectField,
        ::grost::__private::flavors::Groto,
        1u32,
      >
    where
      ::grost::__private::reflection::SchemaTypeReflection<Vec<I>>:
        ::grost::__private::reflection::Reflectable<
            Vec<I>,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
    {
      type Reflection = ::grost::__private::reflection::ObjectField;
      const REFLECTION: &'static Self::Reflection = &{
        ::grost::__private::reflection::ObjectFieldBuilder {
                    name: "id",
                    description: "",
                    ty: <::grost::__private::reflection::SchemaTypeReflection<
                        Vec<I>,
                    > as ::grost::__private::reflection::Reflectable<Vec<I>>>::REFLECTION,
                }
                    .build()
      };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I, M> ::grost::__private::reflection::Reflectable<GenericWithMarkerVec<I, M>>
      for ::grost::__private::reflection::WireFormatReflection<
        GenericWithMarkerVec<I, M>,
        ::grost::__private::flavors::Groto,
        1u32,
      >
    where
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
    {
      type Reflection = <::grost::__private::marker::ListMarker<
                Vec<I>,
                ::grost::__private::marker::GenericMarker<
                    <Vec<
                        I,
                    > as ::grost::__private::convert::State<
                        ::grost::__private::convert::Flattened<
                            ::grost::__private::convert::Inner,
                        >,
                    >>::Output,
                    M,
                >,
            > as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Groto,
            >>::Format;
      const REFLECTION: &'static Self::Reflection = <::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      > as ::grost::__private::flavors::DefaultWireFormat<
        ::grost::__private::flavors::Groto,
      >>::STATIC_REF;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I, M> ::grost::__private::reflection::Reflectable<GenericWithMarkerVec<I, M>>
      for ::grost::__private::reflection::IdentifierReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
          GenericWithMarkerVec<I, M>,
          <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
          ::grost::__private::flavors::Groto,
          1u32,
        >,
      >
    where
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
    {
      type Reflection =
        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier;
      const REFLECTION: &Self::Reflection = &{
        (::grost::__private::flavors::groto::Identifier::new)(
          <<::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
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
    impl<I, M> ::grost::__private::reflection::Reflectable<GenericWithMarkerVec<I, M>>
      for ::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::IdentifierReflection<
          ::grost::__private::reflection::ObjectFieldReflection<
            GenericWithMarkerVec<I, M>,
            <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
            ::grost::__private::flavors::Groto,
            1u32,
          >,
        >,
      >
    where
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
    {
      type Reflection = [::core::primitive::u8];
      const REFLECTION: &Self::Reflection = {
        (::grost::__private::flavors::groto::Identifier::encode)(
                        <::grost::__private::reflection::IdentifierReflection<
                            ::grost::__private::reflection::ObjectFieldReflection<
                                GenericWithMarkerVec<I, M>,
                                <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
                                ::grost::__private::flavors::Groto,
                                1u32,
                            >,
                        > as ::grost::__private::reflection::Reflectable<
                            GenericWithMarkerVec<I, M>,
                        >>::REFLECTION,
                    )
                    .as_slice()
      };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
            I,
            M,
        > ::grost::__private::reflection::Reflectable<GenericWithMarkerVec<I, M>>
        for ::grost::__private::reflection::Len<
            ::grost::__private::reflection::EncodeReflection<
                ::grost::__private::reflection::IdentifierReflection<
                    ::grost::__private::reflection::ObjectFieldReflection<
                        GenericWithMarkerVec<I, M>,
                        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
                        ::grost::__private::flavors::Groto,
                        1u32,
                    >,
                >,
            >,
        >
        where
            M: ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Groto,
                >
                + ::grost::__private::marker::Marker<
                    Marked = <Vec<
                        I,
                    > as ::grost::__private::convert::State<
                        ::grost::__private::convert::Flattened<
                            ::grost::__private::convert::Inner,
                        >,
                    >>::Output,
                >,
        {
            type Reflection = ::core::primitive::usize;
            const REFLECTION: &Self::Reflection = {
                &<::grost::__private::reflection::EncodeReflection<
                    ::grost::__private::reflection::IdentifierReflection<
                        ::grost::__private::reflection::ObjectFieldReflection<
                            GenericWithMarkerVec<I, M>,
                            <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
                            ::grost::__private::flavors::Groto,
                            1u32,
                        >,
                    >,
                > as ::grost::__private::reflection::Reflectable<
                    GenericWithMarkerVec<I, M>,
                >>::REFLECTION
                    .len()
            };
        }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I, M> ::grost::__private::reflection::Reflectable<GenericWithMarkerVec<I, M>>
      for ::grost::__private::reflection::TagReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
          GenericWithMarkerVec<I, M>,
          <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag,
          ::grost::__private::flavors::Groto,
          1u32,
        >,
      >
    {
      type Reflection =
        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag;
      const REFLECTION: &Self::Reflection =
        &{ (::grost::__private::flavors::groto::Tag::new)(1u32) };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I, M> ::grost::__private::reflection::Reflectable<GenericWithMarkerVec<I, M>>
      for ::grost::__private::reflection::EncodeReflection<
        ::grost::__private::reflection::TagReflection<
          ::grost::__private::reflection::ObjectFieldReflection<
            GenericWithMarkerVec<I, M>,
            <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag,
            ::grost::__private::flavors::Groto,
            1u32,
          >,
        >,
      >
    {
      type Reflection = [::core::primitive::u8];
      const REFLECTION: &Self::Reflection = {
        (::grost::__private::flavors::groto::Tag::encode)(
          <::grost::__private::reflection::TagReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
              GenericWithMarkerVec<I, M>,
              <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag,
              ::grost::__private::flavors::Groto,
              1u32,
            >,
          > as ::grost::__private::reflection::Reflectable<GenericWithMarkerVec<I, M>>>::REFLECTION,
        )
        .as_slice()
      };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I, M> ::grost::__private::reflection::Reflectable<GenericWithMarkerVec<I, M>>
      for ::grost::__private::reflection::Len<
        ::grost::__private::reflection::EncodeReflection<
          ::grost::__private::reflection::TagReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
              GenericWithMarkerVec<I, M>,
              <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag,
              ::grost::__private::flavors::Groto,
              1u32,
            >,
          >,
        >,
      >
    {
      type Reflection = ::core::primitive::usize;
      const REFLECTION: &Self::Reflection = {
        &<::grost::__private::reflection::EncodeReflection<
          ::grost::__private::reflection::TagReflection<
            ::grost::__private::reflection::ObjectFieldReflection<
              GenericWithMarkerVec<I, M>,
              <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Tag,
              ::grost::__private::flavors::Groto,
              1u32,
            >,
          >,
        > as ::grost::__private::reflection::Reflectable<GenericWithMarkerVec<I, M>>>::REFLECTION
          .len()
      };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I, M> ::grost::__private::reflection::Reflectable<GenericWithMarkerVec<I, M>>
      for ::grost::__private::reflection::WireSchemaTypeReflection<
        ::grost::__private::reflection::ObjectFieldReflection<
          GenericWithMarkerVec<I, M>,
          <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::WireType,
          ::grost::__private::flavors::Groto,
          1u32,
        >,
      >
    where
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
    {
      type Reflection =
        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::WireType;
      const REFLECTION: &Self::Reflection = &{
        <<::grost::__private::marker::ListMarker<
                    Vec<I>,
                    ::grost::__private::marker::GenericMarker<
                        <Vec<
                            I,
                        > as ::grost::__private::convert::State<
                            ::grost::__private::convert::Flattened<
                                ::grost::__private::convert::Inner,
                            >,
                        >>::Output,
                        M,
                    >,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Groto,
                >>::Format as ::grost::__private::flavors::WireFormat<
                    ::grost::__private::flavors::Groto,
                >>::WIRE_TYPE
      };
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I, M> ::grost::__private::reflection::Reflectable<GenericWithMarkerVec<I, M>>
      for GenericWithMarkerVec<I, M>
    where
      ::grost::__private::reflection::SchemaTypeReflection<Vec<I>>:
        ::grost::__private::reflection::Reflectable<
            Vec<I>,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
    {
      type Reflection = ::grost::__private::reflection::SchemaType;
      const REFLECTION: &'static ::grost::__private::reflection::SchemaType = &{
        ::grost::__private::reflection::SchemaType::Object(
                    &::grost::__private::reflection::ObjectBuilder {
                        name: "GenericWithMarkerVec",
                        description: "",
                        fields: &[
                            &::grost::__private::reflection::ObjectFieldBuilder {
                                name: "id",
                                description: "",
                                ty: <::grost::__private::reflection::SchemaTypeReflection<
                                    Vec<I>,
                                > as ::grost::__private::reflection::Reflectable<
                                    Vec<I>,
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
    impl<I, M> ::grost::__private::reflection::Reflectable<GenericWithMarkerVec<I, M>>
      for ::grost::__private::reflection::Reflection<
        GenericWithMarkerVec<I, M>,
        ::grost::__private::reflection::Object,
        ::grost::__private::flavors::Groto,
      >
    where
      ::grost::__private::reflection::SchemaTypeReflection<Vec<I>>:
        ::grost::__private::reflection::Reflectable<
            Vec<I>,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
    {
      type Reflection = ::grost::__private::reflection::Object;
      const REFLECTION: &'static ::grost::__private::reflection::Object = &{
        ::grost::__private::reflection::ObjectBuilder {
                    name: "GenericWithMarkerVec",
                    description: "",
                    fields: &[
                        &::grost::__private::reflection::ObjectFieldBuilder {
                            name: "id",
                            description: "",
                            ty: <::grost::__private::reflection::SchemaTypeReflection<
                                Vec<I>,
                            > as ::grost::__private::reflection::Reflectable<
                                Vec<I>,
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
    impl<I, M> ::grost::__private::reflection::Reflectable<GenericWithMarkerVec<I, M>>
      for ::grost::__private::reflection::SchemaTypeReflection<
        ::grost::__private::reflection::Reflection<
          GenericWithMarkerVec<I, M>,
          ::grost::__private::reflection::Object,
          ::grost::__private::flavors::Groto,
        >,
      >
    where
      ::grost::__private::reflection::SchemaTypeReflection<Vec<I>>:
        ::grost::__private::reflection::Reflectable<
            Vec<I>,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
    {
      type Reflection = ::grost::__private::reflection::Object;
      const REFLECTION: &'static ::grost::__private::reflection::Object = &{
        ::grost::__private::reflection::ObjectBuilder {
                    name: "GenericWithMarkerVec",
                    description: "",
                    fields: &[
                        &::grost::__private::reflection::ObjectFieldBuilder {
                            name: "id",
                            description: "",
                            ty: <::grost::__private::reflection::SchemaTypeReflection<
                                Vec<I>,
                            > as ::grost::__private::reflection::Reflectable<
                                Vec<I>,
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
    impl<I, M> GenericWithMarkerVec<I, M> {
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
      /// Returns the field reflection of the field `GenericWithMarkerVec.id`.
      #[inline]
      const fn id_reflection() -> ::grost::__private::reflection::ObjectFieldReflection<
        GenericWithMarkerVec<I, M>,
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
    impl<I, M> ::grost::__private::indexer::Indexable<::grost::__private::flavors::Groto>
      for GenericWithMarkerVec<I, M>
    {
      type Indexer = GenericWithMarkerVecIndex;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl GenericWithMarkerVecIndex {
      /// Returns the field reflection of the corresponding field.
      #[allow(non_camel_case_types, clippy::type_complexity)]
      #[inline]
      pub const fn reflection<I, M>(&self) -> &'static ::grost::__private::reflection::ObjectField
      where
        ::grost::__private::reflection::SchemaTypeReflection<Vec<I>>:
          ::grost::__private::reflection::Reflectable<
              Vec<I>,
              Reflection = ::grost::__private::reflection::SchemaType,
            >,
      {
        match self {
          Self::Id => <::grost::__private::reflection::ObjectFieldReflection<
            GenericWithMarkerVec<I, M>,
            ::grost::__private::reflection::ObjectField,
            ::grost::__private::flavors::Groto,
            1u32,
          > as ::grost::__private::reflection::Reflectable<GenericWithMarkerVec<I, M>>>::REFLECTION,
        }
      }
    }
    #[automatically_derived]
    impl GenericWithMarkerVecIndex {
      /// The number of variants of this field indexer.
      pub const VARIANTS: ::core::primitive::usize = 2usize;
      /// The first field indexer.
      pub const FIRST: Self = Self::Id;
      /// The last field indexer.
      pub const LAST: Self = Self::Id;
      /// Returns the next field indexer.
      ///
      /// Returns `None` if there are no more fields.
      #[inline]
      pub const fn next(&self) -> ::core::option::Option<Self> {
        match self {
          Self::Id => ::core::option::Option::None,
        }
      }
      /// Returns the previous field indexer.
      ///
      /// Returns `None` if there are no previous fields.
      #[inline]
      pub const fn prev(&self) -> ::core::option::Option<Self> {
        match self {
          Self::Id => ::core::option::Option::None,
        }
      }
      /// Returns the remaining number of fields.
      #[inline]
      pub const fn remaining(&self) -> ::core::primitive::usize {
        Self::LAST.index() - self.index()
      }
      const fn index(&self) -> ::core::primitive::usize {
        match self {
          Self::Id => 0usize,
        }
      }
    }
    #[automatically_derived]
    impl ::core::iter::Iterator for GenericWithMarkerVecIndex {
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
    impl ::core::iter::DoubleEndedIterator for GenericWithMarkerVecIndex {
      fn next_back(&mut self) -> ::core::option::Option<Self> {
        Self::prev(self)
      }
    }
    #[automatically_derived]
    impl ::core::iter::FusedIterator for GenericWithMarkerVecIndex {}
    #[automatically_derived]
    impl ::core::iter::ExactSizeIterator for GenericWithMarkerVecIndex {
      fn len(&self) -> ::core::primitive::usize {
        self.remaining()
      }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I, M> ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>
      for GenericWithMarkerVec<I, M>
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      Vec<I>: ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>,
    {
      type Selector = GenericWithMarkerVecSelector<I, M>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I, M> ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>
      for PartialGenericWithMarkerVec<I, M>
    where
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output: ::core::marker::Sized,
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      Vec<I>: ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>,
    {
      type Selector = GenericWithMarkerVecSelector<I, M>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<'__grost_lifetime__, I, M, __GROST_READ_BUFFER__, __GROST_BUFFER__>
      ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>
      for PartialGenericWithMarkerVecRef<
        '__grost_lifetime__,
        I,
        M,
        __GROST_READ_BUFFER__,
        __GROST_BUFFER__,
      >
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          <::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
          > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output: ::core::marker::Sized,
      Vec<I>: ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>,
    {
      type Selector = GenericWithMarkerVecSelector<I, M>;
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<I, M> ::core::fmt::Debug for GenericWithMarkerVecSelector<I, M>
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      Vec<I>: ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>,
    {
      fn fmt(
        &self,
        f: &mut ::core::fmt::Formatter<'_>,
      ) -> ::core::result::Result<(), ::core::fmt::Error> {
        f.debug_struct("GenericWithMarkerVecSelector")
          .field("id", &self.id)
          .finish()
      }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<I, M> ::core::cmp::PartialEq for GenericWithMarkerVecSelector<I, M>
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      Vec<I>: ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>,
    {
      fn eq(&self, other: &Self) -> ::core::primitive::bool {
        self.id == other.id
      }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<I, M> ::core::cmp::Eq for GenericWithMarkerVecSelector<I, M>
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      Vec<I>: ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>,
    {
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<I, M> ::core::hash::Hash for GenericWithMarkerVecSelector<I, M>
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      Vec<I>: ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>,
    {
      fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
      }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<I, M> ::core::clone::Clone for GenericWithMarkerVecSelector<I, M>
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      Vec<I>: ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>,
    {
      fn clone(&self) -> Self {
        Self {
          id: ::core::clone::Clone::clone(&self.id),
          _m: ::core::marker::PhantomData,
        }
      }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<I, M> ::core::marker::Copy for GenericWithMarkerVecSelector<I, M>
        where
            ::grost::__private::marker::ListMarker<
                Vec<I>,
                ::grost::__private::marker::GenericMarker<
                    <Vec<
                        I,
                    > as ::grost::__private::convert::State<
                        ::grost::__private::convert::Flattened<
                            ::grost::__private::convert::Inner,
                        >,
                    >>::Output,
                    M,
                >,
            >: ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Groto,
            >,
            Vec<
                I,
            >: ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Groto,
            >,
            <Vec<
                I,
            > as ::grost::__private::selection::Selectable<
                ::grost::__private::flavors::Groto,
            >>::Selector: ::core::marker::Copy,
        {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<I, M> ::grost::__private::selection::Selector<::grost::__private::flavors::Groto>
      for GenericWithMarkerVecSelector<I, M>
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      Vec<I>: ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>,
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
        <<Vec<
                    I,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::flip(&mut self.id);
        self
      }
      fn merge(&mut self, other: Self) -> &mut Self {
        <<Vec<
                    I,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::merge(&mut self.id, other.id);
        self
      }
    }
    #[automatically_derived]
    impl<I, M> GenericWithMarkerVecSelector<I, M>
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      Vec<I>: ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>,
    {
      /// Returns a selector with the default values.
      #[inline]
      pub const fn new() -> Self {
        Self {
          id: <<Vec<I> as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Groto,
          >>::Selector as ::grost::__private::selection::Selector<
            ::grost::__private::flavors::Groto,
          >>::DEFAULT,
          _m: ::core::marker::PhantomData,
        }
      }
      /// Returns a selector which selects nothing.
      #[inline]
      pub const fn empty() -> Self {
        Self {
          id: <<Vec<I> as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Groto,
          >>::Selector as ::grost::__private::selection::Selector<
            ::grost::__private::flavors::Groto,
          >>::NONE,
          _m: ::core::marker::PhantomData,
        }
      }
      /// Returns a selector which selects all.
      #[inline]
      pub const fn all() -> Self {
        Self {
          id: <<Vec<I> as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Groto,
          >>::Selector as ::grost::__private::selection::Selector<
            ::grost::__private::flavors::Groto,
          >>::ALL,
          _m: ::core::marker::PhantomData,
        }
      }
      /// Returns `true` if the selector selects nothing.
      #[inline]
      pub fn is_empty(&self) -> ::core::primitive::bool {
        <<Vec<
                    I,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::is_empty(&self.id)
      }
      /// Returns `true` if the selector selects all.
      #[inline]
      pub fn is_all(&self) -> ::core::primitive::bool {
        <<Vec<
                    I,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::is_all(&self.id)
      }
      /// Returns the number of selected fields.
      #[inline]
      pub fn selected(&self) -> ::core::primitive::usize {
        let mut num = 0;
        if self.is_id_selected() {
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
        num
      }
      /// Returns an iterator over the selected fields.
      #[inline]
      pub fn iter_selected<'__grost_lifetime__>(
        &'__grost_lifetime__ self,
      ) -> GenericWithMarkerVecSelectorIter<'__grost_lifetime__, I, M, true> {
        GenericWithMarkerVecSelectorIter::new(self, self.selected())
      }
      /// Returns an iterator over the unselected fields.
      #[inline]
      pub fn iter_unselected<'__grost_lifetime__>(
        &'__grost_lifetime__ self,
      ) -> GenericWithMarkerVecSelectorIter<'__grost_lifetime__, I, M, false> {
        GenericWithMarkerVecSelectorIter::new(self, self.unselected())
      }
      /// Returns `true` if such field is selected.
      #[inline]
      pub fn is_selected(&self, idx: GenericWithMarkerVecIndex) -> ::core::primitive::bool {
        match idx {
          GenericWithMarkerVecIndex::Id => self.is_id_selected(),
        }
      }
      /// Returns `true` if such field is unselected.
      #[inline]
      pub fn is_unselected(&self, idx: GenericWithMarkerVecIndex) -> ::core::primitive::bool {
        match idx {
          GenericWithMarkerVecIndex::Id => self.is_id_unselected(),
        }
      }
      /// Select the `GenericWithMarkerVec.id` field
      #[inline]
      pub fn select_id(&mut self) -> &mut Self {
        self.id = <<Vec<I> as ::grost::__private::selection::Selectable<
          ::grost::__private::flavors::Groto,
        >>::Selector as ::grost::__private::selection::Selector<
          ::grost::__private::flavors::Groto,
        >>::DEFAULT;
        self
      }
      /// Unselect the `GenericWithMarkerVec.id` field
      #[inline]
      pub fn unselect_id(&mut self) -> &mut Self {
        self.id = <<Vec<I> as ::grost::__private::selection::Selectable<
          ::grost::__private::flavors::Groto,
        >>::Selector as ::grost::__private::selection::Selector<
          ::grost::__private::flavors::Groto,
        >>::NONE;
        self
      }
      /// Update the `GenericWithMarkerVec.id` field
      #[inline]
      pub fn update_id(
        &mut self,
        value: <Vec<I> as ::grost::__private::selection::Selectable<
          ::grost::__private::flavors::Groto,
        >>::Selector,
      ) -> &mut Self {
        self.id = value;
        self
      }
      /// Set or unset the `GenericWithMarkerVec.id` field
      #[inline]
      pub fn maybe_id(
        mut self,
        val: <Vec<I> as ::grost::__private::selection::Selectable<
          ::grost::__private::flavors::Groto,
        >>::Selector,
      ) -> Self {
        self.id = val;
        self
      }
      /// Get a reference to the selector of `GenericWithMarkerVec.id` field
      #[inline]
      pub const fn id_ref(
        &self,
      ) -> &<Vec<I> as ::grost::__private::selection::Selectable<
        ::grost::__private::flavors::Groto,
      >>::Selector {
        &self.id
      }
      /// Get a mutable reference to the selector of `GenericWithMarkerVec.id` field
      #[inline]
      pub const fn id_mut(
        &mut self,
      ) -> &mut <Vec<I> as ::grost::__private::selection::Selectable<
        ::grost::__private::flavors::Groto,
      >>::Selector {
        &mut self.id
      }
      /// Set the `GenericWithMarkerVec.id` field
      #[inline]
      pub fn with_id(mut self) -> Self {
        self.id = <<Vec<I> as ::grost::__private::selection::Selectable<
          ::grost::__private::flavors::Groto,
        >>::Selector as ::grost::__private::selection::Selector<
          ::grost::__private::flavors::Groto,
        >>::DEFAULT;
        self
      }
      /// Unset the `GenericWithMarkerVec.id` field
      #[inline]
      pub fn without_id(mut self) -> Self {
        self.id = <<Vec<I> as ::grost::__private::selection::Selectable<
          ::grost::__private::flavors::Groto,
        >>::Selector as ::grost::__private::selection::Selector<
          ::grost::__private::flavors::Groto,
        >>::NONE;
        self
      }
      /// Returns `true` if the `GenericWithMarkerVec.id` field is selected
      #[inline]
      pub fn is_id_selected(&self) -> ::core::primitive::bool {
        !<<Vec<
                    I,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::is_empty(&self.id)
      }
      /// Returns `true` if the `GenericWithMarkerVec.id` field is unselected
      #[inline]
      pub fn is_id_unselected(&self) -> ::core::primitive::bool {
        <<Vec<
                    I,
                > as ::grost::__private::selection::Selectable<
                    ::grost::__private::flavors::Groto,
                >>::Selector as ::grost::__private::selection::Selector<
                    ::grost::__private::flavors::Groto,
                >>::is_empty(&self.id)
      }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<'__grost_lifetime__, I, M, const __GROST_SELECTED__: ::core::primitive::bool>
      GenericWithMarkerVecSelectorIter<'__grost_lifetime__, I, M, __GROST_SELECTED__>
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      Vec<I>: ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>,
    {
      #[inline]
      const fn new(
        selector: &'__grost_lifetime__ GenericWithMarkerVecSelector<I, M>,
        num: ::core::primitive::usize,
      ) -> Self {
        Self {
          selector,
          index: ::core::option::Option::Some(GenericWithMarkerVecIndex::FIRST),
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
    impl<'__grost_lifetime__, I, M, const __GROST_SELECTED__: ::core::primitive::bool>
      ::core::iter::Iterator
      for GenericWithMarkerVecSelectorIter<'__grost_lifetime__, I, M, __GROST_SELECTED__>
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      Vec<I>: ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>,
      ::grost::__private::reflection::SchemaTypeReflection<Vec<I>>:
        ::grost::__private::reflection::Reflectable<
            Vec<I>,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
    {
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
              return ::core::option::Option::Some(idx.reflection::<I, M>());
            }
          } else if self.selector.is_unselected(idx) {
            self.index = idx.next();
            self.yielded += 1;
            return ::core::option::Option::Some(idx.reflection::<I, M>());
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
    impl<'__grost_lifetime__, I, M, const __GROST_SELECTED__: ::core::primitive::bool>
      ::core::iter::FusedIterator
      for GenericWithMarkerVecSelectorIter<'__grost_lifetime__, I, M, __GROST_SELECTED__>
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      Vec<I>: ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>,
      ::grost::__private::reflection::SchemaTypeReflection<Vec<I>>:
        ::grost::__private::reflection::Reflectable<
            Vec<I>,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
    {
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<'__grost_lifetime__, I, M, const __GROST_SELECTED__: ::core::primitive::bool>
      ::core::iter::ExactSizeIterator
      for GenericWithMarkerVecSelectorIter<'__grost_lifetime__, I, M, __GROST_SELECTED__>
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      Vec<I>: ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>,
      ::grost::__private::reflection::SchemaTypeReflection<Vec<I>>:
        ::grost::__private::reflection::Reflectable<
            Vec<I>,
            Reflection = ::grost::__private::reflection::SchemaType,
          >,
    {
      fn len(&self) -> ::core::primitive::usize {
        self.remaining()
      }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I, M>
      ::grost::__private::convert::Transform<
        Self,
        Self,
        ::grost::__private::flavors::groto::LengthDelimited,
        ::grost::__private::flavors::Groto,
      > for GenericWithMarkerVec<I, M>
    {
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
    impl<'__grost_lifetime__, I, M, __GROST_READ_BUFFER__, __GROST_BUFFER__>
      ::grost::__private::decode::Decode<
        '__grost_lifetime__,
        Self,
        ::grost::__private::flavors::groto::LengthDelimited,
        __GROST_READ_BUFFER__,
        __GROST_BUFFER__,
        ::grost::__private::flavors::Groto,
      > for PartialGenericWithMarkerVec<I, M>
    where
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output: ::core::marker::Sized,
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          <::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
          > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output: ::core::marker::Sized,
      Vec<I>:
        ::grost::__private::decode::Decode<
            '__grost_lifetime__,
            <Vec<I> as ::grost::__private::convert::State<
              ::grost::__private::convert::PartialRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_BUFFER__,
                <::grost::__private::marker::ListMarker<
                  Vec<I>,
                  ::grost::__private::marker::GenericMarker<
                    <Vec<I> as ::grost::__private::convert::State<
                      ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                    >>::Output,
                    M,
                  >,
                > as ::grost::__private::flavors::DefaultWireFormat<
                  ::grost::__private::flavors::Groto,
                >>::Format,
                ::grost::__private::flavors::Groto,
              >,
            >>::Output,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            ::grost::__private::flavors::Groto,
          >,
      Vec<I>:
        ::grost::__private::convert::Transform<
            <Vec<I> as ::grost::__private::convert::State<
              ::grost::__private::convert::PartialRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_BUFFER__,
                <::grost::__private::marker::ListMarker<
                  Vec<I>,
                  ::grost::__private::marker::GenericMarker<
                    <Vec<I> as ::grost::__private::convert::State<
                      ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                    >>::Output,
                    M,
                  >,
                > as ::grost::__private::flavors::DefaultWireFormat<
                  ::grost::__private::flavors::Groto,
                >>::Format,
                ::grost::__private::flavors::Groto,
              >,
            >>::Output,
            <Vec<I> as ::grost::__private::convert::State<
              ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
            >>::Output,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
    {
      fn decode(
        context: &'__grost_lifetime__ <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Context,
        src: __GROST_READ_BUFFER__,
      ) -> ::core::result::Result<
        (::core::primitive::usize, Self),
        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
      >
      where
        Self: ::core::marker::Sized + '__grost_lifetime__,
        __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf + '__grost_lifetime__,
        __GROST_BUFFER__: ::grost::__private::buffer::Buffer<
            <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Unknown<
              __GROST_READ_BUFFER__,
            >,
          > + '__grost_lifetime__,
      {
        <PartialGenericWithMarkerVec<I, M> as ::grost::__private::decode::Decode<
          '__grost_lifetime__,
          PartialGenericWithMarkerVecRef<
            '__grost_lifetime__,
            I,
            M,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
          >,
          ::grost::__private::flavors::groto::LengthDelimited,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          ::grost::__private::flavors::Groto,
        >>::decode(context, src)
        .and_then(|(read, input)| {
          <PartialGenericWithMarkerVec<I, M> as ::grost::__private::convert::Transform<
            PartialGenericWithMarkerVecRef<
              '__grost_lifetime__,
              I,
              M,
              __GROST_READ_BUFFER__,
              __GROST_BUFFER__,
            >,
            PartialGenericWithMarkerVec<I, M>,
            ::grost::__private::flavors::groto::LengthDelimited,
            ::grost::__private::flavors::Groto,
          >>::transform(input)
          .map(|input| (read, input))
        })
      }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<'__grost_lifetime__, I, M, __GROST_READ_BUFFER__, __GROST_BUFFER__>
      ::grost::__private::decode::Decode<
        '__grost_lifetime__,
        Self,
        ::grost::__private::flavors::groto::LengthDelimited,
        __GROST_READ_BUFFER__,
        __GROST_BUFFER__,
        ::grost::__private::flavors::Groto,
      > for GenericWithMarkerVec<I, M>
    where
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output: ::core::marker::Sized,
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          <::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
          > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output: ::core::marker::Sized,
      Vec<I>:
        ::grost::__private::decode::Decode<
            '__grost_lifetime__,
            <Vec<I> as ::grost::__private::convert::State<
              ::grost::__private::convert::PartialRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_BUFFER__,
                <::grost::__private::marker::ListMarker<
                  Vec<I>,
                  ::grost::__private::marker::GenericMarker<
                    <Vec<I> as ::grost::__private::convert::State<
                      ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                    >>::Output,
                    M,
                  >,
                > as ::grost::__private::flavors::DefaultWireFormat<
                  ::grost::__private::flavors::Groto,
                >>::Format,
                ::grost::__private::flavors::Groto,
              >,
            >>::Output,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            ::grost::__private::flavors::Groto,
          >,
      Vec<I>:
        ::grost::__private::convert::Transform<
            <Vec<I> as ::grost::__private::convert::State<
              ::grost::__private::convert::PartialRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_BUFFER__,
                <::grost::__private::marker::ListMarker<
                  Vec<I>,
                  ::grost::__private::marker::GenericMarker<
                    <Vec<I> as ::grost::__private::convert::State<
                      ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                    >>::Output,
                    M,
                  >,
                > as ::grost::__private::flavors::DefaultWireFormat<
                  ::grost::__private::flavors::Groto,
                >>::Format,
                ::grost::__private::flavors::Groto,
              >,
            >>::Output,
            <Vec<I> as ::grost::__private::convert::State<
              ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
            >>::Output,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
      Vec<I>:
        ::grost::__private::convert::Transform<
            <Vec<I> as ::grost::__private::convert::State<
              ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
            >>::Output,
            Vec<I>,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
    {
      fn decode(
        context: &'__grost_lifetime__ <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Context,
        src: __GROST_READ_BUFFER__,
      ) -> ::core::result::Result<
        (::core::primitive::usize, Self),
        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
      >
      where
        Self: ::core::marker::Sized + '__grost_lifetime__,
        __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf + '__grost_lifetime__,
        __GROST_BUFFER__: ::grost::__private::buffer::Buffer<
            <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Unknown<
              __GROST_READ_BUFFER__,
            >,
          > + '__grost_lifetime__,
      {
        <PartialGenericWithMarkerVec<I, M> as ::grost::__private::decode::Decode<
          '__grost_lifetime__,
          PartialGenericWithMarkerVec<I, M>,
          ::grost::__private::flavors::groto::LengthDelimited,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          ::grost::__private::flavors::Groto,
        >>::decode(context, src)
        .and_then(|(read, input)| {
          <GenericWithMarkerVec<I, M> as ::grost::__private::convert::Transform<
            PartialGenericWithMarkerVec<I, M>,
            GenericWithMarkerVec<I, M>,
            ::grost::__private::flavors::groto::LengthDelimited,
            ::grost::__private::flavors::Groto,
          >>::transform(input)
          .map(|input| (read, input))
        })
      }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I, M>
      ::grost::__private::convert::Transform<
        Self,
        Self,
        ::grost::__private::flavors::groto::LengthDelimited,
        ::grost::__private::flavors::Groto,
      > for PartialGenericWithMarkerVec<I, M>
    where
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output: ::core::marker::Sized,
    {
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
    impl<I, M>
      ::grost::__private::convert::Transform<
        PartialGenericWithMarkerVec<I, M>,
        GenericWithMarkerVec<I, M>,
        ::grost::__private::flavors::groto::LengthDelimited,
        ::grost::__private::flavors::Groto,
      > for GenericWithMarkerVec<I, M>
    where
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output: ::core::marker::Sized,
      Vec<I>:
        ::grost::__private::convert::Transform<
            <Vec<I> as ::grost::__private::convert::State<
              ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
            >>::Output,
            Vec<I>,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
    {
      fn transform(
        input: PartialGenericWithMarkerVec<I, M>,
      ) -> ::core::result::Result<
        GenericWithMarkerVec<I, M>,
        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
      > {
        ::core::result::Result::Ok(Self {
          id: {
            if let ::core::option::Option::Some(value) = input.id {
              <Vec<I> as ::grost::__private::convert::Transform<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
                >>::Output,
                Vec<I>,
                <::grost::__private::marker::ListMarker<
                  Vec<I>,
                  ::grost::__private::marker::GenericMarker<
                    <Vec<I> as ::grost::__private::convert::State<
                      ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                    >>::Output,
                    M,
                  >,
                > as ::grost::__private::flavors::DefaultWireFormat<
                  ::grost::__private::flavors::Groto,
                >>::Format,
                ::grost::__private::flavors::Groto,
              >>::transform(value)?
            } else {
              ::core::default::Default::default()
            }
          },
          _m: (::core::default::Default::default)(),
        })
      }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<I, M>
      ::grost::__private::convert::PartialTransform<
        Self,
        ::core::option::Option<Self>,
        ::grost::__private::flavors::groto::LengthDelimited,
        ::grost::__private::flavors::Groto,
      > for PartialGenericWithMarkerVec<I, M>
    where
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output: ::core::marker::Sized,
      Vec<I>:
        ::grost::__private::convert::PartialTransform<
            <Vec<I> as ::grost::__private::convert::State<
              ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
            >>::Output,
            ::core::option::Option<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
              >>::Output,
            >,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          > + ::core::marker::Sized,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output: ::grost::__private::selection::Selectable<
          ::grost::__private::flavors::Groto,
          Selector = <Vec<I> as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Groto,
          >>::Selector,
        >,
      Vec<I>: ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>,
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
    {
      fn partial_transform(
        input: PartialGenericWithMarkerVec<I, M>,
        selector: &GenericWithMarkerVecSelector<I, M>,
      ) -> ::core::result::Result<
        ::core::option::Option<PartialGenericWithMarkerVec<I, M>>,
        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
      > {
        let mut this = Self::new();
        if let ::core::option::Option::Some(value) = input.id {
          if selector.is_id_selected() {
            this.id = <Vec<I> as ::grost::__private::convert::PartialTransform<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
              >>::Output,
              ::core::option::Option<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
                >>::Output,
              >,
              <::grost::__private::marker::ListMarker<
                Vec<I>,
                ::grost::__private::marker::GenericMarker<
                  <Vec<I> as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                  >>::Output,
                  M,
                >,
              > as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Groto,
              >>::Format,
              ::grost::__private::flavors::Groto,
            >>::partial_transform(value, selector.id_ref())?;
          }
        }
        ::core::result::Result::Ok((!this.is_empty()).then_some(this))
      }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<'__grost_lifetime__, I, M, __GROST_READ_BUFFER__, __GROST_BUFFER__>
      ::grost::__private::convert::Transform<
        Self,
        Self,
        ::grost::__private::flavors::groto::LengthDelimited,
        ::grost::__private::flavors::Groto,
      >
      for PartialGenericWithMarkerVecRef<
        '__grost_lifetime__,
        I,
        M,
        __GROST_READ_BUFFER__,
        __GROST_BUFFER__,
      >
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          <::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
          > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output: ::core::marker::Sized,
    {
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
      I,
      M,
      __GROST_READ_BUFFER__,
      __GROST_BUFFER__,
    >
      ::grost::__private::decode::Decode<
        '__grost_decode_lifetime__,
        Self,
        ::grost::__private::flavors::groto::LengthDelimited,
        __GROST_READ_BUFFER__,
        __GROST_BUFFER__,
        ::grost::__private::flavors::Groto,
      >
      for PartialGenericWithMarkerVecRef<
        '__grost_lifetime__,
        I,
        M,
        __GROST_READ_BUFFER__,
        __GROST_BUFFER__,
      >
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          <::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
          > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output: ::core::marker::Sized,
      Vec<I>:
        ::grost::__private::decode::Decode<
            '__grost_decode_lifetime__,
            <Vec<I> as ::grost::__private::convert::State<
              ::grost::__private::convert::PartialRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_BUFFER__,
                <::grost::__private::marker::ListMarker<
                  Vec<I>,
                  ::grost::__private::marker::GenericMarker<
                    <Vec<I> as ::grost::__private::convert::State<
                      ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                    >>::Output,
                    M,
                  >,
                > as ::grost::__private::flavors::DefaultWireFormat<
                  ::grost::__private::flavors::Groto,
                >>::Format,
                ::grost::__private::flavors::Groto,
              >,
            >>::Output,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            ::grost::__private::flavors::Groto,
          >,
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
        __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf + '__grost_decode_lifetime__,
        __GROST_BUFFER__: ::grost::__private::buffer::Buffer<
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
                        _ if identifier
                            .eq(
                                <::grost::__private::reflection::IdentifierReflection<
                                    ::grost::__private::reflection::ObjectFieldReflection<
                                        GenericWithMarkerVec<I, M>,
                                        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
                                        ::grost::__private::flavors::Groto,
                                        1u32,
                                    >,
                                > as ::grost::__private::reflection::Reflectable<
                                    GenericWithMarkerVec<I, M>,
                                >>::REFLECTION,
                            ) => {
                            if offset >= buf_len {
                                return ::core::result::Result::Err(
                                    ::core::convert::Into::into(
                                        ::grost::__private::error::Error::buffer_underflow(),
                                    ),
                                );
                            }
                            if this.id.is_some() {
                                return ::core::result::Result::Err(
                                    ::core::convert::Into::into(
                                        ::grost::__private::error::Error::duplicated_field(
                                            "id",
                                            ::core::any::type_name::<Vec<I>>(),
                                            *<::grost::__private::reflection::IdentifierReflection<
                                                ::grost::__private::reflection::ObjectFieldReflection<
                                                    GenericWithMarkerVec<I, M>,
                                                    <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Identifier,
                                                    ::grost::__private::flavors::Groto,
                                                    1u32,
                                                >,
                                            > as ::grost::__private::reflection::Reflectable<
                                                GenericWithMarkerVec<I, M>,
                                            >>::REFLECTION,
                                        ),
                                    ),
                                );
                            }
                            let (read, value) = (<Vec<
                                I,
                            > as ::grost::__private::decode::Decode<
                                '__grost_decode_lifetime__,
                                <Vec<
                                    I,
                                > as ::grost::__private::convert::State<
                                    ::grost::__private::convert::PartialRef<
                                        '__grost_lifetime__,
                                        __GROST_READ_BUFFER__,
                                        __GROST_BUFFER__,
                                        <::grost::__private::marker::ListMarker<
                                            Vec<I>,
                                            ::grost::__private::marker::GenericMarker<
                                                <Vec<
                                                    I,
                                                > as ::grost::__private::convert::State<
                                                    ::grost::__private::convert::Flattened<
                                                        ::grost::__private::convert::Inner,
                                                    >,
                                                >>::Output,
                                                M,
                                            >,
                                        > as ::grost::__private::flavors::DefaultWireFormat<
                                            ::grost::__private::flavors::Groto,
                                        >>::Format,
                                        ::grost::__private::flavors::Groto,
                                    >,
                                >>::Output,
                                <::grost::__private::marker::ListMarker<
                                    Vec<I>,
                                    ::grost::__private::marker::GenericMarker<
                                        <Vec<
                                            I,
                                        > as ::grost::__private::convert::State<
                                            ::grost::__private::convert::Flattened<
                                                ::grost::__private::convert::Inner,
                                            >,
                                        >>::Output,
                                        M,
                                    >,
                                > as ::grost::__private::flavors::DefaultWireFormat<
                                    ::grost::__private::flavors::Groto,
                                >>::Format,
                                __GROST_READ_BUFFER__,
                                __GROST_BUFFER__,
                                ::grost::__private::flavors::Groto,
                            >>::decode)(context, src.slice(offset..))?;
                            this.id = ::core::option::Option::Some(value);
                            offset += read;
                        }
                        _ => {
                            if context.err_on_unknown() {
                                return ::core::result::Result::Err(
                                    ::core::convert::Into::into(
                                        ::grost::__private::error::Error::unknown_identifier(
                                            ::core::any::type_name::<GenericWithMarkerVec<I, M>>(),
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
                                    .__grost_buffer__
                                    .get_or_insert_with(|| __GROST_BUFFER__::new());
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
      I,
      M,
      __GROST_READ_BUFFER__,
      __GROST_BUFFER__,
    >
      ::grost::__private::decode::Decode<
        '__grost_decode_lifetime__,
        PartialGenericWithMarkerVecRef<
          '__grost_lifetime__,
          I,
          M,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
        >,
        ::grost::__private::flavors::groto::LengthDelimited,
        __GROST_READ_BUFFER__,
        __GROST_BUFFER__,
        ::grost::__private::flavors::Groto,
      > for GenericWithMarkerVec<I, M>
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          <::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
          > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output: ::core::marker::Sized,
      Vec<I>:
        ::grost::__private::decode::Decode<
            '__grost_decode_lifetime__,
            <Vec<I> as ::grost::__private::convert::State<
              ::grost::__private::convert::PartialRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_BUFFER__,
                <::grost::__private::marker::ListMarker<
                  Vec<I>,
                  ::grost::__private::marker::GenericMarker<
                    <Vec<I> as ::grost::__private::convert::State<
                      ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                    >>::Output,
                    M,
                  >,
                > as ::grost::__private::flavors::DefaultWireFormat<
                  ::grost::__private::flavors::Groto,
                >>::Format,
                ::grost::__private::flavors::Groto,
              >,
            >>::Output,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            ::grost::__private::flavors::Groto,
          >,
      '__grost_decode_lifetime__: '__grost_lifetime__,
    {
      fn decode(
        context: &'__grost_decode_lifetime__ <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Context,
        src: __GROST_READ_BUFFER__,
      ) -> ::core::result::Result<
        (
          ::core::primitive::usize,
          PartialGenericWithMarkerVecRef<
            '__grost_lifetime__,
            I,
            M,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
          >,
        ),
        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
      >
      where
        PartialGenericWithMarkerVecRef<
          '__grost_lifetime__,
          I,
          M,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
        >: ::core::marker::Sized + '__grost_decode_lifetime__,
        __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf + '__grost_decode_lifetime__,
        __GROST_BUFFER__: ::grost::__private::buffer::Buffer<
            <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Unknown<
              __GROST_READ_BUFFER__,
            >,
          > + '__grost_decode_lifetime__,
      {
        <PartialGenericWithMarkerVecRef<
          '__grost_lifetime__,
          I,
          M,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
        > as ::grost::__private::decode::Decode<
          '__grost_decode_lifetime__,
          PartialGenericWithMarkerVecRef<
            '__grost_lifetime__,
            I,
            M,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
          >,
          ::grost::__private::flavors::groto::LengthDelimited,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          ::grost::__private::flavors::Groto,
        >>::decode(context, src)
      }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<
      '__grost_lifetime__,
      '__grost_decode_lifetime__,
      I,
      M,
      __GROST_READ_BUFFER__,
      __GROST_BUFFER__,
    >
      ::grost::__private::decode::Decode<
        '__grost_decode_lifetime__,
        PartialGenericWithMarkerVecRef<
          '__grost_lifetime__,
          I,
          M,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
        >,
        ::grost::__private::flavors::groto::LengthDelimited,
        __GROST_READ_BUFFER__,
        __GROST_BUFFER__,
        ::grost::__private::flavors::Groto,
      > for PartialGenericWithMarkerVec<I, M>
    where
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          <::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
          > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output: ::core::marker::Sized,
      Vec<I>:
        ::grost::__private::decode::Decode<
            '__grost_decode_lifetime__,
            <Vec<I> as ::grost::__private::convert::State<
              ::grost::__private::convert::PartialRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_BUFFER__,
                <::grost::__private::marker::ListMarker<
                  Vec<I>,
                  ::grost::__private::marker::GenericMarker<
                    <Vec<I> as ::grost::__private::convert::State<
                      ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                    >>::Output,
                    M,
                  >,
                > as ::grost::__private::flavors::DefaultWireFormat<
                  ::grost::__private::flavors::Groto,
                >>::Format,
                ::grost::__private::flavors::Groto,
              >,
            >>::Output,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            ::grost::__private::flavors::Groto,
          >,
      '__grost_decode_lifetime__: '__grost_lifetime__,
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output: ::core::marker::Sized,
    {
      fn decode(
        context: &'__grost_decode_lifetime__ <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Context,
        src: __GROST_READ_BUFFER__,
      ) -> ::core::result::Result<
        (
          ::core::primitive::usize,
          PartialGenericWithMarkerVecRef<
            '__grost_lifetime__,
            I,
            M,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
          >,
        ),
        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
      >
      where
        PartialGenericWithMarkerVecRef<
          '__grost_lifetime__,
          I,
          M,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
        >: ::core::marker::Sized + '__grost_decode_lifetime__,
        __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf + '__grost_decode_lifetime__,
        __GROST_BUFFER__: ::grost::__private::buffer::Buffer<
            <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Unknown<
              __GROST_READ_BUFFER__,
            >,
          > + '__grost_decode_lifetime__,
      {
        <PartialGenericWithMarkerVecRef<
          '__grost_lifetime__,
          I,
          M,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
        > as ::grost::__private::decode::Decode<
          '__grost_decode_lifetime__,
          PartialGenericWithMarkerVecRef<
            '__grost_lifetime__,
            I,
            M,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
          >,
          ::grost::__private::flavors::groto::LengthDelimited,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          ::grost::__private::flavors::Groto,
        >>::decode(context, src)
      }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<'__grost_lifetime__, I, M, __GROST_READ_BUFFER__, __GROST_BUFFER__>
      ::grost::__private::convert::Transform<
        PartialGenericWithMarkerVecRef<
          '__grost_lifetime__,
          I,
          M,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
        >,
        PartialGenericWithMarkerVec<I, M>,
        ::grost::__private::flavors::groto::LengthDelimited,
        ::grost::__private::flavors::Groto,
      > for PartialGenericWithMarkerVec<I, M>
    where
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output: ::core::marker::Sized,
      Vec<I>:
        ::grost::__private::convert::Transform<
            <Vec<I> as ::grost::__private::convert::State<
              ::grost::__private::convert::PartialRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_BUFFER__,
                <::grost::__private::marker::ListMarker<
                  Vec<I>,
                  ::grost::__private::marker::GenericMarker<
                    <Vec<I> as ::grost::__private::convert::State<
                      ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                    >>::Output,
                    M,
                  >,
                > as ::grost::__private::flavors::DefaultWireFormat<
                  ::grost::__private::flavors::Groto,
                >>::Format,
                ::grost::__private::flavors::Groto,
              >,
            >>::Output,
            <Vec<I> as ::grost::__private::convert::State<
              ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
            >>::Output,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          <::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
          > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output: ::core::marker::Sized,
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          <::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
          > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output: ::core::marker::Sized,
      __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf + '__grost_lifetime__,
      __GROST_BUFFER__: ::grost::__private::buffer::Buffer<
          <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Unknown<
            __GROST_READ_BUFFER__,
          >,
        > + '__grost_lifetime__,
    {
      fn transform(
        input: PartialGenericWithMarkerVecRef<
          '__grost_lifetime__,
          I,
          M,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
        >,
      ) -> ::core::result::Result<
        Self,
        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
      > {
        let mut this = Self::new();
        if let ::core::option::Option::Some(value) = input.id {
          this.id =
            ::core::option::Option::Some(<Vec<I> as ::grost::__private::convert::Transform<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::PartialRef<
                  '__grost_lifetime__,
                  __GROST_READ_BUFFER__,
                  __GROST_BUFFER__,
                  <::grost::__private::marker::ListMarker<
                    Vec<I>,
                    ::grost::__private::marker::GenericMarker<
                      <Vec<I> as ::grost::__private::convert::State<
                        ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                      >>::Output,
                      M,
                    >,
                  > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Groto,
                  >>::Format,
                  ::grost::__private::flavors::Groto,
                >,
              >>::Output,
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
              >>::Output,
              <::grost::__private::marker::ListMarker<
                Vec<I>,
                ::grost::__private::marker::GenericMarker<
                  <Vec<I> as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                  >>::Output,
                  M,
                >,
              > as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Groto,
              >>::Format,
              ::grost::__private::flavors::Groto,
            >>::transform(value)?);
        }
        ::core::result::Result::Ok(this)
      }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl<'__grost_lifetime__, I, M, __GROST_READ_BUFFER__, __GROST_BUFFER__>
      ::grost::__private::convert::PartialTransform<
        PartialGenericWithMarkerVecRef<
          '__grost_lifetime__,
          I,
          M,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
        >,
        ::core::option::Option<PartialGenericWithMarkerVec<I, M>>,
        ::grost::__private::flavors::groto::LengthDelimited,
        ::grost::__private::flavors::Groto,
      > for PartialGenericWithMarkerVec<I, M>
    where
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output: ::core::marker::Sized,
      Vec<I>: ::grost::__private::selection::Selectable<::grost::__private::flavors::Groto>,
      Vec<I>:
        ::grost::__private::convert::PartialTransform<
            <Vec<I> as ::grost::__private::convert::State<
              ::grost::__private::convert::PartialRef<
                '__grost_lifetime__,
                __GROST_READ_BUFFER__,
                __GROST_BUFFER__,
                <::grost::__private::marker::ListMarker<
                  Vec<I>,
                  ::grost::__private::marker::GenericMarker<
                    <Vec<I> as ::grost::__private::convert::State<
                      ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                    >>::Output,
                    M,
                  >,
                > as ::grost::__private::flavors::DefaultWireFormat<
                  ::grost::__private::flavors::Groto,
                >>::Format,
                ::grost::__private::flavors::Groto,
              >,
            >>::Output,
            ::core::option::Option<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
              >>::Output,
            >,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
      >>::Output: ::core::marker::Sized
        + ::grost::__private::selection::Selectable<
          ::grost::__private::flavors::Groto,
          Selector = <Vec<I> as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Groto,
          >>::Selector,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          <::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
          > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output: ::core::marker::Sized
        + ::grost::__private::selection::Selectable<
          ::grost::__private::flavors::Groto,
          Selector = <Vec<I> as ::grost::__private::selection::Selectable<
            ::grost::__private::flavors::Groto,
          >>::Selector,
        >,
      ::grost::__private::marker::ListMarker<
        Vec<I>,
        ::grost::__private::marker::GenericMarker<
          <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
          M,
        >,
      >: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>,
      M: ::grost::__private::flavors::DefaultWireFormat<::grost::__private::flavors::Groto>
        + ::grost::__private::marker::Marker<
          Marked = <Vec<I> as ::grost::__private::convert::State<
            ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
          >>::Output,
        >,
      Vec<I>: ::grost::__private::convert::State<
          ::grost::__private::convert::PartialRef<
            '__grost_lifetime__,
            __GROST_READ_BUFFER__,
            __GROST_BUFFER__,
            <::grost::__private::marker::ListMarker<
              Vec<I>,
              ::grost::__private::marker::GenericMarker<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                >>::Output,
                M,
              >,
            > as ::grost::__private::flavors::DefaultWireFormat<
              ::grost::__private::flavors::Groto,
            >>::Format,
            ::grost::__private::flavors::Groto,
          >,
        >,
      <Vec<I> as ::grost::__private::convert::State<
        ::grost::__private::convert::PartialRef<
          '__grost_lifetime__,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
          <::grost::__private::marker::ListMarker<
            Vec<I>,
            ::grost::__private::marker::GenericMarker<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
              >>::Output,
              M,
            >,
          > as ::grost::__private::flavors::DefaultWireFormat<
            ::grost::__private::flavors::Groto,
          >>::Format,
          ::grost::__private::flavors::Groto,
        >,
      >>::Output: ::core::marker::Sized,
      __GROST_READ_BUFFER__: ::grost::__private::buffer::ReadBuf + '__grost_lifetime__,
      __GROST_BUFFER__: ::grost::__private::buffer::Buffer<
          <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Unknown<
            __GROST_READ_BUFFER__,
          >,
        > + '__grost_lifetime__,
    {
      fn partial_transform(
        input: PartialGenericWithMarkerVecRef<
          '__grost_lifetime__,
          I,
          M,
          __GROST_READ_BUFFER__,
          __GROST_BUFFER__,
        >,
        selector: &GenericWithMarkerVecSelector<I, M>,
      ) -> ::core::result::Result<
        ::core::option::Option<Self>,
        <::grost::__private::flavors::Groto as ::grost::__private::flavors::Flavor>::Error,
      > {
        let mut this = Self::new();
        if let ::core::option::Option::Some(value) = input.id {
          if selector.is_id_selected() {
            this.id = <Vec<I> as ::grost::__private::convert::PartialTransform<
              <Vec<I> as ::grost::__private::convert::State<
                ::grost::__private::convert::PartialRef<
                  '__grost_lifetime__,
                  __GROST_READ_BUFFER__,
                  __GROST_BUFFER__,
                  <::grost::__private::marker::ListMarker<
                    Vec<I>,
                    ::grost::__private::marker::GenericMarker<
                      <Vec<I> as ::grost::__private::convert::State<
                        ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                      >>::Output,
                      M,
                    >,
                  > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Groto,
                  >>::Format,
                  ::grost::__private::flavors::Groto,
                >,
              >>::Output,
              ::core::option::Option<
                <Vec<I> as ::grost::__private::convert::State<
                  ::grost::__private::convert::Partial<::grost::__private::flavors::Groto>,
                >>::Output,
              >,
              <::grost::__private::marker::ListMarker<
                Vec<I>,
                ::grost::__private::marker::GenericMarker<
                  <Vec<I> as ::grost::__private::convert::State<
                    ::grost::__private::convert::Flattened<::grost::__private::convert::Inner>,
                  >>::Output,
                  M,
                >,
              > as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Groto,
              >>::Format,
              ::grost::__private::flavors::Groto,
            >>::partial_transform(value, selector.id_ref())?;
          }
        }
        ::core::result::Result::Ok((!this.is_empty()).then_some(this))
      }
    }
  };
}

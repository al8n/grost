use core::marker::PhantomData;

use grost::Object;


// #[derive(Object)]
struct Generic<I, M> {
  // #[grost(tag = 1, generic(marker = "M"))]
  id: I,
  // #[grost(skip)]
  _m: PhantomData<M>,
}

/// Reflectable.
pub trait Reflectable1<F: ?Sized> {
  type Reflection: ?Sized + 'static;

  /// The reflection of this type
  const REFLECTION: Self::Reflection;
}


#[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl<I, M> Reflectable1<Generic<I, M>>
        for ::grost::__private::reflection::WireFormatReflection<
            Generic<I, M>,
            ::grost::__private::flavors::Groto,
            1u32,
        >
        where
            M: ::grost::flavors::DefaultWireFormat<::grost::flavors::Groto> + ::grost::marker::Marker<Marked = I>,
            M::Format: 'static,
        {
            type Reflection = <::grost::__private::marker::GenericMarker<
                M,
            > as ::grost::__private::flavors::DefaultWireFormat<
                ::grost::__private::flavors::Groto,
            >>::Format;
            const REFLECTION: Self::Reflection = <::grost::__private::marker::GenericMarker<
                    M,
                > as ::grost::__private::flavors::DefaultWireFormat<
                    ::grost::__private::flavors::Groto,
                >>::WIRE_FORMAT;
        }

#[test]
fn compile() {}

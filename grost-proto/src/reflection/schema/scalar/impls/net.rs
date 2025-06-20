use core::net::*;

macro_rules! impl_reflectable {
  ($($ty:ty:$variant:ident),+$(,)?) => {
    $(
      impl $crate::reflection::Reflectable<$ty> for $crate::reflection::SchemaTypeReflection<$ty> {
        type Reflection = $crate::reflection::SchemaType;

        const REFLECTION: &Self::Reflection = &$crate::reflection::SchemaType::Scalar($crate::reflection::Scalar::$variant);
      }
    )*
  };
}

impl_reflectable!(
  IpAddr:Ip,
  Ipv4Addr:Ipv4,
  Ipv6Addr:Ipv6,
  SocketAddr:SocketAddr,
  SocketAddrV4:SocketAddrV4,
  SocketAddrV6:SocketAddrV6,
);

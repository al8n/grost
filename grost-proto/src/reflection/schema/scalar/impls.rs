macro_rules! impl_reflectable_with_variant {
  ($($ty:ty $([ $(const $g:ident: usize), +$(,)? ])?:$variant:ident),+$(,)?) => {
    $(
      impl $( < $(const $g: usize)* > )? $crate::reflection::Reflectable<$ty> for $crate::reflection::TypeReflection<$ty> {
        type Reflection = $crate::reflection::Type;

        const REFLECTION: &Self::Reflection = &$crate::reflection::Type::Scalar($crate::reflection::Scalar::$variant);
      }
    )*
  };
}

mod bytes;
mod net;
mod numbers;
mod regex;
mod string;
mod time;
mod uuid;

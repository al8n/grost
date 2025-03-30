macro_rules! impl_output_type {
  ($($ty:ty),+$(,)?) => {
    $(
      impl OutputType for $ty {
        type Serialized<'a>
          = Self
        where
          Self: Sized + 'a;

        type Borrowed<'a>
          = &'a Self
        where
          Self: 'a;

        type SerializedOwned
          = Self
        where
          Self: Sized + 'static;
      }
    )*
  };
}

mod ip_addr;
mod socket_addr;
mod url;
mod domain;
mod host;
mod host_addr;

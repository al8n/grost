use quote::quote;
use syn::{GenericParam, Generics, Ident, Type};

#[derive(Debug, Clone)]
pub struct PartialDecodedObjectFlavor {
  ty: Type,
  generics: Generics,
}

impl PartialDecodedObjectFlavor {
  /// Returns the type of the partial decoded object which applies the flavor.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the generics of the partial decoded object which applies the flavor.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  pub(super) fn from_ast<M, F>(
    flavor_name: &Ident,
    flavor_type: &Type,
    object: &super::super::GenericObjectAst<M, F>,
    fields: &[super::super::GenericField<F>],
  ) -> darling::Result<Self> {
    let partial_decoded_object = &object.partial_decoded;
    let partial_decoded_object_name = partial_decoded_object.name().clone();
    let unknown_buffer_param = &object.unknown_buffer_param;
    let lifetime_param = &object.lifetime_param;
    let original_generics = &object.generics;
    let mut generics = Generics::default();
    let mut ty_params = Vec::new();

    // push the lifetime generic parameter first
    generics.params.extend(
      original_generics
        .lifetimes()
        .cloned()
        .map(GenericParam::from),
    );
    ty_params.extend(original_generics.lifetimes().map(|l| {
      let lt = &l.lifetime;
      quote! { #lt }
    }));

    generics
      .params
      .push(syn::GenericParam::Lifetime(lifetime_param.clone()));
    ty_params.push({
      let lt = &lifetime_param.lifetime;
      quote! { #lt }
    });

    // push the original type generic parameters
    generics.params.extend(
      original_generics
        .type_params()
        .cloned()
        .map(GenericParam::from),
    );
    ty_params.extend(original_generics.type_params().map(|t| {
      let ident = &t.ident;
      quote! { #ident }
    }));
    ty_params.push(quote! { #flavor_type });

    generics
      .params
      .push(syn::GenericParam::Type(unknown_buffer_param.clone()));
    ty_params.push({
      let ident = &unknown_buffer_param.ident;
      quote! { #ident }
    });

    // push the original const generic parameters last
    generics.params.extend(
      original_generics
        .const_params()
        .cloned()
        .map(GenericParam::from),
    );
    ty_params.extend(original_generics.const_params().map(|c| {
      let ident = &c.ident;
      quote! { #ident }
    }));

    if let Some(where_clause) = original_generics.where_clause.as_ref() {
      generics
        .make_where_clause()
        .predicates
        .extend(where_clause.predicates.iter().cloned());
    }

    fields
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .for_each(|f| {
        let ff = f
          .flavors()
          .get(flavor_name)
          .expect("Field flavor already checked when constructing the AST");
        let type_constraints = ff.partial_decoded().type_constraints();
        if !type_constraints.is_empty() {
          generics
            .make_where_clause()
            .predicates
            .extend(type_constraints.iter().cloned());
        }
      });

    let ty = syn::parse2(quote! {
      #partial_decoded_object_name<#(#ty_params),*>
    })?;
    Ok(Self { ty, generics })
  }
}

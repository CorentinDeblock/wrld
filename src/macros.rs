use quote::format_ident;

use crate::converter::{convert_type_to_wgpu, has_type};
use crate::parser::TokenVertexFormat;

#[derive(Debug)]
struct Entity {
    fields: Vec<EntityFields>
}

#[derive(Debug)]
struct EntityFieldsAttrs {
    name: String,
    data: u32
}

#[derive(Debug)]
struct EntityFields {
    attrs: Vec<EntityFieldsAttrs>,
    name: proc_macro2::Ident,
    ty: syn::Type
}

fn get_entity_field(field: &syn::Field) -> Option<EntityFields> {
    let mut attrs: Vec<EntityFieldsAttrs> = Vec::new();

    field.attrs.iter().for_each(|a| {
        a.path.segments.iter().for_each(|ps| {
            let attr_data : syn::LitInt = a.parse_args().unwrap();

            attrs.push(EntityFieldsAttrs {
                name: ps.ident.to_string(),
                data: attr_data.base10_parse().unwrap(),
            });
        })
    });

    let entity_fields = EntityFields {
        attrs,
        name: field.ident.clone().unwrap(),
        ty: field.ty.clone()
    };

    Some(entity_fields)
}

pub fn derive_wrsl_desc(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn::DeriveInput {ident, data, ..} = syn::parse_macro_input!(item as syn::DeriveInput);
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, ..}),
        ..
    }) = data 
    {
        named
    } else {
        panic!("This is not supported");
    };

    let entity = Entity {
        fields: fields.iter().filter_map(|field| {get_entity_field(field)}).collect()
    };

    let mut attrs : Vec<proc_macro2::TokenStream> = Vec::new();

    let mut offset:u64 = 0;
    let mut shader_locations: Vec<u32> = Vec::new();

    for i in entity.fields {
        for attr in i.attrs {
            let format = convert_type_to_wgpu(attr.name.as_str(), attr.data);
            let tty = TokenVertexFormat { attribute: format.wgpu_type.ty};
            let shader_location = format.shader_location;

            if shader_locations.contains(&shader_location) {
                panic!("Cannot have two time the same location in the same struct");
            }

            shader_locations.push(shader_location);

            attrs.push(quote::quote! {
                wgpu::VertexAttribute {
                    offset: #offset,
                    format: #tty,
                    shader_location: #shader_location
                }
            });

            offset += format.wgpu_type.offset;
        }
    }

    quote::quote! {
        impl #ident {
            pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
                wgpu::VertexBufferLayout {
                    array_stride: #offset as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[#(#attrs),*]
                }
            }
        }
    }.into()
}

pub fn derive_wrsl_buffer_data(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn::DeriveInput {ident, data, ..} = syn::parse_macro_input!(item as syn::DeriveInput);
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, ..}),
        ..
    }) = data 
    {
        named
    } else {
        panic!("This is not supported");
    };

    let entity = Entity {
        fields: fields.iter().filter_map(|field| {get_entity_field(field)}).collect()
    };

    let ident_name = format_ident!("{}{}", ident, "BufferData");

    let mut struct_fields : Vec<proc_macro2::TokenStream> = Vec::new();
    let mut equal_fields : Vec<proc_macro2::TokenStream> = Vec::new();
    let mut partial_eq_fields : Vec<proc_macro2::TokenStream> = Vec::new();
    let mut into_fields : Vec<proc_macro2::TokenStream> = Vec::new();

    entity.fields.iter().for_each(|f| {
        let mut process = false;

        for i in &f.attrs {
            process = has_type(&i.name);
        }

        if process
        {
            let name = f.name.clone();
            let ty = f.ty.clone();

            struct_fields.push(quote::quote! {
                #name: #ty
            });

            equal_fields.push(quote::quote! {
                #name: other_data_from_ident_to_into.#name
            });

            partial_eq_fields.push(quote::quote! {
                self.#name == other_ident_data_boolean_condition.#name
            });

            into_fields.push(quote::quote! {
                #name: other_ident_data_to_into_const.#name
            });
        }
    });

    quote::quote! {
        #[repr(C)]
        #[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
        struct #ident_name {
            #(#struct_fields),*
        }

        impl From<#ident> for #ident_name {
            fn from(other_data_from_ident_to_into: #ident) -> Self {
                Self {
                    #(#equal_fields),*
                }
            }
        }

        impl From<&'static #ident> for #ident_name {
            fn from(other_data_from_ident_to_into: &'static #ident) -> Self {
                Self {
                    #(#equal_fields),*
                }
            }
        }

        impl PartialEq<#ident> for #ident_name {
            fn eq(&self, other_ident_data_boolean_condition: &#ident) -> bool {
                #(#partial_eq_fields)&&*
            }
        }

        impl FromIterator<#ident> for Vec<#ident_name> {
            fn from_iter<T: IntoIterator<Item = #ident>>(iter: T) -> Self {
                let mut vec_data_from_ident_from_iterator = Vec::new();

                for c in iter {
                    vec_data_from_ident_from_iterator.push(c.into());
                }

                vec_data_from_ident_from_iterator
            }
        }

        impl FromIterator<&'static #ident> for Vec<#ident_name> {
            fn from_iter<T: IntoIterator<Item = &'static #ident>>(iter: T) -> Self {
                let mut vec_data_from_ident_single_from_iterator : Vec<VertexBufferData> = Vec::new();

                for c in iter {
                    vec_data_from_ident_single_from_iterator.push(c.into());
                }

                vec_data_from_ident_single_from_iterator
            }
        }

        impl #ident_name {
            pub const fn const_into(other_ident_data_to_into_const: &#ident) -> Self {
                Self {
                    #(#into_fields),*
                }
            }
        }

        impl #ident {
            pub fn mutate<'a>(other_data_from_ident_to_mutate: &'a Vec<#ident_name>) -> &'a [u8] {
                bytemuck::cast_slice(other_data_from_ident_to_mutate.as_slice())
            }

            pub fn transmute(other_data_from_ident_to_transmute: &'static [Self]) -> Vec<#ident_name> {
                other_data_from_ident_to_transmute.into_iter().collect::<Vec<#ident_name>>() 
            }
        }
    }.into()
}
use crate::converter::{convert_type_to_wgpu, has_type};
use crate::parser::TokenVertexFormat;
use crate::parser::parse_attrs;

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

    parse_attrs(&field.attrs,Box::new(|attr| {
        let lint : syn::LitInt = attr.attribute.parse_args().unwrap();

        attrs.push(EntityFieldsAttrs {
            name: attr.segment.ident.to_string(),
            data: lint.base10_parse().unwrap()
        });
    }));

    let entity_fields = EntityFields {
        attrs,
        name: field.ident.clone().unwrap(),
        ty: field.ty.clone()
    };

    Some(entity_fields)
}

fn require_repr_c(attrs : &std::vec::Vec<syn::Attribute>) {
    let mut valid = false;

    parse_attrs(&attrs, Box::new(|attr| {
        let repr_attr = attr.attribute.parse_args::<syn::Ident>().unwrap().to_string();
        if attr.segment.ident.to_string() == "repr" && (repr_attr == "C" || repr_attr == "transparent") {
            valid = true;
        }
    }));

    if !valid {
        panic!("wrld::Desc derive macro require #[repr(C)] or #[repr(transparent)] attribute for safety measure");
    }
}

pub fn derive_wrsl_desc(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let syn::DeriveInput {ident, data, attrs, ..} = syn::parse_macro_input!(item as syn::DeriveInput);
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, ..}),
        ..
    }) = data 
    {
        named
    } else {
        panic!("This is not supported");
    };

    require_repr_c(&attrs);

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

    let subclass_name = quote::format_ident!("{}{}", ident, "BufferData");

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
        struct #subclass_name {
            #(#struct_fields),*
        }

        impl From<#ident> for #subclass_name {
            fn from(other_data_from_ident_to_into: #ident) -> Self {
                Self {
                    #(#equal_fields),*
                }
            }
        }

        impl From<&'static #ident> for #subclass_name {
            fn from(other_data_from_ident_to_into: &'static #ident) -> Self {
                Self {
                    #(#equal_fields),*
                }
            }
        }

        impl PartialEq<#ident> for #subclass_name {
            fn eq(&self, other_ident_data_boolean_condition: &#ident) -> bool {
                #(#partial_eq_fields)&&*
            }
        }

        impl FromIterator<#ident> for Vec<#subclass_name> {
            fn from_iter<T: IntoIterator<Item = #ident>>(iter: T) -> Self {
                let mut vec_data_from_ident_from_iterator = Vec::new();

                for c in iter {
                    vec_data_from_ident_from_iterator.push(c.into());
                }

                vec_data_from_ident_from_iterator
            }
        }

        impl FromIterator<&'static #ident> for Vec<#subclass_name> {
            fn from_iter<T: IntoIterator<Item = &'static #ident>>(iter: T) -> Self {
                let mut vec_data_from_ident_single_from_iterator : Vec<VertexBufferData> = Vec::new();

                for c in iter {
                    vec_data_from_ident_single_from_iterator.push(c.into());
                }

                vec_data_from_ident_single_from_iterator
            }
        }

        impl #subclass_name {
            pub const fn const_into(other_ident_data_to_into_const: &#ident) -> Self {
                Self {
                    #(#into_fields),*
                }
            }
        }

        impl #ident {
            pub fn mutate<'a>(other_data_from_ident_to_mutate: &'a Vec<#subclass_name>) -> &'a [u8] {
                bytemuck::cast_slice(other_data_from_ident_to_mutate.as_slice())
            }

            pub fn transmute(other_data_from_ident_to_transmute: &'static [Self]) -> Vec<#subclass_name> {
                other_data_from_ident_to_transmute.into_iter().collect::<Vec<#subclass_name>>() 
            }
        }
    }.into()
}
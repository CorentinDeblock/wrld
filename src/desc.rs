use crate::converter::{convert_type_to_wgpu};
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
    attrs: Vec<EntityFieldsAttrs>
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
        attrs
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
            let tty = TokenVertexFormat { attribute: format.ty};
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

            offset += format.offset;
        }
    }

    quote::quote! {
        impl #ident {
            pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<#ident>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[#(#attrs),*]
                }
            }
        }
    }.into()
}
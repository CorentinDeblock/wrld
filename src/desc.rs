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
    // let ident = match &field.ident {
    //     Some(id) => Some(format!("{}", id)),
    //     None => None
    // };

    // let ty_ident = match &field.ty {
    //     syn::Type::Path(syn::TypePath {
    //         path: syn::Path {segments , ..},
    //         ..
    //     }) => segments.first().and_then(|s| Some(format!("{}", s.ident))),
    //     _ => None
    // };

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
    let mut shader_location: u32 = 0;

    let mut shader_locations: Vec<u32> = Vec::new();

    for i in entity.fields {
        for attr in i.attrs {
            let format = convert_type_to_wgpu(attr.name.as_str(), attr.data);
            let tty = TokenVertexFormat { attribute: format.ty};

            if shader_locations.contains(&format.shader_location) {
                panic!("Cannot have two time the same location in the same struct");
            }

            shader_locations.push(format.shader_location);

            attrs.push(quote::quote! {
                wgpu::VertexAttribute {
                    offset: #offset,
                    format: #tty,
                    shader_location: #shader_location
                }
            });

            offset += format.offset;
            shader_location += 1;
        }
    }

    eprintln!("Length of attrs {}", attrs.len());

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
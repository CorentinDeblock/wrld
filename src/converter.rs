const F16 : usize = 2;

#[derive(Clone)]
pub struct TypeToWGPU {
    pub offset: u64,
    pub ty: wgpu::VertexFormat,
    pub shader_location: u32
}

pub fn convert_type_to_wgpu(name: &str, shader_location: u32) -> TypeToWGPU {
    match name {
        "u32" => TypeToWGPU { 
            offset: std::mem::size_of::<u32>() as u64, 
            ty: wgpu::VertexFormat::Uint32, 
            shader_location
        },
        "f32" => TypeToWGPU { 
            offset: std::mem::size_of::<f32>() as u64, 
            ty: wgpu::VertexFormat::Float32, 
            shader_location
        },
        "s32" => TypeToWGPU { 
            offset: std::mem::size_of::<i32>() as u64, 
            ty: wgpu::VertexFormat::Sint32, 
            shader_location
        },
        "f64" => TypeToWGPU { 
            offset: std::mem::size_of::<f64>() as u64, 
            ty: wgpu::VertexFormat::Float64, 
            shader_location
        },
        "u8x2" => TypeToWGPU {
            offset: std::mem::size_of::<[u8; 2]>() as u64,
            ty: wgpu::VertexFormat::Uint8x2,
            shader_location
        },
        "u8x4" => TypeToWGPU {
            offset: std::mem::size_of::<[u8; 4]>() as u64,
            ty: wgpu::VertexFormat::Uint8x4,
            shader_location
        },
        "s8x2" => TypeToWGPU {
            offset: std::mem::size_of::<[i8; 2]>() as u64,
            ty: wgpu::VertexFormat::Sint8x2,
            shader_location
        },
        "s8x4" => TypeToWGPU {
            offset: std::mem::size_of::<[i8; 4]>() as u64,
            ty: wgpu::VertexFormat::Sint8x4,
            shader_location
        },
        "un8x2" => TypeToWGPU {
            offset: std::mem::size_of::<[u8; 2]>() as u64,
            ty: wgpu::VertexFormat::Unorm8x2,
            shader_location
        },
        "un8x4" => TypeToWGPU {
            offset: std::mem::size_of::<[u8; 4]>() as u64,
            ty: wgpu::VertexFormat::Unorm8x4,
            shader_location
        },
        "sn8x2" => TypeToWGPU {
            offset: std::mem::size_of::<[i8; 2]>() as u64,
            ty: wgpu::VertexFormat::Snorm8x2,
            shader_location
        },
        "sn8x4" => TypeToWGPU {
            offset: std::mem::size_of::<[i8; 4]>() as u64,
            ty: wgpu::VertexFormat::Snorm8x4,
            shader_location
        },
        "u16x2" => TypeToWGPU {
            offset: std::mem::size_of::<[u16; 2]>() as u64,
            ty: wgpu::VertexFormat::Uint16x2,
            shader_location
        },
        "u16x4" => TypeToWGPU {
            offset: std::mem::size_of::<[u16; 4]>() as u64,
            ty: wgpu::VertexFormat::Uint16x4,
            shader_location
        },
        "s16x2" => TypeToWGPU {
            offset: std::mem::size_of::<[i16; 2]>() as u64,
            ty: wgpu::VertexFormat::Sint16x2,
            shader_location
        },
        "s16x4" => TypeToWGPU {
            offset: std::mem::size_of::<[i16; 4]>() as u64,
            ty: wgpu::VertexFormat::Sint16x4,
            shader_location
        },
        "un16x2" => TypeToWGPU {
            offset: std::mem::size_of::<[u16; 2]>() as u64,
            ty: wgpu::VertexFormat::Unorm16x2,
            shader_location
        },
        "un16x4" => TypeToWGPU {
            offset: std::mem::size_of::<[u16; 4]>() as u64,
            ty: wgpu::VertexFormat::Unorm16x4,
            shader_location
        },
        "sn16x2" => TypeToWGPU {
            offset: std::mem::size_of::<[i16; 2]>() as u64,
            ty: wgpu::VertexFormat::Snorm16x2,
            shader_location
        },
        "sn16x4" => TypeToWGPU {
            offset: std::mem::size_of::<[i16; 4]>() as u64,
            ty: wgpu::VertexFormat::Snorm16x4,
            shader_location
        },
        "f16x2" => TypeToWGPU {
            offset: (F16 * 2) as u64,
            ty: wgpu::VertexFormat::Float16x2,
            shader_location
        },
        "f16x4" => TypeToWGPU {
            offset: (F16 * 4) as u64,
            ty: wgpu::VertexFormat::Float16x4,
            shader_location
        },
        "f32x2" => TypeToWGPU {
            offset: std::mem::size_of::<[f32; 2]>() as u64,
            ty: wgpu::VertexFormat::Float32x2,
            shader_location
        },
        "f32x3" => TypeToWGPU {
            offset: std::mem::size_of::<[f32; 3]>() as u64,
            ty: wgpu::VertexFormat::Float32x3,
            shader_location
        },
        "f32x4" => TypeToWGPU {
            offset: std::mem::size_of::<[f32; 4]>() as u64,
            ty: wgpu::VertexFormat::Float32x4,
            shader_location
        },
        "u32x2" => TypeToWGPU {
            offset: std::mem::size_of::<[u32; 2]>() as u64,
            ty: wgpu::VertexFormat::Uint32x2,
            shader_location
        },
        "u32x3" => TypeToWGPU {
            offset: std::mem::size_of::<[u32; 3]>() as u64,
            ty: wgpu::VertexFormat::Uint32x3,
            shader_location
        },
        "u32x4" => TypeToWGPU {
            offset: std::mem::size_of::<[u32; 4]>() as u64,
            ty: wgpu::VertexFormat::Uint32x4,
            shader_location
        },
        "s32x2" => TypeToWGPU {
            offset: std::mem::size_of::<[i32; 2]>() as u64,
            ty: wgpu::VertexFormat::Sint32x2,
            shader_location
        },
        "s32x3" => TypeToWGPU {
            offset: std::mem::size_of::<[i32; 3]>() as u64,
            ty: wgpu::VertexFormat::Sint32x3,
            shader_location
        },
        "s32x4" => TypeToWGPU {
            offset: std::mem::size_of::<[i32; 4]>() as u64,
            ty: wgpu::VertexFormat::Sint32x4,
            shader_location
        },
        "f64x2" => TypeToWGPU {
            offset: std::mem::size_of::<[f64; 2]>() as u64,
            ty: wgpu::VertexFormat::Float64x2,
            shader_location
        },
        "f64x3" => TypeToWGPU {
            offset: std::mem::size_of::<[f32; 3]>() as u64,
            ty: wgpu::VertexFormat::Float64x3,
            shader_location
        },
        "f64x4" => TypeToWGPU {
            offset: std::mem::size_of::<[f64; 4]>() as u64,
            ty: wgpu::VertexFormat::Float64x4,
            shader_location
        },
        _ => panic!("Type not supported")
    }
}
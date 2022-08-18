// MIT License

// Copyright (c) 2022 BrindilleDeLaForet

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! wrld is a easy, fast, and more secure way of writing buffer descriptor for wgpu renderpipeline
//! 
//! WARNING : Be aware that WRLD is still under development and should not be use on a "production ready" code.
use proc_macro::{TokenStream};

mod converter;
mod parser;
mod desc;

/// Desc is a proc derive macro that allow you to describe a structure as a description to pass to a renderpipeline.
///
/// Basically it will transform a structure like for example

/// ```
/// use wrld::Desc;

/// #[derive(Desc)]
/// struct Test {
///     #[f32x3(0)] position: Vector3
///     #[f32x4(1)] color: Vector4
/// }
/// ```
/// into
/// ```
/// impl Test {
///     pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
///         wgpu::VertexBufferLayout {
///             array_stride: std::mem::size_of::<Test>() as wgpu::BufferAddress,
///             step_mode: wgpu::VertexStepMode::Vertex,
///             attributes: &[
///                 wgpu::VertexAttribute {
///                     offset: 0u64,
///                     format: wgpu::VertexFormat::Float32x3,
///                     shader_location: 0u32,
///                 },
///                 wgpu::VertexAttribute {
///                     offset: 12u64,
///                     format: wgpu::VertexFormat::Float32x4,
///                     shader_location: 1u32,
///                 },
///             ],
///         }
///     }
/// }
/// ```
#[proc_macro_derive(Desc, attributes(
    u8x2, u8x4, s8x2, s8x4, un8x2, un8x4, sn8x2, sn8x4,
    u16x2, u16x4, s16x2, s16x4, un16x2, un16x4, sn16x2, sn16x4, f16x2, f16x4,
    f32, f32x2, f32x3, f32x4,
    u32, u32x2, u32x3, u32x4,
    s32, s32x2, s32x3, s32x4,
    f64, f64x2, f64x3, f64x4
))]
pub fn derive_wrsl_desc(item: TokenStream) -> TokenStream {
    use desc::derive_wrsl_desc;
    derive_wrsl_desc(item)
}


/*
#[macro_export]
macro_rules! Adapter {
    ($($win: expr), *) => {
        {
            let instance = wgpu::Instance::new(wgpu:::Backends::all());
            let surface = unsafe {instance.create_surface(&$win)};
            let adapter = instance.request_adapter({
                &wgpu::RequestAdapterOptions {
                    power_preference,
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false
                }
            }).await.unwrap();

            adapter
        }
    };
}
*/
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
mod macros;

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
    use macros::derive_wrsl_desc;
    derive_wrsl_desc(item)
}

/// A macro to handle any type of chaotic structure.
/// 
/// ## What is a chaotic structure ? And what are the structure different type ?
/// 
/// - Chaotic structure :
/// 
/// structure that have attribute but the fields are not ordered (basically put everywhere and not on the top of the structure)
/// 
/// for example
/// ```
/// #[derive(wgpu::Desc)]
/// struct Vertex {
///     some_data: String,
///     #[f32x2(0)] position: [f32; 2],
///     some_other_data: TypeDefinedByUser,
///     #[f32x4(1)] color: [f32; 4]
/// }
/// ```
/// 
/// is a chaotic structure because crates like bytemuck will interpret this structure like this.
/// 
/// ```
/// struct Vertex {
///     some_data: String,
///     position: [f32; 2]
/// }
/// ```
/// 
/// - Ordered structure 
/// 
/// is a structure that does put attribute field on the top of the structure.
/// 
/// for example
/// ```
/// #[derive(wgpu::Desc)]
/// struct Vertex {
///     #[f32x2(0)] position: [f32; 2],
///     #[f32x4(1)] color: [f32; 4],
///     some_data: String,
///     some_other_data: TypeDefinedByUser
/// }
/// ```
/// 
/// is a ordered structure and bytemuck will interpret this structure like this.
/// 
/// ```
/// struct Vertex {
///     position: [f32; 2],
///     color: [f32; 4]
/// }
/// ```
/// 
/// before that macro, structure like this (chaotic structure)
/// ```
/// #[derive(wgpu::Desc)]
/// struct Vertex {
///     uv: [f32; 2],
///     #[f32x2(0)] position: [f32; 2],
///     data: String,
///     #[f32x4(1)] color: [f32; 4]
/// }
/// ```
/// Where not very well handled by wrld, because bytemuck will not look for attribute data. 
/// Which create undefined behaviour on structure data and will not correspond to what we expect to receive.
/// 
/// A solution to that was to reorder structure data fields (ordered structure)
/// ```
/// #[derive(wgpu::Desc)]
/// struct Vertex {
///     #[f32x2(0)] position: [f32; 2],
///     #[f32x4(1)] color: [f32; 4],
///     
///     uv: [f32; 4],
///     data: String
/// }
/// ```
/// But now with BufferData this is not a problem anymore.
/// BufferData handle any type of chaotic structure so that does mean that this structure for example
/// ```
/// #[derive(wgpu::Desc)]
/// struct Vertex {
///     uv: [f32; 4],
///     #[f32x2(0)] position: [f32; 2],
///     data: String,
///     #[f32x4(1)] color: [f32; 4]
/// }
/// ```
/// Is handled via this macro and will have the result of what we expect it from.
/// 
/// ## How it's working ?
/// 
/// BufferData create a ordered structure from a chaotic structure. 
/// It take any array or variable and transform it to is correponding ordered structure
/// it also provide function and trait converter accordingly.
/// 
/// ## Example
/// 
/// Take this structure
/// ```
/// #[derive(wrld::Desc, wrld::BufferData)]
/// struct Vertex {
///     texture: SomeTextureType,
///     #[f32x3(0)] position: [f32; 3],
///     message: String,
///     #[f21x3(1)] scale: [f32; 3]
/// }
/// ```
/// 
/// This structure will result in this implementation
/// 
/// ```
/// #[repr(C)]
/// #[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
/// struct VertexBufferData {
///     position: [f32; 3],
///     scale: [f32; 3]
/// }
///
/// impl From<Vertex> for VertexBufferData {
///     fn from(other_data_from_ident_to_into: Vertex) -> Self {
///         Self {
///             position: other_data_from_ident_to_into.position,
///             scale: other_data_from_ident_to_into.scale
///         }
///     }
/// }
///
/// impl From<&'static Vertex> for VertexBufferData {
///     fn from(other_data_from_ident_to_into: &'static Vertex) -> Self {
///         Self {
///            position: other_data_from_ident_to_into.position,
///            scale: other_data_from_ident_to_into.scale
///         }
///     }
/// }
///
/// impl PartialEq<Vertex> for VertexBufferData {
///     fn eq(&self, other_ident_data_boolean_condition: &Vertex) -> bool {
///         position == other_ident_data_boolean_condition.position && scale: other_ident_data_boolean_condition.scale
///     }
/// }
///
/// impl FromIterator<Vertex> for Vec<VertexBufferData> {
///     fn from_iter<T: IntoIterator<Item = Vertex>>(iter: T) -> Self {
///         let mut vec_data_from_ident_from_iterator = Vec::new();
///
///         for c in iter {
///             vec_data_from_ident_from_iterator.push(c.into());
///         }
///
///         vec_data_from_ident_from_iterator
///     }
/// }
///
/// impl FromIterator<&'static Vertex> for Vec<VertexBufferData> {
///     fn from_iter<T: IntoIterator<Item = &'static Vertex>>(iter: T) -> Self {
///         let mut vec_data_from_ident_single_from_iterator : Vec<VertexBufferData> = Vec::new();
///
///         for c in iter {
///             vec_data_from_ident_single_from_iterator.push(c.into());
///         }
///
///         vec_data_from_ident_single_from_iterator
///     }
/// }
///
/// impl Vertex {
///     pub const fn const_into(self) -> VertexBufferData {
///         VertexBufferData {
///             position: self.position,
///             scale: self.scale,
///         }
///     }
///
///     pub fn mutate<'a>(other_data_from_ident_to_mutate: &'a Vec<VertexBufferData>) -> &'a [u8] {
///         bytemuck::cast_slice(other_data_from_ident_to_mutate.as_slice())
///     }
///
///     pub fn transmute(other_data_from_ident_to_transmute: &'static [Self]) -> Vec<VertexBufferData> {
///         other_data_from_ident_to_transmute.into_iter().collect::<Vec<VertexBufferData>>() 
///     }
/// }
/// ```
/// Also bytemuck is used for converting structure data to wgpu
/// 
/// ## How to use it ?
/// 
/// When you create any chaotic structure for wrld. Just put wrld::BufferData derive macro at the top
/// 
/// ```
/// #[derive(wrld::Desc, wrld::BufferData)]
/// struct Vertex {
///     texture: SomeTextureType,
///     #[f32x3(0)] position: [f32; 3],
///     message: String,
///     #[f21x3(1)] scale: [f32; 3]
/// }
/// ```
/// 
/// ### Single variable conversion.
/// 
/// If you only need to convert a single variable. You can do that.
/// 
/// ```
/// let data : VertexBufferData = Vertex { 
///     texture: SomeTextureType::new(), 
///     position: [0.0, 0.0, 0.0], 
///     message: String::from("something"),
///     scale: [1.0, 1.0, 1.0]
/// }.into()
/// ```
/// 
/// ### Array conversion
/// 
/// Array conversion is a little bit more complex. We can't use the .into() because rust will not allow that.
/// This is why you will need to transmute the const array first and then mutate it.
/// 
/// ```
/// const data : [Vertex] = [Vertex { 
///     texture: SomeTextureType::new(), 
///     position: [0.0, 0.0, 0.0], 
///     message: String::from("something"),
///     scale: [1.0, 1.0, 1.0]
/// }, Vertex { 
///     texture: SomeTextureType::new(), 
///     position: [0.0, 1.0, 0.0], 
///     message: String::from("something 2"),
///     scale: [1.0, 1.0, 1.0]
/// }]
/// 
/// fn main() {
///     let arr : &[u8] = Vertex::mutate(&Vertex::transmute(data));
///     
///     // With wgpu create_buffer_init
///     let device = wgpu::Device::new()
///     
///     let vertex_buffer = device.create_buffer_init(
///         &wgpu::utils::BufferInitDescriptor {
///             label: Some("Buffer init"),
///             contents: Vertex::mutate(&Vertex::transmute(data)),
///             usage: wgpu::BufferUsages::VERTEX
///     })
/// }
/// ```
/// 
/// ## Why you have created a another macro instead of putting it in wrld::Desc ?
/// 
/// 1. Prevent wrld to be too much invasive.
/// 2. BufferData is not always needed.
/// 3. BufferData is made to handle chaotic structure and not ordered one. (related to 2.)
/// 
/// There is also know problem about naming const variable the same as the quote generated code variable.
/// There is a simple workaround that is to name const variable all uppercase or just change name of the const variable.
/// However this problem only occurs on const variable
#[proc_macro_derive(BufferData)]
pub fn derive_wrsl_buffer_data(item: TokenStream) -> TokenStream {
    use macros::derive_wrsl_buffer_data;
    derive_wrsl_buffer_data(item)
}
# WRLD (Wgpu Rust Language Descriptor)

WRLD is a macro to create a description for rust structure for wgpu render pipeline.

WRLD description is based on Learn wgpu tutorial.

WARNING : Be aware that WRLD is still under development and should not be use on a "production ready" code.

## Getting started

Since WRLD is only a derive macro, WRLD does not come with any wgpu dependency in is Cargo.toml.

It allow WRLD to be more flexible with your need.

So in order to use WRLD, you will need to put 2 dependencies in your Cargo.toml.
```toml
wrld = "Your version"
wgpu = "Your version"
```
And that's it.

## Example

baisc rust structure.
```rust
struct Test {
    position: [f32; 2],
    color: [f32; 4]
}
```
With WRLD : 
```rust
use wrld::Desc;

#[derive(Desc)]
struct Test {
    #[f32x2(0)] position: [f32; 2],
    #[f32x4(1)] color: [f32; 4]
}
```
Will produce
```rust
impl Test {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Test>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0u64,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 0u32,
                },
                wgpu::VertexAttribute {
                    offset: 12u64,
                    format: wgpu::VertexFormat::Float32x4,
                    shader_location: 1u32,
                },
            ],
        }
    }
}
```
# WRLD (Wgpu Rust Language Descriptor)

WRLD is a set of derive macro to make easy, pleasent and more safe wgpu code.

WRLD description is based on Learn wgpu tutorial.

WARNING : Be aware that WRLD is still under development and should not be use on a "production ready" code.

## Motivation

The main reason of wrld was to create VertexBufferLayout with only one macro that support all of the type that wgpu support natively.

However the more i develop it. The more i see features that could be implemented that wgpu dosen't support out of the box.

## Getting started

To get started with wrld, just put wrld in your cargo.toml dependency
```toml
wrld = "Your version"
```
And that's it.

## Example

basic rust structure.
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
                    format: wgpu::VertexFormat::Float32x2,
                    shader_location: 0u32,
                },
                wgpu::VertexAttribute {
                    offset: 8u64,
                    format: wgpu::VertexFormat::Float32x4,
                    shader_location: 1u32,
                },
            ],
        }
    }
}
```

## Changelog

[Changelog](CHANGELOG.md)
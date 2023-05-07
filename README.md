# WRLD (Wgpu Rust Language Descriptor)

WRLD is a set of derive macro to make easy, pleasent and more safe wgpu code.

WRLD description is based on Learn wgpu tutorial.

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

#[repr(C)]
#[derive(Desc)]
struct Test {
    #[f32x2(0)] position: [f32; 2],
    #[f32x4(1)] color: [f32; 4]
}

// or
#[repr(transparent)]
#[derive(Desc)]
struct TestTransparent {
    #[f32x2(0)] position: f32,
    #[f32x4(1)] color: f32
}

```
Will produce
```rust
impl Test {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            // let size_f32 = size_of::<f32>() = 4
            // let f32x2 = size_f32 * 2 = 8;
            // let f32x4 = size_f32 * 4 = 16;
            // let array_stride = 8 + 16 = 24;

            array_stride: 24 as wgpu::BufferAddress // array_stride variable,
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

## Running test

WRLD has now some basic test, like basic desc structure testing, basic desc instance testing and buffer data testing. It's not totally complete but it will do for now. Feel free to add test if needed and do a pull request.

To run test

```bash
cargo test --test integration_test -- --nocapture
```

## Changelog

[Changelog](CHANGELOG.md)
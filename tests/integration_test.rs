use wrld::{Desc, DescInstance, BufferData};

#[repr(C)]
#[derive(Desc, Debug)]
struct Vertex {
    #[f32x2(0)] position: [f32; 2],
    #[f32x4(1)] color: [f32; 4]
}

#[repr(C)]
#[derive(DescInstance)]
struct VertexInstance {
    #[f32x2(0)] position: [f32; 2],
    #[f32x4(1)] color: [f32; 4]
}

#[repr(C)]
#[derive(Desc, BufferData, Debug)]
struct VertexDBD {
    #[f32x2(0)] position: [f32; 2],
    data: &'static str,
    #[f32x4(1)] color: [f32; 4]
}

#[repr(C)]
#[derive(DescInstance, BufferData)]
struct VertexDIBD {
    #[f32x2(0)] position: [f32; 2],
    data: &'static str,
    #[f32x4(1)] color: [f32; 4]
}

const DESC_DATA : [VertexDBD; 2] = [
    VertexDBD { position: [0.0, 0.0], data: "hello", color: [1.0, 0.5, 0.5, 1.0]},
    VertexDBD { position: [1.0, 0.0], data: "hello", color: [1.0, 0.5, 0.5, 1.0]}
];

const DESC_INSTANCE_DATA : [VertexDIBD; 2] = [
    VertexDIBD { position: [0.0, 0.0], data: "hello", color: [1.0, 0.5, 0.5, 1.0]},
    VertexDIBD { position: [1.0, 0.0], data: "hello", color: [1.0, 0.5, 0.5, 1.0]}
];

#[test]
fn desc() {
    println!("Vertex data struct \n{:?}\n", Vertex { position: [0.0, 0.0], color: [1.0, 0.5, 0.5, 1.0]});
    println!("Description of Vertex struct \n{:?}\n", Vertex::desc());
}

#[test]
fn desc_instance() {
    println!("Instance description of Vertex struct \n{:?}\n", VertexInstance::desc());
}

#[test]
fn desc_buffer_data() {
    println!("Vertex data struct array \n{:?}\n", DESC_DATA);
    println!("Result of mutate vertex desc buffer data : \n{:?}\n", mutate_vertex_d_b_d!(&DESC_DATA));
}

#[test]
fn desc_instance_buffer_data() {
    println!("Result of mutate vertex desc instance buffer data : \n{:?}\n", mutate_vertex_d_i_b_d!(&DESC_INSTANCE_DATA));
}
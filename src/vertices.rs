#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]

pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241 * 2., 0.49240386 * 2., 0.0],
        tex_coords: [0.4131759, 0.00759614],
    }, // A
    Vertex {
        position: [-0.49513406 * 2., 0.06958647 * 2., 0.0],
        tex_coords: [0.0048659444, 0.43041354],
    }, // B
    Vertex {
        position: [-0.21918549 * 2., -0.44939706 * 2., 0.0],
        tex_coords: [0.28081453, 0.949397],
    }, // C
    Vertex {
        position: [0.35966998 * 2., -0.3473291 * 2., 0.0],
        tex_coords: [0.85967, 0.84732914],
    }, // D
    Vertex {
        position: [0.44147372 * 2., 0.2347359 * 2., 0.0],
        tex_coords: [0.9414737, 0.2652641],
    }, // E
];

// pub const CUBE_VERTICES: &[Vertex] = &[
//     // south
//     Vertex {
//         position: [0., 0., 0.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [0., 1., 0.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [1., 1., 0.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [0., 0., 0.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [1., 1., 0.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [1., 0., 0.],
//         color: [1.0, 0.0, 0.0],
//     },
//     // east
//     Vertex {
//         position: [1., 0., 0.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [1., 1., 0.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [1., 1., 1.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [1., 0., 0.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [1., 1., 1.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [1., 0., 1.],
//         color: [1.0, 0.0, 0.0],
//     },
//     //. north
//     Vertex {
//         position: [1., 0., 1.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [1., 1., 1.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [0., 1., 1.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [1., 0., 1.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [0., 1., 1.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [0., 0., 1.],
//         color: [1.0, 0.0, 0.0],
//     },
//     // west
//     Vertex {
//         position: [0., 0., 1.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [0., 1., 1.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [0., 1., 0.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [0., 0., 1.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [0., 1., 0.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [0., 0., 0.],
//         color: [1.0, 0.0, 0.0],
//     },
//     // top
//     Vertex {
//         position: [0., 1., 0.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [0., 1., 1.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [1., 1., 1.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [0., 1., 0.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [1., 1., 1.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [1., 1., 0.],
//         color: [1.0, 0.0, 0.0],
//     },
//     // bottom
//     Vertex {
//         position: [1., 0., 1.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [0., 0., 1.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [0., 0., 0.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [1., 0., 1.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [0., 0., 0.],
//         color: [1.0, 0.0, 0.0],
//     },
//     Vertex {
//         position: [1., 0., 0.],
//         color: [1.0, 0.0, 0.0],
//     },
// ];

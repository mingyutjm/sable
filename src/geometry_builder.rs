use crate::vertex::Vertex;

pub(crate) fn build_sphere(radius: i32, level: i32) {
    
}

pub(crate) fn build_box(length: i32) -> ([Vertex; 8], [u16; 36]) {
    /* Box vertices
    *            5-------6              ^ y
    *           /|      /|              |   > z
    *		   1-------2 |              |  /
    *          | 4-----|-7              | /
    *          |/      |/               0-------> x 左手坐标系
    *          0-------3                组织三角面时注意法向量朝向
    */
    let vertices: [Vertex; 8] = [
        Vertex {
            position: [-1.0, -1.0, -1.0],
            color: [1.0, 1.0, 1.0, 1.0],
        },
        Vertex {
            position: [-1.0, 1.0, -1.0],
            color: [0.0, 0.0, 0.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0, -1.0],
            color: [1.0, 0.0, 0.0, 1.0],
        },
        Vertex {
            position: [1.0, -1.0, -1.0],
            color: [0.0, 0.5, 0.0, 1.0],
        },
        Vertex {
            position: [-1.0, -1.0, 1.0],
            color: [0.0, 0.0, 1.0, 1.0],
        },
        Vertex {
            position: [-1.0, 1.0, 1.0],
            color: [1.0, 1.0, 0.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0, 1.0],
            color: [0.0, 1.0, 1.0, 1.0],
        },
        Vertex {
            position: [1.0, -1.0, 1.0],
            color: [1.0, 0.0, 1.0, 1.0],
        },
    ];

    let indicies: [u16; 36] = [
        // Front
        0, 1, 2,
        0, 2, 3,

        // Back
        4, 6, 5,
        4, 7, 6,

        // Left
        0, 5, 1,
        0, 4, 5,

        // Right
        3, 2, 6,
        3, 6, 7,

        // Up
        1, 5, 6,
        1, 6, 2,

        // Down
        0, 3, 7,
        0, 7, 4
    ];

    (vertices, indicies)
}

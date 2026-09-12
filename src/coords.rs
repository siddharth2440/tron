use crate::state::Vertex;

pub const VERTICES: &[Vertex] = &[
            // TRIANGLE 1
            // TOP LEFT
            Vertex {
                vertex: [-0.5, 0.5, 0.0],
                texture_coords: [0.0, 0.0],
            },

            // TOP RIGHT
            Vertex {
                vertex: [0.5, 0.5, 0.0],
                texture_coords: [1.0, 0.0],
            },

            // BOTTOM LEFT
            Vertex {
                vertex: [-0.5, -0.5, 0.0],
                texture_coords: [0.0, 1.0],
            },


            // TRIANGLE 2
            // TOP RIGHT
            Vertex {
                vertex: [0.5, 0.5, 0.0],
                texture_coords: [1.0, 0.0],
            },

            // BOTTOM RIGHT
            Vertex {
                vertex: [0.5, -0.5, 0.0],
                texture_coords: [1.0, 1.0],
            },

            // BOTTOM LEFT
            Vertex {
                vertex: [-0.5, -0.5, 0.0],
                texture_coords: [0.0, 1.0],
            },

        ];



pub const CUBE_VERTICES: &[Vertex] = &[

    // =========================================================
    // FRONT FACE
    // =========================================================

    Vertex {
        vertex: [-0.5, -0.5, -0.5],
        texture_coords: [0.0, 0.0],
    },
    Vertex {
        vertex: [0.5, -0.5, -0.5],
        texture_coords: [1.0, 0.0],
    },
    Vertex {
        vertex: [0.5, 0.5, -0.5],
        texture_coords: [1.0, 1.0],
    },

    Vertex {
        vertex: [0.5, 0.5, -0.5],
        texture_coords: [1.0, 1.0],
    },
    Vertex {
        vertex: [-0.5, 0.5, -0.5],
        texture_coords: [0.0, 1.0],
    },
    Vertex {
        vertex: [-0.5, -0.5, -0.5],
        texture_coords: [0.0, 0.0],
    },


    // =========================================================
    // BACK FACE
    // =========================================================

    Vertex {
        vertex: [-0.5, -0.5, 0.5],
        texture_coords: [0.0, 0.0],
    },
    Vertex {
        vertex: [0.5, -0.5, 0.5],
        texture_coords: [1.0, 0.0],
    },
    Vertex {
        vertex: [0.5, 0.5, 0.5],
        texture_coords: [1.0, 1.0],
    },

    Vertex {
        vertex: [0.5, 0.5, 0.5],
        texture_coords: [1.0, 1.0],
    },
    Vertex {
        vertex: [-0.5, 0.5, 0.5],
        texture_coords: [0.0, 1.0],
    },
    Vertex {
        vertex: [-0.5, -0.5, 0.5],
        texture_coords: [0.0, 0.0],
    },


    // =========================================================
    // LEFT FACE
    // =========================================================

    Vertex {
        vertex: [-0.5, 0.5, 0.5],
        texture_coords: [1.0, 0.0],
    },
    Vertex {
        vertex: [-0.5, 0.5, -0.5],
        texture_coords: [1.0, 1.0],
    },
    Vertex {
        vertex: [-0.5, -0.5, -0.5],
        texture_coords: [0.0, 1.0],
    },

    Vertex {
        vertex: [-0.5, -0.5, -0.5],
        texture_coords: [0.0, 1.0],
    },
    Vertex {
        vertex: [-0.5, -0.5, 0.5],
        texture_coords: [0.0, 0.0],
    },
    Vertex {
        vertex: [-0.5, 0.5, 0.5],
        texture_coords: [1.0, 0.0],
    },


    // =========================================================
    // RIGHT FACE
    // =========================================================

    Vertex {
        vertex: [0.5, 0.5, 0.5],
        texture_coords: [1.0, 0.0],
    },
    Vertex {
        vertex: [0.5, 0.5, -0.5],
        texture_coords: [1.0, 1.0],
    },
    Vertex {
        vertex: [0.5, -0.5, -0.5],
        texture_coords: [0.0, 1.0],
    },

    Vertex {
        vertex: [0.5, -0.5, -0.5],
        texture_coords: [0.0, 1.0],
    },
    Vertex {
        vertex: [0.5, -0.5, 0.5],
        texture_coords: [0.0, 0.0],
    },
    Vertex {
        vertex: [0.5, 0.5, 0.5],
        texture_coords: [1.0, 0.0],
    },


    // =========================================================
    // BOTTOM FACE
    // =========================================================

    Vertex {
        vertex: [-0.5, -0.5, -0.5],
        texture_coords: [0.0, 1.0],
    },
    Vertex {
        vertex: [0.5, -0.5, -0.5],
        texture_coords: [1.0, 1.0],
    },
    Vertex {
        vertex: [0.5, -0.5, 0.5],
        texture_coords: [1.0, 0.0],
    },

    Vertex {
        vertex: [0.5, -0.5, 0.5],
        texture_coords: [1.0, 0.0],
    },
    Vertex {
        vertex: [-0.5, -0.5, 0.5],
        texture_coords: [0.0, 0.0],
    },
    Vertex {
        vertex: [-0.5, -0.5, -0.5],
        texture_coords: [0.0, 1.0],
    },


    // =========================================================
    // TOP FACE
    // =========================================================

    Vertex {
        vertex: [-0.5, 0.5, -0.5],
        texture_coords: [0.0, 1.0],
    },
    Vertex {
        vertex: [0.5, 0.5, -0.5],
        texture_coords: [1.0, 1.0],
    },
    Vertex {
        vertex: [0.5, 0.5, 0.5],
        texture_coords: [1.0, 0.0],
    },

    Vertex {
        vertex: [0.5, 0.5, 0.5],
        texture_coords: [1.0, 0.0],
    },
    Vertex {
        vertex: [-0.5, 0.5, 0.5],
        texture_coords: [0.0, 0.0],
    },
    Vertex {
        vertex: [-0.5, 0.5, -0.5],
        texture_coords: [0.0, 1.0],
    },
];
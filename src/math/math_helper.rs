use vek::Mat4;

pub fn mat4_as_array(matrix: Mat4<f32>) -> [[f32; 4]; 4] {
    let cols = matrix.cols;
    [
        [cols[0].x, cols[0].y, cols[0].z, cols[0].w],
        [cols[1].x, cols[1].y, cols[1].z, cols[1].w],
        [cols[2].x, cols[2].y, cols[2].z, cols[2].w],
        [cols[3].x, cols[3].y, cols[3].z, cols[3].w],
    ]
}

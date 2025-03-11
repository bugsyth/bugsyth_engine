use std::fmt::Debug;
use vek::Mat4;

#[derive(Debug, Clone)]
pub struct Skeleton {
    pub joints: Vec<SkeletonJoint>,
    pub bone_matrices: Vec<[[f32; 4]; 4]>,
    pub inverse_bind_matrices: Vec<[[f32; 4]; 4]>,
}

#[derive(Debug, Clone)]
pub struct SkeletonJoint {
    pub inverse_bind_matrix: Mat4<f32>,
    pub indices: JointIndices,
}

#[derive(Clone)]
pub struct JointIndices {
    pub index: usize,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl Debug for JointIndices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nJointIndices \n  index: {}\n  parent: {:?}\n  children: {:?}\n",
            self.index, self.parent, self.children
        )
    }
}

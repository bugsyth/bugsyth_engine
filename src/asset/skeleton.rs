use std::fmt::Debug;
use vek::Mat4;

#[derive(Debug, Clone)]
pub struct Skeleton {
    pub joint_index_map: Vec<usize>,
    pub joints: Vec<SkeletonJoint>,
    pub origin_bone_matrices: Vec<Mat4<f32>>,
    // In array form since they are sent to gpu
    pub bone_matrices: Vec<[[f32; 4]; 4]>,
    pub inverse_bind_matrices: Vec<[[f32; 4]; 4]>,
}

impl Skeleton {
    pub fn update_bone_matrices(&mut self, animated_transforms: &[Mat4<f32>]) {
        for joint in &self.joints {
            self.bone_matrices[joint.indices.index] = (animated_transforms[joint.indices.index]
                * self.origin_bone_matrices[joint.indices.index])
                .into_col_arrays();
        }
    }
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

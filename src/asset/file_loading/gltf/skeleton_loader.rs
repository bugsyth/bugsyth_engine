// This stuff hurts my head

use crate::asset::skeleton::{JointIndices, Skeleton, SkeletonJoint};
use gltf::{buffer::Data, Node, Skin};
use vek::Mat4;

pub fn build_skeleton(skin: Skin<'_>, buffers: &[Data]) -> Skeleton {
    let mut buffer = Vec::new();
    build_indices(
        &mut buffer,
        &skin.joints().collect::<Vec<Node<'_>>>()[0],
        None,
    );

    let inverse_bind_matrices: Vec<[[f32; 4]; 4]> = skin
        .reader(|buffer| Some(&buffers[buffer.index()]))
        .read_inverse_bind_matrices()
        .unwrap()
        .collect();

    let joints: Vec<SkeletonJoint> = (0..buffer.len())
        .map(|i| SkeletonJoint {
            inverse_bind_matrix: Mat4::from_col_arrays(inverse_bind_matrices[i]),
            indices: buffer[i].clone(),
        })
        .collect();

    let bone_matrices: Vec<[[f32; 4]; 4]> = joints
        .iter()
        .map(|joint| {
            skin.joints().collect::<Vec<Node<'_>>>()[joint.indices.index]
                .transform()
                .matrix()
        })
        .collect();

    // was used to get correct bone transforms but doesnt do that??
    // let root = joints
    //     .iter()
    //     .filter(|joint| joint.indices.parent.is_none())
    //     .collect::<Vec<&SkeletonJoint>>()[0];

    // calculate_bone_transforms(&mut bone_matrices, root, &joints);

    Skeleton {
        joints,
        bone_matrices,
        inverse_bind_matrices,
    }
}

// fn calculate_bone_transforms(
//     matrices: &mut Vec<[[f32; 4]; 4]>,
//     joint: &SkeletonJoint,
//     joints: &Vec<SkeletonJoint>,
// ) {
//     for child in &joint.indices.children {
//         let mat1 = Mat4::from_col_arrays(matrices[joint.indices.index]);
//         let mat2 = Mat4::from_col_arrays(matrices[*child]);
//         let bind_matrix = joints[joint.indices.index].inverse_bind_matrix;
//         matrices[*child] = (mat1 * mat2).into_col_arrays();
//         calculate_bone_transforms(matrices, &joints[*child], joints);
//     }
// }

fn build_indices(buffer: &mut Vec<JointIndices>, node: &Node<'_>, parent: Option<usize>) {
    let mut joint_indices = JointIndices {
        index: node.index(),
        parent,
        children: Vec::new(),
    };
    for child in node.children() {
        joint_indices.children.push(child.index());
        build_indices(buffer, &child, Some(node.index()));
    }
    buffer.push(joint_indices);
}

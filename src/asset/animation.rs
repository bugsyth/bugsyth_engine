pub mod keyframe;
use gltf::{
    animation::{Property, Sampler, Target},
    buffer::Data,
};
use keyframe::KeyFrameData;
use vek::{Mat4, Quaternion, Vec3, Vec4};

use crate::asset::skeleton::Skeleton;

pub struct Animations {
    animations: Vec<Animation>,
}

impl Animations {
    pub fn new(animations: Vec<Animation>) -> Self {
        Self { animations }
    }

    pub fn get_animatied_transforms(&self, skeleton: &Skeleton, time: f32) -> Vec<Mat4<f32>> {
        let mut animated_transforms: Vec<Mat4<f32>> = vec![Mat4::identity(); skeleton.joints.len()];

        let mut mats = Vec::new();
        for (i, animation) in self.animations.iter().enumerate() {
            let mut node_index = 0;
            for channel in &animation.channels {
                for (i, index) in skeleton.joint_index_map.iter().enumerate() {
                    if *index == channel.target_node_index {
                        node_index = i;
                    }
                }

                let keyframes = &channel.keyframes;

                match channel.target_property {
                    Property::Translation => {
                        let interpolated = KeyFrameData::interpolate_keyframe(
                            &keyframes.times,
                            &keyframes.values,
                            time,
                            3,
                        );
                        let trans: Mat4<f32> =
                            Mat4::translation_3d(Vec3::from_slice(&interpolated));
                        mats.push(trans);
                    }
                    Property::Rotation => {
                        let interpolated = KeyFrameData::interpolate_keyframe(
                            &keyframes.times,
                            &keyframes.values,
                            time,
                            4,
                        );
                        let quat: Mat4<f32> =
                            Mat4::from(Quaternion::from_vec4(Vec4::from_slice(&interpolated)));
                        mats.push(quat);
                    }
                    Property::Scale => {
                        let interpolated = KeyFrameData::interpolate_keyframe(
                            &keyframes.times,
                            &keyframes.values,
                            time,
                            3,
                        );
                        let scale: Mat4<f32> = Mat4::scaling_3d(Vec3::from_slice(&interpolated));
                        mats.push(scale);
                    }
                    _ => println!(
                        "Property type {:?} isn't supported",
                        channel.target_property
                    ),
                }
            }
            if (i + 1) % 3 == 0 {
                animated_transforms[node_index] = mats[0] * mats[1] * mats[2];
                mats = Vec::new();
            }
        }

        // for mat in &animated_transforms {
        //     print_matrix(mat);
        // }
        animated_transforms
    }
}

pub struct Animation {
    channels: Vec<AnimationData>,
}

impl Animation {
    pub fn new(channels: Vec<AnimationData>) -> Self {
        Self { channels }
    }
}

pub struct AnimationData {
    target_node_index: usize,
    target_property: Property,
    keyframes: KeyFrameData,
}

impl AnimationData {
    pub fn new(target: &Target<'_>, buffers: &[Data], sampler: &Sampler) -> Self {
        Self {
            target_node_index: target.node().index(),
            target_property: target.property(),
            keyframes: KeyFrameData::extract_keyframes(sampler.input(), sampler.output(), buffers),
        }
    }
}

fn print_matrix(matrix: &Mat4<f32>) {
    println!("[");
    for col in matrix.into_col_arrays() {
        println!("  {:?}", col);
    }
    println!("]");
}

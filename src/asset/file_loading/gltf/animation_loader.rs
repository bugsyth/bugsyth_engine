use gltf::{buffer::Data, iter::Animations};

use crate::asset::animation;

pub fn get_animation_data(animations: Animations, buffers: &[Data]) -> animation::Animations {
    let mut animations_data = Vec::new();
    for animation in animations {
        for channel in animation.channels() {
            let sampler = channel.sampler();
            let target = channel.target();

            animations_data.push(animation::Animation::new(vec![
                animation::AnimationData::new(&target, buffers, &sampler),
            ]));
        }
        break;
    }
    animation::Animations::new(animations_data)
}

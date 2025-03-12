use gltf::{buffer::Data, Accessor};

pub struct KeyFrameData {
    pub times: Vec<f32>,
    pub values: Vec<f32>,
}

impl KeyFrameData {
    pub fn extract_keyframes(
        times_accessor: Accessor,
        values_accessor: Accessor,
        buffers: &[Data],
    ) -> Self {
        let times = read_accessor_data(&times_accessor, buffers);
        let values = read_accessor_data(&values_accessor, buffers);
        Self { times, values }
    }

    pub fn interpolate_keyframe(
        times: &Vec<f32>,
        values: &Vec<f32>,
        time: f32,
        dimensions: usize,
    ) -> Vec<f32> {
        if times.is_empty() || values.is_empty() {
            return vec![0.0; dimensions];
        }

        let last_index = times.len() - 1;

        if time <= times[0] {
            return values[0..dimensions].to_vec();
        }
        if time >= times[last_index] {
            return values[last_index * dimensions..(last_index + 1) * dimensions].to_vec();
        }

        let index = times.iter().position(|&t| t > time).unwrap_or(last_index);

        let t0 = times[index - 1];
        let t1 = times[index];
        let alpha = (time - t0) / (t1 - t0);

        let mut interpolated = Vec::with_capacity(dimensions);
        for i in 0..dimensions {
            let v0 = values[(index - 1) * dimensions + i];
            let v1 = values[index * dimensions + i];
            interpolated.push(v0 + alpha * (v1 - v0));
        }

        interpolated
    }
}

fn read_accessor_data(accessor: &Accessor, buffers: &[Data]) -> Vec<f32> {
    let view = accessor.view().expect("No buffer view");
    let buffer_index = view.buffer().index();
    let buffer_data = &buffers[buffer_index];

    // Calculate byte offset and length
    let offset = view.offset() + accessor.offset();
    let length = accessor.count() * accessor.dimensions().multiplicity();
    let start = offset;
    let end = offset + length * 4; // Assuming f32 data

    let bytes = &buffer_data[start..end];
    let mut floats = Vec::new();

    for chunk in bytes.chunks_exact(4) {
        floats.push(f32::from_le_bytes(chunk.try_into().unwrap()));
    }

    floats
}

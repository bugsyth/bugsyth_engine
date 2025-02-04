use glium::{index::IndicesSource, vertex::MultiVerticesSource, DrawParameters};

pub trait Drawable {
    fn get_vbo(&self) -> impl MultiVerticesSource;
    fn get_ibo(&self) -> impl Into<IndicesSource>;
    fn get_program(&self) -> String;
    fn get_draw_params(&self) -> DrawParameters;
}

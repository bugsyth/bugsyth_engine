use glium::vertex::VertexBufferAny;

pub struct Model {
    vbo: VertexBufferAny,
}

impl Model {
    pub(crate) fn new(vbo: VertexBufferAny) -> Self
where {
        Self { vbo }
    }

    pub fn get_vbo(&self) -> &VertexBufferAny {
        &self.vbo
    }
}

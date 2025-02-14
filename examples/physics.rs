use bugsyth_engine::{
    physics::{
        physics_object::{ColliderType, PhysicsObject},
        shapes::{aabb::AABB, Shape},
    },
    prelude::*,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

fn main() -> EngineResult {
    let (event_loop, mut ctx) = init("physics", (960, 720))?;
    ctx.new_program(
        "3d",
        "
    in vec3 position;
    in vec3 color;

    out vec3 v_color;

    uniform mat4 persp;
    uniform mat4 view;
    uniform mat4 matrix;

    void main() {
        v_color = color;
        mat4 model_view = view * matrix;
        gl_Position = persp * model_view * vec4(position, 1.0);
    }
    ",
        "
    in vec3 v_color;

    out vec4 color;

    void main() {
        color = vec4(v_color, 1.0);
    }
    ",
        None,
    )?;
    let mut world = World::new();
    let rect = Rect::new(
        &ctx,
        Vec3::new(2.0, 3.0, 2.0),
        1.0,
        1.0,
        1.0,
        ColliderType::Dynamic,
    );
    let floor = Rect::new(&ctx, Vec3::zero(), 20.0, 2.0, 20.0, ColliderType::Static);
    let rect2 = Rect::new(
        &ctx,
        Vec3::new(0.0, 2.0, 0.0),
        1.0,
        1.0,
        1.0,
        ColliderType::Static,
    );
    let rect3 = Rect::new(
        &ctx,
        Vec3::new(5.0, 2.0, 5.0),
        1.0,
        1.0,
        1.0,
        ColliderType::Dynamic,
    );
    world.add_object(&rect.physics_object);
    world.add_object(&rect2.physics_object);
    world.add_object(&rect3.physics_object);
    world.add_object(&floor.physics_object);
    let game = Game {
        world,
        rects: vec![rect, rect2, rect3, floor],
    };
    run(game, event_loop, ctx)?;
    Ok(())
}

struct Game {
    world: World,
    rects: Vec<Rect<'static>>,
}

impl GameState for Game {
    fn update(&mut self, ctx: &mut Context) {
        bugsyth_engine::context::camera::CameraState::free_cam(ctx.dt, ctx, 3.0, 1.0);

        let speed = 1.0 * ctx.dt;
        let mut translation = Vec3::new(0.0, -speed, 0.0);

        // Best movement controls ever
        if ctx.input.is_key_pressed(KeyCode::KeyI) {
            translation.x += speed;
        }
        if ctx.input.is_key_pressed(KeyCode::KeyJ) {
            translation.z += speed;
        }
        if ctx.input.is_key_pressed(KeyCode::KeyK) {
            translation.x -= speed;
        }
        if ctx.input.is_key_pressed(KeyCode::KeyL) {
            translation.z -= speed;
        }

        self.rects[0]
            .physics_object
            .borrow_mut()
            .shape
            .translate(translation);

        self.world.update();
    }
    fn draw(&mut self, ctx: &mut Context, renderer: &mut impl Renderer) {
        renderer.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        for rect in &self.rects {
            let pos = rect.physics_object.borrow().shape.get_position();
            renderer
                .draw(
                    ctx,
                    rect,
                    &uniform! {
                        persp: ctx.camera.get_perspective(),
                        view: ctx.camera.get_view(),
                        matrix: [
                            [1.0, 0.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [pos.x, pos.y, pos.z, 1.0],
                        ],
                    },
                )
                .unwrap();
        }
    }
}

struct Rect<'a> {
    vbo: VertexBuffer<Vertex>,
    ibo: NoIndices,
    draw_params: DrawParameters<'a>,

    physics_object: Rc<RefCell<PhysicsObject>>,
}

impl<'a> Rect<'a> {
    fn new(
        ctx: &Context,
        pos: Vec3<f32>,
        width: f32,
        height: f32,
        length: f32,
        collider_type: ColliderType,
    ) -> Self {
        let vbo = VertexBuffer::new(
            &ctx.display,
            &[
                // Top Face (+Y)
                Vertex {
                    position: [0.0, height, 0.0],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [width, height, 0.0],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [width, height, length],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [0.0, height, length],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [0.0, height, 0.0],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [width, height, length],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                // Bottom Face (-Y)
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [width, 0.0, 0.0],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [width, 0.0, length],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [0.0, 0.0, length],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [width, 0.0, length],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                // Front Face (+Z)
                Vertex {
                    position: [0.0, 0.0, length],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [width, 0.0, length],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [width, height, length],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [0.0, height, length],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [0.0, 0.0, length],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [width, height, length],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                // Back Face (-Z)
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [width, 0.0, 0.0],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [width, height, 0.0],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [0.0, height, 0.0],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [width, height, 0.0],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                // Right Face (+X)
                Vertex {
                    position: [width, 0.0, 0.0],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [width, 0.0, length],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [width, height, length],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [width, height, 0.0],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [width, 0.0, 0.0],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [width, height, length],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                // Left Face (-X)
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [0.0, 0.0, length],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [0.0, height, length],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [0.0, height, 0.0],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [0.0, 0.0, 0.0],
                    color: [rand::random(), rand::random(), rand::random()],
                },
                Vertex {
                    position: [0.0, height, length],
                    color: [rand::random(), rand::random(), rand::random()],
                },
            ],
        )
        .unwrap();

        let physics_object = Rc::new(RefCell::new(PhysicsObject::new(
            Shape::AABB(AABB::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(width, height, length),
            )),
            collider_type,
        )));
        physics_object.borrow_mut().shape.translate(pos);
        Self {
            vbo,
            ibo: NoIndices(PrimitiveType::TrianglesList),
            draw_params: DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                ..Default::default()
            },

            physics_object,
        }
    }
}

impl<'a> Drawable for Rect<'a> {
    fn get_vbo(&self) -> impl MultiVerticesSource {
        &self.vbo
    }
    fn get_ibo(&self) -> impl Into<IndicesSource> {
        &self.ibo
    }
    fn get_program(&self) -> String {
        "3d".to_string()
    }
    fn get_draw_params(&self) -> DrawParameters {
        self.draw_params.clone()
    }
}

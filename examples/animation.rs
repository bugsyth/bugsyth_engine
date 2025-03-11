use bugsyth_engine::{asset::Asset, prelude::*};
use glium::buffer::{Buffer, BufferMode, BufferType};

fn main() -> EngineResult {
    let (event_loop, mut ctx) = init("gltf", (960, 720))?;
    // Normals not set right
    ctx.new_program(
        "skeleton",
        "
    #version 430

in vec3 position;
in vec3 normal;
in vec2 tex_coords;
in vec4 color;
in ivec4 joint_set;
in vec4 weights;

out vec2 v_tex_coords;
out vec4 v_color;

uniform mat4 persp;
uniform mat4 view;

const int MAX_BONES = 100;
const int MAX_BONE_INFLUENCE = 4;

layout(std430, binding = 1) buffer BoneMatrices {
    mat4 bone_matrices[];
};
layout(std430, binding = 0) buffer InverseBoneMatrices {
    mat4 inverse_bone_matrices[];
};

void main() {
    vec4 total_position = vec4(0.0);
    vec3 total_normal = vec3(0.0);

    for (int i = 0; i < MAX_BONE_INFLUENCE; i++) {
        if (joint_set[i] == -1) {
            continue;
        }
        if (joint_set[i] >= MAX_BONES) {
            total_position = vec4(position, 1.0);
            break;
        }

        mat4 inverse_bind_matrix = inverse_bone_matrices[joint_set[i]];
        mat4 bone_transform = bone_matrices[joint_set[i]];

        vec4 transformed_position = bone_transform * inverse_bind_matrix * vec4(position, 1.0);

        total_position += transformed_position * weights[i];

        mat3 normal_transform = mat3(bone_transform);
        total_normal += normal_transform * normal * weights[i];
    }

    v_tex_coords = tex_coords;
    v_color = color;

    gl_Position = persp * view * total_position;
}

    ",
        "
    #version 140

    in vec2 v_tex_coords;
    in vec4 v_color;

    out vec4 color;

    uniform sampler2D u_tex;

    void main() {
        color = vec4(v_color * texture(u_tex, v_tex_coords));
    }
    ",
        None,
    )
    .unwrap();
    let assets = asset::load_gltf(&ctx.display, "resources/suzanne.glb")?;
    let mut objs = Vec::new();
    for asset in assets {
        objs.push(Obj {
            asset,
            ibo: NoIndices(PrimitiveType::TrianglesList),
            draw_params: DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                backface_culling: BackfaceCullingMode::CullClockwise,
                ..Default::default()
            },
        });
    }
    let game = Game {
        objs,
        tex: Texture::new(&ctx, "resources/lil_man/lil_man.png")?,
    };
    run(game, event_loop, ctx)?;
    Ok(())
}

struct Game {
    objs: Vec<Obj<'static>>,
    tex: Texture,
}

impl GameState for Game {
    fn update(&mut self, ctx: &mut Context) {
        bugsyth_engine::context::camera::CameraState::free_cam(ctx.dt, ctx, 1.0, 1.0);
    }
    fn draw(&mut self, ctx: &mut Context, renderer: &mut impl Renderer) {
        renderer.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        for obj in &self.objs {
            let ssbo: Buffer<[[[f32; 4]; 4]]> = Buffer::new(
                &ctx.display,
                obj.asset.skeleton.clone().unwrap().bone_matrices.as_slice(),
                BufferType::ShaderStorageBuffer,
                BufferMode::Default,
            )
            .unwrap();

            let ssbo2 = Buffer::new(
                &ctx.display,
                obj.asset
                    .skeleton
                    .clone()
                    .unwrap()
                    .inverse_bind_matrices
                    .as_slice(),
                BufferType::ShaderStorageBuffer,
                BufferMode::Default,
            )
            .unwrap();

            renderer
                .draw(
                    ctx,
                    obj,
                    &uniform! {
                        persp: ctx.camera.get_perspective(),
                        view: ctx.camera.get_view(),
                        BoneMatrices: &ssbo,
                        InverseBoneMatrices: &ssbo2,
                        u_tex: self.tex.get_texture_no_filtering(),
                    },
                )
                .unwrap();
        }
    }
}

struct Obj<'a> {
    asset: Asset,
    ibo: NoIndices,
    draw_params: DrawParameters<'a>,
}

impl<'a> Drawable for Obj<'a> {
    fn get_vbo(&self) -> impl MultiVerticesSource {
        self.asset.model.get_vbo()
    }
    fn get_ibo(&self) -> impl Into<IndicesSource> {
        self.ibo
    }
    fn get_program(&self) -> String {
        "skeleton".to_string()
    }
    fn get_draw_params(&self) -> DrawParameters {
        self.draw_params.clone()
    }
}

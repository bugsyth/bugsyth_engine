use bugsyth_engine::prelude::*;

fn main() -> EngineResult {
    let (event_loop, ctx) = init("gltf", (960, 720))?;
    let _ = asset::load_gltf(&ctx.display, "resources/lil_man/lil_man.glb")?;
    let game = Game {};
    run(game, event_loop, ctx)?;
    Ok(())
}

struct Game {}

impl GameState for Game {}

use ray_tracing::renderer::Renderer;

fn main() {
    let mut renderer = Renderer::new((1080.0, 1080.0));

    //renderer.setup_scene();
    renderer.start();
}
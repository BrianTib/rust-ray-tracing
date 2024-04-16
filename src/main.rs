use ray_tracing::renderer::Renderer;

fn main() {
    let mut renderer = Renderer::new((1280, 1280));

    renderer.setup_scene();
    renderer.start();
}
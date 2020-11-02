use crate::scene::scene::Scene;

mod camera;
mod objects;
mod utils;
mod vec;
mod scene;

fn main() {
    let scene = Scene::setup_complex_scene();
    scene.multithreadet_rendering()
}

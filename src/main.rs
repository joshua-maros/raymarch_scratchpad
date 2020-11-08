use raymarch_scratchpad::*;

fn main() {
    let mut scene = Scene::new();
    scene.set_sky_color((0.3, 0.7, 0.9));
    scene.add_object(Sphere {
        origin: (0, 0, 10).into(),
        radius: 1.0,
    });
    scene.add_object(Cube {
        origin: (0, 2, 10).into(),
        size: (10, 0.1, 10).into(),
    });
    scene.add_light(DirectionalLight {
        direction: Vec3::from((1, 1, 0.5)).normalized(),
        size: 0.0,
        color: 1.into(),
    });

    let renderer = Renderer {
        size: 100,
        samples: 32,
        num_bounces: 5,
        camera_size: 0.3,
        exposure: 0.7,
        pixel_size: 0.5,
    };
    renderer.render(&scene, "test.png");
}

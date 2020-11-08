use raymarch_scratchpad::*;

fn main() {
    let mut scene = Scene::new();
    scene.set_sky_color((0.3, 0.7, 0.9));
    scene.add_object(sphere().scaled(2).translated((0, 0, 20)));
    scene.add_object(cube((20, 0.1, 20)).translated((0, 2, 20)));
    scene.add_light(DirectionalLight {
        direction: Vec3::from((1, 1, 0.5)).normalized(),
        percent_size: 0.5,
        color: 1.into(),
    });
    scene.add_light(PointLight {
        origin: (-2, -1, 20).into(),
        radius: 0.5,
        color: Vec3::from((1, 1, 0)) * 10,
    });

    let renderer = Renderer {
        size: 100,
        samples: 256,
        num_bounces: 5,
        camera_size: 0.3,
        pixel_size: 0.5,
        post_process: (AdjustExposure(1.1), AcesFilmicCurve)
    };
    renderer.render(&scene, "test.png");
}

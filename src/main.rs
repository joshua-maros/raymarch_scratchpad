use raymarch_scratchpad::*;

fn main() {
    let mut scene = Scene::new();
    scene.set_sky_color((0.3, 0.7, 0.9));

    let clay = BasicMaterial {
        ..Default::default()
    };
    let pink = BasicMaterial {
        base_color: (1, 0.1, 1).into(),
        ..Default::default()
    };
    scene.add_object(sphere(pink.clone()).scaled(2).translated((0, 0, 20)));
    scene.add_object(cube(clay.clone(), (20, 0.1, 20)).translated((0, 2, 20)));
    scene.add_light(DirectionalLight {
        direction: Vec3::from((1, 1, 0.5)).normalized(),
        percent_size: 0.5,
        color: 1.into(),
    });

    let renderer = Renderer {
        size: 100,
        samples: 32,
        num_bounces: 20,
        camera_size: 0.3,
        pixel_size: 0.667,
        post_process: (AdjustExposure(1.5), AcesFilmicCurve),
    };
    renderer.render(&scene, "test.png");
}

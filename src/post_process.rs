use crate::Vec3;

pub trait PostProcessor {
    fn process_pixel(&self, pixel: Vec3) -> Vec3;
}

#[derive(Clone, Debug)]
pub struct AdjustExposure(pub f32);

impl PostProcessor for AdjustExposure {
    fn process_pixel(&self, pixel: Vec3) -> Vec3 {
        pixel * self.0
    }
}

#[derive(Clone, Debug)]
/// https://knarkowicz.wordpress.com/2016/01/06/aces-filmic-tone-mapping-curve/
/// makes your colors look nicer
pub struct AcesFilmicCurve;

impl PostProcessor for AcesFilmicCurve {
    fn process_pixel(&self, pixel: Vec3) -> Vec3 {
        const A: f32 = 2.51;
        const B: f32 = 0.03;
        const C: f32 = 2.43;
        const D: f32 = 0.59;
        const E: f32 = 0.14;
        ((pixel * (pixel * A + B)) / (pixel * (pixel * C + D) + E)).saturated()
    }
}

impl PostProcessor for () {
    fn process_pixel(&self, pixel: Vec3) -> Vec3 {
        pixel
    }
}

impl<A: PostProcessor, B: PostProcessor> PostProcessor for (A, B) {
    fn process_pixel(&self, pixel: Vec3) -> Vec3 {
        self.1.process_pixel(self.0.process_pixel(pixel))
    }
}

impl<A: PostProcessor, B: PostProcessor, C: PostProcessor> PostProcessor for (A, B, C) {
    fn process_pixel(&self, pixel: Vec3) -> Vec3 {
        self.2
            .process_pixel(self.1.process_pixel(self.0.process_pixel(pixel)))
    }
}

impl<A: PostProcessor, B: PostProcessor, C: PostProcessor, D: PostProcessor> PostProcessor
    for (A, B, C, D)
{
    fn process_pixel(&self, pixel: Vec3) -> Vec3 {
        self.3.process_pixel(
            self.2
                .process_pixel(self.1.process_pixel(self.0.process_pixel(pixel))),
        )
    }
}

impl<A: PostProcessor, B: PostProcessor, C: PostProcessor, D: PostProcessor, E: PostProcessor>
    PostProcessor for (A, B, C, D, E)
{
    fn process_pixel(&self, pixel: Vec3) -> Vec3 {
        self.4.process_pixel(
            self.3.process_pixel(
                self.2
                    .process_pixel(self.1.process_pixel(self.0.process_pixel(pixel))),
            ),
        )
    }
}

impl<
        A: PostProcessor,
        B: PostProcessor,
        C: PostProcessor,
        D: PostProcessor,
        E: PostProcessor,
        F: PostProcessor,
    > PostProcessor for (A, B, C, D, E, F)
{
    fn process_pixel(&self, pixel: Vec3) -> Vec3 {
        self.5.process_pixel(
            self.4.process_pixel(
                self.3.process_pixel(
                    self.2
                        .process_pixel(self.1.process_pixel(self.0.process_pixel(pixel))),
                ),
            ),
        )
    }
}

impl<
        A: PostProcessor,
        B: PostProcessor,
        C: PostProcessor,
        D: PostProcessor,
        E: PostProcessor,
        F: PostProcessor,
        G: PostProcessor,
    > PostProcessor for (A, B, C, D, E, F, G)
{
    fn process_pixel(&self, pixel: Vec3) -> Vec3 {
        self.6.process_pixel(
            self.5.process_pixel(
                self.4.process_pixel(
                    self.3.process_pixel(
                        self.2
                            .process_pixel(self.1.process_pixel(self.0.process_pixel(pixel))),
                    ),
                ),
            ),
        )
    }
}

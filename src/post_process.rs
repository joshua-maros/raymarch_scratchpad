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
/// https://github.com/TheRealMJP/BakingLab/blob/master/BakingLab/ACES.hlsl
/// makes your colors look nicer
pub struct AcesFilmicCurve;

impl PostProcessor for AcesFilmicCurve {
    fn process_pixel(&self, pixel: Vec3) -> Vec3 {
        let pixel = Vec3::new(
            (pixel * (0.59719, 0.35458, 0.04823)).sum(),
            (pixel * (0.07600, 0.90834, 0.01566)).sum(),
            (pixel * (0.02840, 0.13383, 0.83777)).sum(),
        );
        let a = pixel * (pixel + 0.0245786) + 0.000090537;
        let b = pixel * (pixel * 0.983729 + 0.4329510) + 0.238081;
        let pixel = a / b;
        let pixel = Vec3::new(
            (pixel * (1.60475, -0.53108, -0.07367)).sum(),
            (pixel * (-0.10208, 1.10813, -0.00605)).sum(),
            (pixel * (-0.00327, -0.07276, 1.07602)).sum(),
        );
        pixel.saturated()
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

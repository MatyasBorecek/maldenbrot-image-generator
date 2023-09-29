pub struct MandelbrotConfig {
    pub width: u32,
    pub height: u32,
    pub scale_x: f64,
    pub scale_y: f64,
}

impl MandelbrotConfig {
    pub fn new(width: u32, height: u32, scale_x: f64, scale_y: f64) -> Self {
        MandelbrotConfig {
            width,
            height,
            scale_x,
            scale_y,
        }
    }
}

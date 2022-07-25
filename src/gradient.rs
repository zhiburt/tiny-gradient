//! A module contains [Gradient] generator.

use libm::powf;

use crate::rgb::RGB;

/// Gradient generator.
/// 
/// It implements an [Iterator] interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gradient {
    from: RGB,
    to: RGB,
    steps: usize,
}

impl Gradient {
    /// Creates [Gradient] generator from one color to another in N steps.
    pub fn new(from: RGB, to: RGB, steps: usize) -> Self {
        Self { from, to, steps }
    }
}

impl IntoIterator for Gradient {
    type Item = RGB;
    type IntoIter = GradientIter;

    fn into_iter(self) -> Self::IntoIter {
        GradientIter {
            gradient: self,
            i: 0,
        }
    }
}

/// [Gradient] iterator which yields [RGB].
pub struct GradientIter {
    gradient: Gradient,
    i: usize,
}

impl Iterator for GradientIter {
    type Item = RGB;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.gradient.steps {
            return None;
        }

        let mut mix = self.i as f32 / (self.gradient.steps - 1) as f32;
        if mix.is_nan() {
            mix = 0.0;
        }

        self.i += 1;

        let color = mix_color(self.gradient.from, self.gradient.to, mix);

        Some(color)
    }
}

// Mix [0..1]
//      0   --> all c1
//      0.5 --> equal mix of c1 and c2
//      1   --> all c2
fn mix_color(c1: RGB, c2: RGB, mix: f32) -> RGB {
    //Convert color from 0..255 to 0..1
    let c1 = normalize_rgb(c1);
    let c2 = normalize_rgb(c2);

    //Invert sRGB gamma compression
    let c1 = srgb_inverse_companding(c1);
    let c2 = srgb_inverse_companding(c2);

    let mut c = rgb_linear_interpolation(c1, c2, mix);

    //Apply adjustment factor to each rgb value based
    if c.r + c.g + c.b != 0.0 {
        //Compute a measure of brightness of the two colors using empirically determined gamma
        let gamma = 0.43;
        let c1_bright = rgb_brightness(c1, gamma);
        let c2_bright = rgb_brightness(c2, gamma);

        //Interpolate a new brightness value, and convert back to linear light
        let brightness = linear_interpolation(c1_bright, c2_bright, mix);
        let intensity = powf(brightness, 1.0 / gamma);

        let factor = intensity / (c.r + c.g + c.b);

        c = RGB {
            r: c.r * factor,
            g: c.g * factor,
            b: c.b * factor,
        };
    }

    //Reapply sRGB gamma compression
    let c = srgb_companding(c);

    normalize_back_rgb(c)
}

//Inverse Red, Green, and Blue
fn srgb_inverse_companding(c: RGB<f32>) -> RGB<f32> {
    RGB {
        r: srgb_inverse_color(c.r),
        b: srgb_inverse_color(c.b),
        g: srgb_inverse_color(c.g),
    }
}

fn srgb_inverse_color(c: f32) -> f32 {
    if c > 0.04045 {
        powf((c + 0.055) / 1.055, 2.4)
    } else {
        c / 12.92
    }
}

fn normalize_rgb(c: RGB) -> RGB<f32> {
    RGB {
        r: normalize_color(c.r),
        g: normalize_color(c.g),
        b: normalize_color(c.b),
    }
}

fn normalize_back_rgb(c: RGB<f32>) -> RGB {
    RGB {
        r: (c.r * 255.9999) as u8,
        g: (c.g * 255.9999) as u8,
        b: (c.b * 255.9999) as u8,
    }
}

fn normalize_color(c: u8) -> f32 {
    c as f32 / 255.0
}

fn rgb_linear_interpolation(c1: RGB<f32>, c2: RGB<f32>, mix: f32) -> RGB<f32> {
    RGB {
        r: linear_interpolation(c1.r, c2.r, mix),
        g: linear_interpolation(c1.g, c2.g, mix),
        b: linear_interpolation(c1.b, c2.b, mix),
    }
}

fn linear_interpolation(c1: f32, c2: f32, frac: f32) -> f32 {
    (c1 * (1.0 - frac)) + (c2 * frac)
}

fn srgb_companding(c: RGB<f32>) -> RGB<f32> {
    RGB {
        r: srgb_apply_companding_color(c.r),
        g: srgb_apply_companding_color(c.g),
        b: srgb_apply_companding_color(c.b),
    }
}

fn srgb_apply_companding_color(c: f32) -> f32 {
    if c > 0.0031308 {
        1.055 * powf(c, 1.0 / 2.4) - 0.055
    } else {
        c * 12.92
    }
}

fn rgb_brightness(c: RGB<f32>, gamma: f32) -> f32 {
    powf(c.r + c.g + c.b, gamma)
}

#[cfg(test)]
mod tests {
    use super::{mix_color, Gradient, RGB};

    #[test]
    fn mix_color_test() {
        assert_eq!(
            mix_color(RGB::new(0, 0, 0), RGB::new(255, 255, 255), 0.50),
            RGB::new(123, 123, 123),
        );
        assert_eq!(
            mix_color(RGB::new(0, 0, 0), RGB::new(255, 255, 255), 0.25),
            RGB::new(56, 56, 56),
        );
        assert_eq!(
            mix_color(RGB::new(0, 0, 0), RGB::new(255, 255, 255), 0.1),
            RGB::new(14, 14, 14),
        );
        assert_eq!(
            mix_color(RGB::new(0, 0, 0), RGB::new(255, 255, 255), 1.0),
            RGB::new(255, 255, 255),
        );
        assert_eq!(
            mix_color(RGB::new(0, 0, 0), RGB::new(255, 255, 255), 0.0),
            RGB::new(0, 0, 0),
        );
    }

    #[test]
    fn gradient_test() {
        test_gradient(
            Gradient::new(RGB::new(0, 0, 0), RGB::new(255, 255, 255), 0).into_iter(),
            &[],
        );
        test_gradient(
            Gradient::new(RGB::new(0, 0, 0), RGB::new(255, 255, 255), 1).into_iter(),
            &[RGB::new(0, 0, 0)],
        );
        test_gradient(
            Gradient::new(RGB::new(0, 0, 0), RGB::new(255, 255, 255), 2).into_iter(),
            &[RGB::new(0, 0, 0), RGB::new(255, 255, 255)],
        );
        test_gradient(
            Gradient::new(RGB::new(0, 0, 0), RGB::new(255, 255, 255), 3).into_iter(),
            &[
                RGB::new(0, 0, 0),
                RGB::new(123, 123, 123),
                RGB::new(255, 255, 255),
            ],
        );
    }

    fn test_gradient(mut iter: impl Iterator<Item = RGB>, expected: &[RGB]) {
        for rgb in expected {
            let got = iter.next().unwrap();
            assert_eq!(got, *rgb);
        }

        assert!(iter.next().is_none());
    }
}

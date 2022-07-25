use crate::RGB;

/// A list of built-in gradient themes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Gradient {
    /// Atlast
    Atlast,
    /// Cristal
    Cristal,
    /// Teen
    Teen,
    /// Mind
    Mind,
    /// Morning
    Morning,
    /// Vice
    Vice,
    /// Passion
    Passion,
    /// Fruit
    Fruit,
    /// Retro
    Retro,
    /// Summer
    Summer,
    /// Rainbow
    Rainbow,
    /// Pastel
    Pastel,
    /// Monsoon
    Monsoon,
    /// Forest
    Forest,
    /// Instagram
    Instagram,
}

impl IntoIterator for &Gradient {
    type Item = RGB;
    type IntoIter = GradientIter<'static>;

    fn into_iter(self) -> Self::IntoIter {
        let colors = match self {
            Gradient::Atlast => GRADIENT_ATLAST.as_ref(),
            Gradient::Cristal => GRADIENT_CRISTAL.as_ref(),
            Gradient::Teen => GRADIENT_TEEN.as_ref(),
            Gradient::Mind => GRADIENT_MIND.as_ref(),
            Gradient::Morning => GRADIENT_MORNING.as_ref(),
            Gradient::Vice => GRADIENT_VICE.as_ref(),
            Gradient::Passion => GRADIENT_PASSION.as_ref(),
            Gradient::Fruit => GRADIENT_FRUIT.as_ref(),
            Gradient::Retro => GRADIENT_RETRO.as_ref(),
            Gradient::Summer => GRADIENT_SUMMER.as_ref(),
            Gradient::Rainbow => GRADIENT_RAINBOW.as_ref(),
            Gradient::Pastel => GRADIENT_PASTEL.as_ref(),
            Gradient::Monsoon => GRADIENT_MONSOON.as_ref(),
            Gradient::Forest => GRADIENT_FOREST.as_ref(),
            Gradient::Instagram => GRADIENT_INSTAGRAM.as_ref(),
        };

        GradientIter {
            colors: colors.iter(),
        }
    }
}

impl IntoIterator for Gradient {
    type Item = RGB;
    type IntoIter = GradientIter<'static>;

    fn into_iter(self) -> Self::IntoIter {
        (&self).into_iter()
    }
}

const GRADIENT_ATLAST: [RGB; 3] = [
    RGB::new(0xFE, 0xAC, 0x5E),
    RGB::new(0xC7, 0x9D, 0xD0),
    RGB::new(0x4B, 0xC0, 0xC8),
];

const GRADIENT_CRISTAL: [RGB; 2] = [RGB::new(0xBD, 0xFF, 0xF3), RGB::new(0x5A, 0xC2, 0x9A)];

const GRADIENT_TEEN: [RGB; 3] = [
    RGB::new(0x77, 0xA1, 0xD3),
    RGB::new(0x79, 0xCB, 0xCA),
    RGB::new(0xE6, 0x84, 0xAE),
];

const GRADIENT_MORNING: [RGB; 2] = [RGB::new(0xFF, 0x5F, 0x6D), RGB::new(0xFF, 0xC3, 0x71)];

const GRADIENT_VICE: [RGB; 2] = [RGB::new(0x5E, 0xE7, 0xDF), RGB::new(0xB4, 0x90, 0xCA)];

const GRADIENT_PASSION: [RGB; 2] = [RGB::new(0xF4, 0x3B, 0x47), RGB::new(0x45, 0x3A, 0x94)];

const GRADIENT_FRUIT: [RGB; 2] = [RGB::new(0xFF, 0x4E, 0x50), RGB::new(0xF9, 0xD4, 0x23)];

const GRADIENT_SUMMER: [RGB; 2] = [RGB::new(0xfd, 0xbb, 0x2d), RGB::new(0x22, 0xc1, 0xc3)];

const GRADIENT_MIND: [RGB; 3] = [
    RGB::new(0x47, 0x3B, 0x7B),
    RGB::new(0x35, 0x84, 0xA7),
    RGB::new(0x30, 0xD2, 0xBE),
];

const GRADIENT_RETRO: [RGB; 9] = [
    RGB::new(0x3f, 0x51, 0xb1),
    RGB::new(0x5a, 0x55, 0xae),
    RGB::new(0x7b, 0x5f, 0xac),
    RGB::new(0x8f, 0x6a, 0xae),
    RGB::new(0xa8, 0x6a, 0xa4),
    RGB::new(0xcc, 0x6b, 0x8e),
    RGB::new(0xf1, 0x82, 0x71),
    RGB::new(0xf3, 0xa4, 0x69),
    RGB::new(0xf7, 0xc9, 0x78),
];

const GRADIENT_RAINBOW: [RGB; 6] = [
    RGB::new(189, 19, 84),
    RGB::new(228, 108, 33),
    RGB::new(226, 166, 29),
    RGB::new(46, 163, 44),
    RGB::new(54, 83, 238),
    RGB::new(87, 32, 131),
];

const GRADIENT_PASTEL: [RGB; 5] = [
    RGB::new(255, 223, 204),
    RGB::new(255, 243, 219),
    RGB::new(203, 235, 195),
    RGB::new(173, 215, 219),
    RGB::new(137, 143, 173),
];

const GRADIENT_MONSOON: [RGB; 6] = [
    RGB::new(181, 199, 204),
    RGB::new(139, 161, 173),
    RGB::new(88, 123, 137),
    RGB::new(36, 76, 102),
    RGB::new(64, 125, 108),
    RGB::new(125, 178, 144),
];

const GRADIENT_FOREST: [RGB; 5] = [
    RGB::new(69, 55, 48),
    RGB::new(130, 94, 65),
    RGB::new(44, 62, 57),
    RGB::new(65, 90, 69),
    RGB::new(108, 112, 88),
];

const GRADIENT_INSTAGRAM: [RGB; 3] = [
    RGB::new(0x83, 0x3a, 0xb4),
    RGB::new(0xfd, 0x1d, 0x1d),
    RGB::new(0xfc, 0xb0, 0x45),
];

#[derive(Debug, Clone)]
pub struct GradientIter<'a> {
    colors: core::slice::Iter<'a, RGB>,
}

impl Iterator for GradientIter<'_> {
    type Item = RGB;

    fn next(&mut self) -> Option<Self::Item> {
        self.colors.next().copied()
    }
}

impl ExactSizeIterator for GradientIter<'_> {
    fn len(&self) -> usize {
        self.colors.len()
    }
}

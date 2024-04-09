use image::{DynamicImage, GenericImageView, Rgb};

pub type Color = Rgb<u8>;
pub type Image = DynamicImage;

#[derive(Clone)]
pub struct ImageColor {
    pub image: Image,
    pub color: Color,
}

pub enum PrimaryColorMethod {
    Average,
    Kmeans(u32),
}

pub enum ColorSimilarityMethod {
    DotProduct,
}

pub fn get_primary_color(image: Image, method: PrimaryColorMethod) -> ImageColor {
    use PrimaryColorMethod as M;
    match method {
        M::Average => average_color(image),
        M::Kmeans(clusters) => todo!(),
    }
}

pub fn color_similarity(a: Color, b: Color, method: ColorSimilarityMethod) -> u32 {
    use ColorSimilarityMethod as M;
    match method {
        M::DotProduct => dot_product(a, b),
    }
}

fn dot_product(a: Color, b: Color) -> u32 {
    a[0] as u32 * b[0] as u32 + a[1] as u32 * b[1] as u32 + a[2] as u32 * b[2] as u32
}

fn average_color(image: Image) -> ImageColor {
    let dimensions = image.dimensions();
    let total_pixels = dimensions.0 as u64 * dimensions.1 as u64;
    let sum = image.pixels().fold([0u64; 3], |mut acc, e| {
        acc[0] = acc[0]
            .checked_add(e.2[0] as u64)
            .expect("Sum got way too big");
        acc[1] = acc[1]
            .checked_add(e.2[1] as u64)
            .expect("Sum got way too big");
        acc[2] = acc[2]
            .checked_add(e.2[2] as u64)
            .expect("Sum got way too big");
        acc
    });
    let color = Color::from([
        (sum[0] / total_pixels) as u8,
        (sum[1] / total_pixels) as u8,
        (sum[2] / total_pixels) as u8,
    ]);

    ImageColor { image, color }
}

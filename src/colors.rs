use image::{DynamicImage, GenericImageView, Rgb};
use lab::Lab;

pub type Color = Rgb<u8>;
pub type Image = DynamicImage;

#[derive(Clone)]
pub struct ImageColor {
    pub image: Image,
    pub color: Color,
}

pub fn get_primary_color(image: Image, k_means: u32) -> ImageColor {
    if k_means < 2 {
        average_color(image)
    } else {
        todo!()
    }
}

pub fn color_similarity(a: Color, b: Color) -> f32 {
    // The Delta-E '00 equation, graciously stolen from: https://gitlab.com/ryanobeirne/deltae under
    // an MIT license. The reason I didn't use the crate is because I only needed this one
    // function.

    let lab_0 = Lab::from_rgb(&a.0);
    let lab_1 = Lab::from_rgb(&b.0);

    let chroma_0 = (lab_0.a.powi(2) + lab_0.b.powi(2)).sqrt();
    let chroma_1 = (lab_1.a.powi(2) + lab_1.b.powi(2)).sqrt();
    let c_bar = (chroma_0 + chroma_1) / 2.0;
    let g = 0.5 * (1.0 - (c_bar.powi(7) / (c_bar.powi(7) + 25_f32.powi(7))).sqrt());

    #[inline]
    fn get_h_prime(a: f32, b: f32) -> f32 {
        let h_prime = b.atan2(a).to_degrees();
        if h_prime < 0.0 {
            h_prime + 360.0
        } else {
            h_prime
        }
    }

    let a_prime_0 = lab_0.a * (1.0 + g);
    let a_prime_1 = lab_1.a * (1.0 + g);
    let c_prime_0 = (a_prime_0.powi(2) + lab_0.b.powi(2)).sqrt();
    let c_prime_1 = (a_prime_1.powi(2) + lab_1.b.powi(2)).sqrt();
    let l_bar_prime = (lab_0.l + lab_1.l) / 2.0;
    let c_bar_prime = (c_prime_0 + c_prime_1) / 2.0;
    let h_prime_0 = get_h_prime(a_prime_0, lab_0.b);
    let h_prime_1 = get_h_prime(a_prime_1, lab_1.b);
    let h_bar_prime = if (h_prime_0 - h_prime_1).abs() > 180.0 {
        if (h_prime_0 - h_prime_1) < 360.0 {
            (h_prime_0 + h_prime_1 + 360.0) / 2.0
        } else {
            (h_prime_0 + h_prime_1 - 360.0) / 2.0
        }
    } else {
        (h_prime_0 + h_prime_1) / 2.0
    };
    let t = 1.0 - 0.17 * ((h_bar_prime - 30.0).to_radians()).cos()
        + 0.24 * ((2.0 * h_bar_prime).to_radians()).cos()
        + 0.32 * ((3.0 * h_bar_prime + 6.0).to_radians()).cos()
        - 0.20 * ((4.0 * h_bar_prime - 63.0).to_radians()).cos();
    let mut delta_h = h_prime_1 - h_prime_0;
    if delta_h > 180.0 && h_prime_1 <= h_prime_0 {
        delta_h += 360.0;
    } else if delta_h > 180.0 {
        delta_h -= 360.0;
    };
    let delta_l_prime = lab_1.l - lab_0.l;
    let delta_c_prime = c_prime_1 - c_prime_0;
    let delta_h_prime = 2.0 * (c_prime_0 * c_prime_1).sqrt() * (delta_h.to_radians() / 2.0).sin();
    let s_l = 1.0
        + ((0.015 * (l_bar_prime - 50.0).powi(2)) / (20.00 + (l_bar_prime - 50.0).powi(2)).sqrt());
    let s_c = 1.0 + 0.045 * c_bar_prime;
    let s_h = 1.0 + 0.015 * c_bar_prime * t;
    let delta_theta = 30.0 * (-((h_bar_prime - 275.0) / 25.0).powi(2)).exp();
    let r_c = 2.0 * (c_bar_prime.powi(7) / (c_bar_prime.powi(7) + 25_f32.powi(7))).sqrt();
    let r_t = -(r_c * (2.0 * delta_theta.to_radians()).sin());
    let k_l = 1.0;
    let k_c = 1.0;
    let k_h = 1.0;
    ((delta_l_prime / (k_l * s_l)).powi(2)
        + (delta_c_prime / (k_c * s_c)).powi(2)
        + (delta_h_prime / (k_h * s_h)).powi(2)
        + (r_t * (delta_c_prime / (k_c * s_c)) * (delta_h_prime / (k_h * s_h))))
        .sqrt()
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

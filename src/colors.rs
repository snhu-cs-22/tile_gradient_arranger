use image::{DynamicImage, GenericImageView, Pixel};
use lab::Lab;

pub type Color = Lab;
pub type Image = DynamicImage;

#[derive(Clone)]
pub struct ImageColor {
    pub image: Image,
    pub color: Color,
}

pub fn get_primary_color(image: Image, k: u32) -> ImageColor {
    let pixels = image
        .pixels()
        .map(|(_, _, p)| p.to_rgb())
        .map(|p| Color::from_rgb(&p.0));
    let color = k_means(pixels, k);
    ImageColor { image, color }
}

pub fn color_similarity(lab_0: Color, lab_1: Color) -> f32 {
    // The Delta-E '00 equation, graciously stolen from: https://gitlab.com/ryanobeirne/deltae under
    // an MIT license. The reason I didn't use the crate is because I only needed this one
    // function.

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

fn k_means<I>(colors: I, k: u32) -> Color
where
    I: Iterator<Item = Color> + Clone,
{
    if k < 2 {
        return average_color(colors);
    }

    // Build list of "random" centroids
    let mut centroids_and_totals = colors
        .clone()
        // TODO: make this less stupid
        .step_by(colors.clone().count() / k as usize)
        .take(k as usize)
        .map(|c| (c, 0))
        .collect::<Vec<_>>();

    // Iterate towards local maximum
    for _ in 0..5 {
        // Reset totals to zero
        centroids_and_totals.iter_mut().for_each(|(_, t)| *t = 0);

        // Assign closest centroid to each color
        let closest_centroids = colors.clone().map(|color| {
            (
                color,
                *centroids_and_totals
                    .iter()
                    .map(|(c, _)| c)
                    .max_by(|&a, &b| {
                        color_similarity(color, *a).total_cmp(&color_similarity(color, *b))
                    })
                    .unwrap(),
            )
        });

        // Group colors by centroid
        let mut groups = vec![Vec::new(); k as usize];
        for (color, closest) in closest_centroids {
            for ((centroid, _), group) in centroids_and_totals.iter().zip(groups.iter_mut()) {
                if closest == *centroid {
                    group.push(color);
                }
            }
        }

        // Reassign centroids to the average color in their group
        for ((centroid, total), group) in centroids_and_totals.iter_mut().zip(groups.iter()) {
            *total = group.len();
            *centroid = average_color(group.iter().map(|c| *c));
        }
    }

    centroids_and_totals
        .iter()
        .max_by_key(|(_, t)| t)
        .unwrap()
        .0
}

fn average_color<I>(colors: I) -> Color
where
    I: Iterator<Item = Color>,
{
    let (sum, count) = colors.fold(([0f32; 3], 0usize), |mut acc, e| {
        acc.0[0] += e.l;
        acc.0[1] += e.a;
        acc.0[2] += e.b;
        acc.1 += 1;
        acc
    });

    Color {
        l: sum[0] / count as f32,
        a: sum[1] / count as f32,
        b: sum[2] / count as f32,
    }
}

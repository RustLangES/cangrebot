use ab_glyph::{FontRef, PxScale};
use image::imageops::overlay;
use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Rgba};
mod error;
use error::GenWelcomeError;

const FONT_SIZE: PxScale = PxScale { x: 36., y: 36. };

/// # Errors
///
/// Will return Err in any of the following circumstances:
/// - Fails to generate an image
/// - Fails to load a font
/// - Fails type conversion
pub fn generate(
    bg: &str,
    avatar: &[u8],
    member_name: &str,
    members: Option<u64>,
    bold_font: &[u8],
    regular_font: &[u8],
    out: &str,
) -> Result<(), GenWelcomeError> {
    // Fonts
    let bold = FontRef::try_from_slice(bold_font)?;
    let regular = FontRef::try_from_slice(regular_font)?;

    let avatar = image::load_from_memory(avatar)?;
    let avatar = avatar.resize(256, 256, image::imageops::Lanczos3);
    let avatar = round(&avatar);
    let mut background = image::open(bg)?;
    let (w, _h) = background.dimensions();

    overlay(&mut background, &avatar, 412, 87);
    let w_msg = format!("{member_name} acaba de caer en el servidor");

    // Welcome message
    let (t1_x, _t1_y) = imageproc::drawing::text_size(FONT_SIZE, &bold, &w_msg);
    imageproc::drawing::draw_text_mut(
        &mut background,
        Rgba([255, 255, 255, 255]),
        ((w / 2) - (t1_x / 2)).try_into()?,
        429,
        FONT_SIZE,
        &bold,
        &w_msg,
    );

    if let Some(members) = members.as_ref() {
        let n_msg = format!("Eres el Rustaceo #{members}");
        // Member number
        let (t2_x, _t2_y) = imageproc::drawing::text_size(FONT_SIZE, &regular, &n_msg);
        imageproc::drawing::draw_text_mut(
            &mut background,
            Rgba([255, 255, 255, 255]),
            ((w / 2) - (t2_x / 2)).try_into()?,
            488,
            FONT_SIZE,
            &regular,
            &n_msg,
        );
    }

    Ok(background.save(out)?)
}

fn round<I: GenericImageView<Pixel = Rgba<u8>>>(avatar: &I) -> impl GenericImage<Pixel = Rgba<u8>> {
    let (width, height) = avatar.dimensions();
    let radius = f64::from(width) / 2.0;
    let mut mask = ImageBuffer::new(width, height);
    let center = (f64::from(width) / 2.0, f64::from(height) / 2.0);

    for (x, y, pixel) in mask.enumerate_pixels_mut() {
        let dx = f64::from(x) - center.0 + 0.5; // +0.5 para centrar el pixel
        let dy = f64::from(y) - center.1 + 0.5;
        if dx.powi(2) + dy.powi(2) <= radius.powi(2) {
            *pixel = Rgba([255, 255, 255, 255]);
        } else {
            *pixel = Rgba([0, 0, 0, 0]);
        }
    }

    // Aplica la máscara al avatar redimensionado
    ImageBuffer::from_fn(width, height, |x, y| {
        let mask_pixel = mask.get_pixel(x, y).0[3];
        let avatar_pixel = avatar.get_pixel(x, y);
        if mask_pixel > 0 {
            avatar_pixel
        } else {
            avatar_pixel.map_with_alpha(|f| f, |_| 0)
        }
    })
}

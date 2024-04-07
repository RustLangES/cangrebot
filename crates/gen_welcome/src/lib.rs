use ab_glyph::{FontRef, PxScale};
use image::imageops::overlay;
use image::{GenericImage, GenericImageView, ImageBuffer, ImageError, Pixel, Rgba};

const FONT_SIZE: PxScale = PxScale {
    x: 36.,
    y: 36.
};

pub fn generate(
    bg: &str,
    avatar: &[u8],
    member_name: &str,
    members: u64,
    out: &str,
) -> Result<(), ImageError> {
    // Fonts
    let bold = FontRef::try_from_slice(include_bytes!("../../../static/fonts/WorkSans-Bold.ttf"))
        .unwrap();
    let regular =
        FontRef::try_from_slice(include_bytes!("../../../static/fonts/WorkSans-Regular.ttf"))
            .unwrap();

    let avatar = image::load_from_memory(&avatar)?;
    let avatar = avatar.resize(256, 256, image::imageops::Lanczos3);
    let avatar = round(&avatar);
    let mut background = image::open(bg)?;
    let (w, _h) = background.dimensions();

    overlay(&mut background, &avatar, 412, 87);
    let w_msg = format!("{member_name} acaba de caer en el servidor");
    let n_msg = format!("Eres el Rustaceo #{members}");

    // Welcome message
    let (t1_x, _t1_y) = imageproc::drawing::text_size(FONT_SIZE, &bold, &w_msg);
    imageproc::drawing::draw_text_mut(
        &mut background,
        Rgba([255, 255, 255, 255]),
        ((w / 2) - (t1_x / 2)) as i32,
        429,
        FONT_SIZE,
        &bold,
        &w_msg,
    );

    // Member number
    let (t2_x, _t2_y) = imageproc::drawing::text_size(FONT_SIZE, &regular, &n_msg);
    imageproc::drawing::draw_text_mut(
        &mut background,
        Rgba([255, 255, 255, 255]),
        ((w / 2) - (t2_x / 2)) as i32,
        488,
        FONT_SIZE,
        &regular,
        &n_msg,
    );

    background.save(out)
}

fn round<I: GenericImageView<Pixel = Rgba<u8>>>(avatar: &I) -> impl GenericImage<Pixel = Rgba<u8>> {
    let (width, height) = avatar.dimensions();
    let radius = width as f32 / 2.0;
    let mut mask = ImageBuffer::new(width, height);
    let center = (width as f32 / 2.0, height as f32 / 2.0);

    for (x, y, pixel) in mask.enumerate_pixels_mut() {
        let dx = x as f32 - center.0 + 0.5; // +0.5 para centrar el pixel
        let dy = y as f32 - center.1 + 0.5;
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

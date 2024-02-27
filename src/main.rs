use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use show_image::{event, ImageView, ImageInfo, create_window};
use rand::Rng;
mod noise;
use core::f64::consts::PI;



#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let imgx = 800;
    let imgy = 800;

    let mut imgbuf: RgbImage = ImageBuffer::new(imgx, imgy);

    for x in 0..imgx {
        for y in 0..imgy {
            //println!("perlin: {}", noise::perlin2d(x.into(), y.into(), 0.1, 4) * 256 as f64);
            *imgbuf.get_pixel_mut(x, y) = image::Rgb([(noise::perlin2d(x.into(), y.into(), 0.07, 5) * 256 as f64) as u8,(noise::perlin2d(x.into(), y.into(), 0.07, 5) * 256 as f64) as u8,(noise::perlin2d(x.into(), y.into(), 0.07, 5) * 256 as f64) as u8]);
        }
    }
        
    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("perlinnoise.png").unwrap();
    
    let image = ImageView::new(ImageInfo::rgb8(800, 800), &imgbuf);

    // Create a window with default options and display the image.
    let window = create_window("perlin noise", Default::default())?;
    window.set_image("image-001", image)?;

    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            println!("{:#?}", event);
            if event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
                break;
            }
        }
    }
    Ok(())
}

use image::{RgbImage, Rgb};

struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

struct Vector2 {
    x: f32,
    y: f32,
}

fn gradient(input: Vector2) -> Vector3 {
    return Vector3{x: input.x, y: input.y, z: 0.0};
}

fn main() {
    let height = 200;
    let width = 300;
    let mut img = RgbImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let v3 = gradient(Vector2{x: 2.0*x as f32/width as f32 - 1.0, y: 2.0*y as f32/height as f32 - 1.0});
            img.put_pixel(x, y, Rgb([(v3.x*255.0) as u8, (v3.y*255.0) as u8, (v3.z*255.0) as u8]));
        }
    }
    img.save("out.bmp");
}

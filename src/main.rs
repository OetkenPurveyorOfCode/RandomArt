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

#[derive(Debug)]
enum Node {
    X,
    Y,
    Number(f32),
    Add(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Triple(Box<Node>, Box<Node>, Box<Node>),
}

fn gradient(input: Vector2) -> Vector3 {
    return Vector3{x: input.x, y: input.x, z: input.x};
}

fn cool(input: Vector2) -> Vector3 {
    if input.x * input.y >= 0.0 {
        return Vector3{x: input.x, y: input.y, z: 1.0};
    }
    else {
        let r = input.x % input.y;
        return Vector3{x: r, y: r, z: r};
    }
}

fn eval(node: Node, uv: Vector2) -> Vector3 {
    return Vector3{x: uv.x, y: uv.x, z: uv.x};
}

fn main() {
    let height = 200;
    let width = 300;
    let node : Node = Node::Triple(
        Box::new(Node::Add(Box::new(Node::X), Box::new(Node::Y))),
        Box::new(Node::Y), 
        Box::new(Node::Number(0.5))
    );
    dbg!(&node);
    let mut img = RgbImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let v3 = cool(Vector2{x: 2.0*x as f32/width as f32 - 1.0, y: 2.0*y as f32/height as f32 - 1.0});
            img.put_pixel(x, y, Rgb([((v3.x+1.0)*255.0/2.0) as u8, ((v3.y+1.0)*255.0/2.0) as u8, ((v3.z+1.0)*255.0/2.0) as u8]));
        }
    }
    img.save("out.bmp").expect("Failed to save image");
}

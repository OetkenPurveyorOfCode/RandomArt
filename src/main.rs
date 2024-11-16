use image::{RgbImage, Rgb};
use thiserror::Error;

#[derive(Clone)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Clone)]
struct Vector2 {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone)]
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

#[derive(Error, Debug)]
enum EvalError {
    #[error("TypeError:Unexpected operand types for Add: `{0:?}` + `{1:?}`")]
    AddError(Node, Node),
    #[error("TypeError:Unexpected operand types for Add: `{0:?}` * `{1:?}`")]
    MulError(Node, Node),
}

fn eval(node: Node, uv: Vector2) -> Result<Node, EvalError> {
    match node {
        Node::X => {return Ok(Node::Number(uv.x));}
        Node::Y => {return Ok(Node::Number(uv.y));}
        Node::Number(_) => {return Ok(node);}
        Node::Add(x, y) => {
            let lhs = eval(*x, uv.clone())?;
            let rhs = eval(*y, uv.clone())?;
            match (lhs, rhs) {
                (Node::Number(a), Node::Number(b)) => {
                    return Ok(Node::Number(a+b));
                }
                (a, b) => {
                    return Err(EvalError::AddError(a, b));
                }
            }
        }
        Node::Mul(x, y) => {
            let lhs = eval(*x, uv.clone())?;
            let rhs = eval(*y, uv.clone())?;
            match (lhs, rhs) {
                (Node::Number(a), Node::Number(b)) => {
                    return Ok(Node::Number(a*b));
                }
                (a, b) => {
                    return Err(EvalError::MulError(a, b));
                }
            }
        }
        Node::Triple(a, b, c) => {
            return Ok(Node::Triple(a, b, c));
        }
    }
}

fn eval_render(node: Node, uv: Vector2) -> Vector3 {
    let node = eval(node, uv).expect("Error evaluating");
    match node {
        Node::Triple(a, b, c) => {
            match (*a, *b, *c) {
                (Node::Number(a), Node::Number(b), Node::Number(c)) => {
                    return Vector3{x: a, y: b, z: c};
                }
                (a, b, c) => {
                    panic!("Expected number found {a:?} {b:?} {c:?}");
                }
            }
        }
        _ => {
            panic!("Wrong result type (expected triple): {node:?}");
        }
    }
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

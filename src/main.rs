use image::{Rgb, RgbImage};
use thiserror::Error;
use std::io::Write;
use rand::Rng;
use rand::distributions::{Distribution, Uniform};

#[derive(Debug, Clone)]
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
    Boolean(bool),
    GreaterThan(Box<Node>, Box<Node>),
    IfThenElse(Box<Node>, Box<Node>, Box<Node>),
    Add(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Fmodf(Box<Node>, Box<Node>),
    Triple(Box<Node>, Box<Node>, Box<Node>),
    Rule(usize),
    Random,
    Rand,
}


#[derive(Error, Debug)]
enum EvalError {
    #[error("Rand Error: Rand not allowed at runtime, use Random instead.")]
    RandError,
    #[error("Rule Error: Rule not allowed at runtime`{0:?}`")]
    RuleError(usize),
    #[error("TypeError:Unexpected operand types for Add: `{0:?}` + `{1:?}`")]
    AddError(Node, Node),
    #[error("TypeError:Unexpected operand types for Mul: `{0:?}` * `{1:?}`")]
    MulError(Node, Node),
    #[error("TypeError:Unexpected operand types for Fmodf: `{0:?}` % `{1:?}`")]
    FmodfError(Node, Node),
    #[error("TypeError:Unexpected operand types for GreaterThan: `{0:?}` > `{1:?}`")]
    GreaterThanError(Node, Node),
    #[error("TypeError:Unexpected operand types for IfThenElse: `{0:?}` ? `{1:?}` : `{2:?}`")]
    IfThenElseError(Node, Node, Node),
}

fn eval(node: Node, uv: Vector2) -> Result<Node, EvalError> {
    match node {
        Node::Rule(x) => {
            return Err(EvalError::RuleError(x));
        }
        Node::Rand => {
            return Err(EvalError::RandError);
        }
        Node::Random => {
            let mut rng = rand::thread_rng();
            let uniform = Uniform::new(-1.0, 1.0);
            let v: f32 = uniform.sample(&mut rng);
            return Ok(Node::Number(v));
        }
        Node::X => {
            return Ok(Node::Number(uv.x));
        }
        Node::Y => {
            return Ok(Node::Number(uv.y));
        }
        Node::Number(_) => {
            return Ok(node);
        }
        Node::Boolean(_) => {
            return Ok(node);
        }
        Node::Add(x, y) => {
            let lhs = eval(*x, uv.clone())?;
            let rhs = eval(*y, uv.clone())?;
            match (lhs, rhs) {
                (Node::Number(a), Node::Number(b)) => {
                    return Ok(Node::Number(a + b));
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
                    return Ok(Node::Number(a * b));
                }
                (a, b) => {
                    return Err(EvalError::MulError(a, b));
                }
            }
        }
        Node::Fmodf(x, y) => {
            let lhs = eval(*x, uv.clone())?;
            let rhs = eval(*y, uv.clone())?;
            match (lhs, rhs) {
                (Node::Number(a), Node::Number(b)) => {
                    return Ok(Node::Number(a % b));
                }
                (a, b) => {
                    return Err(EvalError::FmodfError(a, b));
                }
            }
        }
        Node::GreaterThan(x, y) => {
            let lhs = eval(*x, uv.clone())?;
            let rhs = eval(*y, uv.clone())?;
            match (lhs, rhs) {
                (Node::Number(a), Node::Number(b)) => {
                    return Ok(Node::Boolean(a > b));
                }
                (a, b) => {
                    return Err(EvalError::GreaterThanError(a, b));
                }
            }
        }
        Node::IfThenElse(cond, then_, else_) => {
            let cond = eval(*cond, uv.clone())?;
            let then_ev = eval(*then_, uv.clone())?;
            let else_ev = eval(*else_, uv.clone())?;
            match (cond, then_ev, else_ev) {
                (Node::Boolean(cond), a, b) => {
                    // TODO: ensure a and b have the same type
                    if cond {
                        return Ok(a);
                    } else {
                        return Ok(b);
                    }
                }
                (a, b, c) => {
                    return Err(EvalError::IfThenElseError(a, b, c));
                }
            }
        }
        Node::Triple(a, b, c) => {
            return Ok(Node::Triple(
                Box::new(eval(*a, uv.clone())?),
                Box::new(eval(*b, uv.clone())?),
                Box::new(eval(*c, uv.clone())?),
            ));
        }
    }
}

fn eval_render(node: Node, uv: Vector2) -> Vector3 {
    let node = eval(node, uv).expect("Error evaluating");
    match node {
        Node::Triple(a, b, c) => match (*a, *b, *c) {
            (Node::Number(a), Node::Number(b), Node::Number(c)) => {
                return Vector3 { x: a, y: b, z: c };
            }
            (a, b, c) => {
                panic!("Expected number found {a:?} {b:?} {c:?}");
            }
        },
        _ => {
            panic!("Wrong result type (expected triple): {node:?}");
        }
    }
}

fn triple(a: Node, b: Node, c: Node) -> Node {
    return Node::Triple(Box::new(a), Box::new(b), Box::new(c));
}

fn gt(a: Node, b: Node) -> Node {
    return Node::GreaterThan(Box::new(a), Box::new(b));
}

fn mul(a: Node, b: Node) -> Node {
    return Node::Mul(Box::new(a), Box::new(b));
}

fn add(a: Node, b: Node) -> Node {
    return Node::Add(Box::new(a), Box::new(b));
}

fn fmodf(a: Node, b: Node) -> Node {
    return Node::Fmodf(Box::new(a), Box::new(b));
}

fn ifthenelse(a: Node, b: Node, c: Node) -> Node {
    return Node::IfThenElse(Box::new(a), Box::new(b), Box::new(c));
}

#[derive(Debug, Clone)]
struct Rule {
    node: Node,
    prob: f32,
}

fn random_art_node(grammar: &Vec<Vec<Rule>>, node: Node, depth: usize, terminal: usize) -> Node {
    match node {
        Node::X | Node::Y | Node::Boolean(_) | Node::Random | Node::Number(_) => {
            return node;
        }
        Node::Add(x, y) => {
            let lhs = random_art_node(grammar, *x, depth, terminal);
            let rhs = random_art_node(grammar, *y, depth, terminal);
            return add(lhs, rhs);
        }
        Node::Mul(x, y) => {
            let lhs = random_art_node(grammar, *x, depth, terminal);
            let rhs = random_art_node(grammar, *y, depth, terminal);
            return mul(lhs, rhs);
        }
        Node::Fmodf(x, y) => {
            let lhs = random_art_node(grammar, *x, depth, terminal);
            let rhs = random_art_node(grammar, *y, depth, terminal);
            return fmodf(lhs, rhs);
        }
        Node::GreaterThan(x, y) => {
            let lhs = random_art_node(grammar, *x, depth, terminal);
            let rhs = random_art_node(grammar, *y, depth, terminal);
            return gt(lhs, rhs);
        }
        Node::IfThenElse(cond, then_, else_) => {
            let cond = random_art_node(grammar, *cond, depth, terminal);
            let then_ = random_art_node(grammar, *then_, depth, terminal);
            let else_ = random_art_node(grammar, *else_, depth, terminal);
            return ifthenelse(cond, then_, else_);
        }
        Node::Triple(x, y, z) => {
            let x = random_art_node(grammar, *x, depth, terminal);
            let y = random_art_node(grammar, *y, depth, terminal);
            let z = random_art_node(grammar, *z, depth, terminal);
            return triple(x, y, z);
        }
        Node::Rule(rule_index) => {
            return random_art_rule(grammar, rule_index, depth - 1, terminal);
        }
        Node::Rand => {
            let mut rng = rand::thread_rng();
            let uniform = Uniform::new(-1.0, 1.0);
            let v: f32 = uniform.sample(&mut rng);
            return Node::Number(v);
        }
    }
}

/*fn is_terminal_rule(node: Node) -> bool {
    match (Node) => {
        Node::Rule(_) => {
            return false;
        }
        Node::Add(x, y) | Node::Mul(x, y) | Node::Fmodf(x, y) | Node::GreaterThan(x, y) => {
            return is_terminal_rule(x) && is_terminal_rule(y);
        }
        Node::Triple(x, y, z) | Node::IfThenElse(x, y, z) => {
            return is_terminal_rule(x) && is_terminal_rule(y) && is_terminal_rule(z);
        }
        _ => {
            return true;
        }
    }
}*/

fn random_art_rule(grammar: &Vec<Vec<Rule>>, entry: usize, depth: usize, terminal: usize) -> Node {
    if depth <= 0 {
        return Node::X;
    }
    let ruleset: &Vec<Rule> = &grammar[entry];
    let mut rng = rand::thread_rng();
    // TODO take probabilities into account
    let index = rng.gen_range(0..ruleset.len());
    let rule = &ruleset[index];
    return random_art_node(grammar, rule.node.clone(), depth, terminal);
}

fn main() {
    let height = 200;
    let width = 300;
    
    let grammar : Vec<Vec<Rule>> = vec![
        vec![
            Rule{
                node: triple(
                    Node::Rule(2),
                    Node::Rule(2),
                    Node::Rule(2)
                ),
                prob: 1.0
            },
        ],
        vec![
            Rule{
                node: Node::Rand,
                prob: 1.0/3.0,
            },
            Rule{
                node: Node::X,
                prob: 1.0/3.0,
            },
            Rule{
                node: Node::Y,
                prob: 1.0/3.0,
            },
        ],
        vec![
            Rule{
                node: Node::Rule(1),
                prob: 1.0/4.0,
            },
            Rule{
                node: add(Node::Rule(2), Node::Rule(2)),
                prob: 3.0/8.0,
            },
            Rule{
                node: mul(Node::Rule(2), Node::Rule(2)),
                prob: 3.0/8.0,
            }
        ]
    ];
    /*dbg!(&grammar);
    let node: Node = ifthenelse(
        gt(mul(Node::X, Node::Y), Node::Number(0.0)),
        triple(Node::X, Node::Y, Node::Number(1.0)),
        triple(
            fmodf(Node::X, Node::Y),
            fmodf(Node::X, Node::Y),
            fmodf(Node::X, Node::Y)
        )
    );
    */
    let node = random_art_rule(&grammar, 0, 20, 1);
    
    dbg!(eval_render(node.clone(), Vector2 { x: 0.0, y: 0.0 }));
    dbg!(&node);
    let mut img = RgbImage::new(width, height);
    let mut percent = 0.0;
    for y in 0..height {
        for x in 0..width {
            let uv = Vector2 {
                x: 2.0 * x as f32 / width as f32 - 1.0,
                y: 2.0 * y as f32 / height as f32 - 1.0,
            };
            let v3 = eval_render(node.clone(), uv);
            img.put_pixel(
                x,
                y,
                Rgb([
                    ((v3.x + 1.0) * 255.0 / 2.0) as u8,
                    ((v3.y + 1.0) * 255.0 / 2.0) as u8,
                    ((v3.z + 1.0) * 255.0 / 2.0) as u8,
                ]),
            );
        }
        if y as f32 > percent*(height as f32)/100.0 {
            print!("Progress {percent}%\r");
            let _ = std::io::stdout().flush();
            percent += 1.0;
        }
    }
    println!();
    img.save("out.bmp").expect("Failed to save image");
}

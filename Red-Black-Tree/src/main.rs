#[derive(Debug)]
pub struct RbTree {
    root: RbNodeLink,
}

#[derive(Debug)]
enum RbColor {
    RED,
    BLACK
}

type RbNodeLink = Option<Box<RbNode>>;

#[derive(Debug)]
struct RbNode {
    color: RbColor,
    key: i32,
    left_child: RbNodeLink,
    right_child: RbNodeLink,
    parent: RbNodeLink,
}

impl RbTree {

    fn new() -> Self {
        RbTree {root: None}
    }

    fn insert(&self) {

    }
    
    fn delete(&self) {
    
    }
}

fn left_rotate(mut n1: RbNode) {

    // let test = RbNode{color: RbColor::RED, key: 5, left_child: None, right_child: None, parent: None};
    // let test2 = RbNode{color: RbColor::RED, key: 5, left_child: None, right_child: None, parent: None};

    // match test.parent {
    //     None => println!("FUck me"),
    //     Some(mut p) => {
    //         println!("{:?}", p);
    //         p = Box::new(test2);
    //         println!("{:?}", p);
    //     }
    // }

    //let s: Option<Box<i32>> = Some(Box::new(32));

    // match s {
    //     None => println!("FUck me"),
    //     Some(i) => {
    //         println!("{}", i);
    //         i = Box::new(53);
    //         println!("{}", i);
    //     }
    // }

    //let mut p = n1.parent;

    match n1.parent {
        None => println!("Parent == None in left_rotate"),
        Some(mut p) => {
            println!("{:?}", p);
            p.right_child = n1.left_child;
            n1.left_child = Some(p);
            n1.left_child.unwrap().parent = Some(Box::new(n1));
            n1.parent = n1.left_child.unwrap().parent;
            
        }
    }

    //p.right_child = n1.left_child
    //n1.left_child = None
}

fn right_rotate() {

}



fn main() {
    println!("Hello, world!");
    let r: RbTree;
}




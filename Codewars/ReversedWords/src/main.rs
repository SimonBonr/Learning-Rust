use std::ops::Index;

fn main() {
    //println!();
    //let str = reverse_words("Hello, world!");

    let str = maskify("12798391");

    println!("\"{}\"", str);
    //calc("(2 + 4)* 2 * 6 - 5 - 2 -5 / 5 + 7");
    //calc( "(2 + 4 * 5)");
    calc("12* 123/(-5 + 2)");

    sum_intervals(&[(1, 5), (10, 20), (1, 6), (16, 19), (5, 11)]);
}



fn reverse_words(words: &str) -> String {
    //let backwards = words.rsplit(" ").map(|w| w + " ").collect();
    //words.rsplit(" ").map(|s| s.to_owned() + " ").collect::<String>().trim().to_string()
    words.rsplit(" ").collect::<Vec<_>>().join(" ")

}

fn maskify(cc: &str) -> String {
    let mut s:String = String::new(); 
    let mut i = cc.len();
    cc.chars().for_each(|c| {
        if i <= 4 {
            s.push(c);
            
        } else {
            s.push('#');
        }
        i -= 1;
    });
    s
}

//https://www.codewars.com/kata/52a78825cdfc2cfc87000005/train/rust,

fn calc(expr: &str) -> f64 {

    let mut expr_mut = expr.to_string();
    
    let mut has_parenthesis = true;

    while has_parenthesis {

    
        match expr_mut.find(')') {
            None => {
                has_parenthesis = false;
            }
            Some(pr_i) => {
                let pl_i = expr_mut[0..pr_i].rfind('(').unwrap();
                let val = eval(&expr_mut[pl_i+1..pr_i]).to_string();

                let replace_str = format!("{}{}", val, " ".repeat(pr_i - pl_i - val.len() + 1));

                expr_mut.replace_range(pl_i..pr_i+1, replace_str.as_str());
                println!("{}", expr_mut);
            }
        }
    }

    eval(&expr_mut)
}


fn eval(expr: &str) -> f64 {

    let mut add_op = 0;
    let add_secs = expr.split(['+', '-']);
    //println!("Trying: {:?} ", add_secs.collect::<Vec<_>>());
    println!("{}", expr);

    //let add_secs = expr.split(['+', '-']);
    let mut add_res = 0.0;


    for (i, add_sec) in add_secs.enumerate() {

        //println!("Trying: {} ", add_sec);

        let mut mult_op = 0;

        let mut mult_res = 1.0;

        let mut next_neg = false;

        for (j, sec) in add_sec.split(['*', '/']).enumerate() {

            if j == 0 {
                println!("{}", sec);

                mult_res *= sec.trim().parse::<f64>().unwrap();
                
            } else {

                match add_sec.chars().nth(mult_op) {
                    Some(c) if c == '*' => {
                        mult_res *= sec.trim().parse::<f64>().unwrap();
                        //println!("{}", mult_res);
                        //println!("Times");
                    }
                    Some(c) if c == '/' => {
                        mult_res /= sec.trim().parse::<f64>().unwrap();
                        //println!("Div");
                    }
                    _ => {
                        println!("ERROR {} at {}", add_sec, mult_op);
                    }
                }
                mult_op += 1;
            }
            mult_op += sec.len();
        }

        if i == 0 {
            add_res += mult_res
        } else {

            match expr.chars().nth(add_op) {
                Some(c) if c == '+' => {
                    add_res += mult_res;
                    //println!("adding {}", mult_res);
                    //println!("Times");
                }
                Some(c) if c == '-' => {
                    add_res -= mult_res;
                    //println!("subbing {}", mult_res);
                    //println!("Div");
                }
                _ => {
                    println!("ERROR {} at {}", expr, add_op);
                }
            }
            add_op += 1;
        }
        add_op += mult_op;
    }

    add_res

}


//Apparently one can sort vecs of tuples 🙄
//Trees would've been a great datatype for this.

fn sum_intervals(intv: &[(i32, i32)]) -> i32 {

    let mut intv2: Vec<(i32, i32)> = intv.to_vec();
    let mut changed = true;

    while (changed) {

        let mut uniques: Vec<(i32, i32)> = Vec::new();
        let mut skips: Vec<usize> =  Vec::new();
        
        for i in 0..intv2.len() {

            if skips.contains(&i) {
                continue;
            }
            let mut int_curr = intv2[i].clone();

            for j in i..intv2.len() {

                match reduce_overlap(&int_curr, &intv2[j]) {
                    None => {}
                    Some(new_i) => {
                        int_curr = new_i;
                        skips.push(j);
                    }
                }

            }
            uniques.push(int_curr);
        }

        if uniques.eq(&intv2) {
            intv2 = uniques;
            changed = false
        } else {
            
            intv2 = uniques;
        }

    }
    

    intv2.iter().fold(0, |mut sum, ele| {sum += ele.1 - ele.0; sum})

    
}


fn reduce_overlap(i1: &(i32, i32), i2: &(i32, i32)) -> Option<(i32, i32)> {

    if i1.1 < i2.0 || i2.1 < i1.0 {
        return None
    }

    let min = std::cmp::min(i1.0, i2.0);
    let max = std::cmp::max(i1.1, i2.1);
    
    return Some((min, max))
}




// 1-4, 0-3 = 1, 1
// 1-4, 2-5 = -1, -1
// 1-4, 0-5 = 1, -1
// 1-4, 2-3 = -1, 1
// 1-4, 5-6 
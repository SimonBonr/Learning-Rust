fn main() {
    //println!();
    //let str = reverse_words("Hello, world!");

    let str = maskify("12798391");

    println!("\"{}\"", str);

    sum_intervals(&[(1, 5), (10, 20), (1, 6), (16, 19), (5, 11)]);
}

//! Do this: https://www.codewars.com/kata/52a78825cdfc2cfc87000005/train/rust, basically implement a AST

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
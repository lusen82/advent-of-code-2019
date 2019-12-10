
pub fn day_4_a(){
    let existing_valid_pairs = &exists_pair;
    day_4(existing_valid_pairs);
}
pub fn day_4_b(){
    let existing_valid_pairs = &hoover_pairs;
    day_4(existing_valid_pairs);
}
fn day_4(existing_valid_pairs: &dyn Fn(i32, i32, i32, i32, i32, i32) -> bool){
    let start = vec![1,9,7,4,8,7];
    let end = vec![6,7,3,2,5,1];
    let start_val = 197487;
    let end_val = 673251;
    let mut possible = vec![];
    for i in start[0]..end[0] + 1 {
        println!(" i is {}", i);
        for j in i..10 {
            for k in j..10 {
                for l in k..10 {
                    for m in l..10 {
                        for n in m..10 {

                            let valid_pairs = existing_valid_pairs(i, j, k, l, m, n);
                            let res = vec![i,j,k,l,m,n];
                            if valid_pairs {
                                if (i != end[0] || concat(&res) < end_val) &&
                                    (i != start[0] || concat(&res) > start_val) {
                                    println!("Posssible {:?} ", &res);
                                    possible.push(res);

                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("end");
    println!("Size is {}", possible.len());
    //   197487..673251;

}

fn concat(vec: &Vec<i32>) -> i32 {
    vec.iter().fold(0, |acc, elem| acc * 10 + *elem)
}

fn hoover_pairs(i: i32, j: i32, k: i32, l: i32, m: i32, n: i32) -> bool{
    let vector = vec![i, j, k, l, m, n];
    for iter in 0..5 {
        if vector[iter] == vector[iter + 1] {
            if (iter == 0 || vector[iter - 1 ] != vector[iter]) && (iter == 4 || vector[iter + 2] != vector[iter]){
                return true;
            }
        }
    }
    return false;
}

fn exists_pair(i: i32, j: i32, k: i32, l: i32, m: i32, n: i32) -> bool{
    let vector = vec![i, j, k, l, m, n];
    for iter in 0..5 {
        if vector[iter] == vector[iter + 1] {
            return true;
        }
    }
    return false;
}


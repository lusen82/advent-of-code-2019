use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn day_8() {
    let mut file = File::open("inp8.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_line(&mut buf);
    println!("buf ; {:?}", &buf);
    let rows = &buf.len() / (25 * 6);
    println!("rows {}", rows);
    let mut layers = vec![];
    let mut layer = vec![];
    let mut fewest_zeros = (1000,0);
    for iter in 0..100{
        let mut nr_of_zeros = 0 as usize;
        for jiter in 0..6 {
            let vector_of_layers_rows: Vec<char> = buf.chars().take(25).collect();
            buf = String::from(&buf[25..]);
            let zero_chars: Vec<&char> = vector_of_layers_rows.iter().filter(|c| **c == '0').collect();
            nr_of_zeros += zero_chars.iter().count();
            layer.push(vector_of_layers_rows.clone());
        }
        if nr_of_zeros < fewest_zeros.0 {
            fewest_zeros = (nr_of_zeros, iter);
        }
        layers.push(layer.clone());
        println!("Iter {}, jiter, Size of buf {}", iter, buf.chars().count());
    }
    //println!("rows {:?}", &layers);
    println!("Fewest zeros was layer {} with {} zeros.", &fewest_zeros.1, &fewest_zeros.0);

    let this_layer = &layers[fewest_zeros.1];
    let flat: Vec<&char> = this_layer.iter().flatten().collect();
    let ones: Vec<&&char> = flat.iter().filter(|c| ***c == '1').collect();
    let twos: Vec<&&char> = flat.iter().filter(|c| ***c == '2').collect();
    println!("Number of two digits multiplied with nr of one digits is {}", ones.len() * twos.len());
}

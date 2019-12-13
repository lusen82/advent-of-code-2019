use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn day_8() -> Vec<Vec<Vec<char>>>{
    let mut file = File::open("inp8.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_line(&mut buf);
    println!("buf ; {:?}", &buf);
    let rows = &buf.len() / (25 * 6);
    println!("rows {}", rows);
    let mut layers = vec![];
    let mut fewest_zeros = (1000,0);
    for iter in 0..100{
        let mut layer = vec![];
        let mut nr_of_zeros = 0 as usize;
        for jiter in 0..6 {
            //println!("Original buf: {:?}", &buf);
            let vector_of_layers_rows: Vec<char> = buf.chars().take(25).collect();
            buf = String::from(&buf[25..]);
            //println!("Print this vec: {:?}", &vector_of_layers_rows);
            //println!("Left {:?}", &buf);
            let zero_chars: Vec<&char> = vector_of_layers_rows.iter().filter(|c| **c == '0').collect();
            nr_of_zeros += zero_chars.iter().count();
            layer.push(vector_of_layers_rows.clone());
        }
        if nr_of_zeros < fewest_zeros.0 {
            fewest_zeros = (nr_of_zeros, iter);
        }
        println!("Layer: {:?}", layer);
        println!("Layer dimension: {} {}", layer.len(), layer[0].len());
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

    return layers;
}

pub fn day_8_b() {
    let layers = day_8();
    let mut resulting_layer = vec![vec!['.'; 25]; 6];
    for layer in 0..100 {
        for row in 0..6 {
            for c in 0..25 {
                if resulting_layer[row][c] == '.' {
                    if layers[layer][row][c] != '2' {
                        resulting_layer[row][c] = match layers[layer][row][c] {
                            '0' => '#',
                            '1' => ' ',
                            _ => 'E',
                        };

                    }
                }
            }
        }
        for row in &resulting_layer {
            println!("{:?}", row);
        }
    }
    for row in &resulting_layer {
        for r in row {
            print!("{}", r);
        }
        println!("-");
    }
}

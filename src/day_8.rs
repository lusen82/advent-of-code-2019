use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn day_8(row_count: usize, col_count: usize) -> Vec<Vec<Vec<char>>>{
    let mut file = File::open("inp8.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_line(&mut buf);
    println!("buf ; {:?}", &buf);
    let rows = &buf.len() / (col_count * row_count);
    println!("rows {}", rows);
    let mut layers = vec![];
    let mut fewest_zeros = (1000,0);
    for iter in 0..rows{
        let mut layer = vec![];
        let mut nr_of_zeros = 0 as usize;
        for jiter in 0..row_count {
            //println!("Original buf: {:?}", &buf);
            let vector_of_layers_rows: Vec<char> = buf.chars().take(col_count).collect();
            buf = String::from(&buf[col_count..]);
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
    let row_count = 6;
    let col_count = 25;
    let layers = day_8(row_count, col_count);
    let mut resulting_layer = vec![vec!['.'; col_count]; row_count];
    println!("layaers count = {}", layers.len());
    for layer in 0..layers.len() {
        let layer_ = 99-layer;
        println!("layer: {}", layer_);
        for row in 0..row_count{
            for c in 0..col_count {
                println!("row: {}", row);
                println!("c: {}", c);
                //if resulting_layer[row][c] == '.' {
                    let layer_value = layers[layer_][row][c];
                    println!("pop: {}", layer_value);
                    if layer_value != '2' {
                        resulting_layer[row][c] = match layer_value {
                            '0' => ' ',
                            '1' => '#',
                            _ => 'E',
                        };

                    }
                //}
            }
        }
        for row in &resulting_layer {
            println!("{:?}", row);
        }
        //for row in &resulting_layer {
        //    println!("{:?}", row);
        //}
    }
    for row in &resulting_layer {
        for r in row {
            print!("{}", r);
        }
        println!("-");
    }

    for row in &resulting_layer {
        println!("{:?}", row);
    }
}

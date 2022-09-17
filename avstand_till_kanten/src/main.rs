use std::{io::stdin, usize};
fn main() {
    println!("Please enter number of rows: ");
    let mut rows: String = String::new();
    stdin().read_line(&mut rows).unwrap();
    let rows: usize = rows.trim().parse().unwrap();

    println!("Please enter nr. of columns: ");
    let mut columns: String = String::new();
    stdin().read_line(&mut columns).unwrap();
    let columns: usize = columns.trim().parse().unwrap();

    println!("Nr. of rows: {}", rows);
    println!("Nr. of columns: {}", columns);

    let mut two_d_vector: Vec<Vec<i32>> = vec![vec![1; columns]; rows];

    print(rows, columns, &mut two_d_vector);
}
fn print(rows: usize, columns: usize, two_d_vector: &mut Vec<Vec<i32>>) {
    for i in 0..rows {
        for j in 0..columns {
            let distance = shortest_distance(i, j, rows, columns);

            two_d_vector[i][j] = distance as i32;
            if distance > 9 {
                print!(".");
            } else {
                print!("{}", two_d_vector[i][j]);
            }
        }
        println!("");
    }
}
fn shortest_distance(i: usize, j: usize, rows: usize, columns: usize) -> usize {
    if distance_x(j, columns) < distance_y(i, rows) {
        return distance_x(j, columns);
    } else {
        return distance_y(i, rows);
    }
}
fn distance_x(j: usize, columns: usize) -> usize {
    if j < columns / 2 {
        return j + 1;
    } else {
        return columns - j;
    }
}
fn distance_y(i: usize, rows: usize) -> usize {
    if i < rows / 2 {
        return i + 1;
    } else {
        return rows - i;
    }
}

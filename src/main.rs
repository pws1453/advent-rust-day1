use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
//use std::vec::Vec;

fn main() -> std::io::Result<()>  {
    let file = File::open("./src/elves").expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut totals = vec![0];
    let mut index = 0;
    //let lines = BufReader::new(file).lines();
    //println!("{}",linect);
    for line in reader.lines() {
        if let Ok(lin) = line{
            if lin.is_empty() {
                index += 1;
                totals.push(0)
            } else {
                let temp = lin.parse::<i32>().expect("Unable to convert");
                totals[index] = totals[index] + temp
            }
        }   
    }
    totals.sort();
    totals.reverse();
    let (print, _) = totals.split_at(3);
    println!("Top 3 calories: {:?}", print);
    let sum: i32 = print.iter().sum();
    println!("Sum of top 3: {}", sum);
    Ok(())
}

use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::io::prelude::*;

mod vec;
use vec::Vec3; 

fn create_image() {
    let path = Path::new("image.ppm");
    let display = path.display();

    match File::create(&path) {
        Err(why) => panic!("could not create {}: {}", display, why),
        Ok(file) => file
    };
}

fn write_to_image(line: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("image.ppm")
        .unwrap();

    if let Err(why) = writeln!(file, "{}", line) {
        panic!("couldn't write to file: {}", why);
    }
}

fn main() {
    /*
    create_image();

    write_to_image(String::from("P3\n256 256\n255\n"));

    let mut color: String;
    for i in 0..256 {
        println!("line {}", i);
        for j in 0..256 {
            color = format!("{} {} 0", i, j);
            write_to_image(color);
        }
    }
    */

    let mut v1: Vec3 = Vec3::zero();
    v1.x = 2.;
    println!("v1 = {} {} {}", v1.x, v1.y, v1.z);

    let v2: Vec3 = Vec3::new(1., 2., 3.);
    println!("v2 = {} {} {}", v2.x, v2.y, v2.z);

    let sum: Vec3 = &v1 + &v2;
    println!("sum = {} {} {}", sum.x, sum.y, sum.z);
    
    let sum_scalar: Vec3 = &v2 + 2.;
    println!("sum scalar = {} {} {}", sum_scalar.x, sum_scalar.y, sum_scalar.z);
}

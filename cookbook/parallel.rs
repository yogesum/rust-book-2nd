#!/usr/bin/env run-cargo-script
// cargo-deps: rayon, rand, glob, image

extern crate rayon;
extern crate rand;
extern crate glob;
extern crate image;

use rayon::prelude::*;

fn par_mut() {
    let mut arr = [0, 7, 9, 11];
    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("{:?}", arr);
}

fn par_predicate_test() {
    let mut vec = vec![2, 4, 6, 8];

    assert!(!vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(!vec.par_iter().any(|n| *n > 8 ));
    assert!(vec.par_iter().all(|n| *n <= 8 ));

    vec.push(9);
    println!("{:?}", vec);
    assert!(vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(!vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(vec.par_iter().any(|n| *n > 8 ));
    assert!(!vec.par_iter().all(|n| *n <= 8 ));
}

fn par_search() {
    let v = vec![6, 2, 1, 9, 3, 8, 11];

    let f1 = v.par_iter().find_any(|&&x| x == 9);
    let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
    let f3 = v.par_iter().find_any(|&&x| x > 8);

    assert_eq!(f1, Some(&9));
    assert_eq!(f2, Some(&8));
    assert!(f3 > Some(&8));
}

fn par_sort() {
    use rand::{Rng, thread_rng};
    use rand::distributions::Alphanumeric;

    let mut vec = vec![String::new(); 100_000];
    vec.par_iter_mut().for_each(|p| {
        let mut rng = thread_rng();
        *p = (0..5).map(|_| rng.sample(&Alphanumeric)).collect()
    });
    vec.par_sort_unstable();
}

struct Person {
    age: u32,
}

fn par_map_reduce() {
    let v: Vec<Person> = vec![
        Person { age: 23 },
        Person { age: 19 },
        Person { age: 42 },
        Person { age: 17 },
        Person { age: 17 },
        Person { age: 31 },
        Person { age: 30 },
    ];

    let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30 =  v.par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30)
        .reduce(|| 0, |x, y| x + y);

    let alt_sum_30: u32 = v.par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30)
        .sum();

    let avg_over_30 = sum_over_30 as f32 / num_over_30;
    let alt_avg_over_30 = alt_sum_30 as f32 / num_over_30;

    assert!((avg_over_30 - alt_avg_over_30).abs() < std::f32::EPSILON);
    println!("The average age of people older than 30 is {}", avg_over_30);
}

use std::path::Path;

fn make_thumbnail<PA, PB>(original: PA, thumb_dir: PB, longest_edge: u32)
    -> Result<(), std::io::Error>
where
    PA: AsRef<Path>,
    PB: AsRef<Path>,
{
   let img = image::open(original.as_ref()).unwrap();
   let file_path = thumb_dir.as_ref().join(original);

   Ok(img.resize(longest_edge, longest_edge, image::FilterType::Nearest)
    .save(file_path)?) 
}

fn par_thumbnail() -> Result<(), glob::PatternError> {
    use std::fs::create_dir_all;
    use glob::{glob_with, MatchOptions};

    let options: MatchOptions = Default::default();
    let files: Vec<_> = glob_with("*.png", &options)?
        .filter_map(|x| x.ok())
        .collect();

    if files.len() == 0 {
        panic!("No .png files found in current directory")
    }

    let thumb_dir = "thumbnails";
    create_dir_all(thumb_dir).unwrap();

    println!("Saving {} thumbnails into '{}'...", files.len(), thumb_dir);

    let image_failures: Vec<_> = files
        .par_iter()
        .map(|path| {
            make_thumbnail(path, thumb_dir, 300)
                .map_err(|_| path.display().to_string())
        })
        .filter_map(|x| x.err())
        .collect();

    image_failures.iter().for_each(|x| println!("{}", x));

    println!("{} thumbnails saved successfully", files.len() - image_failures.len());
    Ok(())
}

fn main() {
    par_mut();
    par_predicate_test();
    par_search();
    par_sort();
    par_map_reduce();
    par_thumbnail().unwrap();
}
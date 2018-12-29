#!/usr/bin/env run-cargo-script
// cargo-deps: crossbeam, lazy_static,  walkdir, ring, num_cpus, threadpool, num, image

extern crate crossbeam;
#[macro_use]
extern crate lazy_static;
extern crate walkdir;
extern crate ring;
extern crate num_cpus;
extern crate threadpool;
extern crate num;
extern crate image;

use std::cmp;

fn main() -> Result<(), &'static str> {
    let arr = &[-4, 1, 10, 25];
    let max = find_max(arr, 0, arr.len());
    assert_eq!(25, max);

    global_mut_state().unwrap();
    calculate_sha1().unwrap();
    draw_fractal().unwrap();
    Ok(())
}

fn find_max(arr: &[i32], start: usize, end: usize) -> i32 {
    const THRESHOLD: usize = 2;
    if end - start <= THRESHOLD {
        return *arr.iter().max().unwrap();
    }

    let mid = start + (end - start) / 2;
    crossbeam::thread::scope(|scope| {
        let left = scope.spawn(|_| find_max(arr, start, mid));
        let right = scope.spawn(|_| find_max(arr, mid, end));

        cmp::max(left.join().unwrap(), right.join().unwrap())
    }).unwrap()
}

use std::sync::Mutex;

lazy_static! {
    static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn insert(fruit: &str) -> Result<(), &'static str> {
    let mut db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;
    db.push(fruit.to_string());
    Ok(())
}

fn global_mut_state() -> Result<(), &'static str> {
    insert("apple")?;
    insert("orange")?;
    insert("peach")?;

    {
        let db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;
        db.iter().enumerate().for_each(|(i, item)| println!("{}: {}", i, item));
    }

    insert("grape")?;
    Ok(())
}

use std::{fs::File, path::Path};

fn is_iso(entry: &Path) -> bool {
    match entry.extension() {
        Some(e) if e.to_string_lossy().to_lowercase() == "iso" => true,
        _ => false,
    }
}

fn compute_digest<P: AsRef<Path>>(filepath: P)
    -> Result<(ring::digest::Digest, P), std::io::Error> {
    use std::io::{BufReader, Read};
    use ring::digest::{Context, SHA1};
    
    let mut buf_reader = BufReader::new(File::open(&filepath)?);
    let mut context = Context::new(&SHA1);
    let mut buffer = [0; 1024];

    loop {
        let count = buf_reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok((context.finish(), filepath))
}

fn calculate_sha1() -> Result<(), std::io::Error> {
    use walkdir::WalkDir;
    use threadpool::ThreadPool;
    use std::sync::mpsc::channel;

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    for entry in WalkDir::new("/home/yogesh/Downloads/utils")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir() && is_iso(e.path())) {
            let path = entry.path().to_owned();
            let tx = tx.clone();
            pool.execute(move || {
                let digest = compute_digest(path);
                tx.send(digest).expect("Could not send data!");
            });
        }

    drop(tx);
    for t in rx.iter() {
        let (sha, path) = t?;
        println!("{:?} {:?}", sha, path);
    }

    Ok(())
}

use image::{Pixel, Rgb};

fn wavelength_to_rgb(wavelength: u32) -> Rgb<u8> {
    let wave = wavelength as f32;

    let (r, g, b) = match wavelength {
        380...439 => ((440. - wave) / (440. - 380.), 0.0, 1.0),
        440...489 => (0.0, (wave - 440.) / (490. - 440.), 1.0),
        490...509 => (0.0, 1.0, (510. - wave) / (510. - 490.)),
        510...579 => ((wave - 510.) / (580. - 510.), 1.0, 0.0),
        580...644 => (1.0, (645. - wave) / (645. - 580.), 0.0),
        645...780 => (1.0, 0.0, 0.0),
        _ => (0.0, 0.0, 0.0),
    };

    let factor = match wavelength {
        380...419 => 0.3 + 0.7 * (wave - 380.) / (420. - 380.),
        701...780 => 0.3 + 0.7 * (780. - wave) / (780. - 700.),
        _ => 1.0,
    };

    let (r, g, b) = (normalize(r, factor), normalize(g, factor), normalize(b, factor));
    Rgb::from_channels(r, g, b, 0)
}

fn normalize(color: f32, factor: f32) -> u8 {
    ((color * factor).powf(0.8) * 255.) as u8
}

use num::complex::Complex;

fn julia(c: Complex<f32>, x: u32, y: u32, width: u32, height: u32, max_iter: u32) -> u32 {
    let width = width as f32;
    let height = height as f32;

    let mut z = Complex {
        re: 3.0 * (x as f32 - 0.5 * width) / width,
        im: 2.0 * (y as f32 - 0.5 * height) / height,
    };

    let mut i = 0;
    for t in 0..max_iter {
        if z.norm() >= 2.0 {
            break;
        }
        z = z * z + c;
        i = t;
    }
    i
}

fn draw_fractal() -> Result<(), std::io::Error> {
    use std::sync::mpsc::channel;
    use threadpool::ThreadPool;

    let (width, height) = (1920, 1080);
    let mut img = image::ImageBuffer::new(width, height);
    let iterations = 300;

    let c = Complex::new(-0.8, 0.156);

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    for y in 0..height {
        let tx = tx.clone();
        pool.execute(move || for x in 0..width {
            let i = julia(c, x, y, width, height, iterations);
            let pixel = wavelength_to_rgb(380 + i * 400 / iterations);
            tx.send((x, y, pixel)).expect("Could not send data!");
        });
    }

    for _ in 0..(width * height) {
        let (x, y, pixel) = rx.recv().unwrap();
        img.put_pixel(x, y, pixel);
    }
    let _ = img.save("output.png")?;
    Ok(())
}
use clap::{Parser};
use chemfiles::{Frame, Selection, Trajectory};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to sdf file
    filename: String,

    /// Number of partitions to split
    partitions: usize,
}

fn main() {
    let args = Args::parse();

    let npartitions = args.partitions;
    let filename = args.filename;

    let mut trajectory = Trajectory::open(&filename, 'r').unwrap();
    let mut frame = Frame::new();

    let nmols = trajectory.nsteps();
    println!("Number of molecules: {}", nmols);

    let per_partition: i32= (nmols / npartitions) as i32;

    let mut partition = 0;
    let mut count = 0;
    let mut out = Trajectory::open(&format!("{}-{}.sdf", filename, partition), 'w').unwrap();

    while let Ok(_) = trajectory.read(&mut frame) {
        if count == per_partition {
            partition += 1;
            count = 0;
            let fname_without_extension = filename.split('.').collect::<Vec<&str>>()[0];
            out = Trajectory::open(&format!("{}-{}.sdf", fname_without_extension, partition), 'w').unwrap();
        }
        out.write(&frame).unwrap();
        count += 1;
    }
}
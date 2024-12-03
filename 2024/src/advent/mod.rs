mod util;
mod d01;
mod d02;
mod d03;

pub fn run (exercise: u8, part: u8, file: String) -> Result<(), Box<dyn std::error::Error>> {
    match (exercise, part) {
        (1,1) => { d01::pt1(file) }
        (1,2) => { d01::pt2(file) }
        (2,1) => { d02::pt1(file) }
        (2,2) => { d02::pt2(file) }
        (3,1) => { d03::pt1(file) }
        (3,2) => { d03::pt2(file) }
        _ => { print!("No such exercise found: {}, pt{}", exercise, part); Ok(()) }
    }
}

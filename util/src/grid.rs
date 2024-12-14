#[allow(dead_code)]
pub fn print<T>(grid: &[Vec<T>], spacing: usize, dot: T)
where
    T: Sized + Eq + PartialEq + std::fmt::Display,
{
    for row in grid.iter() {
        for item in row.iter() {
            if *item == dot {
                print!("{0:>1$}", '.', spacing);
            } else {
                print!("{0:>1$}", item, spacing);
            }
        }
        println!();
    }
}

#[allow(dead_code)]
pub fn print_grid<T>(grid: &mut [&mut [T]], spacing: usize)
where
    T: Sized + std::fmt::Display,
{
    for row in grid.iter() {
        for item in row.iter() {
            print!("{0:>1$}", item, spacing);
        }
        println!();
    }
}


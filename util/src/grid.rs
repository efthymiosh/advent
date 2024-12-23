pub fn print<T>(grid: &[Vec<T>], spacing: usize, dot: T)
where
    T: Sized + Eq + PartialEq + std::fmt::Display,
{
    print!("{0:>1$}", ' ', spacing);
    for i in 0..grid[0].len() {
        print!("{0:>1$}", i, spacing);
    }
    println!();
    for (idx, row) in grid.iter().enumerate() {
        print!("{0:>1$}", idx, spacing);
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

pub fn find<T>(grid: &[Vec<T>], element: &T) -> Option<(usize, usize)>
where
    T: PartialEq,
{
    grid.iter()
        .enumerate()
        .find_map(|(x, ve)| ve.iter().position(|n| n == element).map(|y| (x, y)))
}

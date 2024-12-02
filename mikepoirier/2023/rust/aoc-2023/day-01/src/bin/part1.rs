use day_01::{part1, Result};
use input::load_file;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = load_file(1)?;
    let part1 = part1(input);
    println!("Part 1: {part1}");

    Ok(())
}

use day_02::{part2, Result};
use input::load_file;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = load_file(2)?;
    let part2 = part2(input)?;
    println!("Part 2: {part2}");
    Ok(())
}

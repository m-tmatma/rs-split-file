use std::fs::File;
use memmap::Mmap;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let input = &args[1];

    let file = File::open(input)?;
    let mmap = unsafe { Mmap::map(&file)? };

    let file_size = mmap.len();
    let div_size  = 4;
    let data_size = file_size / div_size;

    // 入力ファイルをサイズで 4 分割する
    for i in 0..div_size {
        let data = &mmap[i*data_size..i*data_size+data_size];

        let output = format!("{}.part{}", input, i);
        let mut file = File::create(output)?;
        file.write_all(data)?;
    }

    let data = &mmap[div_size*data_size..file_size];

    let output = format!("{}.part{}", input, div_size);
    let mut file = File::create(output)?;
    file.write_all(data)?;

    Ok(())
}

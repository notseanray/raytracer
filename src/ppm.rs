use std::{
    fs::File,
    io::{Result, Write},
};

// P3
// TODO generic
struct Ppm {
    width: usize,
    height: usize,
    max_color_value: u8,
    data: Vec<u8>,
}

impl Ppm {
    pub fn write(self, filename: &str) -> Result<()> {
        let mut row = 0;
        let header = format!(
            "P3\n{} {}\n{}\n{}",
            self.width,
            self.height,
            self.max_color_value,
            self.data
                .chunks(3 * self.width)
                .map(|x| {
                    row += 1;
                    println!("Output {}/{}", row, self.height);
                    format!(
                    "{}\n",
                    x.iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )})
                .collect::<Vec<String>>()
                .join("")
        );
        File::create_new(filename)?.write_all(header.as_bytes())?;
        Ok(())
    }
}

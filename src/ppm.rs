use std::fs::File;
use std::io::{Read, Result as IOResult, Error as IOError};

fn make_io_error(text: &'static str) -> IOError
{
    IOError::new(std::io::ErrorKind::Other, text)
}

fn read_to_end_of_line(input: &mut impl Read)
{
    let mut byte = [0;1];
    loop
    {
        input.read_exact(&mut byte).unwrap();
        if byte[0] == b'\n'
        {
            return;
        }
    }
}

fn read_decimal(input: &mut impl Read) -> IOResult<u16>
{
    let mut byte = [0;1];

    loop
    {
        input.read_exact(&mut byte).unwrap();
        match byte[0]
        {
            b' ' | b't' | b'\r' => {},
            b'#' => read_to_end_of_line(input),
            _ => break,
        }
    }

    if !byte[0].is_ascii_digit()
    {
        return Err(make_io_error("failed parsing number"));
    }
    let mut result: u16 = 0;
    while byte[0].is_ascii_digit()
    {
        let value = u16::from(byte[0] - b'0');
        result = result
            .checked_mul(10)
            .map(|result| result + value)
            .ok_or_else(|| 0).unwrap();

        input.read_exact(&mut byte).unwrap();
    }

    if byte[0].is_ascii_whitespace()
    {
        Ok(result)
    }
    else
    {
        eprintln!("Invalid character after number");
        return Err(make_io_error("Invalid character after number"));
    }
}

fn parse_ppm(input: &mut impl Read) -> IOResult<Vec<u8>>
{
    let mut header = [0;2];
    input.read_exact(&mut header).expect("Failed to read of bytes");
    if header != *b"P6"
    {
        eprintln!("Wrong ppm format header");
        return Err(make_io_error("Wrong ppm format"))
    }
    read_to_end_of_line(input);
    let width = read_decimal(input)?;
    let height = read_decimal(input)?;
    let max = read_decimal(input)?;

    if max != 255
    {
        eprintln!("Image declares max pixel value of {} but expected 255", max)
    }

    let image_size = (height as usize * width as usize) * 3;

    let mut image = vec![0;image_size];

    input.read_exact(&mut image).unwrap();

    return Ok(image);
}

pub fn parse_from_file(path: &str) -> IOResult<Vec<u8>>
{
    let mut file = File::open(path).expect("Error opening file");
    parse_ppm(&mut file)
}

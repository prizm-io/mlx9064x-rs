use std::env;
use std::error::Error as StdError;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use linux_embedded_hal::I2cdev;
use mlx9064x::{Mlx90640Driver, Mlx90641Driver, Mlx90642Driver};

fn main() -> Result<(), AnyError> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        return Err(AnyError::String(
            "Three arguments required: [640|641|642] <I2C bus> <camera address>".to_string(),
        ));
    }
    let address: u8 = if args[3].starts_with("0x") {
        let hex_digits = args[3].split_at(2).1;
        u8::from_str_radix(&hex_digits, 16)?
    } else {
        args[3].parse()?
    };
    let bus_path = Path::new(&args[2]);
    let bus = I2cdev::new(bus_path)?;
    let (temperatures, width) = match args[1].as_ref() {
        "640" => {
            let mut camera = Mlx90640Driver::new(bus, address)?;
            let mut temperatures = vec![0f32; camera.height() * camera.width()];
            let delay = Duration::from_millis(500);
            wait_for_frame(
                || {
                    camera
                        .generate_image_if_ready(&mut temperatures)
                        .map_err(AnyError::from)
                },
                delay,
            )?;
            wait_for_frame(
                || {
                    camera
                        .generate_image_if_ready(&mut temperatures)
                        .map_err(AnyError::from)
                },
                delay,
            )?;
            (temperatures, camera.width())
        }
        "641" => {
            let mut camera = Mlx90641Driver::new(bus, address)?;
            let mut temperatures = vec![0f32; camera.height() * camera.width()];
            let delay = Duration::from_millis(500);
            wait_for_frame(
                || {
                    camera
                        .generate_image_if_ready(&mut temperatures)
                        .map_err(AnyError::from)
                },
                delay,
            )?;
            wait_for_frame(
                || {
                    camera
                        .generate_image_if_ready(&mut temperatures)
                        .map_err(AnyError::from)
                },
                delay,
            )?;
            (temperatures, camera.width())
        }
        "642" => {
            let mut camera = Mlx90642Driver::new(bus, address)?;
            let mut temperatures = vec![0f32; camera.height() * camera.width()];
            let delay = Duration::from_millis(500);
            wait_for_frame(
                || {
                    camera
                        .generate_image_if_ready(&mut temperatures)
                        .map_err(AnyError::from)
                },
                delay,
            )?;
            wait_for_frame(
                || {
                    camera
                        .generate_image_if_ready(&mut temperatures)
                        .map_err(AnyError::from)
                },
                delay,
            )?;
            (temperatures, camera.width())
        }
        _ => {
            return Err(AnyError::String(
                "The second argument must be 640, 641, or 642".to_string(),
            ));
        }
    };
    print_temperatures(&temperatures, width);
    println!();
    Ok(())
}

fn print_temperatures(temperatures: &[f32], width: usize) {
    for (count, temperature) in temperatures.iter().enumerate() {
        if count % width == 0 {
            println!();
        }
        print!("{:4.2}  ", temperature);
    }
}

fn wait_for_frame<F>(mut generator: F, delay: Duration) -> Result<(), AnyError>
where
    F: FnMut() -> Result<bool, AnyError>,
{
    while !generator()? {
        sleep(delay);
    }
    Ok(())
}

// It's anyhow::Error, but less functional and less tested.
#[derive(Debug)]
enum AnyError {
    Wrapped(Box<dyn StdError + 'static>),
    String(String),
}

impl std::fmt::Display for AnyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyError::Wrapped(err) => write!(f, "{}", err),
            AnyError::String(s) => write!(f, "{}", s),
        }
    }
}

impl<E> From<E> for AnyError
where
    E: StdError + 'static,
{
    fn from(err: E) -> Self {
        AnyError::Wrapped(Box::new(err))
    }
}

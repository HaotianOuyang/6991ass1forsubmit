mod my_module;

use clap::Parser;
use my_module::logo_reader::parse_logo_file;
use my_module::logo_interpreter::Interpreter;



/// A simple program to parse four arguments using clap.
#[derive(Parser)]
struct Args {
    /// Path to a file
    file_path: std::path::PathBuf,

    /// Path to an svg or png image
    image_path: std::path::PathBuf,

    /// Height
    height: u32,

    /// Width
    width: u32,
}


fn main() -> Result<(), ()> {
    let args: Args = Args::parse();

    // Access the parsed arguments
    let file_path = args.file_path; // file that I "need" to read
    let image_path = args.image_path;
    let height = args.height;
    let width = args.width;

    let program = match parse_logo_file(file_path){
        Ok(program) => program,
        Err(e) => {
            eprintln!("Error parsing logo file: {}", e);
            return Err(());
        }
    };
    let mut interpreter = Interpreter::new(width as i32, height as i32);

    let image = match interpreter.interpret(&program){
        Ok(image) => image,
        Err(e) => {
            eprint!("Error interpreting program: {}", e);
            return Err(());
        }
    };
    //let image = Image::new(width, height);
    //read file proccess as program
    match image_path.extension().and_then(|s| s.to_str()) {
        Some("svg") => {
            let res = image.save_svg(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving svg: {e}");
                return Err(());
            }
        }
        Some("png") => {
            let res = image.save_png(&image_path);
            if let Err(e) = res {
                eprintln!("Error saving png: {e}");
                return Err(());
            }
        }
        _ => {
            eprintln!("File extension not supported");
            return Err(());
        }
    }

    Ok(())
}

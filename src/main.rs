use std::fs::{self, File};
use std::io::{self, copy};
use std::path::Path;

fn main() {
    // Exit the program with the appropriate exit code based on the result of the `extract_zip` function.
    std::process::exit(match extract_zip() {
        Ok(_) => 0, // Exit with code 0 if extraction succeeds.
        Err(e) => {
            eprintln!("{}", e); // Print the error message if extraction fails.
            1 // Exit with code 1 if extraction fails.
        }
    });
}

fn extract_zip() -> Result<(), io::Error> {
    // Collect command line arguments into a vector.
    let args: Vec<String> = std::env::args().collect();

    // Check if there are enough arguments.
    if args.len() < 2 {
        println!("Usage: {} <file_name>", args[0]); // Print the usage message if there are not enough arguments.
        return Ok(()); // Return early with Ok(()) if there are not enough arguments.
    }

    // Get the file name from the command line arguments.
    let fname = Path::new(&args[1]);

    // Open the file.
    let file = File::open(&fname)?;

    // Create a ZipArchive from the file.
    let mut archive = zip::ZipArchive::new(file)?;

    // Iterate over each file in the archive.
    for i in 0..archive.len() {
        // Get the file at the current index.
        let mut file = archive.by_index(i)?;

        // Get the path to extract the file to.
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue, // Skip to the next file if the path is None.
        };

        // Get the comment associated with the file.
        let comment = file.comment();
        if !comment.is_empty() {
            println!("File {} comment: {}", i, comment); // Print the file comment if it's not empty.
        }

        // Check if the file is a directory.
        if file.name().ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display()); // Print a message indicating the directory extraction.
            fs::create_dir_all(&outpath)?; // Create the directory.
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            ); // Print a message indicating the file extraction and its size.

            // Create parent directories if they don't exist.
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }

            // Create and copy the file contents to the output path.
            let mut outfile = File::create(&outpath)?;
            copy(&mut file, &mut outfile)?;
        }

        // Set file permissions if running on a Unix-like system.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(()) // Return Ok(()) if extraction succeeds.
}

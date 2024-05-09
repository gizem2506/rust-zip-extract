use std::fs;
use std::io;

fn main() {
    // Step 1: Process the real program in the `real_main` function
    // For a clean exit, use `std::process::exit` in the `main` function
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    // Step 2: Create a vector to collect user input from the CLI
    let args: Vec<_> = std::env::args().collect();

    // Step 3: Show usage instructions if arguments are less than 2
    if args.len() < 2 {
        println!("Usage: {} <file_name>", args[0]);
        return 1;
    }

    // Step 4: The file name is at the 2nd position (index 1) in the arguments
    let fname = std::path::Path::new(&*args[1]);

    // Step 5: Open the file using standard fs
    let file = match fs::File::open(&fname) {
        Ok(file) => file,
        Err(_) => {
            println!("The specified file could not be found or opened.");
            return 1;
        }
    };

    // Step 6: Use the archive reader function
    let mut archive = match zip::ZipArchive::new(file) {
        Ok(archive) => archive,
        Err(_) => {
            println!("The zip archive could not be opened or is invalid.");
            return 1;
        }
    };

    // Step 7: Iterate over each file in the archive
    for i in 0..archive.len() {
        let mut file = match archive.by_index(i) {
            Ok(file) => file,
            Err(_) => {
                println!("The file in the archive could not be opened.");
                return 1;
            }
        };

        // Step 8: Set the path where the files will be extracted
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        // Step 9: Get the file comment and print if it exists
        let comment = file.comment();
        if !comment.is_empty() {
            println!("File {} comment: {}", i, comment);
        }

        // Step 10: Check if the file is a directory
        if (*file.name()).ends_with('/') {
            // Step 11: If the file is a directory, create the directory
            println!("File {} extracted to \"{}\"", i, outpath.display());
            if let Err(_) = fs::create_dir_all(&outpath) {
                println!("An error occurred while creating the directory.");
                return 1;
            }
        } else {
            // Step 12: If the file is a regular file, extract the file
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );

            // Step 13: Check the parent directory of the file and create it if it doesn't exist
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    if let Err(_) = fs::create_dir_all(&p) {
                        println!("An error occurred while creating the directory.");
                        return 1;
                    }
                }
            }

            // Step 14: Create a new file to extract the file and copy its content
            let mut outfile = match fs::File::create(&outpath) {
                Ok(outfile) => outfile,
                Err(_) => {
                    println!("An error occurred while creating the file.");
                    return 1;
                }
            };

            if let Err(_) = io::copy(&mut file, &mut outfile) {
                println!("An error occurred while copying the file.");
                return 1;
            }
        }

        // Step 15: Get and set permissions for the extracted files (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                if let Err(_) = fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)) {
                    println!("An error occurred while setting file permissions.");
                    return 1;
                }
            }
        }
    }

    // Step 16: The program has completed successfully, return the exit code
    0
}
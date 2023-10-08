use std::env;
use std::process::Command;
use std::path::Path;

fn main() {
    // Check if GPG is installed
    if Command::new("gpg")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_err()
    {
        eprintln!("GPG (GNU Privacy Guard) is not installed. Please install it first.");
        std::process::exit(1);
    }

    // Check for the correct number of arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <file_to_encrypt> <output_file>", args[0]);
        std::process::exit(1);
    }

    let file_to_encrypt = &args[1];
    let output_file = &args[2];

    // Check if the input file exists
    if !Path::new(file_to_encrypt).exists() {
        eprintln!("File '{}' does not exist.", file_to_encrypt);
        std::process::exit(1);
    }

    // Prompt the user for a passphrase securely
    let passphrase = rpassword::read_password_from_tty(Some("Enter passphrase: ")).unwrap();

    // Encrypt the file with GPG using the provided passphrase and custom output file name
    let encrypt_status = Command::new("gpg")
        .arg("--symmetric")
        .arg("--batch")
        .arg("--passphrase")
        .arg(passphrase)
        .arg("--output")
        .arg(output_file)
        .arg(file_to_encrypt)
        .status();

    match encrypt_status {
        Ok(status) if status.success() => {
            println!("Encryption successful. The encrypted file is '{}'", output_file);
        }
        _ => {
            eprintln!("Encryption failed.");
            std::process::exit(1);
        }
    }
}

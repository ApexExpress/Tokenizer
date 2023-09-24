use std::env;
use std::process::Command;

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
    if args.len() != 2 {
        eprintln!("Usage: {} <file_to_encrypt>", args[0]);
        std::process::exit(1);
    }

    let file_to_encrypt = &args[1];

    // Encrypt the file with GPG
    let encrypt_status = Command::new("gpg")
        .arg("-c")
        .arg(file_to_encrypt)
        .status();

    match encrypt_status {
        Ok(status) if status.success() => {
            println!("Encryption successful. The encrypted file is {}.gpg", file_to_encrypt);
        }
        _ => {
            eprintln!("Encryption failed.");
            std::process::exit(1);
        }
    }
}


if [ $? -eq 0 ]; then
    echo "Encryption successful. The encrypted file is ${file_to_encrypt}.gpg"
else
    echo "Encryption failed."
fi

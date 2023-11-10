#include <iostream>
#include <fstream>
#include <cstdlib>
#include <cstring>
#include <vector>

int main(int argc, char* argv[]) {
    // Check if GPG is installed
    if (system("gpg --version > /dev/null 2>&1") != 0) {
        std::cerr << "GPG (GNU Privacy Guard) is not installed. Please install it first." << std::endl;
        return 1;
    }

    // Check for the correct number of arguments
    if (argc != 3) {
        std::cerr << "Usage: " << argv[0] << " <file_to_encrypt> <output_file>" << std::endl;
        return 1;
    }

    const char* file_to_encrypt = argv[1];
    const char* output_file = argv[2;

    // Check if the input file exists
    std::ifstream input(file_to_encrypt);
    if (!input) {
        std::cerr << "File '" << file_to_encrypt << "' does not exist." << std::endl;
        return 1;
    }
    input.close();

    // Prompt the user for a passphrase securely
    std::cout << "Enter passphrase: ";
    std::string passphrase;
    std::cin >> passphrase;

    // Encrypt the file with GPG using the provided passphrase and custom output file name
    std::string gpg_command = "gpg --symmetric --batch --passphrase '";
    gpg_command += passphrase;
    gpg_command += "' --output '";
    gpg_command += output_file;
    gpg_command += "' '";
    gpg_command += file_to_encrypt;
    gpg_command += "'";
    
    int encrypt_status = system(gpg_command.c_str());

    if (encrypt_status == 0) {
        std::cout << "Encryption successful. The encrypted file is '" << output_file << "'" << std::endl;
        return 0;
    } else {
        std::cerr << "Encryption failed." << std::endl;
        return 1;
    }
}

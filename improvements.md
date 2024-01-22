
Additionally, broken software is an issue: here are some improvements for your code base.

Specify Output File Name: Allow the user to specify the name of the encrypted output 
file instead of always appending ".gpg" to the input file name.

Password Prompt: Instead of hardcoding a password or passphrase in the code, 
you can prompt the user to enter a passphrase securely.

Error Handling: Improve error handling by providing more informative error 
messages and handling different error cases separately.

File Existence Check: Check if the input file exists before attempting encryption.

Logging: Add logging to keep track of what the program is doing, especially if you plan to run it in a production environment.

Decryption Option: Add an option to decrypt files in addition to encrypting them.

Usage Instructions: Improve the usage message to provide more detailed instructions on how to use the program, 
including available options.

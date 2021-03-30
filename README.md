Circuit Lab Simulation software, developed by Sleep Ibis LLC and funded by Northwestern University,
is circuit simulation software specifically designed to assist teachers in designing and
deploying circuit labs for beginning physics/engineering  students.
The program runs on Windows, MacOs and Linux operating systems.
The program was written in the [rust](https://www.rust-lang.org/) programming language.


## Getting started with Rust

If you are new to rust please take a look 
[here](https://www.rust-lang.org/learn/get-started), the rust installation guide, and
[here](https://doc.rust-lang.org/book/), a beginner to intermediate guide useful for those getting
started.



## Getting started programming

After the rust compiler has been installed work with the circuit simulation can begin. 
Circuit simulation documentation can be accessed using the following command, `cargo doc --open`. Enter this command into your favorite shell/terminal. 
The command will compile the documentation and bring it up in your favorite browser. Don't be alarmed if takes a while
depending on the operating system it can take as long as a minute. 
The documentation is the started point for those who want to alter the source code.


To compile to source code use following command,
`cargo run --release`.
The `--release` flag dictates a release build will be compiled. When shipping a new version of the program you should always compile using the release flag.


## Getting started as a user
Once you've gotten the program up and running working with it should be fairly intuitive. The panel on the right side of the window contains instructions to help users get started.
Additionally, circuitsim_instructions.txt, which can be found the root directory of this package, offers additionally instructions for teachers and TA's.





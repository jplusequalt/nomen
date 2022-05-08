`nomen` is a miniature CLI for easily creating, updating, and removing aliases from your terminal.

## Installing
The following is subject to change as I am still new to packaging, and the steps to get this up and running are a bit awkward:

Clone the repo
`git clone https://github.com/jplusequalt/nomen.git`
Build the project (you will need Cargo/rustc):
`cargo build --release`
Add nomen to your path by adding the following to your .bashrc file:
`export PATH=$PATH:/where_you_saved_the_repo/nomen/target/release`
Verify that nomen is working by running:
`nomen 0.1.0

USAGE:
    nomen <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    create    Create new aliases
    help      Print this message or the help of the given subcommand(s)
    list      List aliases
    remove    Remove aliases
    update    Update existing aliases
`

## Much to come, lots to do
nomen is still very much a work in progress. There are some immediate features that will follow shortly:
- Add a config command to configure where to place aliases
- Add more rebust error handling

Some features that I would like to add, but I'm unsure about their scope:
- Add support for zsh, possibly powershell
- auto-completion

## Contribute!
This is a solo project for the meantime, however, I would love to have others contribute!
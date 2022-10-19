`nomen` is a miniature CLI for easily creating, updating, and removing aliases from your terminal.

## Installing
The following is subject to change as I am still new to packaging, and the steps to get this up and running are a bit awkward:

Clone the repo

`git clone https://github.com/jplusequalt/nomen.git`

Build the project (you will need Cargo/rustc):

`cargo build --release`

Add nomen to your path by adding the following to your .bashrc file:

`export PATH=$PATH:/where_you_saved_the_repo/nomen/target/release`

Verify that nomen is working by typing `nomen`:

```

nomen 0.1.0

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
```


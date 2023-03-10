use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(
    author = "Author Name",
    version,
    about = "A Very simple Package Hunter"
)]
struct Arguments {
    package_name: String,
    max_depth: usize,
}

fn main() {
    println!(
        "{} - {}\n{}{}{}{}",
        crate_name!(),
        crate_version!(),
        "Enter .exit to quit.\n",
        "Enter .help for usage hints.\n",
        "Connected to a transient in-memory database.\n",
        "Use '.open FILENAME' to reopen on a persistent database."
    );
}

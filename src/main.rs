// ssh-manager: Command-line SSH server manager
//
// Copyright © 2018 Hugo Locurcio and contributors
//
// Licensed under the Apache 2.0 license or the MIT license (at your option),
// see `LICENSE-APACHE.md` and `LICENSE-MIT.md` included in the source
// distribution for more information.

extern crate ansi_term;
#[macro_use]
extern crate clap;
extern crate serde;
extern crate toml;

use clap::App;

struct SshServer {
    name: String,
    user: String,
    host: String,
    port: Option<u16>,
}

// Adds a server to the SSH servers list.
fn add_server(name: &str, host: &str) {
    println!("Added server \"{}\": {}", name, host);
}

// Main program execution (parses command-line arguments and does stuff based on them).
fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Add a server
    if let Some(matches) = matches.subcommand_matches("add") {
        add_server(
            matches.value_of("name").unwrap(),
            matches.value_of("host").unwrap(),
        )
    }
}

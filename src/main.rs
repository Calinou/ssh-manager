// ssh-manager: Command-line SSH server manager
//
// Copyright © 2018 Hugo Locurcio and contributors
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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

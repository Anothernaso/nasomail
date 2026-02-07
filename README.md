#  NasoMail
## A Custom Mailing System

Lightweight, self-hosted mail infrastructure.

---

NasoMail servers are designed to have just a few people (users) each, but could theoretically have hundreds of users.


Note that this has not been tested yet and is just a very rough estimation.

---

## How to Build

### Prerequisites

Building NasoMail should be extremely easy
since it has very few system dependencies.


Before building make sure that you have
the [Rust compiler toolchain](https://rust-lang.org/learn/get-started/) and
the [Cargo](https://doc.rust-lang.org/cargo/index.html) build system installed,


if not, you can install them using your 
system package manager,

e.g,
```sh
sudo dnf install rust  # Install the Rust toolchain
sudo dnf install cargo # Install the Cargo build system
```
on [Red Hat](https://www.redhat.com/) distributions such as [Fedora](https://fedoraproject.org/)

or by running the [rustup](https://rust-lang.org/learn/get-started/) install script:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Building the Project

First, clone the repository:
```sh
git clone https://github.com/Anothernaso/nasomail.git
```
Then, enter the project directory:
```sh
cd nasomail
```

Finally, build it using [Cargo](https://doc.rust-lang.org/cargo/index.html):
```sh
cargo build
```

---

## How to Run

### Prerequisites

[How to Run](#how-to-run)

Make sure that you are in the project directory.

### Running the Server

First, enter the server directory:
```sh
cd nasomail_server
```

Then, run the server using [Cargo](https://doc.rust-lang.org/cargo/index.html):
```sh
cargo run
```

### Running the Client

First, enter the client directory:
```sh
cd nasomail_client
```

Then, run the client using [Cargo](https://doc.rust-lang.org/cargo/index.html):
```sh
cargo run
```

Now you should see a list of available commands.

---

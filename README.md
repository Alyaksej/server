# Server

This program is used to receive data via a Unix socket and then process it in a connected library written in C.

## Description of programm

The following libraries are connected to the program:

`use tokio::net::UnixDatagram`

`use tokio::io::Interest`

`use std::{fs, io}`

`use std::os::raw::{c_int}`

`use std::time::Instant`

First of all we need to connect library `libc` for providing all of the definitions necessary to easily interoperate with C code (or "C-like" code) on each of the platforms that Rust supports. 

Then ve need to connect library `array_processing`, its description you can see [here](https://github.com/Alyaksej/array_processing)

After connecting the library we need to defining an external function `array_processing` using the Rust FFI to interface the function eritten on C.

The program receives data via a Unix socket. The socket file is created automatically when the program is launched at `/tmp/socket_data.sock`

the maximum volume of the buffer receiving data is 2000,000,000 bytes. After processing of recievin data array the result is writing to `result_c_ptr`.

The `Cargo.toml` file contains the dependencies required by the server.

```
[dependencies]
tokio = { version = "1", features = ["full"] }
libc = "0.2.0"
```
The program also counts the megabytes received by the server and calculates the operating time of the program. 

The file `build.rs` contains the name of the C library and its path.

## Building and running the program

As programm written o Rust you need to install `Cargo` and `rustup`  on your PC, process of instalation described [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Next step is clone project files to your PC usin command and build the programm
```
git clone git@github.com:Alyaksej/server.git
cd server
cargo build --release
cd build/release
./server
```

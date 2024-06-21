# Server

This program is used to receive data via a Unix socket and then process it in a connected library written in C.

## Description of programm

The following libraries are connected to the program:

use tokio::net::UnixDatagram;

use tokio::io::Interest;

use std::{fs, io};

use std::os::raw::{c_int};

use std::time::Instant;

The program receives data via a Unix socket. The socket file is created automatically when the program is launched at /tmp/socket_data.sock

the maximum volume of the buffer receiving data is 2000,000,000 bytes,

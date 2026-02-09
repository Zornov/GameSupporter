use anyhow::{ensure, Result};
use bytemuck::{bytes_of, Pod, Zeroable};
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;
use crate::mouse::mouse::Mouse;

/// KmBox communication protocol.
#[repr(u32)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
enum Cmd {
    Connect = 0xAF3C2828,
    Move = 0xAEDE7345,
    AutoMove = 0xAEDE7346,
    Left = 0x9823AE8D,
    Right = 0x238D8212,
    Reboot = 0xAA8855AA,
}

/// Header for KmBox packets.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Header {
    mac: u32,
    time: u32,
    index: u32,
    cmd: u32,
}

// Mouse event packet for KmBox.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct MousePacket {
    head: Header,
    button: i32,
    x: i32,
    y: i32,
    wheel: i32,
}

/// KmBox mouse control client.
pub struct KmBox {
    socket: UdpSocket,
    addr: SocketAddr,
    mac: u32,
    index: u32,
}

impl KmBox {
    /// Creates a new `KmBox` client and connects;
    /// - `ip`: IP address of the KmBox device.
    /// - `port`: UDP port of the KmBox device.
    /// - `mac_hex`: MAC address of the KmBox device as a hex string (e.g. "AABBCCDD").
    pub fn connect(ip: &str, port: u16, mac_hex: &str) -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_read_timeout(Some(Duration::from_millis(1000)))?;

        let addr: SocketAddr = format!("{ip}:{port}").parse()?;
        let mac = u32::from_str_radix(mac_hex, 16)?;

        let mut km = Self { socket, addr, mac, index: 0 };
        km.send_cmd(Cmd::Connect)?;

        let mut buf = [0u8; 1024];
        let (len, from) = km.socket.recv_from(&mut buf)?;

        ensure!(from == km.addr, "Received response from unexpected address: {from}");
        ensure!(len >= size_of::<Header>(), "Received packet too small");

        println!("Connected to {addr}");
        Ok(km)
    }

    /// Generate header for packets, incrementing the index for each new packet.
    fn next_header(&mut self, rand: u32, cmd: Cmd) -> Header {
        self.index += 1;

        Header {
            mac: self.mac,
            time: rand,
            index: self.index,
            cmd: cmd as u32
        }
    }

    /// Send a command packet with no additional data.
    fn send_cmd(&mut self, cmd: Cmd) -> Result<()> {
        let header = self.next_header(0, cmd);

        self.socket.send_to(bytes_of(&header), self.addr)?;
        Ok(())
    }

    /// Send a mouse event packet with the specified parameters.
    fn send_mouse(&mut self, rand: u32, cmd: Cmd, button: i32, x: i32, y: i32) -> Result<()> {
        let packet = MousePacket {
            head: self.next_header(rand, cmd),
            button, x, y,
            wheel: 0,
        };
        self.socket.send_to(bytes_of(&packet), self.addr)?;
        Ok(())
    }
}

/// Implement the `Mouse` trait for `KmBox`.
/// Translates high-level mouse actions into KmBox actions.
impl Mouse for KmBox {

    /// Press or release the left mouse button.
    fn left(&mut self, down: bool) {
        self.send_mouse(0, Cmd::Left, down as i32, 0, 0).ok();
    }

    /// Press or release the right mouse button.
    fn right(&mut self, down: bool) {
        self.send_mouse(0, Cmd::Right, down as i32, 0, 0).ok();
    }


    /// Move the mouse cursor by a relative delta (x, y) without interpolation(delay).
    fn move_delta(&mut self, x: i32, y: i32) {
        self.send_mouse(0, Cmd::Move, 0, x, y).ok();
    }

    /// Move the mouse cursor to an absolute position (x, y) with interpolation over a specified delay in milliseconds.
    fn move_auto(&mut self, x: i32, y: i32, delay: u32) { self.send_mouse(delay, Cmd::AutoMove, 0, x, y).ok(); }


    /// Reboot the remote device.
    fn reboot(&mut self) {
        self.send_cmd(Cmd::Reboot).ok();
    }

    /// Shutdown the remote device.
    fn shutdown(&mut self) {
        unimplemented!()
    }
}
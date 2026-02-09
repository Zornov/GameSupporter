use anyhow::{ensure, Result};
use bytemuck::{bytes_of, Pod, Zeroable};
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::time::Duration;
use rand::random;
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
    SetConfig = 0x1D3D3323,
    ShowPic = 0x12334883, // not implemented yet
    Reboot = 0xAA8855AA,
}

/// Header for KmBox packets.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Header {
    mac: u32,
    rand: u32,
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
    wheel: i32
}

// Set configuration packet for KmBox.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct ConfigPacket {
    head: Header,
    data: [u8; 1024],
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
        let header = km.next_header(Cmd::Connect);
        km.send(header)?;

        let mut buf = [0u8; 1024];
        let (len, from) = km.socket.recv_from(&mut buf)?;

        ensure!(from == km.addr, "Received response from unexpected address: {from}");
        ensure!(len >= size_of::<Header>(), "Received packet too small");

        println!("Connected to {addr}");
        Ok(km)
    }

    pub fn set_config(&mut self, ip: Ipv4Addr, port: u16) -> Result<()> {
        let mut packet = ConfigPacket {
            head: self.next_header(Cmd::SetConfig),
            data: [0u8; 1024]
        };
        println!("{:?}", ip);
        packet.head.rand = u32::from(ip);
        println!("Setting IP: {}", ip);

        let port_bytes = port.to_be_bytes();
        packet.data[0] = port_bytes[0];
        packet.data[1] = port_bytes[1];

        self.send(packet)?;

        Ok(())
    }

    pub fn set_picture(&mut self, _image: Vec<u8>) -> Result<()> {
        unimplemented!()
    }

    /// Generate header for packets, incrementing the index for each new packet.
    fn next_header(&mut self, cmd: Cmd) -> Header {
        self.index = self.index.wrapping_add(1);

        Header {
            mac: self.mac,
            rand: random(),
            index: self.index,
            cmd: cmd as u32
        }
    }

    fn send<T: bytemuck::Pod>(&mut self, packet: T) -> Result<()> {
        self.socket.send_to(bytes_of(&packet), self.addr)?;
        Ok(())
    }
}

/// Implement the `Mouse` trait for `KmBox`.
/// Translates high-level mouse actions into KmBox actions.
impl Mouse for KmBox {

    /// Press or release the left mouse button.
    fn left(&mut self, down: bool) {
        // self.send_mouse(Cmd::Left, down as i32, 0, 0).ok();
    }

    /// Press or release the right mouse button.
    fn right(&mut self, down: bool) {
        // self.send_mouse(Cmd::Right, down as i32, 0, 0).ok();
    }


    /// Move the mouse cursor by a relative delta (x, y) without interpolation(delay).
    fn move_delta(&mut self, x: i32, y: i32) {
        // self.send_mouse(Cmd::Move, 0, x, y).ok();
    }

    /// Move the mouse cursor to an absolute position (x, y) with interpolation over a specified delay in milliseconds.
    fn move_auto(&mut self, x: i32, y: i32, delay: u32) {
        // self.send_mouse(Cmd::AutoMove, 0, x, y).ok();
    }


    /// Reboot the remote device.
    fn reboot(&mut self) {
        let header = self.next_header(Cmd::Reboot);
        self.send(header).ok();
    }

    /// Shutdown the remote device.
    fn shutdown(&mut self) {
        unimplemented!()
    }
}
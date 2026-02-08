use anyhow::{ensure, Result};
use bytemuck::{bytes_of, Pod, Zeroable};
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use crate::mouse::mouse::Mouse;

#[repr(u32)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
enum Cmd {
    Connect = 0xAF3C2828,
    Move = 0xAEDE7345,
    Left = 0x9823AE8D,
    Right = 0x238D8212,
    Reboot = 0xAA8855AA,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Header {
    mac: u32,
    rand: u32,
    index: u32,
    cmd: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct MousePacket {
    head: Header,
    button: i32,
    x: i32,
    y: i32,
    wheel: i32,
    _reserved: [i32; 10],
}

pub struct KmBox {
    socket: UdpSocket,
    addr: SocketAddr,
    mac: u32,
    index: u32,
}

impl KmBox {
    pub fn connect(ip: &str, port: u16, mac_hex: &str) -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_read_timeout(Some(Duration::from_millis(1000)))?;

        let addr: SocketAddr = format!("{ip}:{port}").parse()?;
        let mac = u32::from_str_radix(mac_hex, 16)?;

        let mut km = Self { socket, addr, mac, index: 0 };
        km.send_cmd(Cmd::Connect)?;

        let mut buf = [0u8; 1024];
        let (len, from) = km.socket.recv_from(&mut buf)?;

        ensure!(from == km.addr, "unexpected sender: {from}");
        ensure!(len >= size_of::<Header>(), "response too small");

        println!("Connected to {addr}");
        Ok(km)
    }

    fn next_header(&mut self, cmd: Cmd) -> Header {
        self.index = self.index.wrapping_add(1);
        Header { mac: self.mac, rand: 0, index: self.index, cmd: cmd as u32 }
    }

    fn send_cmd(&mut self, cmd: Cmd) -> Result<()> {
        let header = self.next_header(cmd);
        self.socket.send_to(bytes_of(&header), self.addr)?;
        Ok(())
    }

    fn send_mouse(&mut self, cmd: Cmd, button: i32, x: i32, y: i32) -> Result<()> {
        let packet = MousePacket {
            head: self.next_header(cmd),
            button, x, y,
            wheel: 0,
            _reserved: [0; 10],
        };
        self.socket.send_to(bytes_of(&packet), self.addr)?;
        Ok(())
    }
}

impl Mouse for KmBox {
    fn left(&mut self, down: bool) {
        self.send_mouse(Cmd::Left, down as i32, 0, 0).ok();
    }

    fn right(&mut self, down: bool) {
        self.send_mouse(Cmd::Right, down as i32, 0, 0).ok();
    }

    fn move_delta(&mut self, x: i32, y: i32) {
        self.send_mouse(Cmd::Move, 0, x, y).ok();
    }

    fn reboot(&mut self) {
        self.send_cmd(Cmd::Reboot).ok();
    }

    fn shutdown(&mut self) {
        unimplemented!()
    }
}
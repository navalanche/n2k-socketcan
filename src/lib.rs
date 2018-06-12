//!
//! Wrapper around the socketcan crate to read and write n2k messages.
//! For a description of n2k see: See <https://en.wikipedia.org/wiki/NMEA_2000>.


/*
Copyright (C) 2018 Erwin Gribnau

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

extern crate n2k_base;
extern crate socketcan;

use n2k_base::n2k::N2kMessage;
use n2k_base::n2k::header::N2kHeader;
use socketcan::{CANSocket, CANSocketOpenError, CANFrame};

pub struct N2kSocket {
    can_socket: CANSocket
}

impl N2kSocket {
    pub fn open(ifname: &str) -> Result<N2kSocket, CANSocketOpenError> {
        let can_socket = CANSocket::open(ifname)?;
        Ok(N2kSocket { can_socket: can_socket })
    }

    pub fn receive_message(&self) -> Box<N2kMessage> {
        let frame = self.can_socket.read_frame().unwrap();
        let id = frame.id();
        let data = frame.data().clone();
        let header = N2kHeader::from_raw(id);
        Box::new(N2kMessage::new(header, data))
    }

    pub fn write_message(&self, message: &N2kMessage) -> Result<(), ()> {
        let header = message.get_header();
        let body = message.get_body();

        let id: u32 = header.into();

        let frame = CANFrame::new(id, &body, false, false).unwrap();

        let result = self.can_socket.write_frame(&frame);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }
}
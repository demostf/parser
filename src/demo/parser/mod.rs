use bitstream_reader::{BitRead, BitSkip, FromUtf8Error, LittleEndian, ReadError};

pub use self::messagetypeanalyser::MessageTypeAnalyser;
use crate::demo::gamevent::{GameEventValue, GameEventValueType};
use crate::demo::header::Header;
use crate::demo::message::packetentities::EntityId;
use crate::demo::packet::datatable::{ClassId, SendTableName};
use crate::demo::packet::Packet;
use crate::demo::parser::analyser::Analyser;
pub use crate::demo::parser::analyser::MatchState;
pub use crate::demo::parser::handler::{DemoHandler, MessageHandler};
pub use crate::demo::parser::state::ParserState;
use crate::Stream;
use err_derive::Error;

mod analyser;
mod error;
pub mod gamestateanalyser;
mod handler;
mod messagetypeanalyser;
mod state;

pub use self::error::*;
use crate::demo::parser::handler::BorrowMessageHandler;
use serde::export::PhantomData;

pub trait Parse: Sized {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self>;
}

impl<T: BitRead<LittleEndian>> Parse for T {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        Self::read(stream).map_err(ParseError::from)
    }
}

pub trait ParseBitSkip {
    fn parse_skip(stream: &mut Stream) -> Result<()>;
}

impl<T: BitSkip<LittleEndian>> ParseBitSkip for T {
    #[inline(always)]
    fn parse_skip(stream: &mut Stream) -> Result<()> {
        Self::skip(stream).map_err(ParseError::from)
    }
}

pub struct DemoParser<A: MessageHandler> {
    handler: DemoHandler<A>,
    stream: Stream,
}

impl DemoParser<Analyser> {
    pub fn new(stream: Stream) -> DemoParser<Analyser> {
        DemoParser::new_with_analyser(stream, Analyser::new())
    }

    pub fn new_all(stream: Stream) -> DemoParser<Analyser> {
        DemoParser::new_all_with_analyser(stream, Analyser::new())
    }
}

impl<A: MessageHandler> DemoParser<A> {
    pub fn new_with_analyser(stream: Stream, analyser: A) -> DemoParser<A> {
        DemoParser {
            handler: DemoHandler::with_analyser(analyser),
            stream,
        }
    }

    pub fn new_all_with_analyser(stream: Stream, analyser: A) -> DemoParser<A> {
        DemoParser {
            handler: DemoHandler::parse_all_with_analyser(analyser),
            stream,
        }
    }

    pub fn parse(self) -> Result<(Header, A::Output)> {
        let (header, mut ticker) = self.ticker()?;
        while ticker.tick()? {
            // noop
        }
        Ok((header, ticker.into_state()))
    }

    pub fn ticker(mut self) -> Result<(Header, DemoTicker<A>)> {
        let header = Header::read(&mut self.stream)?;
        let ticker = DemoTicker {
            handler: self.handler,
            packets: RawPacketStream::new(self.stream),
        };
        Ok((header, ticker))
    }
}

pub struct RawPacketStream {
    stream: Stream,
    ended: bool,
}

impl RawPacketStream {
    pub fn new(stream: Stream) -> Self {
        RawPacketStream {
            stream,
            ended: false,
        }
    }

    pub fn next(&mut self, state: &ParserState) -> Result<Option<Packet>> {
        if self.ended {
            Ok(None)
        } else {
            match Packet::parse(&mut self.stream, state) {
                Ok(Packet::Stop(_)) => {
                    self.ended = true;
                    Ok(None)
                }
                Ok(packet) => Ok(Some(packet)),
                Err(e) => {
                    self.ended = true;
                    Err(e)
                }
            }
        }
    }
}

pub struct DemoTicker<A: MessageHandler> {
    handler: DemoHandler<A>,
    packets: RawPacketStream,
}

impl<A: MessageHandler> DemoTicker<A> {
    /// Process the next packet
    ///
    /// returns whether or not there are still packets left in the demo
    pub fn tick(&mut self) -> Result<bool> {
        Ok(
            if let Some(packet) = self.packets.next(self.handler.get_parser_state())? {
                self.handler.handle_packet(packet);

                true
            } else {
                false
            },
        )
    }

    pub fn into_state(self) -> A::Output {
        self.handler.into_output()
    }
}

impl<A: MessageHandler + BorrowMessageHandler> DemoTicker<A> {
    pub fn state(&self) -> &A::Output {
        self.handler.borrow_output()
    }
}

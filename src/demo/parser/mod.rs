use bitbuffer::{BitError, BitRead, LittleEndian};

pub use self::messagetypeanalyser::MessageTypeAnalyser;

use crate::demo::header::Header;

use crate::demo::packet::Packet;
use crate::demo::parser::analyser::Analyser;
pub use crate::demo::parser::analyser::MatchState;
pub use crate::demo::parser::handler::{DemoHandler, MessageHandler};
pub use crate::demo::parser::state::ParserState;
use crate::Stream;

mod analyser;
mod error;
pub mod gamestateanalyser;
pub mod handler;
mod messagetypeanalyser;
mod state;

pub use self::error::*;
use crate::demo::parser::handler::BorrowMessageHandler;

pub trait Parse<'a>: Sized {
    fn parse(stream: &mut Stream<'a>, state: &ParserState) -> Result<Self>;
}

impl<'a, T: BitRead<'a, LittleEndian>> Parse<'a> for T {
    fn parse(stream: &mut Stream<'a>, _state: &ParserState) -> Result<Self> {
        Self::read(stream).map_err(ParseError::from)
    }
}

pub trait ParseBitSkip<'a> {
    fn parse_skip(stream: &mut Stream<'a>) -> Result<()>;
}

impl<'a, T: BitRead<'a, LittleEndian>> ParseBitSkip<'a> for T {
    #[inline(always)]
    fn parse_skip(stream: &mut Stream<'a>) -> Result<()> {
        Self::skip(stream).map_err(ParseError::from)
    }
}

pub struct DemoParser<'a, A: MessageHandler> {
    handler: DemoHandler<'a, A>,
    stream: Stream<'a>,
}

impl<'a> DemoParser<'a, Analyser> {
    pub fn new(stream: Stream<'a>) -> DemoParser<Analyser> {
        DemoParser::new_with_analyser(stream, Analyser::new())
    }

    pub fn new_all(stream: Stream<'a>) -> DemoParser<Analyser> {
        DemoParser::new_all_with_analyser(stream, Analyser::new())
    }
}

impl<'a, A: MessageHandler> DemoParser<'a, A> {
    pub fn new_with_analyser(stream: Stream<'a>, analyser: A) -> DemoParser<A> {
        DemoParser {
            handler: DemoHandler::with_analyser(analyser),
            stream,
        }
    }

    pub fn new_all_with_analyser(stream: Stream<'a>, analyser: A) -> DemoParser<A> {
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

    /// A Ticker provides a way to step trough the demo packet by packet
    /// while allowing to see the intermediate states
    pub fn ticker(mut self) -> Result<(Header, DemoTicker<'a, A>)> {
        let header = Header::read(&mut self.stream)?;
        let ticker = DemoTicker {
            handler: self.handler,
            packets: RawPacketStream::new(self.stream),
        };
        Ok((header, ticker))
    }
}

#[derive(Clone)]
pub struct RawPacketStream<'a> {
    stream: Stream<'a>,
    ended: bool,
    incomplete: bool,
}

impl<'a> RawPacketStream<'a> {
    pub fn new(stream: Stream<'a>) -> Self {
        RawPacketStream {
            stream,
            ended: false,
            incomplete: false,
        }
    }

    pub fn next(&mut self, state: &ParserState) -> Result<Option<Packet<'a>>> {
        if self.ended {
            Ok(None)
        } else {
            match Packet::parse(&mut self.stream, state) {
                Ok(Packet::Stop(_)) => {
                    self.ended = true;
                    Ok(None)
                }
                Ok(packet) => Ok(Some(packet)),
                Err(ParseError::ReadError(BitError::NotEnoughData { .. })) => {
                    self.ended = true;
                    self.incomplete = true;
                    Ok(None)
                }
                Err(e) => {
                    self.ended = true;
                    Err(e)
                }
            }
        }
    }
}

pub struct DemoTicker<'a, A: MessageHandler> {
    handler: DemoHandler<'a, A>,
    packets: RawPacketStream<'a>,
}

impl<'a, A: MessageHandler> DemoTicker<'a, A> {
    /// Process the next packet
    ///
    /// returns whether or not there are still packets left in the demo
    pub fn tick(&mut self) -> Result<bool> {
        Ok(
            if let Some(packet) = self.packets.next(&self.handler.state_handler)? {
                self.handler.handle_packet(packet)?;

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

impl<A: MessageHandler + BorrowMessageHandler> DemoTicker<'_, A> {
    pub fn state(&self) -> &A::Output {
        self.handler.borrow_output()
    }
}

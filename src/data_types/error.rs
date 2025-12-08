use graphics::*;
use snafu::{Backtrace, Whatever, prelude::*};

pub type Result<T> = std::result::Result<T, EditorError>;

#[allow(unreachable_code)]
#[derive(Debug, snafu::Snafu)]
pub enum EditorError {
    #[snafu(display("Currently Unhandled data error. BACKTRACE: {backtrace:?}"))]
    Unhandled {
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    AddrParseError {
        source: std::net::AddrParseError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    GraphicsError {
        source: GraphicsError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    Io {
        source: std::io::Error,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    UnicodeError {
        source: std::str::Utf8Error,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    ParseError {
        source: std::string::ParseError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    ParseNum {
        source: std::num::ParseIntError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    RodioAudio {
        source: rodio::PlayError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    RodioStreamError {
        source: rodio::StreamError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    RodioDecoderError {
        source: rodio::decoder::DecoderError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    Surface {
        source: wgpu::SurfaceError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    WGpu {
        source: wgpu::Error,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    Device {
        source: wgpu::RequestDeviceError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    ImageError {
        source: image::ImageError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    Other {
        source: OtherError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    EventLoop {
        source: winit::error::EventLoopError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    EventLoopExternal {
        source: winit::error::ExternalError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
    #[snafu(transparent)]
    OsError {
        source: winit::error::OsError,
        #[snafu(backtrace)]
        backtrace: Backtrace,
    },
}

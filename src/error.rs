use sdl2::video;
use std::error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum BuildManagerError {
    SdlContextError(String),
    VideoSubsystemError(String),
    AudioSubsystemError(String),
    WindowError(video::WindowBuildError),
    AudioDeviceError(String),
    WindowCanvasError(sdl2::IntegerOrSdlError),
    EventPumpError(String),
}

impl Display for BuildManagerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildManagerError::SdlContextError(e) => {
                write!(f, "Build SDL2 context failed, {}", e)
            }
            BuildManagerError::VideoSubsystemError(e) => {
                write!(f, "Build video subsystem failed, {}", e)
            }
            BuildManagerError::AudioSubsystemError(e) => {
                write!(f, "Build audio subsystem failed, {}", e)
            }
            BuildManagerError::WindowError(e) => {
                write!(f, "Build window failed, {}", e)
            }
            BuildManagerError::AudioDeviceError(e) => {
                write!(f, "Build audio device failed, {}", e)
            }
            BuildManagerError::WindowCanvasError(e) => {
                write!(f, "Build window canvas failed, {}", e)
            }
            BuildManagerError::EventPumpError(e) => {
                write!(f, "Build event pump failed, {}", e)
            }
        }
    }
}

impl error::Error for BuildManagerError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

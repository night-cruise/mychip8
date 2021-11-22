use sdl2::video;
use std::error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum BuildPlatformError {
    SdlContextError(String),
    VideoSubsystemError(String),
    AudioSubsystemError(String),
    WindowError(video::WindowBuildError),
    AudioDeviceError(String),
    WindowCanvasError(sdl2::IntegerOrSdlError),
    EventPumpError(String),
}

impl Display for BuildPlatformError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildPlatformError::SdlContextError(e) => {
                write!(f, "Build SDL2 context failed, {}", e)
            }
            BuildPlatformError::VideoSubsystemError(e) => {
                write!(f, "Build video subsystem failed, {}", e)
            }
            BuildPlatformError::AudioSubsystemError(e) => {
                write!(f, "Build audio subsystem failed, {}", e)
            }
            BuildPlatformError::WindowError(e) => {
                write!(f, "Build window failed, {}", e)
            }
            BuildPlatformError::AudioDeviceError(e) => {
                write!(f, "Build audio device failed, {}", e)
            }
            BuildPlatformError::WindowCanvasError(e) => {
                write!(f, "Build window canvas failed, {}", e)
            }
            BuildPlatformError::EventPumpError(e) => {
                write!(f, "Build event pump failed, {}", e)
            }
        }
    }
}

impl error::Error for BuildPlatformError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

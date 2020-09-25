use thiserror::Error;
use wiiuse_sys::*;

/// An error produced during runtime
#[derive(Error, Debug)]
pub enum WiiBoardError {
    #[error("No wii boards were found")]
    NoBoardsFound,
    #[error("Not a wii board")]
    NotABoard,
    #[error("Connection Failed")]
    ConnectionFailed,
    #[error("Connection to the WiiBoard dropped")]
    ConnectionDropped,
}

type Result<T> = std::result::Result<T, WiiBoardError>;

/// An abstraction over a Wii Balance Board
pub struct WiiBoard {
    wiimotes: *mut *mut wiimote_t,
}

/// Data retreived from `WiiBoard::poll()`. This is the interpolated data, not the raw data.
#[derive(Debug, Clone, Copy)]
pub struct WiiBoardData {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
}

impl WiiBoard {
    /// Create a new WiiBoard, waiting `timeout_seconds` for a board to begin syncing.
    pub fn new(timeout_seconds: u32) -> Result<Self> {
        unsafe {
            // TODO: Allow up to 4 remotes to connect, but then search within those for balance boards?
            let wiimotes = wiiuse_init(1);
            let found = wiiuse_find(wiimotes, 1, timeout_seconds as _);
            if found == 0 {
                return Err(WiiBoardError::NoBoardsFound);
            }

            let connected = wiiuse_connect(wiimotes, 1);
            if connected == 0 {
                return Err(WiiBoardError::ConnectionFailed);
            }

            Ok(Self { wiimotes })
        }
    }

    /// Poll for events. Returns `Ok(None)` if there were no events, but the runtime is still ok.
    pub fn poll(&self) -> Result<Option<WiiBoardData>> {
        let wiimote = unsafe { (*self.wiimotes).as_ref().unwrap() };
        if !WIIMOTE_IS_CONNECTED(wiimote) {
            return Err(WiiBoardError::ConnectionDropped);
        }

        let poll = unsafe { wiiuse_poll(self.wiimotes, 1) };
        if poll != 0 && wiimote.event == WIIUSE_EVENT_TYPE_WIIUSE_EVENT {
            if wiimote.exp.type_ != EXP_WII_BOARD as i32 {
                return Err(WiiBoardError::NotABoard);
            }
            let wii_board = unsafe { wiimote.exp.__bindgen_anon_1.wb };
            Ok(Some(wii_board.into()))
        } else {
            Ok(None)
        }
    }
}

impl From<wii_board_t> for WiiBoardData {
    fn from(wb: wii_board_t) -> Self {
        Self {
            top_left: wb.tl,
            top_right: wb.tr,
            bottom_left: wb.bl,
            bottom_right: wb.br,
        }
    }
}

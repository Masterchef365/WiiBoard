use anyhow::Result;
use wiiboard::{WiiBoard, WiiBoardPoll};

fn main() -> Result<()> {
    let board = WiiBoard::new(10)?;
    loop {
        if let WiiBoardPoll::Balance(data) = board.poll()? {
            dbg!(data);
        }
    }
}
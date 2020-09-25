use anyhow::Result;
use wiiboard::WiiBoard;

fn main() -> Result<()> {
    let board = WiiBoard::new(10)?;
    loop {
        if let Some(data) = board.poll()? {
            dbg!(data);
        }
    }
}

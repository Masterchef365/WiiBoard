use anyhow::Result;
use wiiboard::WiiBoardRealtime;

fn main() -> Result<()> {
    let board = WiiBoardRealtime::new(10, 10);
    loop {
        if let Some(data) = board.poll()? {
            dbg!(data);
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
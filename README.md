# WiiBoard
Dead simple abstraction over [WiiUse](https://github.com/wiiuse/wiiuse) for trivial interaction with Wii Balance Boards.

## Installation
You will need to install WiiUse, which can be found at the GitHub link above. There is also an Arch Linux package simply called `wiiuse`.

## Usage
Once `WiiBoard::new()` is called, you will need to hit the red sync button in the Balance Board's battery compartment. If all goes right, the two will sync and you will begin receiving messages. Note that calling `WiiBoard::poll()` does not block block.

## Some problems I ran into, and how to resolve them
* Turns out bluetooth was disabled via `rfkill`. All I had to do was run `sudo rfkill unblock all`. 
* Do not pair with or connect to the device. WiiUse handles every part of communication, refer to their docs for debugging.

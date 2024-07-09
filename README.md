# Rgboy

### Goals
This project originated as I was looking into writing an emulator in Rust.  
Initially I was interested in gameboy emulation (re: the project name).  
I quickly learned that was out of scope and investigated NES emulation instead.  

In that investigation I found the incredible [nes_ebook](https://github.com/bugzmanov/nes_ebook/tree/master?tab=readme-ov-file#readme) project & guide.  
I have so far followed the guide pretty closely outside of how opcodes are all defined.  
I have used this project as a way to get to learn Rust's powerful macros.  
This method offers performant opcode modules over the suggested Dictionary!  
I am really proud of how this part of the project turned out overall.

The current target for this project is to create a function G6502 emulator which can run a simple [Snake game](https://gist.github.com/wkjagt/9043907) created for the processor.  

Currently Snake is in a somewhat playable but does have some apparent issues that need investigation. Namely the Snake tail position does not seem to update correctly.

### Setup
This project should work out of the box ideally, thanks to the great SDL2 library.  
I will update here once I have attempted project setup elsewhere.
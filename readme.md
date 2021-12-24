### Usage
Chip8 interpreter is written in Rust for learning purposes. The program can be run by
./chip8 <name>.rom <execution_speed>



### Chip8:
A simple and interpreted programming language which was first used on 
some do-it yourself computer systems.

### Memory:
4KiB -> 4096 bytes, locations: 0 ... 4095 (0xFFF)
The first 512 bytes are not accessible, reserved for Chip8: 0...512(0x200)
The following is the memory map of Chip8:

+---------------+= 0xFFF (4095) End of Chip-8 RAM
|               |
|               |
|               |
|               |
|               |
| 0x200 to 0xFFF|
|     Chip-8    |
| Program / Data|
|     Space     |
|               |
|               |
|               |
+- - - - - - - -+= 0x600 (1536) Start of ETI 660 Chip-8 programs
|               |
|               |
|               |
+---------------+= 0x200 (512) Start of most Chip-8 programs
| 0x000 to 0x1FF|
| Reserved for  |
|  interpreter  |
+---------------+= 0x000 (0) Start of Chip-8 RAM

### Registers:
16 general purpose byte registers, which are reffered to as Vx, where x €  {0, 1, ..., E, F}
2 byte register I, where the 12 lowest bits are used for storing memory addresses.

VF register should not be used, but it is used by some instructions as a FLAG.

Two special purposed byte registers: one for delay, one for sound timer.
When these two are nonzero, they are automatically decremented at a rate of 60Hz.

PC (program counter) is 2 byte, stores currently executing address.
SP (stack pointer) is 1 byte, points to the topmost level of the stack. Stack is array of
16 2byte values, which store address that the interpreter should return to, when it finishes the
subroutine.

### Keyboard:
Use the leftmost 4x4 keys (1 2 3 4/ Q W E R/.../ Y X C V)

### Display:
64x32 pixel monochrome display.
Sprites are the main element. This is a group of bytes which are a binary representation
of the desired picture. They may be up to 15 bytes in size (sprite dim: 8x15)

### Timers 'n sounds:

Both are decreased at the rate of 60Hz (by one, until they are zero). While the sound register
is higher than zero, the chip8 emits sound (only one tone!).

### Instruction set:
Important information: All instructions are 2 bytes long and are stored most-significant-byte first. In memory, 
the first byte of each instruction should be located at an even addresses. 
If a program includes sprite data, it should be padded so any
 instructions following it will be properly situated in RAM.


#### Instructions:

00E0 :: clear the display

00EE :: return from the subroutine. Interpreter sets the PC to the address at the top of
the stack, then subtracts 1 from the SP.

1nnn :: jump to location nnn.
Set the PC to nnn.

2nnn :: call subroutine at nnn.
The interpreter increments the stack pointer, then puts the current PC on the top of the
stack. PC is set to nnn.

3xkk :: skip next instruction if Vx == kk
The interpreter compares register Vx to kk and if they are equal, it increments the PC by 2.

4xkk :: similar to above, but if Vx != kk

5xy0 :: similar to above, but if Vx == Vy

( 9xy0 :: similar to above, but if Vx != Vy)

6xkk :: set Vx to kk, store kk into register Vx

7xkk :: set Vx = Vx + kk

8xy0 :: set Vx = Vy

8xy1 :: set Vx = Vx OR Vy (bitwise)

8xy2 :: set Vx = Vx AND Vy (bitwise)

8xy3 :: set Vx = Vx XOR Vy (bitwise)

8xy4 :: set Vx = Vx + Vy, VF = carry
if the result is greater than 255, set VF to 1, otherwise 0. Result is stored in Vx.

8xy5 :: set Vx = Vx - Vy, set VF = NOT borrow
if Vx > Vy, VF is set to 1, otherwise 0. Then Vy is subtracted from Vx.

8xy6 :: set Vx = Vx shift right by 1
Before shifting, set VF to 1, if lsb of Vx is 1, otherwise 0


8xy7 :: SUBN Vx, Vy
Set Vx = Vy - Vx, set VF = NOT borrow.

If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.


8xyE :: SHL Vx {, Vy}
Set Vx = Vx SHL 1.

If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.


9xy0 :: SNE Vx, Vy
Skip next instruction if Vx != Vy.

The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.


Annn :: LD I, addr
Set I = nnn.

The value of register I is set to nnn.


Bnnn :: JP V0, addr
Jump to location nnn + V0.

The program counter is set to nnn plus the value of V0.


Cxkk :: RND Vx, byte
Set Vx = random byte AND kk.

The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.


Dxyn :: DRW Vx, Vy, nibble
Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.


Ex9E :: SKP Vx
Skip next instruction if key with the value of Vx is pressed.

Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.


ExA1 :: SKNP Vx
Skip next instruction if key with the value of Vx is not pressed.

Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.


Fx07 :: LD Vx, DT
Set Vx = delay timer value.

The value of DT is placed into Vx.


Fx0A :: LD Vx, K
Wait for a key press, store the value of the key in Vx.

All execution stops until a key is pressed, then the value of that key is stored in Vx.


Fx15 :: LD DT, Vx
Set delay timer = Vx.

DT is set equal to the value of Vx.


Fx18 :: LD ST, Vx
Set sound timer = Vx.

ST is set equal to the value of Vx.


Fx1E :: ADD I, Vx
Set I = I + Vx.

The values of I and Vx are added, and the results are stored in I.


Fx29 :: LD F, Vx
Set I = location of sprite for digit Vx.

The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.


Fx33 :: LD B, Vx
Store BCD representation of Vx in memory locations I, I+1, and I+2.

The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.


Fx55 :: LD [I], Vx
Store registers V0 through Vx in memory starting at location I.

The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.


Fx65 :: LD Vx, [I]
Read registers V0 through Vx from memory starting at location I.

The interpreter reads values from memory starting at location I into registers V0 through Vx.


### "The loop"




## Sources
http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
https://austinmorlan.com/posts/chip8_emulator/
https://en.wikipedia.org/wiki/CHIP-8
https://chip-8.github.io/links/


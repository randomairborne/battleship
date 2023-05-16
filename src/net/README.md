# proto

a simple protocol for battleship

on establish, the client is player 2, and the server is player 1. Player 1 goes first.

The player whos turn it is must send a packet consisting of a single byte. The four most significant bits are for the X, the least significant four are for the Y. The values of both X and Y **must** be between 0 and 9. The other should respond with a byte which
has its most significant bit signify if the shot has already been taken with a 1. If the shot has already been taken, the rest of the bits in this byte MUST be 0. The second MSB should signify if the shot was a hit or a miss. If the shot was a hit, the bit should be 1, otherwise, it should be 0. The third MSB should be if the ship was sunk. The 4th bit shall signify if the shot won the game. The 5th bit is currently unused and should be set to 0. The 3 least significant bits will serve as a signifier of which ship has been hit, in this mapping:

1. Patrol Boat
2. Submarine
3. Destroyer
4. Battleship
5. Aircraft Carrier

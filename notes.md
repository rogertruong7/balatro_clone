HighCard =>      (5,   1),
Pair =>          (10,  2),
TwoPair =>       (20,  2),
ThreeOfAKind =>  (30,  3),
Straight =>      (30,  4),
Flush =>         (35,  4),
FullHouse =>     (40,  4),
FourOfAKind =>   (60,  7),
StraightFlush => (100, 8),
FiveOfAKind =>   (120, 12),
FlushHouse =>    (140, 14),
FlushFive =>     (160, 16),

Code is not unnecessarily repeated.
Code is abstracted appropriately.
Types are used appropriately to express data in the program.
The design does not impose unnecessary constraints on either the caller or callee through borrowing, lifetimes or ownership.
Uses traits sensibly to add expressiveness.
Data structures used are appropriate to store data.
Functions perform error handling; cases that are expected do not panic.
Code is sensibly organised, and split into appropriate modules.
Documentation, where provided, is correct and readable.
(optional) Uses external crates effectively to achieve the above goals.
(optional) Where code is designed in a sub-optimal way, comments about how to improve it are made under "Design Limitations".
# 0001 The cairn

**Status:** implemented
**Date:** 2026-10-02

## Goal

A tower of blocks you take apart one at a time, putting each one you pull on
top, until it comes down. One player, one tower, one number at the end: how many
you got out before it fell.

## Why this game exists

Blitzkit grew spheres in spec 0030 and spent four specs on them: rolling, spin,
striking off centre, a solver that lets them rest on each other. Marble, carom
and poolhall are what came of that, and all three are games about round things
rolling on a flat floor.

Specs 0034, 0035 and 0036 gave it boxes, orientation, contact patches, warm
starting and sleeping. Nothing has asked for any of it yet. This does, and it
asks for all of it at once: a block only rests on another because a contact is
four points rather than one, and a stack only stands still because a contact
remembers what it pushed with last frame.

It is also the first game here where the player is not trying to hit something.
The decision every turn is which block can go, which is a judgement about what is
holding what up, and there is no way to be told the answer.

## What the tower is, and what it is not

**Twelve levels of four blocks**, laid side by side with a gap between them and
turned a quarter turn each level. A block is square in section, and four of them
with their gaps come to exactly a block's length, so every level is square in
plan and the one above lands square on it. Forty eight blocks.

Four to a level and not three, because three is the game this will be compared
to and because four leaves a margin: take one of four and three remain, two of
them still under the level above. Take one of three and the level is a see-saw.

**Not the open lattice of two that spec 0036 describes.** That shape was chosen
to be hard on the engine, which it is: every contact is a small patch near a
corner and the load runs down four columns of corners. It is also unplayable,
and this spec is where that was noticed. A level of two has each block of the
level above resting on it at one end, so taking either one away drops the level
above however carefully it is done. There is no first move.

The lattice stays where it is, as the engine's test and its example. The game's
tower is this one. Spec 0036 says the game's tower is the lattice and is wrong
about it.

**Nothing it shares with the game it will be compared to.** That one is a
registered mark, and so are the things that make it recognisable: fifty four
blocks, three to a level, eighteen levels, and a block half again as wide as it
is thick. Forty eight, four, twelve, and square in section.

## Behavior

**A turn is one block.** Point at a block and pull it out along its own length,
towards you. A block can be pulled from either end. While it is moving the tower
is not reset or paused: the thing you are deciding is whether the rest can
spare it, and the answer arrives as it happens.

**A block that comes out goes on top.** Dropped onto the top level, turned the
way that level runs, which is what makes the tower taller and worse the longer
you last. A level that is not yet full takes the next one; a full level starts a
new one.

**The top level is not a source.** Nor the one below it, which is the rule that
stops the game being about taking the block you just put down. Everything from
the third level down is fair.

**It falls when the tower drops.** The height of the highest block that is still
part of the tower is watched, and when it falls by more than one level the run
is over. Not when a block hits the floor, because a block that has been pulled
free and dropped is a block hitting the floor, and that happens on purpose.

**The number is how many came out.** Not time, not height. The run ends and
says the number, and space starts another one.

## What it asks of blitzkit

- `Body::block`, orientation, and the inertia tensor, per spec 0034. A block
  that has been pulled free tumbles, and it looks wrong if it does not.
- Box against box and box against the world, per spec 0035. The whole game is
  blocks resting on blocks.
- A `Solver` kept across frames, per spec 0036. Without warm starting a twelve
  level tower sags the moment it is built, and without sleeping it leans while
  the player is thinking.
- `Camera::ray_through`, per spec 0025, and a ray against a box, which the
  tower example has as a local function. If this needs it too, that is the
  second caller and the point at which it is worth a spec of its own.
- `Body::strike` and `Body::wake`, per spec 0032 and 0036.

## What it will not have

No second player. No timer. No undo. No camera that moves on its own. No sound
that is not a block landing on a block.

## The numbers are measured, not reasoned

Spec 0036 measured a twenty level lattice. This is a twelve level stack of four,
which is a different thing, and the friction that lets a block be slid out
without dragging its neighbours with it is a number to find rather than to pick.
Nothing in here is settled until it has been run.

## Acceptance criteria

- A new tower is twelve levels of four. — `tower::tests::a_new_tower_is_twelve_of_four`
- Every level is turned across the one below it. — `tower::tests::every_level_crosses_the_one_below`
- A level is four blocks side by side with a gap, as wide as a block is long, and a block is square in section. — `tower::tests::a_level_is_four_blocks_side_by_side_with_a_gap`
- A tower is standing when it is built, to within the sag a stack of this height
  keeps. — `rules::tests::a_new_tower_stands`
- The top two levels cannot be pulled from. — `rules::tests::the_top_two_are_safe`
- Any level below them can. — `rules::tests::anything_lower_can_go`
- A block pulled out goes onto the top level, turned the way that level runs. — `rules::tests::what_comes_out_goes_on_top`
- A full top level starts a new one. — `rules::tests::a_full_level_starts_another`
- The count goes up by one for each block that comes out. — `rules::tests::the_count_is_what_came_out`
- A run ends when the tower drops by more than a level. — `rules::tests::a_drop_ends_it`
- A block dropped on purpose does not end the run by itself. — `rules::tests::a_dropped_block_is_not_a_collapse`
- The same pulls in the same order give the same tower. — `rules::tests::the_same_run_twice_is_the_same_run`

### Verified by hand

- The tower stands still while nothing is happening, rather than settling under the cursor.
- A block slides out without dragging its neighbours when it is free, and takes them with it when it is not.
- A block that has been pulled free tumbles rather than sliding flat.
- Pulling the wrong one brings the tower down in a way that looks like a tower coming down.

## What the build taught

**Two from the same level is half of it.** The first tests here took blocks 0, 1
and 2, which is three quarters of the bottom level, and the tower came down on
the second one. That is the rules working rather than failing, and it is the
whole shape of the game: a level of four can spare one and not two.

**The gap between blocks is not decoration.** There was none at first, so the
four blocks of a level were pressed against each other side to side and held by
their sides as well as from above. Drawing one out dragged its neighbours, an
outer block was fatal at every grip tried, and a level read on screen as one
slab with no visible hole where a block had been. Measured, with a gap of 0.08
and a grip of 0.4 a full level can spare any of its four:

```text
       seat 0        seat 1   seat 2        seat 3
0.25   0.47          0.10     0.42          5.62, fatal
0.40   0.34          0.18     0.33          0.73
0.60   3.06, fatal   0.16     0.13          0.60
```

Which is what the first pull of a game should be. The difficulty is meant to come
from the holes that have already been made, not from guessing which of four
identical seats is the cursed one.

**Where the holes are matters as much as how many.** Taking the same inner seat
from every other level punches a line down one side, and the tower came down on
the fourth. Alternating the two inner seats, the same five takes pass. Nothing
in the rules says this; it is what the tower does.

**A block is drawn out rather than struck**, and by a hand that is strong rather
than infinitely strong. Setting its speed outright made it unstoppable, and a
neighbour wedged against something unstoppable leaves at whatever speed the
solver needs to get it out of the way: measured, one went thirty three units.
Pulled at an acceleration instead, with the across-the-length part of its
velocity left alone, a block that is holding something up takes a moment to come
free and what it is holding up gets a shove rather than a launch.

That acceleration is the game's best number. At eighty an inner block slips out
in a second and an outer one takes three and lets you watch it resist, which is
the tower telling you what it was doing. Past a hundred a pulled block can send
a neighbour thirty units or more, and that is the engine rather than the game: a
driven body forced through a loaded contact is a case blitzkit has not been asked
for before.

**A run that is over goes on being stepped.** It did not, and what that looked
like was the tower stopping dead at the instant it was declared down, which is
the instant before any of it has fallen. The one thing a player wants to watch,
frozen on the frame it started.

**One level of drop is not a collapse.** It is exactly the height of the block
just put on top, so losing only that block ended the run. Two.

**The solver forgets when a block is put on top.** A contact is remembered by
the pair it is between and the features that touched, per blitzkit's spec 0036,
and a block that has been moved from the bottom of the tower to the top is not
in any of the contacts it was in. Carrying those over would hand the new
position an impulse the old one earned.

**Nothing is ever removed from the list.** A block that comes out is moved, not
deleted, so no index ever shifts and the level each block belongs to can live in
a plain parallel list. That is the trap this project walked into three times
building blitzkit 0033 through 0036, avoided here by not creating it.

## Out of scope

Two players. Any rule about which hand or how many fingers. Blocks that are not
all the same size. A tower built to a shape the player chooses. Scores kept
between runs. Anything that reads the tower and tells the player what is safe.

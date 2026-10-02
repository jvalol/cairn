# 0001 The cairn

**Status:** draft
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

**Twelve levels of four blocks**, laid side by side and turned a quarter turn
each level. A block is square in section and four long, so a level is four by
four and the tower is forty eight blocks and twelve units tall.

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

- A new tower is twelve levels of four, and every level is turned across the one
  below it. — `tower::tests::a_new_tower_is_twelve_of_four`
- A tower is standing when it is built, to within the sag a stack of this height
  keeps. — `tower::tests::a_new_tower_stands`
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

## Out of scope

Two players. Any rule about which hand or how many fingers. Blocks that are not
all the same size. A tower built to a shape the player chooses. Scores kept
between runs. Anything that reads the tower and tells the player what is safe.

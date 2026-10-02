# 0002 The hand

**Status:** implemented
**Date:** 2026-10-02

## Goal

Taking a block should feel like taking a block. Spec 0001 made the rules and the
physics work and left the player a spectator: click, and watch something happen
to you for the next second or three.

## Why this is a spec and not a tweak

Spec 0001 says the thing you decide every turn is whether the rest can spare the
block, and that the answer arrives as it happens. It does not. The decision is
made at the click and then the hand is taken away, so what arrives is a verdict
on something already done. Four things were wrong with the hand and they are one
thing: the player is not holding the block.

## Behavior

**Press and drag, rather than click and watch.** Holding the left button on a
block grabs it. While it is held, the block is drawn towards wherever the cursor
has dragged to, measured along the block's own length as that length lies on the
screen. Let go and it stops where it is.

So a block that is half out stays half out. That is a real position in this game
and it was not reachable before: it is how you find out whether the one above has
settled onto what is left, and it is the move of leaving a block proud while you
think.

**The hand is still not infinitely strong.** Spec 0001's acceleration stays, and
it is what the drag feeds: the drag says where the block should be, and the block
gets there as fast as eighty units a second a second will carry it against
whatever is leaning on it. Dragging further does not pull harder, it only asks
for more. A block that is pinned does not come, and you can feel that it is not
coming because the cursor has gone somewhere the block has not.

**It comes out the end you grabbed.** Where the ray met the block decides it: the
near half comes towards you, the far half goes away. Not "towards the camera",
which is what it was and which meant walking round the tower to pull from the
other side without ever being told so.

**The tower says what it will allow.** A block under the cursor is one of three
things, and it looks like all three: takeable, too high to take, or being held.
When a block refuses, the reason is said in words rather than by nothing
happening. When a run ends, that is said too.

**The camera looks up and down as well as round.** Right-drag turns and tilts,
scroll comes closer. Tilt is clamped short of overhead and short of the floor,
because neither tells you anything and both look wrong.

## Acceptance criteria

- A drag along the block's length moves it that far, give or take what the tower
  is doing to it. — `hand::tests::a_drag_asks_for_that_much`
- A drag across the block's length asks for nothing. — `hand::tests::a_drag_across_it_asks_for_nothing`
- Letting go leaves a block where it is rather than finishing the pull. — `rules::tests::letting_go_leaves_it`
- A block grabbed by its near half comes out that way. — `hand::tests::it_comes_out_the_end_that_was_grabbed`
- And by its far half, the other way. — `hand::tests::the_other_end_goes_the_other_way`
- A block held past clear still goes on top. — `rules::tests::a_held_block_still_goes_on_top`
- The tilt is clamped short of overhead and of the floor. — `hand::tests::the_tilt_is_clamped`

### Verified by hand

- Dragging a block out feels like drawing it out, and stopping stops it.
- A block that is pinned does not move however far the cursor goes, and that is legible.
- Clicking a block that is too high says why.
- The tower can be looked at from above enough to see which blocks a level still has.

## Out of scope

Two hands. A block that can be turned while held. Pushing a block from the far
side with the cursor on the near side. Any control that is not the mouse.
Undoing a pull. A camera that moves on its own.

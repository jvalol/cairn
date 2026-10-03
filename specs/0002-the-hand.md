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
block grabs it, and while it is held the block follows the cursor: it is drawn to
the place on its own length nearest to whatever the cursor is pointing at. Let go
and it stops where it is.

Nearest point on a line to a ray, not pixels of drag. Pixels was the first
attempt, measured along the block's length as it lay on the screen. It fails
for exactly the blocks half a square tower has. One pointing away from the eye
is a few pixels long however long it is, so its direction is noise and any drag
asks for almost nothing. What that looked like was blocks left hanging part way
out, because finishing the pull had become impossible. A block pointing
straight down the ray is the one case with no answer, and it says so and asks
you to walk round a little.

So a block that is half out stays half out. That is a real position in this
game and it was not reachable before. It is how you find out whether the one
above has settled onto what is left, and how you leave a block proud while you
think.

**The hand is still not infinitely strong.** Spec 0001's acceleration stays,
and it is what the drag feeds. The drag says where the block should be, and it
gets there as fast as eighty units a second a second will carry it. Dragging
further does not pull harder, it only asks for more. A block that is pinned
does not come, and you can feel that it is not coming because the cursor has
gone somewhere the block has not.

**It comes out the end you drag it towards.** Not "towards the camera", which
meant walking round the tower to pull from the other side without ever being
told so. And not "the end you grabbed", which is what this spec said first and
which was worse. A block taken hold of anywhere along its long side says
nothing about which end was meant. Half the time the end chosen pointed into
the tower, every drag asked for a negative distance, the clamp turned that into
nothing, and the block sat there. The drag carries its sign now and the block
goes whichever way it is pulled.

**Either way out counts.** How far a block had to go was measured as the
distance of its middle from the tower's axis, which is a different number for
every seat. An outer block starts a unit and a half out and an inner one half a
unit, so an inner one had to travel 4.17 where an outer needed 3.93. One drawn
out the back of the tower travelled 4.12, finished up lying on the floor where
the camera could not see it, and was never counted. What that looked like was a
drag that did nothing, in one direction only. It is measured along the block's
own length from where it started now, which is the same number wherever the
block sat.

**Once it is down, the blocks are still blocks.** A run that has ended stops
counting and stops stacking, and everything on the floor can still be pushed
about. The rule protecting the top two levels has nothing left to protect, and a
heap of blocks is a thing worth shoving.

**The tower says what it will allow.** A block under the cursor is one of three
things, and it looks like all three: takeable, too high to take, or being held.
When a block refuses, the reason is said in words rather than by nothing
happening. When a run ends, that is said too.

**The camera looks up and down as well as round.** Right-drag turns and tilts,
scroll comes closer. Tilt is clamped short of overhead and short of the floor,
because neither tells you anything and both look wrong.

## Acceptance criteria

- A block goes to the place on its own length nearest to where the cursor points. — `hand::tests::the_block_goes_where_it_is_pointed`
- Including one pointing away from the eye, which is what pixels of drag could not do. — `hand::tests::a_block_pointing_away_still_follows`
- One pointing straight down the ray has no answer rather than a bad one. — `hand::tests::a_block_down_the_ray_has_no_answer`
- Letting go leaves a block where it is rather than finishing the pull. — `rules::tests::letting_go_leaves_it`
- Which way the length was handed over only flips the sign, so where a block was grabbed decides nothing. — `hand::tests::which_way_the_length_was_given_only_flips_the_sign`
- A tower that has come down can still be pushed about, and is not scored. — `rules::tests::the_rubble_is_still_blocks`
- A block drawn out either end counts the same. — `rules::tests::it_counts_either_way_out`
- A block let go before it is out stays where it was let go. — `rules::tests::one_let_go_short_stays_where_it_is`
- The tilt is clamped short of overhead and of the floor. — `hand::tests::the_tilt_is_clamped`

### Verified by hand

- Dragging a block out feels like drawing it out, and stopping stops it.
- A block that is pinned does not move however far the cursor goes, and that is legible.
- Any block in the tower can be taken hold of and dragged, from any side, whichever way its length happens to lie.
- Clicking a block that is too high says why.
- The tower can be looked at from above enough to see which blocks a level still has.

## Out of scope

Two hands. A block that can be turned while held. Pushing a block from the far
side with the cursor on the near side. Any control that is not the mouse.
Undoing a pull. A camera that moves on its own.

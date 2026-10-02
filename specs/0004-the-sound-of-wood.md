# 0004 The sound it makes

**Status:** implemented
**Date:** 2026-10-02

## Goal

Blocks that make a noise when they hit something. A tower coming down is the
best thing in this game and it happens in silence.

## Behavior

**Generated, not loaded**, the same as the grain and as marble's thud, and
written out sample by sample rather than built from the ready-made sources.

**Noise through a few broad resonances**, which is wood landing on a table.
Three attempts came before it and all three were built out of sines: a falling
one, which is a tom; a bar's bending modes, which is a woodblock and did not
read as wood; and a plucked tine on a minor pentatonic, which read as music. A
block landing on a table has no pitch to speak of. It is a burst of noise with
the body of whatever it hit around it, and it is over in a twentieth of a
second.

Three resonances: low for the table taking the weight, middle for the body of
the block, high for its edge. Broad, not tight. Held near one they ring, and a
ringing resonance is a note, which is what all three earlier attempts turned
into. Measured, at 0.95 and above the sound repeats itself two thirds of the way
towards a pure tone; at 0.90 it is half that, and most of what is left is the
low one simply being a lowpass.

**Each block's resonances sit a little off the others'**, the same every time
for the same block. Not a scale: blocks are not tuned, and forty eight at
exactly the same frequencies read as one block hit forty eight times.

**One knock per impact, and an impact is a loss of speed.** A block that is
moving and then is not has hit something. How much speed it lost is how hard,
and how hard decides how loud.

**Below a threshold there is no sound.** A stack of forty eight blocks is always
shuffling slightly, and a knock for every one of those is a rattle that never
stops. The threshold is measured against what a block actually does rather than
guessed.

**A limit on how many sound at once.** A collapse is dozens of impacts in a few
frames, and dozens of knocks together is a bang with no shape. The loudest few
of any one step are heard and the rest are not, which is close to what happens
in a room anyway.

**And the sound may not fall behind what is happening.** The engine plays what it
is handed one sound after another rather than over the top of each other, so a
collapse given to it whole is a collapse still being heard once everything has
come to rest: eighty knocks is six seconds of clattering after the last block has
stopped. Nothing is queued while there is already more than a tenth of a second
waiting. What cannot be played while it is still happening is not played.

**It comes from the direction of the block, not from where the block is.** The
engine's spatial sound fades with one over the distance squared, in world units,
and this camera sits eighteen units back from the tower. Placed where it really
happened, a knock came out at three thousandths of itself, which is why the first
version of this could not be heard at all. It is placed a stride from the ears
in the block's direction instead: the direction carries the left and the right,
and the distance is only about how loud.

**The quietest knock is still a knock.** A clack is fifty milliseconds of noise,
and fifty milliseconds of noise at a third of full is a thing you can miss
entirely. Anything worth making a sound about is worth hearing, so the softest
is a little under half of the loudest rather than nearly nothing.

**The rules make no sound.** `Run::step` writes down what was hit and how hard,
and the window turns that into noise. Nothing in the rules needs an audio device,
so all of it can still be checked without one.

**A block that goes on top is dropped the last of the way**, rather than set
down where it belongs. It was set down, and a block that appears in its place has
not hit anything, so drawing one out was silent: the whole of a turn made no
sound at all. Let go a third of a unit above its seat it lands, knocks, and
settles like everything else.

## What it cost, measured

A settled tower makes no sound over five seconds. Drawing one block out makes one
knock, at a third of full. A whole level drawn out of the bottom, which brings
the tower down, makes eighty knocks with a loudest of full and never more than
three in any one step.

## Acceptance criteria

- A block that lands hard knocks. — `rules::tests::a_landing_knocks`
- A block that is barely moving does not. — `rules::tests::a_settle_is_silent`
- A harder landing is louder than a softer one. — `rules::tests::harder_is_louder`
- No more than a few knocks come out of one step. — `rules::tests::a_collapse_does_not_knock_once_a_block`
- Knocks are taken when they are read, so none is heard twice. — `rules::tests::a_knock_is_heard_once`
- A block keeps its own colour, and the same one every time. — `knock::tests::a_block_keeps_its_colour`
- The quietest landing can still be heard. — `knock::tests::the_quietest_landing_can_still_be_heard`
- The sound cannot fall behind what is happening. — `knock::tests::the_sound_cannot_fall_behind`
- It is not a note, measured against a sine and against plain noise. — `knock::tests::it_is_not_a_note`
- It is over quickly. — `knock::tests::it_is_over_quickly`
- It is loudest at the moment of contact rather than after it. — `knock::tests::it_is_loudest_at_the_moment_of_contact`
- A quiet one is quieter and nothing clips. — `knock::tests::a_quiet_knock_is_quieter_and_nothing_clips`
- The same knock twice is the same knock, which a random pluck would not be. — `knock::tests::the_same_knock_twice_is_the_same_knock`

### Verified by hand

- A tower coming down sounds like wood on a table rather than like an instrument.
- The clattering stops when the blocks do.
- Drawing a block out and laying it on top makes one knock, not none and not a handful.
- A block landing on the left is heard on the left.

## Out of scope

Different sounds for wood on wood and wood on floor. A sound for a block being
drawn out, or for one sliding against another. Reverb, or any sense of a room.
Music. Anything loaded from a file.

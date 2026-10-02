# 0003 What wood looks like

**Status:** implemented
**Date:** 2026-10-02

## Goal

Blocks that look like wood rather than like boxes.

## This one was built before it was written

The flow in this repo is a spec and then the code, and this is the one that went
the other way round: it began as "add grain to each block" and was small enough
to do before it was described. Written afterwards, which is worth saying rather
than backdating. What it did find is below, and that would have been found
either way.

## Behavior

**Drawn, not loaded.** The same as marble's checker and poolhall's stripes: a
plank's grain is a few long lines down its length with the colour shifting
between them, and that is cheaper to write than to ship.

**Grain runs along the board.** The colour changes with how far across a board
you are and hardly at all with how far along. Three waves of different lengths
rather than one, so the lines are not evenly spaced, and speckle on top so they
are not clean.

The speckle runs the same way. Sampled evenly it varied as much along the board
as across, and the whole thing read as noise on wood rather than as wood. It is
sampled coarsely along and finely across, so it streaks.

The lines waver, but only just. The waver was twenty four times bigger to begin
with, and against a line frequency of seventy three that is four radians of
phase: the fine lines swung the whole way along the board and the grain ran
across it. Caught by a test rather than by looking, because at a glance it still
looked like wood.

**Six boards, and a block wears the one its own number names.** Forty eight
blocks of the same plank read as a printed pattern. Its own number rather than
anything random, so the same block is the same wood every time the tower is
built and neighbours in a level differ.

**The wood carries the colour.** What was a brown tint per level is now white
and a slightly duller white, so the levels still read as alternating without
staining the wood.

**A texture wide and short.** Every face of a cube gets the whole texture, and a
block is four long by one across, so the long faces stretch the width over four
units and the ends squeeze it into one. End grain on a real block does not look
like this, and nothing here is close enough to see it.

## Acceptance criteria

- There is one board of each kind, the right size. — `grain::tests::there_is_one_board_of_each_kind`
- The kinds are different wood rather than the same board twice. — `grain::tests::the_kinds_are_different_wood`
- A block is the same wood every time, and its neighbour is not. — `grain::tests::the_same_block_is_always_the_same_wood`
- The grain runs along a board rather than across it. — `grain::tests::the_grain_runs_along_the_board`
- Wood is opaque and stays between pale and dark. — `grain::tests::it_stays_between_pale_and_dark`

### Verified by hand

- A tower reads as wood from across the room and from up close.
- No two blocks of a level are obviously the same plank.

## Out of scope

End grain that looks like end grain. Knots. Bevelled or worn edges. Wood that
is darker where it has been handled. Any texture that is loaded rather than
drawn.

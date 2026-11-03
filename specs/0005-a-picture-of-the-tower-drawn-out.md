# 0005 A picture of the tower drawn out

**Status:** implemented
**Date:** 2026-11-03

## Goal

The frame cairn puts on its readme is held against a picture of what that
frame should be, so a change to the tower, the wood or the light is seen
rather than shipped.

## Why it belongs here

Spec 0001 says what the tower is and 0002 says what the hand does to it, and
every test of either is arithmetic: where a block sits, whether it can be
drawn, how far the pile has settled. Spec 0003 is about what the wood looks
like, and nothing in this repo could see it.

The engine's spec 0076 made the rest one call, so what a game writes is the
reference, the size and what to draw.

**And the staging was already here.** `--screenshot` draws a block out of
levels 1, 3 and 5 and leaves one proud of level 6, because a tower nobody
has touched is a photograph of a setup. That is the frame
`refresh-screenshots` takes, so this holds the picture that is already on
show.

## Behavior

**`Cairn::new` takes `held` rather than reading the arguments.** A picture
test asks for the photographed state without `--screenshot` in its own
command line. `main` passes what the arguments say, which is the only place
that reads them.

**No `update` at all.** The pose runs on `Run`'s own fixed step inside
`new`, before anything is drawn, so the tower has already settled by the
time the test has a renderer. A `dt` of nought would only ease the camera,
and the camera is where `new` put it.

**The picture is the 3D.** The engine's harness leaves the overlay empty, so
the hint line and the refusal are not in shot.

**It is the engine's size**, 320 by 240, said once in `picture::SIZE`.

## Acceptance criteria

- The tower drawn out matches its
  picture. — `tests::the_tower_drawn_out_looks_right`

### Verified by hand

- **The picture itself.** The tower standing with the gap where three
  blocks came out, one lying proud across the face of it, and the whole
  thing throwing a shadow over the floor. The grain is legible, which is
  what spec 0003 is about and what no other test here can see.
- **A control.** The proud block left at level 7 rather than 6 failed the
  test on 1.9% of its pixels. Put back and compared.

## Out of scope

**The hint line and the refusal.** Both are 2D and the engine's harness
draws neither.

**A picture of a tower that has fallen.** A fall is spec 0001's arithmetic
and takes a different pose; one frame of the game is what there is to hold.

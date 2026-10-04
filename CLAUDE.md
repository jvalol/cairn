# cairn

A tower of blocks you take apart one at a time, putting each one you pull on
top, until it comes down. One player, one tower, one number at the end.

The eleventh game on blitzkit, and the first that is about boxes rather than
balls. Marble, carom and poolhall are all round things rolling on a flat floor,
because spheres were all the engine had until specs 0034, 0035 and 0036.

## Building and running

```
cargo run --release
```

Inside `blitzkit-project` this builds against the engine checkout rather than
the published crate, because of the `[patch.crates-io]` in
`blitzkit-project/.cargo/config.toml`. Cloned on its own it builds against
whatever is on crates.io, which is what a stranger gets.

```
cargo test && cargo clippy --all-targets -- -D warnings && cargo fmt --check
```

Or `./check-all` from `blitzkit-project`, which runs that for every crate and
then checks the copy markers, the spec citations and the listings.

## The spec flow

A spec before the code, in `specs/`, numbered in the order written. Each
acceptance criterion names the test that proves it, and `check-specs` fails the
gate when an implemented spec cites a test that does not exist. A draft is not
checked, which is what makes it safe to write one before building it.

## Decisions worth defending

**Four blocks a level, not three.** Three is the game this will be compared to,
and it is also a see-saw: take one of three and the level above rests on two
blocks with a gap where the middle was. Four leaves a margin.

**Not the open lattice of two from blitzkit's spec 0036.** That shape was chosen
to be hard on the engine and it is, but a level of two holds each block above it
at one end, so taking either away drops the level above however carefully it is
done. A game about taking blocks out has to have a first move. The lattice is
the engine's test, and blitzkit's `tower` example is where to see it.

**Nothing shared with the registered one.** Forty eight blocks, four to a level,
twelve levels, square in section. Not fifty four, three, eighteen, and half
again as wide as thick.

**A `Solver` kept across frames, not the free `step`.** Blitzkit's free step
remembers nothing between frames and puts nothing to sleep. That is right for a
few balls on a table and wrong for this: a twelve level stack sags the moment
it is built without warm starting, and leans while the player is thinking
without sleeping.

**The run ends when the tower drops, not when a block lands.** A block pulled
free and dropped is a block hitting the floor on purpose, and that happens every
turn.

## User-facing text

Every string this game draws carries `[COPY - Jake]` until Jake rewrites it into
his own voice and strikes the marker himself. `check-all` reads `src/` as well
as the readmes and holds the gate while one remains.

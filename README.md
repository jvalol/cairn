# cairn

A tower of blocks you take apart one at a time, putting each one you pull on
top, until it comes down. One player, one tower, one number at the end: how many
you got out before it fell.

```
cargo run --release
```

Twelve levels of four blocks, square in section and turned a quarter turn each
level. Point at a block and pull it out along its own length. The ones you get
out go on top, which is what makes it taller and worse the longer you last.

Built on [blitzkit](https://github.com/jvalol/blitzkit), and the first game here
that is about boxes rather than balls: it wants oriented bodies, contact patches
of four points rather than one, and a solver that remembers what each contact
pushed with last frame. Without that last one a stack this tall sags the moment
it is built.

Specs are in [`specs/`](specs/), written before the code.

## Licence

MIT or Apache-2.0, at your option.

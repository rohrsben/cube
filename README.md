Ever started playing with a rubik's cube, just doing the same thing over and over, and then realized "wow, this has been going for a long time. I wonder how long this pattern goes for?"

`cube` answers exactly that question.

# Usage

By default, `cube` initializes a 3x3 cube. The pattern is a list of movements in standard cubing notation: `S'` for a reverse front slice, `X` for a rotation, etc. `cube` will repeat the provided pattern until it reaches its starting state, printing it's current state along the way, and give you some stats at the end.

Movement notation breakdown: `<count?><action><layer?>`, where `count` and `layer` are optional. E.g.: `3M'` will do an `M'` slice 3 times. `layer` is only necessary for slice moves on cubes bigger than 3x3: it allows you to specify which layer the slice move targets. Layers are numbered with 1 being closest to you and furthest up; try it out in the terminal for a better idea of how it works. All told, a full move might look like `2E16` (if your cube is particularly large).

# Roadmap
- [ ] nix build
- [ ] ratatui tui

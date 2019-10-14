# strings-rs

A Rust implementation of the Unix `strings` program.


## Why?

I like knowing how the software I use works. I feel like it makes me a more
effective user of the software, as well as being a nice little exercise.

## Is it faster and GNU `strings`?

Unfortunately not. Since this was just a tiny exercise for me, I don't really
feel the need to make it faster than it is. For normal binary files one would
use `strings` on, it operates fast enough.

Maybe one day I'll do some research and make it more efficient. Maybe...

I'll probably go through classic /bin programs and rewrite them as exercises in
the future, with the goal being to get them somewhat efficient.

## TODO

* Add optional arguments
> this is just an experiment to see if all languages I most commonly use have the same behavior.

### don't take it more seriously than a mere curiosity

### why this?

i was setting the middleware of a webhook and to my surprise doing `os.Getenv("USERNAME")` and setting this same value in the basic auth of my request client (I use Insomnia, but there's no difference at all in using PostMan, Hoppscotch or a cURL) did not work at all.\ Immediately I started wondering how and debugged in the best way possible (no, not with a decent DAP in an IDE) but rather spamming `fmt`'s all around. And at least on Windows machines, the `os` username is the current user, in my case it printed out `raulb`. The solution is pretty straitght forward, just map the value to another key in the `.env` file. Nonetheless, this discovery did catch my eye.\

### personal considerations:

every language used is like C here (Rust, Typescript/Javascript and Go), so perhaps it'd be nice to consider if other paradigms treat it "differently" -- in the sense the way they see the OS -- (not that functional programming languages would be a realistic alternative for webdev like OCaml, [Haskell](https://github.com/ThePrimeagen/CHADstack))

(not sure how obvious it is, tho I haven't faced it before -- setting an enviroment variable as 'username' does not look the smartest but well, programmer lazyness)

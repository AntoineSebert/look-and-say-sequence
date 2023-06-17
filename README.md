# look-and-say-sequence

Calculates the nth term of the [Look-and-say sequence](https://en.wikipedia.org/wiki/Look-and-say_sequence).

## Build and run

```sh
$ ./look-and-say-sequence -n 5
111221
```

## Testing

```sh
$ cargo test
```

## Performance

The digits of this sequence grow linearly, and so does the computing power required to process a given step.
But because the index to reach requires the computation of all the previous steps, the time complexity of this problem is actually $N!$.

# What is this?
A benchmark comparing different ways of allocating a large nested hash in a Ruby program. A result of me digging deeper into [this puzzle](https://stackoverflow.com/questions/77494199/investigating-the-performance-of-a-ruby-gem-with-rust-extension).

# How to Run
```
rake benchmark
```

# Results
```
Warming up --------------------------------------
          Plain Ruby    21.000  i/100ms
         C extension    30.000  i/100ms
    rb-sys extension    14.000  i/100ms
    Magnus extension    11.000  i/100ms
Calculating -------------------------------------
          Plain Ruby    209.161  (± 2.4%) i/s -      1.050k in   5.022543s
         C extension    314.391  (± 2.9%) i/s -      1.590k in   5.061884s
    rb-sys extension    148.691  (± 2.7%) i/s -    756.000  in   5.087406s
    Magnus extension    118.048  (± 3.4%) i/s -    594.000  in   5.037219s

Comparison:
         C extension:      314.4 i/s
          Plain Ruby:      209.2 i/s - 1.50x  slower
    rb-sys extension:      148.7 i/s - 2.11x  slower
    Magnus extension:      118.0 i/s - 2.66x  slower

Plain Ruby = 4.78 ms
C extension = 3.18 ms
rb-sys extension = 6.73 ms
Magnus extension = 8.48 ms
```

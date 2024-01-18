# What is this?
A benchmark comparing different ways of allocating a large nested hash in a Ruby program. A result of me digging deeper into [this puzzle](https://stackoverflow.com/questions/77494199/investigating-the-performance-of-a-ruby-gem-with-rust-extension).

# How to Run
```
rake benchmark
```

# Results
```
Warming up --------------------------------------
          Plain Ruby    19.000  i/100ms
         C extension    31.000  i/100ms
    rb-sys extension    32.000  i/100ms
    Magnus extension    10.000  i/100ms
Calculating -------------------------------------
          Plain Ruby    206.215  (± 1.9%) i/s -      1.045k in   5.069316s
         C extension    314.509  (± 2.5%) i/s -      1.581k in   5.030353s
    rb-sys extension    323.220  (± 3.1%) i/s -      1.632k in   5.054636s
    Magnus extension    115.455  (± 5.2%) i/s -    580.000  in   5.035127s

Comparison:
    rb-sys extension:      323.2 i/s
         C extension:      314.5 i/s - same-ish: difference falls within error
          Plain Ruby:      206.2 i/s - 1.57x  slower
    Magnus extension:      115.5 i/s - 2.80x  slower

Plain Ruby = 4.85 ms
C extension = 3.18 ms
rb-sys extension = 3.10 ms
Magnus extension = 8.68 ms
```

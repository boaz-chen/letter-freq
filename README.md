# letter-freq
Benchmarking letter freq calculations in Rust

Here are the latest results:

```bash
letter counting - HashMap, Fold
                        time:   [65.461 ms 65.697 ms 65.949 ms]
                        change: [-4.2551% -3.8427% -3.3684%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe

Benchmarking letter counting - HashMap, ForLoop: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.2s, or reduce sample count to 90.
letter counting - HashMap, ForLoop
                        time:   [52.493 ms 52.726 ms 52.975 ms]
                        change: [-9.7525% -7.9966% -6.3044%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

letter counting - FxHashMap, Fold
                        time:   [22.622 ms 22.705 ms 22.795 ms]
                        change: [-4.2181% -3.6728% -3.1822%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

letter counting - FxHashMap, ForLoop
                        time:   [15.916 ms 15.962 ms 16.011 ms]
                        change: [-5.4967% -4.9162% -4.3772%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild

letter counting - FxHashMap, ForLoop, 4 threads
                        time:   [4.4693 ms 4.5118 ms 4.5577 ms]
                        change: [-5.8793% -4.2467% -2.5773%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

letter counting - FxHashMap, ForLoop, 8 threads
                        time:   [4.1975 ms 4.2047 ms 4.2127 ms]
                        change: [-2.0246% -1.7290% -1.4537%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

letter counting - FxHashMap, Rayon
                        time:   [5.3938 ms 5.4713 ms 5.5570 ms]
                        change: [-8.4224% -6.1377% -3.7943%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
  ```

Test send and receive throughput for Mozilla's Servo [ipc-channel](https://github.com/servo/ipc-channel) between 2 processes

## start
```
cargo bench
```

## results on MacBook Pro '14

```
sends/1024              time:   [7.8511 us 7.9644 us 8.0932 us]                        
                        thrpt:  [120.66 MiB/s 122.62 MiB/s 124.39 MiB/s]
Found 11 outliers among 100 measurements (11.00%)
  9 (9.00%) high mild
  2 (2.00%) high severe

sends/10240             time:   [61.313 us 61.641 us 61.968 us]                        
                        thrpt:  [157.59 MiB/s 158.43 MiB/s 159.28 MiB/s]

Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

sends/51200             time:   [310.77 us 314.08 us 318.15 us]                        
                        thrpt:  [153.47 MiB/s 155.46 MiB/s 157.12 MiB/s]
Found 15 outliers among 100 measurements (15.00%)
  4 (4.00%) high mild
  11 (11.00%) high severe

sends/102400            time:   [600.69 us 605.17 us 610.21 us]                         
                        thrpt:  [160.04 MiB/s 161.37 MiB/s 162.57 MiB/s]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe
```

```
receives/1024           time:   [7.6571 us 7.6989 us 7.7505 us]                           
                        thrpt:  [126.00 MiB/s 126.84 MiB/s 127.54 MiB/s]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

receives/10240          time:   [60.936 us 61.342 us 61.843 us]                           
                        thrpt:  [157.91 MiB/s 159.20 MiB/s 160.26 MiB/s]
Found 21 outliers among 100 measurements (21.00%)
  4 (4.00%) low severe
  7 (7.00%) high mild
  10 (10.00%) high severe

receives/51200          time:   [311.62 us 316.45 us 322.98 us]                           
                        thrpt:  [151.18 MiB/s 154.30 MiB/s 156.69 MiB/s]
Found 14 outliers among 100 measurements (14.00%)
  7 (7.00%) high mild
  7 (7.00%) high severe

receives/102400         time:   [661.13 us 668.65 us 676.87 us]                            
                        thrpt:  [144.28 MiB/s 146.05 MiB/s 147.71 MiB/s]
Found 10 outliers among 100 measurements (10.00%)
  5 (5.00%) high mild
  5 (5.00%) high severe
```


## Bincode

```
bincode_encode/1024     time:   [8.8087 us 9.0109 us 9.2546 us]                                 
                        thrpt:  [105.52 MiB/s 108.38 MiB/s 110.86 MiB/s]
Found 15 outliers among 100 measurements (15.00%)
  1 (1.00%) low mild
  6 (6.00%) high mild
  8 (8.00%) high severe

bincode_encode/10240    time:   [87.656 us 89.375 us 91.548 us]                                 
                        thrpt:  [106.67 MiB/s 109.27 MiB/s 111.41 MiB/s]
Found 12 outliers among 100 measurements (12.00%)
  6 (6.00%) high mild
  6 (6.00%) high severe

bincode_encode/51200    time:   [433.69 us 439.43 us 447.46 us]                                 
                        thrpt:  [109.12 MiB/s 111.12 MiB/s 112.59 MiB/s]
Found 13 outliers among 100 measurements (13.00%)
  3 (3.00%) high mild
  10 (10.00%) high severe

bincode_encode/102400   time:   [865.30 us 879.94 us 899.56 us]                                  
                        thrpt:  [108.56 MiB/s 110.98 MiB/s 112.86 MiB/s]
Found 16 outliers among 100 measurements (16.00%)
  2 (2.00%) high mild
  14 (14.00%) high severe
```

```
bincode_decode/1024     time:   [2.9798 us 3.1397 us 3.3105 us]                                 
                        thrpt:  [294.99 MiB/s 311.03 MiB/s 327.73 MiB/s]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe

bincode_decode/10240    time:   [29.083 us 31.390 us 34.096 us]                                  
                        thrpt:  [286.42 MiB/s 311.10 MiB/s 335.78 MiB/s]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

bincode_decode/51200    time:   [133.91 us 147.23 us 164.21 us]                                 
                        thrpt:  [297.34 MiB/s 331.64 MiB/s 364.64 MiB/s]
Found 14 outliers among 100 measurements (14.00%)
  3 (3.00%) high mild
  11 (11.00%) high severe

bincode_decode/102400   time:   [333.02 us 359.75 us 393.40 us]                                  
                        thrpt:  [248.24 MiB/s 271.45 MiB/s 293.24 MiB/s]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
```
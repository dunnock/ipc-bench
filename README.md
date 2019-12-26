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
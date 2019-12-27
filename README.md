Test send and receive throughput for Mozilla's Servo [ipc-channel](https://github.com/servo/ipc-channel) between 2 processes

## start
```shell
cargo bench
```

## Results

## Summary

Benches compare sending of byte buffer of a given size either within a `Serialize` derived structure vs plain buffer.

It appears that sending and receiving bytes has about 5-10 times higher throughput than sending serializable object. Surprisingly sending 50KiB is more than 2 times slower than sending 10KiB and 100KiB - should be very platform specific. Sending/receiving bytes buf throughput has high variance. Sending/receiving serializable object has low variance.

Tests performed on MacBook Pro '13 (which sucks at perf tests), tests named by pattern `type/message size`. 

### Send Message structure

Structure of a form, where size of Message::data variable between tests:

```rust
#[derive(Serialize, Deserialize)]
pub struct Message {
	//pub topic: String,
	pub topic: u32,
	pub data: Vec<u8>
}
```

Send `Message.clone()` via IpcSender (Serialized via bincode)

```log
sends/1024              time:   [7.8511 us 7.9644 us 8.0932 us]                        
                        thrpt:  [120.66 MiB/s 122.62 MiB/s 124.39 MiB/s]

sends/10240             time:   [61.313 us 61.641 us 61.968 us]                        
                        thrpt:  [157.59 MiB/s 158.43 MiB/s 159.28 MiB/s]

sends/51200             time:   [310.77 us 314.08 us 318.15 us]                        
                        thrpt:  [153.47 MiB/s 155.46 MiB/s 157.12 MiB/s]

sends/102400            time:   [600.69 us 605.17 us 610.21 us]                         
                        thrpt:  [160.04 MiB/s 161.37 MiB/s 162.57 MiB/s]
```

Receive `Message` via `IpcReceiver` (Deserialized via bincode)

```log
receives/1024           time:   [7.6571 us 7.6989 us 7.7505 us]                           
                        thrpt:  [126.00 MiB/s 126.84 MiB/s 127.54 MiB/s]

receives/10240          time:   [60.936 us 61.342 us 61.843 us]                           
                        thrpt:  [157.91 MiB/s 159.20 MiB/s 160.26 MiB/s]

receives/51200          time:   [311.62 us 316.45 us 322.98 us]                           
                        thrpt:  [151.18 MiB/s 154.30 MiB/s 156.69 MiB/s]

receives/102400         time:   [661.13 us 668.65 us 676.87 us]                            
                        thrpt:  [144.28 MiB/s 146.05 MiB/s 147.71 MiB/s]
```



## IPC bytes channel

```rust
type Message = Vec<u8>
```

Sending bytes buf `Vec<u8>.as_slice()`

```log
bytes_sends/1024        time:   [1.4149 us 1.4572 us 1.5078 us]                              
                        thrpt:  [647.69 MiB/s 670.16 MiB/s 690.22 MiB/s]

bytes_sends/10240       time:   [4.1461 us 4.2852 us 4.4799 us]                               
                        thrpt:  [2.1288 GiB/s 2.2255 GiB/s 2.3002 GiB/s]

bytes_sends/51200       time:   [57.111 us 58.914 us 60.991 us]                              
                        thrpt:  [800.58 MiB/s 828.80 MiB/s 854.97 MiB/s]

bytes_sends/102400      time:   [59.126 us 60.230 us 61.648 us]                               
                        thrpt:  [1.5470 GiB/s 1.5834 GiB/s 1.6129 GiB/s]
```

Sending cloned bytes buf `Vec<u8>.clone().as_slice()`

```log
bytes_sends_cloned/1024 time:   [1.6410 us 1.6559 us 1.6758 us]                                     
                        thrpt:  [582.73 MiB/s 589.75 MiB/s 595.09 MiB/s]

bytes_sends_cloned/10240                                                                             
                        time:   [4.0405 us 4.0725 us 4.1074 us]
                        thrpt:  [2.3219 GiB/s 2.3417 GiB/s 2.3603 GiB/s]

bytes_sends_cloned/51200                                                                             
                        time:   [43.689 us 44.761 us 45.886 us]
                        thrpt:  [1.0392 GiB/s 1.0653 GiB/s 1.0914 GiB/s]

bytes_sends_cloned/102400                                                                            
                        time:   [70.887 us 72.588 us 74.637 us]
                        thrpt:  [1.2778 GiB/s 1.3138 GiB/s 1.3454 GiB/s]
```

Receiving bytes buf as `Vec<u8>`

```log
bytes_receives/1024     time:   [1.3999 us 1.4303 us 1.4700 us]                                 
                        thrpt:  [664.33 MiB/s 682.75 MiB/s 697.62 MiB/s]

bytes_receives/10240    time:   [4.1569 us 4.2248 us 4.3060 us]                                  
                        thrpt:  [2.2147 GiB/s 2.2573 GiB/s 2.2942 GiB/s]

bytes_receives/51200    time:   [38.904 us 42.204 us 45.431 us]                                 
                        thrpt:  [1.0496 GiB/s 1.1298 GiB/s 1.2257 GiB/s]

bytes_receives/102400   time:   [60.481 us 62.754 us 65.247 us]                                  
                        thrpt:  [1.4616 GiB/s 1.5197 GiB/s 1.5768 GiB/s]
```


## Bincode

It does seem that main reason of slow send is Serialization/Deserialization, let's see bincode for 
```rust
#[derive(Serialize, Deserialize)]
pub struct Message {
	//pub topic: String,
	pub topic: u32,
	pub data: Vec<u8>
}
```

Encode

```log
bincode_encode/1024     time:   [8.8087 us 9.0109 us 9.2546 us]                                 
                        thrpt:  [105.52 MiB/s 108.38 MiB/s 110.86 MiB/s]

bincode_encode/10240    time:   [87.656 us 89.375 us 91.548 us]                                 
                        thrpt:  [106.67 MiB/s 109.27 MiB/s 111.41 MiB/s]

bincode_encode/51200    time:   [433.69 us 439.43 us 447.46 us]                                 
                        thrpt:  [109.12 MiB/s 111.12 MiB/s 112.59 MiB/s]

bincode_encode/102400   time:   [865.30 us 879.94 us 899.56 us]                                  
                        thrpt:  [108.56 MiB/s 110.98 MiB/s 112.86 MiB/s]
```

Decode

```log
bincode_decode/1024     time:   [2.9798 us 3.1397 us 3.3105 us]                                 
                        thrpt:  [294.99 MiB/s 311.03 MiB/s 327.73 MiB/s]

bincode_decode/10240    time:   [29.083 us 31.390 us 34.096 us]                                  
                        thrpt:  [286.42 MiB/s 311.10 MiB/s 335.78 MiB/s]

bincode_decode/51200    time:   [133.91 us 147.23 us 164.21 us]                                 
                        thrpt:  [297.34 MiB/s 331.64 MiB/s 364.64 MiB/s]

bincode_decode/102400   time:   [333.02 us 359.75 us 393.40 us]                                  
                        thrpt:  [248.24 MiB/s 271.45 MiB/s 293.24 MiB/s]
```


## Further investigation

[ipc_channel_custom.rs](https://github.com/dunnock/ipc-bench/blob/master/benches/ipc_channel_custom.rs) takes it even further, using the same underlying data layer (just a `Vec<u8>`) with 
channels instantiated either `ipc::channel` or `ipc::bytes_channel`, when sending bytes it uses custom tailored serialization.

As it can be seen custom tailored Serialization/Deserialization implementation working up to 10x times faster than bincode's.

### bincode's serialization with simple `struct(Vec<u8>)` data:

```
sends_custom/1024       time:   [7.1496 us 7.1883 us 7.2374 us]                               
                        thrpt:  [134.93 MiB/s 135.85 MiB/s 136.59 MiB/s]

sends_custom/10240      time:   [56.718 us 56.893 us 57.076 us]                               
                        thrpt:  [171.10 MiB/s 171.65 MiB/s 172.18 MiB/s]

sends_custom/51200      time:   [295.99 us 298.58 us 301.73 us]                               
                        thrpt:  [161.83 MiB/s 163.53 MiB/s 164.97 MiB/s]

sends_custom/102400     time:   [581.99 us 583.39 us 585.09 us]                                
                        thrpt:  [166.91 MiB/s 167.39 MiB/s 167.80 MiB/s]
```

```
receives_custom/1024    time:   [7.1560 us 7.2338 us 7.3347 us]                                  
                        thrpt:  [133.14 MiB/s 135.00 MiB/s 136.47 MiB/s]

receives_custom/10240   time:   [56.625 us 56.965 us 57.326 us]                                  
                        thrpt:  [170.35 MiB/s 171.43 MiB/s 172.46 MiB/s]

receives_custom/51200   time:   [296.42 us 296.94 us 297.62 us]                                  
                        thrpt:  [164.06 MiB/s 164.44 MiB/s 164.73 MiB/s]

receives_custom/102400  time:   [588.19 us 590.89 us 593.70 us]                                   
                        thrpt:  [164.49 MiB/s 165.27 MiB/s 166.03 MiB/s]
```

### custom tailored serialization based on same `Vec<u8>` data layer:

```
sends_custom_bytes/1024 time:   [1.5784 us 1.6231 us 1.6922 us]                                     
                        thrpt:  [577.10 MiB/s 601.66 MiB/s 618.70 MiB/s]
sends_custom_bytes/10240                                                                             
                        time:   [3.6300 us 3.6490 us 3.6735 us]
                        thrpt:  [2.5961 GiB/s 2.6135 GiB/s 2.6272 GiB/s]

sends_custom_bytes/51200                                                                             
                        time:   [29.473 us 29.964 us 30.615 us]
                        thrpt:  [1.5575 GiB/s 1.5914 GiB/s 1.6179 GiB/s]
sends_custom_bytes/102400                                                                            
                        time:   [51.452 us 52.001 us 52.597 us]
                        thrpt:  [1.8132 GiB/s 1.8339 GiB/s 1.8535 GiB/s]
```

```
receives_custom_bytes/1024                                                                             
                        time:   [1.5848 us 1.6060 us 1.6289 us]
                        thrpt:  [599.51 MiB/s 608.05 MiB/s 616.20 MiB/s]

receives_custom_bytes/10240                                                                             
                        time:   [3.6046 us 3.6162 us 3.6305 us]
                        thrpt:  [2.6269 GiB/s 2.6372 GiB/s 2.6457 GiB/s]

receives_custom_bytes/51200                                                                             
                        time:   [30.061 us 31.198 us 32.522 us]
                        thrpt:  [1.4662 GiB/s 1.5284 GiB/s 1.5862 GiB/s]

receives_custom_bytes/102400                                                                            
                        time:   [53.772 us 55.318 us 57.163 us]
                        thrpt:  [1.6683 GiB/s 1.7240 GiB/s 1.7735 GiB/s]
```
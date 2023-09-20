This repository contains the code and results for a performance benchmarking project that compares the performance of different web runtimes, including Go, Rust, Bun, and Node.js, in handling HTTP requests and JSON serialization. The goal of this project was to provide insights into the capabilities of these runtimes and help developers make informed decisions when choosing a runtime for their projects.


##Testing Environment
The benchmark tests were conducted on a local machine with the following specifications:
```
CPU: AMD Ryzen 7 5800X (16 cores) @ 3.800GHz
RAM: 32GB @ 2800 MHz
Storage: High-speed M.2 drive
```

The software versions used for testing were as follows:
```
Rust: rustc 1.74.0-nightly (bdb0fa3ee 2023-09-19)
Go: go version go1.21.1 linux/amd64
Bun: 1.0.2
Node.js: v20.6.1
```

##Running the code

Navigate to automate-benchmarks
run "cargo run " 
wait for results 


To alter test time please change the sleep & wrk command query param

The sleeps inbetween functions are neccesary due to Golang and Rust *ignoring shutdown requests* until i/o operations have completed

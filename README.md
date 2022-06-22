# xdump-rs

Toy hexdump utility made in Rust

## Usage

Build with `cargo build`

```sh
./xdump-rs <input_file>
```

Output:

````
000:    23 20 78 64 75 6d 70 2d 72 73 0d 0a 0d 0a 54 6f     |#.xdump-rs....To|
010:    79 20 68 65 78 64 75 6d 70 20 75 74 69 6c 69 74     |y.hexdump.utilit|
020:    79 20 6d 61 64 65 20 69 6e 20 52 75 73 74 2e 0d     |y.made.in.Rust..|
030:    0a 0d 0a 23 23 20 55 73 61 67 65 0d 0a 0d 0a 42     |...##.Usage....B|
040:    75 69 6c 64 20 77 69 74 68 20 60 63 61 72 67 6f     |uild.with.`cargo|
050:    20 62 75 69 6c 64 60 0d 0a 0d 0a 60 60 60 73 68     |.build`....```sh|
060:    0d 0a 2e 2f 78 64 75 6d 70 2d 72 73 20 3c 69 6e     |.../xdump-rs.<in|
070:    70 75 74 5f 66 69 6c 65 3e 0d 0a 60 60 60 0d 0a     |put_file>..```..|
080:    0d 0a 4f 75 74 70 75 74 3a 0d 0a 0d 0a 60 60 60     |..Output:....```|
090:    0d 0a 0d 0a 60 60 60 0d 0a                          |....```..|
````

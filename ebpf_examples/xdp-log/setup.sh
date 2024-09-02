# Take the build of ebpf program
cargo xtask build-ebpf
# command to see the output
llvm-objdump -S target/bpfel-unknown-none/debug/block-packet
# to run the program
cargo xtask run -- -h
# to see the logs(run in a new terminal)
RUST_LOG=info cargo xtask run -- --iface wlp58s0
# After stoppping using Ctrl+C checking the program get detached
sudo bpftool prog list

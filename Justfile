# Compiler and run .tb file
compile inp args:
    cargo run {{inp}} {{args}} && gcc out.c -o ./target/out && ./target/out

# To generetate AST to stdout
debug inp:
    cargo run {{inp}} --debug

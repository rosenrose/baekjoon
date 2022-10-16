$num = $args[0]
$cwd = pwd

cd $PSScriptRoot

ren "src/main.rs" "main_.rs"
ren "src/$num.rs" "main.rs"
cargo run

ren "src/main.rs" "$num.rs"
ren "src/main_.rs" "main.rs"

cd $cwd

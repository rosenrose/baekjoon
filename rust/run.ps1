$num = $args[0]
$cwd = pwd

cd $PSScriptRoot

ren "src/main.rs" "main_.rs"
cp "src/$num.rs" "src/main.rs"

cargo run

rm "src/main.rs"
ren "src/main_.rs" "main.rs"

cd $cwd

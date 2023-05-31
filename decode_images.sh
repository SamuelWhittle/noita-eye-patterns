for file in ./images/*
do
  cargo run -- --path ${file}
done

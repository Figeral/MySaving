cargo build --target wasm32-unknown-unknown --release  # build project in web assembly with target bit 32 
stellar contract optimize --wasm target/wasm32v1-none/release/my_saving.wasm  # this the standard way but can fail due to incompatibily of version of  the stellar sdk with the rustup compiler
wasm-opt target/wasm32-unknown-unknown/release/your_contract.wasm -O -o target/optimized.wasm # in case the the previous optimisation cli faile , use this one . 
 # it's base on wasm-opt and is part of the binaryen library . so make sure to install that  .ie  sudo apt  install binaryen or brew install binaryen depending on ur os   
 
# This the deployment scipt  if you optimized ur wasm32 file with stellar cli 
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/my_saving.wasm \
  --source fitzgerald \   
  --network testnet \
  --alias my_saving 

# This the deployment scipt  if you optimized ur wasm32 file with wasm-opt cli 
 stellar contract deploy \
  --wasm target/optimized.wasm \
  --source fitzgerald \
  --network testnet \
  --alias my_saving

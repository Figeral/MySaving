# MySaving Project

## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
├── contracts
│   └── Mysaving
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```
As the  required this contract has the following functionalities
- initialize (seed the storage with contract owner address and pause widthdrawal status , ie paused by default) . In case of deployment , should be the first function called. 
- Deposites 
- get user deposites
- get all deposities ( restricted to contract owner)
- get user balance
- widthdrawal 
- get user widthdrawals
- get all widthdrawals ( restricted to contract owner )
- pause and unpause transactions ( restricted to contract owner)
- is pause widthdrawal ( 
## Usage 
### Address
  To use this contract to need to create a local address with stellar or soroban cli on the testnet and fund it . 
  ```bash
    stellar keys generate --global alice --network testnet --fund
  ```

  For testing purpose i deliberate create an address found in the `.stellar/identity` folder called `fitzgerald` with address 
  ```
  https://stellar.expert/explorer/testnet/account/GBMX6SNS6YK2A7QVEUELBMDCK4TDWW7QIAV2EUKDUHI5G5GAJOFN7FDF
  ```
  This address was alway used to deploy the contract and used as owner address in the initialized function

### Live 
The contract  testnet address is `CALLWNMZ2NTX32XBO4UD3IGPD46YJSY46EVRFX73VSEITTRYIYQ4ISXR` and lives at 
```
https://stellar.expert/explorer/testnet/contract/CALLWNMZ2NTX32XBO4UD3IGPD46YJSY46EVRFX73VSEITTRYIYQ4ISXR
```

The complete deployment procedure script is in the `deploy.sh` file. Considering you already install the stellar software development kit . 

### Example 
makeing a deposite 
```bash
stellar contract invoke \
  --id CALLWNMZ2NTX32XBO4UD3IGPD46YJSY46EVRFX73VSEITTRYIYQ4ISXR\
  --source fitzgerald \
  --network testnet \
  -- \
  deposite \       
  --user_addr GBMX6SNS6YK2A7QVEUELBMDCK4TDWW7QIAV2EUKDUHI5G5GAJOFN7FDF   --amount 109200
```

geting balanace 
```
stellar contract invoke \
  --id CALLWNMZ2NTX32XBO4UD3IGPD46YJSY46EVRFX73VSEITTRYIYQ4ISXR\
  --source fitzgerald \
  --network testnet \
  -- \
  get_balance \
```

making a widthdrawal 
```
 --id CALLWNMZ2NTX32XBO4UD3IGPD46YJSY46EVRFX73VSEITTRYIYQ4ISXR\
  --source fitzgerald \
  --network testnet \
  -- \
  widthdrawal \
  --user_addr GBMX6SNS6YK2A7QVEUELBMDCK4TDWW7QIAV2EUKDUHI5G5GAJOFN7FDF   --amount 4600
```
pause widthdrawal ps. only fitzgerald source address can invoke this function since his initialized as the owner 
```
stellar contract invoke \
  --id CALLWNMZ2NTX32XBO4UD3IGPD46YJSY46EVRFX73VSEITTRYIYQ4ISXR\
  --source fitzgerald \
  --network testnet \
  -- \
  pause_widthdrawal \
  --owner_addr GBMX6SNS6YK2A7QVEUELBMDCK4TDWW7QIAV2EUKDUHI5G5GAJOFN7FDF --action pause
```

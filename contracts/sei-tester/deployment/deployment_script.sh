#!/bin/bash

echo -n Customized sei-tester Contract \(../../artifacts/sei_tester.wasm by default\):
read contract
echo
echo -n Customized Key Name:\(admin by default\)
read keyname
echo
echo -n Keyring Password:\(12345678 by default\)
read password
echo

if [ -z "${contract}" ];
then contract=../../artifacts/sei_tester.wasm
fi 
if [ -z "${keyname}" ];
then keyname=admin
fi 
if [ -z "${password}" ];
then password="12345678\n"
fi 

seid=~/go/bin/seid
code=$(printf $password | $seid tx wasm store $contract -y --from=$keyname --chain-id=sei-chain --gas=10000000 --fees=10000000usei --broadcast-mode=block | grep -A 1 -m 1 "code_id" | sed -n 's/.*value: "//p' | sed -n 's/"//p')
admin_addr=$(printf $password |$seid keys show $keyname | grep -A 1 -m 1 "address" | sed -n 's/.*address: //p')
addr=$(printf $password |$seid tx wasm instantiate $code "{}" --from $keyname --broadcast-mode=block --label "sei-tester" --no-admin --chain-id sei-chain --gas=30000000 --fees=300000usei -y | grep -A 1 -m 1 "key: _contract_address" | sed -n 's/.*value: //p' | xargs)

printf $password |$seid tx dex register-contract $addr $code true false 1000000000 -y --from=$keyname --chain-id=sei-chain --fees=10000000usei --gas=10000000 --broadcast-mode=block

printf "\n\nDeployed sei tester contract code id is %s\n" $code

printf "\n\nDeployed sei tester contract address is %s\n" $addr

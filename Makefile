
.PHONY: deploy
deploy:
	near deploy --accountId stake2.nexeranetv2.testnet --wasmFile ./release/staking_pool_test.wasm

.PHONY: init
init:
	near deploy --accountId stake2.nexeranetv2.testnet --wasmFile ./release/staking_pool_test.wasm --initFunction init --initArgs  '{"owner_id": "nexeranetv2.testnet"}'

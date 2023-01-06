### MultiversX devnet/testnet faucet (distribution) smart contract

The primary purpose of the smart contract is to have a simple solution for ESDT tokens faucet on the devnet and testnet. It is helpful for demo purposes and testing.

**Example use case**: You want to showcase some dApp where users can log in using auth providers, and then they need a specific ESDT token to play around with. You can send some tokens to the smart contract, and then users will be able to claim some of them. 

### Assumptions

- Users can claim the maximum tokens. The max is set when depositing
- Users can call the claim endpoint once per 24 hours
- Everyone can send ESDT tokens to the smart contract

### How to deploy and interact with erdpy

For now, the best would be to use the erdpy (example for the devnet) (there will also be an option with the Buildo Begins tool, it should be much simpler then):

Deploy (max tokens per day, a percentage from the whole supply, as an argument, here 1)
```bash
erdpy --verbose contract deploy --chain="D" --project=esdt-faucet-sc --pem="walletKey.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send
```

Deposit and call the setLimit function on sc (arguments for setLimit (here 100), we are depositing 10_000_000 of the token)
```bash
erdpy --verbose contract call <your_smart_contract_address_here> --chain="D" --pem="walletKey.pem" --gas-limit=3000000 --function="ESDTTransfer" --arguments str:BUILDO-890d14 10000000000000000000000000 str:setLimit 100000000000000000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send
```
The `BUILDO-890d14` token has 18 decimal places.

Claim ESDTs (token id (here BUILDO-890d14) and amount to claim (here 3))
```bash
erdpy --verbose contract call <your_smart_contract_address_here> --chain="D" --pem="walletKey.pem" --gas-limit=3000000 --function="claim" --arguments str:BUILDO-890d14 3000000000000000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send
```
The `BUILDO-890d14` token has 18 decimal places.

### How to interact with it using the frontend dApp

It is quite straightforward. The Nextjs dApp is based on [nextjs-dapp-template](https://github.com/xdevguild/nextjs-dapp-template). By default it will use the smart contract listed below. But you can deploy yours and change it in the .env variables.

With the dApp you can login using one of four auth providers and then deposit the ESDTs if you are the manager or claim ESDTs if you need some of them.

The Dapp repository: https://github.com/xdevguild/esdt-faucet-dapp.

Live app on the devnet: https://devnet-multiversx-esdt-faucet.netlify.app

### Currently deployed example smart contract

Just an example, you can use this one or deploy yours.

- [erd1qqqqqqqqqqqqqpgqwd59aum8c7c72ces7cezsmhqd8rqrtwagtksp6jahr](https://devnet-explorer.elrond.com/accounts/erd1qqqqqqqqqqqqqpgqwd59aum8c7c72ces7cezsmhqd8rqrtwagtksp6jahr)

### TODO

- other tokens types (SFT/NFT)
- buildo begins interactions (as general smart contract-related functionality)
- more configuration options?

### Issues, ideas

Please report all bugs and ideas for improvements.

### You may also like

- [Elven Tools](https://github.com/ElvenTools)
- [Elven.js](https://github.com/juliancwirko/elven.js)
- [Buildo Begins](https://github.com/xdevguild/buildo-begins)

import * as anchor from "@coral-xyz/anchor"
import {program} from "./wallet";
import {createNFTTokenMintAccount, getNftMintAccount} from "./nft_token";

export async function nftStake(
    wallet: anchor.Wallet,
    id: string
){
    return await program.methods.nftStake()
        .accounts({
            // @ts-ignore
            nftMintAccount: await getNftMintAccount(id)
        })
        .signers([wallet.payer])
        .rpc()
}
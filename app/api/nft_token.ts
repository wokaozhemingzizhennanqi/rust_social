import * as anchor from "@coral-xyz/anchor"
import {program} from "./wallet";
import BN from "bn.js";

export async function createNFTTokenMintAccount(
    wallet: anchor.Wallet,
    id:string
) {

    // 4. 发送交易
    return await program.methods.createNftTokenAccount(id)
        .accounts({})
        .signers([wallet.payer])
        .rpc();
}

export async function getNftMintAccount(
    id:string
) {
    const [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync([
            Buffer.from("nft"),
            Buffer.from(id),
        ], program.programId,
    );
    return mintAccount;
}
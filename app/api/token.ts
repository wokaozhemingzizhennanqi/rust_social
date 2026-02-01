import * as anchor from "@coral-xyz/anchor"
import {program} from "./wallet";

export async function createTokenMintAccount(
    wallet: anchor.Wallet,
) {
    const METADATA_PROGRAM_ID = new anchor.web3.PublicKey("8Gtnt8JeHqGUmdQpw3Rxxt9itjTAfoNXGkF5LbzT3vj5");
    const [splTokenPda,]=anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("mint_v21"),],
        program.programId,
    );
    const [metadataPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("metadata"),
            METADATA_PROGRAM_ID.toBuffer(),
            splTokenPda.toBuffer(),
        ],
        METADATA_PROGRAM_ID
    );
    return [
        splTokenPda,
        await program.methods.createTokenMintAccount()
            .accounts({
                authority: wallet.publicKey,
                // @ts-ignore
                tokenMetadataProgram: METADATA_PROGRAM_ID,
                // @ts-ignore
                metadataAccount: metadataPda,
            }).rpc()
    ];
}
import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor";
import { RustSocial } from "../../target/types/rust_social";


let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.rustSocial as Program<RustSocial>;

export { program, provider };
export function useDefaultWallet() {

    return anchor.Wallet.local();
}

//Pubkey EH68CzjJA1DgSyuNWhARoXG3hJWNha78qvcD8XdgPFrq
export function useVisitorWallet(){
    const keypair = anchor.web3.Keypair.fromSecretKey(
        new Uint8Array([100,21,127,169,52,63,97,216,200,179,90,171,231,203,234,145,71,79,241,158,236,212,203,60,6,205,101,173,156,37,184,213,216,9,132,213,116,246,142,107,43,134,185,201,246,165,1,33,121,163,13,186,95,135,156,92,176,147,158,224,202,111,255,186])
    );

    return new anchor.Wallet(keypair);
}
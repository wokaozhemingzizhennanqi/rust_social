import {useDefaultWallet, useVisitorWallet} from "./api/wallet";
import {createProfile, getProfile} from "./api/profile";

(async ()=>{
    const defaultWallet = useDefaultWallet();
    const visitWallet = useVisitorWallet();
    const r1 = await createProfile(visitWallet,"Alice");
    console.log("r1",r1);

    const r2 =await getProfile(visitWallet);
    console.log("r2",r2)
})()
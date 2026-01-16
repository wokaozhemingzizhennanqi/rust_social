import {useDefaultWallet, useVisitorWallet} from "./api/wallet";
import {createProfile, getProfile} from "./api/profile";
import "./api/tweet";
import {createLike, createTweet, getTweet} from "./api/tweet";

(async ()=> {
    const defaultWallet = useDefaultWallet();
    const visitWallet = useVisitorWallet();
    try {

        const r1 = await createProfile(defaultWallet, "Alice");
        console.log("r1", r1);

        const r2 = await getProfile(defaultWallet);
        console.log("r2", r2)

        const r11 = await createProfile(visitWallet, "Bob");
        console.log("r11", r11);

        const r22 = await getProfile(visitWallet);
        console.log("r22", r22)


    } catch (e) {
        // console.log(e);
    }

    const [pda, r3] = await createTweet(defaultWallet, "hello world");
    console.log(r3);
    const r4 = await getTweet(defaultWallet, pda)

    console.log(r4);

    const r5 = await createLike(visitWallet, pda)
    console.log(r5);

    const r6 = await getTweet(defaultWallet, pda)

    console.log(r6);
    const r7 = await createLike(visitWallet, pda)

})()
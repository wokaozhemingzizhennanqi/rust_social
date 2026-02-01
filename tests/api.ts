import * as anchor from "@coral-xyz/anchor";
import { main } from "../app/index";

describe("api", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  it("runs api logic", async () => {
    await main();
  });
});

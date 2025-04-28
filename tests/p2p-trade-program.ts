import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { P2pTradeProgram } from "../target/types/p2p_trade_program"; 
import {

  getAssociatedTokenAddressSync,

} from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";

describe("p2p_trade_program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.P2pTradeProgram as Program<P2pTradeProgram>;
  const connection = provider.connection;

  const creator = provider.wallet;

  const tokenForSaleMint = new PublicKey("DNMVCRJKfHe4yz5tcgteSn6e75WHXUtoZZxKjcii2uze");  
  const receivedTokenMint = new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"); 

  const tradeId = new BN(Math.floor(Date.now() / 1000)); 

  const tradeAmount = new BN(100);
  const expectedAmount = new BN(10);
  let creatorAtaForSale: PublicKey;
  before(async () => {
    console.log(`Creator: ${creator.publicKey.toBase58()}`);
    console.log(`Created tokenForSaleMint: ${tokenForSaleMint.toBase58()}`);
    creatorAtaForSale = await getAssociatedTokenAddressSync (
      tokenForSaleMint,   
      creator.publicKey   
    );
    console.log(`Created creatorAtaForSale: ${creatorAtaForSale.toBase58()}`);

     });

  it("Should create a trade escrow successfully", async () => {

    const params: any = {
      tradeId: tradeId,
      tradeAmount: tradeAmount,
      expectedAmount: expectedAmount,
      recipient: null,
    };

    try {
      const txSignature = await program.methods
        .createTrade(params)
        .accounts({
          creator: creator.publicKey,
          receivedTokenMintAccount: receivedTokenMint,
          tokenForSale: tokenForSaleMint,
          creatorAtaForSale: creatorAtaForSale,
        })
        .signers([creator.payer]) 
        .rpc();

      console.log("create_trade transaction signature", txSignature);
      const latestBlockHash = await connection.getLatestBlockhash();
      await connection.confirmTransaction({
          blockhash: latestBlockHash.blockhash,
          lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
          signature: txSignature,
      }, "confirmed");

    } catch (error) {
      console.error("Error during create_trade:", error);
      if (error instanceof anchor.AnchorError) {
          console.error("AnchorError:", error.error);
          console.error("AnchorError:", error.logs);

      }
      throw error; 
    }
  });
});
import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { P2pTradeProgram } from "../target/types/p2p_trade_program";
import {
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import * as fs from 'fs';
import * as os from 'os';
import * as path from 'path';
import { Keypair, PublicKey } from "@solana/web3.js";

describe("p2p_trade_program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.P2pTradeProgram as Program<P2pTradeProgram>;
  const connection = provider.connection;

  const creator = provider.wallet;

  const tokenForSaleMint = new PublicKey("DNMVCRJKfHe4yz5tcgteSn6e75WHXUtoZZxKjcii2uze");
  const receivedTokenMint = new PublicKey("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr");

  const tradeId = new BN(Math.floor(Date.now() / 1000));

  const tradeAmount = new BN(100 * 10 ** 9); // 100 spl tokens
  const expectedAmount = new BN(10 * 10 ** 6); // 10 USDC
  let creatorAtaForSale: PublicKey;
  before(async () => {
    console.log(`Creator: ${creator.publicKey.toBase58()}`);
    console.log(`Created tokenForSaleMint: ${tokenForSaleMint.toBase58()}`);
    creatorAtaForSale = await getAssociatedTokenAddressSync(
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


  const buyerKeypairPath = path.join(
    os.homedir(),
    ".config",
    "solana",
    "buyer-keypair.json"
  );

  const buyerSecretKey = JSON.parse(fs.readFileSync(buyerKeypairPath, "utf-8"));
  const buyerKeypair = Keypair.fromSecretKey(Uint8Array.from(buyerSecretKey));

  // const tradeId = new BN(1746030989);

  it("Should swap tokens successfully", async () => {
    try {
      const tx = await program.methods
        .exchange(tradeId)
        .accounts({
          buyer: buyerKeypair.publicKey,
          owner: creator.publicKey,
          tradeTokenMint: tokenForSaleMint,
          receivedTokenMint: receivedTokenMint,
        } as any)
        .signers([buyerKeypair])
        .rpc();
      console.log("exchange transaction signature", tx);


    } catch (error) {
      console.error("Error during exchange:", error);
      if (error instanceof anchor.AnchorError) {
        console.error("AnchorError:", error.error);
        console.error("AnchorError:", error.logs);

      }
      throw error;
    }

  });

  it.only("Should cancel a trade escrow successfully", async () => {
    try{
      const tx = await program.methods
        .cancel(new BN(1746190754))
        .accounts({
          // owner: creator.publicKey,
          tradeTokenMint: tokenForSaleMint,

        })
        .signers([creator.payer])
        .rpc();
      console.log("cancel transaction signature", tx);

    }catch(error){
      console.error("Error during cancel:", error);
      if (error instanceof anchor.AnchorError) {
        console.error("AnchorError:", error.error);
        console.error("AnchorError:", error.logs);

      }
      throw error;
    }
  });

  it.only("Should fetch all trades successfully", async () => {
    
    try{
    const allEscrowAccounts = await program.account.escrow.all();

    console.log(`Found ${allEscrowAccounts.length} total escrow accounts.`);
    // const activeTrades = allEscrowAccounts.filter(escrow => {
    //   return escrow.account.stage === EscrowStage.ReadyExchange;
    // });
    // console.log(`Found ${allEscrowAccounts.length} active trade offers.`);

    allEscrowAccounts.forEach(trade => {
      const stageName = Object.keys(trade.account.stage)[0];
      console.log(`Trade PDA: ${trade.publicKey.toBase58()}`);
      console.log(` Owner: ${trade.account.owner.toBase58()}`);

      console.log(` Trade ID: ${trade.account.tradeId.toString()}`);
      console.log(` Stage: ${stageName}`);
      console.log(` Token for Sale Mint: ${trade.account.tradeTokenMint.toBase58()}`);
      console.log(` Amount for Sale: ${trade.account.tradeAmount.toString()}`);
      console.log(` Received Token Mint: ${trade.account.receivedTokenMint.toBase58()}`);
      console.log(` Expected Amount: ${trade.account.expectedAmount.toString()}`);
    });
    return allEscrowAccounts;

  } catch (error) {
    console.error("Error fetching escrow accounts:", error);
    return [];
  }
  });

});

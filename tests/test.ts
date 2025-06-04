import {
  Connection,
  Keypair,
  PublicKey,
  LAMPORTS_PER_SOL,
  sendAndConfirmTransaction,
  Transaction,
  SystemProgram,
} from "@solana/web3.js";

import { Schema, serialize } from "borsh";
import { Buffer } from "buffer";
import bs58 from "bs58";
import BN from "bn.js"


const TOKEN_PROGRAM_ID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
const RENT_SYSVAR_ID = new PublicKey("SysvarRent111111111111111111111111111111111");

// 🔧 Replace with your actual deployed program ID
const PROGRAM_ID = new PublicKey("A31CQ2Fp1uSRbegjeE2UcmzNJcRD6WGJdEHqUWhoragC");

// ---------- Instruction Structs ----------
enum TokenSaleInstruction {
  // Init = 0,
  Create = 0,
  FirstBuy = 1,
  Buy = 2,

}


const payer = Keypair.fromSecretKey(new Uint8Array([48, 174, 69, 202, 218, 102, 226, 103, 80, 40, 101, 19, 250, 55, 203, 59, 143, 84, 54, 208, 72, 247, 221, 138, 103, 150, 252, 18, 65, 191, 182, 70, 55, 46, 184, 46, 76, 180, 86, 36, 154, 95, 53, 30, 5, 43, 18, 90, 220, 127, 221, 163, 247, 41, 4, 185, 175, 68, 203, 152, 126, 124, 168, 183]));


const connection = new Connection("https://api.devnet.solana.com", "confirmed");

const pdaSeed = Buffer.from("pda-token");


const mint = Keypair.generate();

const [pda, bump] = PublicKey.findProgramAddressSync([pdaSeed], PROGRAM_ID);
let ata = PublicKey.findProgramAddressSync(
  [
    pda.toBuffer(),
    TOKEN_PROGRAM_ID.toBuffer(),
    mint.publicKey.toBuffer(),
  ],
  ASSOCIATED_TOKEN_PROGRAM_ID,
)

let deata = PublicKey.findProgramAddressSync(
  [
    payer.publicKey.toBuffer(),
    TOKEN_PROGRAM_ID.toBuffer(),
    mint.publicKey.toBuffer(),
  ],
  ASSOCIATED_TOKEN_PROGRAM_ID,
)
// ---------- Utility ----------


const create = (async () => {


  const tx = new Transaction();
  let amount = new BN(1000000)

  const instructionData = Buffer.concat([
    Buffer.from(Int8Array.from([TokenSaleInstruction.Create]).buffer),
    Uint8Array.of(6),
    amount.toBuffer('le', 8),
  ]);

  // Construct your program accounts
  // Note: You must derive and fetch the associated token account as well

  const instruction = {
    keys: [
      { pubkey: mint.publicKey, isSigner: true, isWritable: true },
      { pubkey: payer.publicKey, isSigner: true, isWritable: true },
      { pubkey: RENT_SYSVAR_ID, isSigner: false, isWritable: false },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: pda, isSigner: false, isWritable: true },
      { pubkey: ata[0], isSigner: false, isWritable: true },
      { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      // Add associated_token_account and associated_token_program here...
    ],
    programId: PROGRAM_ID,
    data: instructionData,
  };
  // console.log(instruction.keys)

  tx.add(instruction);

  const txid = await sendAndConfirmTransaction(connection, tx, [payer, mint]);
  console.log("Transaction sent:", txid);
})

const firstBuy = async () => {


  const tx = new Transaction();


  let amount = new BN(1000000000)

  const instructionData = Buffer.concat([
    Buffer.from(Int8Array.from([TokenSaleInstruction.FirstBuy]).buffer),
    Uint8Array.of(30),
    amount.toBuffer('le', 8),
  ]);

  // Construct your program accounts
  // Note: You must derive and fetch the associated token account as well

  const instruction = {
    keys: [
      { pubkey: mint.publicKey, isSigner: true, isWritable: true },
      { pubkey: payer.publicKey, isSigner: true, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: pda, isSigner: false, isWritable: true },
      { pubkey: ata[0], isSigner: false, isWritable: true },
      { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: deata[0], isSigner: false, isWritable: true },
      // Add associated_token_account and associated_token_program here...
    ],
    programId: PROGRAM_ID,
    data: instructionData,
  };
  // console.log(instruction.keys)

  tx.add(instruction);

  const txid = await sendAndConfirmTransaction(connection, tx, [payer, mint]);
  console.log("Transaction sent:", txid);
}


const Buy = async () => {




  const tx = new Transaction();


  let amount = new BN(100000)

  const instructionData = Buffer.concat([
    Buffer.from(Int8Array.from([TokenSaleInstruction.Buy]).buffer),
    amount.toBuffer('le', 8),
  ]);

  // Construct your program accounts
  // Note: You must derive and fetch the associated token account as well

  const instruction = {
    keys: [
      { pubkey: mint.publicKey, isSigner: true, isWritable: true },
      { pubkey: payer.publicKey, isSigner: true, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: pda, isSigner: false, isWritable: true },
      { pubkey: ata[0], isSigner: false, isWritable: true },
      { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: deata[0], isSigner: false, isWritable: true },
      // Add associated_token_account and associated_token_program here...
    ],
    programId: PROGRAM_ID,
    data: instructionData,
  };
  // console.log(instruction.keys)

  tx.add(instruction);

  const txid = await sendAndConfirmTransaction(connection, tx, [payer, mint]);
  console.log("Transaction sent:", txid);
}


const fun = async () => {

  await create();
  await firstBuy();
  await Buy();

}

fun()





  //Token mint created successfully. pda EHDU6rgmBLUkNqC1XRrDKdw5rZwPAxL9HrJavuj1GJoK, ata 9m7XtCLXJYPkrLkzdSJHtvN3JSw555D3RSrZ5q3KLBJm, mint 4k61hX4AgCvoJZ4vnjJAK1uU1SVjDcBLJEz7Mxd5Hex1, payer 4iQk46qU2i1pfbrc7QzPbbD47nZn88mSBEhtZuuWKPRG"
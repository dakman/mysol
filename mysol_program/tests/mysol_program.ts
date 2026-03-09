import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MysolProgram } from "../target/types/mysol_program";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo } from "@solana/spl-token";

describe("mysol_program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.mysolProgram as Program<MysolProgram>;
  const provider = anchor.AnchorProvider.env();
  const user = provider.wallet as anchor.Wallet;

  let usdcMint: PublicKey;
  let vaultPda: PublicKey;
  let vaultTokenAccount: PublicKey;

  before(async () => {
    // Create USDC mint
    usdcMint = await createMint(
      provider.connection,
      user.payer,
      user.publicKey,
      null,
      6 // decimals
    );

    // Derive vault PDA
    [vaultPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), user.publicKey.toBuffer(), Buffer.from("v2")],
      program.programId
    );

    // Derive vault token account (ATA)
    vaultTokenAccount = await anchor.utils.token.associatedAddress({
      mint: usdcMint,
      owner: vaultPda,
    });
  });

  it("Initializes a vault", async () => {
    const dailyLimitSol = new anchor.BN(1_000_000_000); // 1 SOL in lamports
    const dailyLimitUsdc = new anchor.BN(100_000_000); // 100 USDC in micro units
    const enforceDays = new anchor.BN(30);

    const tx = await program.methods
      .initializeVault(dailyLimitSol, dailyLimitUsdc, enforceDays)
      .accounts({
        vault: vaultPda,
        usdcMint,
        vaultTokenAccount,
        user: user.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Initialize vault tx:", tx);

    // Fetch the vault account
    const vaultAccount = await program.account.vaultState.fetch(vaultPda);
    console.log("Vault state:", vaultAccount);
  });

  it("Withdraws SOL within limit", async () => {
    // Fund the vault with 2 SOL
    const fundTx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: user.publicKey,
        toPubkey: vaultPda,
        lamports: 2_000_000_000, // 2 SOL
      })
    );
    await provider.sendAndConfirm(fundTx);

    const amount = new anchor.BN(500_000_000); // 0.5 SOL

    const tx = await program.methods
      .withdrawSol(amount)
      .accounts({
        vault: vaultPda,
        user: user.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Withdraw SOL tx:", tx);
  });

  it("Withdraws USDC within limit", async () => {
    // First, mint some USDC to the vault
    const userTokenAccount = await createAccount(
      provider.connection,
      user.payer,
      usdcMint,
      user.publicKey
    );

    await mintTo(
      provider.connection,
      user.payer,
      usdcMint,
      vaultTokenAccount,
      user.publicKey,
      200_000_000 // 200 USDC
    );

    const amount = new anchor.BN(50_000_000); // 50 USDC

    const tx = await program.methods
      .withdrawUsdc(amount)
      .accounts({
        vault: vaultPda,
        usdcMint,
        vaultTokenAccount,
        userTokenAccount,
        user: user.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("Withdraw USDC tx:", tx);
  });
});

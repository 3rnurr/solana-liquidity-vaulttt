import { web3 } from "@coral-xyz/anchor";

const ASSET_MINT_ADDRESS = "7hwS5d9HwgEXQyEXiLBH8d897Mjas3YrQpDUyR62CrQ6";

async function main() {
  const assetMint = new web3.PublicKey(ASSET_MINT_ADDRESS);
  const authority = pg.wallet.publicKey;
  const programId = pg.program.programId;

  const [vaultConfigPda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault_config"), assetMint.toBuffer()],
    programId
  );

  const [shareMintPda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("share_mint"), vaultConfigPda.toBuffer()],
    programId
  );

  const [vaultTokenPda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault_token"), vaultConfigPda.toBuffer()],
    programId
  );

  const [userPositionPda] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("user_pos"),
      vaultConfigPda.toBuffer(),
      authority.toBuffer(),
    ],
    programId
  );

  console.log("Program ID:", programId.toBase58());
  console.log("Vault Config PDA:", vaultConfigPda.toBase58());
  console.log("Share Mint PDA:", shareMintPda.toBase58());
  console.log("Vault Token PDA:", vaultTokenPda.toBase58());
  console.log("User Position PDA:", userPositionPda.toBase58());

  try {
    const tx1 = await pg.program.methods
      .initializeVault()
      .accounts({
        vaultConfig: vaultConfigPda,
        assetMint: assetMint,
        shareMint: shareMintPda,
        vaultTokenAccount: vaultTokenPda,
        authority: authority,
        tokenProgram: web3.TOKEN_PROGRAM_ID,
        systemProgram: web3.SystemProgram.programId,
        rent: web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc();
    console.log("initializeVault TX:", tx1);
  } catch {
    console.log("initializeVault уже выполнен ранее");
  }

  try {
    const tx2 = await pg.program.methods
      .initUserPosition()
      .accounts({
        vaultConfig: vaultConfigPda,
        userPosition: userPositionPda,
        user: authority,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();
    console.log("initUserPosition TX:", tx2);
  } catch {
    console.log("initUserPosition уже выполнен ранее");
  }

  const configData = await pg.program.account.vaultConfig.fetch(
    vaultConfigPda,
    "confirmed"
  );
  const posData = await pg.program.account.userPosition.fetch(
    userPositionPda,
    "confirmed"
  );

  console.log("--- Состояние в блокчейне ---");
  console.log("VaultConfig authority:", configData.authority.toBase58());
  console.log("VaultConfig total_assets:", configData.totalAssets.toString());
  console.log("UserPosition deposited_amount:", posData.depositedAmount.toString());
}

main();
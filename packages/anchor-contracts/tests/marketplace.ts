import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaSpeedstersMarketplace } from "../target/types/solana_speedsters_marketplace";
import { expect } from "chai";
import { createMint, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";

describe("solana-speedsters-marketplace", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaSpeedstersMarketplace as Program<SolanaSpeedstersMarketplace>;

  let nftMint: anchor.web3.PublicKey;
  let sellerTokenAccount: anchor.web3.PublicKey;
  let buyerTokenAccount: anchor.web3.PublicKey;
  const seller = anchor.web3.Keypair.generate();
  const buyer = anchor.web3.Keypair.generate();
  const marketplaceAuthority = anchor.web3.Keypair.generate();
  let marketplaceStatePda: anchor.web3.PublicKey;
  let marketplaceBump: number;

  before(async () => {
    // Airdrop SOL
    await provider.connection.requestAirdrop(seller.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(buyer.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(marketplaceAuthority.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);

    // Create NFT
    nftMint = await createMint(provider.connection, seller, seller.publicKey, null, 0);

    const sellerAta = await getOrCreateAssociatedTokenAccount(provider.connection, seller, nftMint, seller.publicKey);
    sellerTokenAccount = sellerAta.address;
    await mintTo(provider.connection, seller, nftMint, sellerTokenAccount, seller, 1);

    const buyerAta = await getOrCreateAssociatedTokenAccount(provider.connection, buyer, nftMint, buyer.publicKey);
    buyerTokenAccount = buyerAta.address;

    // Find PDA for marketplace state
    [marketplaceStatePda, marketplaceBump] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("marketplace")],
        program.programId
    );
  });

  it("Should initialize the marketplace", async () => {
    const fee = 200; // 2%
    await program.methods
        .initializeMarketplace(fee)
        .accounts({
            marketplaceState: marketplaceStatePda,
            authority: marketplaceAuthority.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([marketplaceAuthority])
        .rpc();

    const state = await program.account.marketplaceState.fetch(marketplaceStatePda);
    expect(state.authority.toBase58()).to.equal(marketplaceAuthority.publicKey.toBase58());
    expect(state.fee).to.equal(fee);
  });

  it("Should list an NFT for sale", async () => {
    const price = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL);

    const [listing] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("listing"), nftMint.toBuffer()],
      program.programId
    );
    const [escrow] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), nftMint.toBuffer()],
      program.programId
    );

    await program.methods
      .listNft(price)
      .accounts({
        seller: seller.publicKey,
        sellerTokenAccount: sellerTokenAccount,
        nftMint: nftMint,
        escrowTokenAccount: escrow,
        listing: listing,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([seller])
      .rpc();

    const listingAccount = await program.account.listing.fetch(listing);
    expect(listingAccount.seller.toBase58()).to.equal(seller.publicKey.toBase58());
    expect(listingAccount.price.toNumber()).to.equal(price.toNumber());
    expect(listingAccount.isListed).to.be.true;
  });

  it("Should buy the listed NFT", async () => {
    const price = 1 * anchor.web3.LAMPORTS_PER_SOL;
    const fee = 0.02 * price; // 2% fee
    const expectedSellerReceive = price - fee;

    const [listing] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("listing"), nftMint.toBuffer()],
      program.programId
    );
    const [escrow] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), nftMint.toBuffer()],
      program.programId
    );

    const initialSellerBalance = await provider.connection.getBalance(seller.publicKey);
    const initialAuthorityBalance = await provider.connection.getBalance(marketplaceAuthority.publicKey);

    await program.methods
      .buyNft()
      .accounts({
        buyer: buyer.publicKey,
        buyerTokenAccount: buyerTokenAccount,
        seller: seller.publicKey,
        escrowTokenAccount: escrow,
        nftMint: nftMint,
        listing: listing,
        marketplaceState: marketplaceStatePda,
        marketplaceAuthority: marketplaceAuthority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .signers([buyer])
      .rpc();

    // Check balances
    const finalSellerBalance = await provider.connection.getBalance(seller.publicKey);
    const finalAuthorityBalance = await provider.connection.getBalance(marketplaceAuthority.publicKey);

    // Check NFT ownership
    const buyerAta = await provider.connection.getTokenAccountBalance(buyerTokenAccount);
    expect(buyerAta.value.uiAmount).to.equal(1);

    // Check SOL transfer (approximate due to gas fees)
    expect(finalSellerBalance).to.be.closeTo(initialSellerBalance + expectedSellerReceive, 100000);
    expect(finalAuthorityBalance).to.be.closeTo(initialAuthorityBalance + fee, 100000);
  });
});

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftStudy } from "../target/types/nft_study";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SOLANA_SCHEMA,
  Signer,
  SystemProgram,
} from "@solana/web3.js";
import {
  getAssociatedTokenAddress,
  createMint,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAccount,
  getOrCreateAssociatedTokenAccount,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import {
  findMasterEditionPda,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { walletAdapterIdentity } from "@metaplex-foundation/umi-signer-wallet-adapters";
import { publicKey } from "@metaplex-foundation/umi";

describe("nft-study", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.NftStudy as Program<NftStudy>;
  const METADATA_SEED = "metadata";
  const EDITION_SEED = "edition";
  
  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const signer = provider.wallet;

  const umi = createUmi("https://api.devnet.solana.com")
    .use(walletAdapterIdentity(signer))
    .use(mplTokenMetadata());





  // it("Create NFT", async () => {
  //   const MINT_SEED = "MINT_NFT";
  //   let idNft = new anchor.BN(5);

  //   const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [Buffer.from(MINT_SEED), idNft.toBuffer("le", 8)],
  //     program.programId
  //   );
  //   const tokenNftAccount = getAssociatedTokenAddressSync(
  //     mint,
  //     provider.wallet.publicKey,
  //     true
  //   );
   
  //   const metadataAddress = getMetaAddressAccount(mint);
  //   const masterEditionAccount = getMasterEditionAccount(mint);
   

  //   const name = "TEST";
  //   const symbol = "SYMBOL";
  //   const uri =
  //     "https://bafybeia6fwodprjimdag6vlbguq4tcwrewpnux3s3x3dyzgpuragljahu4.ipfs.nftstorage.link/941.json";
  //   try {
  //     const tx = await program.methods
  //       .createNft(name, symbol, uri, idNft)
  //       .accounts({
  //         mint: mint,
  //         nftMetadata: metadataAddress,
  //         masterEditionAccount: masterEditionAccount,
  //         tokenAccount: tokenNftAccount,
  //         authority: provider.wallet.publicKey,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //         tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
  //         metadataProgram: TOKEN_METADATA_PROGRAM_ID,
  //         rent: anchor.web3.SYSVAR_RENT_PUBKEY,
  //       })
  //       .rpc();
  //     console.log("Your transaction signature", tx);
  //     console.log(
  //       `minted nft: https://explorer.solana.com/address/${mint}?cluster=devnet`
  //     );
  //   } catch (error) {
  //     console.log(error);
  //   }
  // });


  it("Create COllections NFT", async () => {
    const name_coll = "TEST_COLLECTION_2";
    const symbol = "";

    const uri ="https://arweave.net/bJ0Ini3fCPMquAAl5ZyJWCroQh5oq8FjkkLeWwoq1YM";
    
  
    
    const mint = getMintCollectionAccount(name_coll);
    const tokenNftAccount = getAssociatedTokenAddressSync( mint,provider.wallet.publicKey, true );

    const metadataAddress = getMetaAddressAccount(mint);
    const masterEditionAccount = getMasterEditionAccount(mint);

    try {
      const tx = await program.methods
        .createCollectionNft(name_coll, symbol, uri)
        .accounts({
          mint: mint,
          nftMetadata: metadataAddress,
          masterEditionAccount: masterEditionAccount,
          tokenAccount: tokenNftAccount,
          authority: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
          metadataProgram: TOKEN_METADATA_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .rpc();
      console.log("Your transaction signature", tx);
      console.log(
        `minted nft: https://explorer.solana.com/address/${mint}?cluster=devnet`
      );
    } catch (error) {
      console.log(error);
    }
  });

  const getMetaAddressAccount =  (mint: PublicKey) => {
    const [metadataAddress] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(METADATA_SEED),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );
    return metadataAddress;
  }
  const getMasterEditionAccount  = (mint: PublicKey) => {
    const [masterEditionAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(METADATA_SEED),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
        Buffer.from(EDITION_SEED),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );
    return masterEditionAccount;
  }
  const getMintCollectionAccount = (name: string) => {
    const MINT_SEED = "MINT_COLLECTION";
    const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(MINT_SEED),
        anchor.utils.bytes.utf8.encode(name),
      ],
      program.programId
    );
    return mint;
  }
 
  
});

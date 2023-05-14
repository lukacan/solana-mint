import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaMint } from "../target/types/solana_mint";
import * as token from '@solana/spl-token';
import { PublicKey } from '@solana/web3.js';
import { SystemProgram } from '@solana/web3.js';
import { Metaplex } from "@metaplex-foundation/js";
import { assert } from "chai";


const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);


describe("solana-mint", async () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);


  const program = anchor.workspace.SolanaMint as Program<SolanaMint>;
  const nft_creator = anchor.web3.Keypair.generate();

  let mint: PublicKey;
  let token_account: PublicKey;
  let metadata_account: PublicKey;
  let metadata_account_bump: number;

  let master_edition_account: PublicKey;
  let master_edition_account_bump: number;



  before("Prepare", async function () {
    await airdrop(provider.connection, nft_creator.publicKey);

    mint = await token.createMint(
      provider.connection,
      nft_creator,
      nft_creator.publicKey,
      nft_creator.publicKey,
      0
    );

    token_account = await token.createAccount(provider.connection, nft_creator, mint, nft_creator.publicKey);
    await token.mintTo(provider.connection, nft_creator, mint, token_account, nft_creator, 1);
    [metadata_account, metadata_account_bump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    [master_edition_account, master_edition_account_bump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
        Buffer.from("edition"),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );


  })

  it("Create NFT!", async () => {
    await program.methods.createNft().accounts({
      nftCreator: nft_creator.publicKey,
      mint: mint,
      tokenAccount: token_account,
      metadataAccount: metadata_account,
      masterEditionAccount: master_edition_account,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      associatedTokenProgram: token.ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: token.TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
      .signers([nft_creator]).rpc();
  });

  it("Test NFT", async () => {
    //const connection = new Connection("http://127.0.0.1:8899", "confirmed");
    const metaplex = Metaplex.make(provider.connection);

    const nft_after = await metaplex.nfts().findByMint({ mintAddress: mint });
    //console.log(nft_after)
    assert.strictEqual(nft_after.updateAuthorityAddress.toString(), nft_creator.publicKey.toString());
    assert.strictEqual(nft_after.mint.address.toString(), mint.toString());
    assert.strictEqual(nft_after.name, "Andrej");
    assert.strictEqual(nft_after.symbol, "Symbol");
    assert.strictEqual(nft_after.uri, "Uri");
    assert.strictEqual(nft_after.isMutable, true);
    assert.strictEqual(nft_after.primarySaleHappened, false);
    assert.strictEqual(nft_after.sellerFeeBasisPoints, 1);
    assert.strictEqual(nft_after.creators[0].address.toString(), nft_creator.publicKey.toString());
    assert.strictEqual(nft_after.creators[0].verified, true);
    assert.strictEqual(nft_after.creators[0].share, 100);
    assert.strictEqual(nft_after.address.toString(), mint.toString());
    assert.strictEqual(nft_after.mint.mintAuthorityAddress.toString(), master_edition_account.toString());
    assert.strictEqual(nft_after.mint.freezeAuthorityAddress.toString(), master_edition_account.toString());
    assert.strictEqual(nft_after.edition.isOriginal, true);
    assert.strictEqual(nft_after.edition.supply.toString(), new anchor.BN(0).toString());
    assert.strictEqual(nft_after.edition.maxSupply.toString(), new anchor.BN(10).toString());

  });
});


export async function airdrop(connection: any, address: any, amount = 2000000000) {
  await connection.confirmTransaction(await connection.requestAirdrop(address, amount), "confirmed");
}

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Project01Favorites } from "../target/types/project_01_favorites";
// import { Favorites } from "../target/types/favorites";

import { assert } from 'chai';

describe('set_favorites', () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.Project01Favorites as Program<Project01Favorites>;
    // const program = anchor.workspace.Favorites as Program<Favorites>;

    it('sets the favorites correctly', async () => {

        // Define the favorite data
        const number = new anchor.BN(42);
        const color = "blue";
        const hobbies = ["reading", "coding", "gaming"];

        // Derive the PDA for the favorites account
        const [favoritesPda, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("favorites"), provider.wallet.publicKey.toBuffer()],
            program.programId
        );

        // set_favorites instruction accounts
        let setFavoritesAccounts = {
            user: provider.wallet.publicKey,
            favorites: favoritesPda,
            systemProgram: anchor.web3.SystemProgram.programId,
        }

        // Call the set_favorites function
        await program.methods.setFavorites(number, color, hobbies)
            .accounts(setFavoritesAccounts)
            .rpc();

        // Fetch the account and verify the data
        const account = await program.account.favorites.fetch(favoritesPda);
        console.log(account);
        assert.equal(account.number.toNumber(), 42);
        assert.equal(account.color, "blue");
        assert.deepEqual(account.hobbies, ["reading", "coding", "gaming"]);
    });
});

use anchor_lang::prelude::*;

declare_id!("HXGTWxrBm1b2QB75xUGp6GRsvGrAtiAK72vXeYznP7uf");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod project_01_favorites {
    use super::*;

    pub fn set_favorites(
        ctx: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("Greetings from {}!", ctx.program_id);

        let user_pubic_key = &ctx.accounts.user.key();
        msg!(
            "User {user_pubic_key}s favorite number is {number}, favorite color is: {color}",
            user_pubic_key = user_pubic_key.to_string(),
            number = number,
            color = color
        );
        msg!(
            "User's hobbies are: {:?}",
            hobbies
        );

        // Set the data in the account
        ctx.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }

}

// Data model
#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

// Context for set_favorites
#[derive(Accounts)]
pub struct SetFavorites<'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}
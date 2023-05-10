use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");


#[program]
pub mod sol_story {

    use super::*;

    

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let story_account = &mut ctx.accounts.story;
        story_account.content = "".to_string();
        Ok(())
    }

    pub fn write_into_story(
        ctx: Context<WriteStory>,
        three_words: String
    )->Result<()>{
        let story = &mut ctx.accounts.story;  
        let split_iterator = three_words.trim().split(" ");
        let mut final_words = Vec::new();
        let mut counter_added = 0;
        for s in split_iterator {
            if s.trim().is_empty() {
                continue;
            }
            if s.trim().len() >= 15 {
                return err!(Errors::WordTooLarge);
            }
            final_words.push(s);
            counter_added += 1;
            if counter_added >= 5 {
                break;
            }
        }
        // Join the 3 words after removing spaces
        let mut joined_words = final_words.join(" ");
        // Add a space at the end with this
        joined_words.push_str(" ");
        // Article content gets immediately updated
        story.content.push_str(&joined_words);

        Ok(())
    }
}


#[account]
pub struct Story {
    pub content: String,
}

#[derive(Accounts)]
pub struct WriteStory<'info> {
    #[account(mut)]
    pub story: Account<'info, Story>,
}


#[derive(Accounts)]  
pub struct Initialize<'info> {  
    #[account(  
        init,  
        payer = person_that_pays,  
        space = 8 // account discriminator  
         + 32 // pubkey  
        + 10000 // make the message max 10k bytes long  
    )]  
    pub story: Account<'info, Story>,  
    #[account(mut)]  
    pub person_that_pays: Signer<'info>,  
    pub system_program: Program<'info, System>,  
}

#[error_code]
pub enum Errors {
    #[msg("word too long")]
    WordTooLarge
}

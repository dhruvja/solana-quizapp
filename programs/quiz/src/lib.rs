use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod quiz {
    use super::*;

    fn print_type_of<T>(_: T) {
        println!("{}", std::any::type_name::<T>())
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let quiz = &mut ctx.accounts.quiz;
        let user = &mut ctx.accounts.user;
        quiz.total_quiz = 0;

        Ok(())
    }

    pub fn add_quiz(ctx: Context<AddQuiz>,quiz_name: String, questions: Vec<Question>) -> Result<()> {
        let quiz = &mut ctx.accounts.quiz;
        let user = &mut ctx.accounts.user;

        let mut vec = Vec::new();

        let item = QuizStruct {
            quiz_name: quiz_name.to_string(),
            host: *user.to_account_info().key,
            questions: questions,
            scores: vec
        };

        quiz.quiz_list.push(item);
        quiz.total_quiz += 1;

        Ok(())
    }

    pub fn calculate_score(ctx: Context<AddQuiz>, index: u8, chosen_options: Vec<u8>) -> Result<()> {

        let quiz = &mut ctx.accounts.quiz;

        let id: usize = index as usize;

        let mut score: u16 = 0;

        let length: usize = quiz.quiz_list[id].questions.len();

        for i in 0..length {
            let x: usize = i as usize;
            if quiz.quiz_list[id].questions[x].right_option == chosen_options[x] {
                score += 10; 
            }
        }

        quiz.quiz_list[id].scores.push(score);

        Ok(())
    }


}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub quiz: Account<'info, Quiz>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddQuiz<'info> {
    #[account(mut)]
    pub quiz: Account<'info, Quiz>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Question {
    pub name: String,
    pub right_option: u8,
    pub options: Vec<String>
}


#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct QuizStruct{
    pub quiz_name: String,
    pub host: Pubkey,
    pub questions: Vec<Question> ,
    pub scores: Vec<u16>
}


#[account]
pub struct Quiz{
    pub total_quiz: u64,
    pub quiz_list: Vec<QuizStruct>
}

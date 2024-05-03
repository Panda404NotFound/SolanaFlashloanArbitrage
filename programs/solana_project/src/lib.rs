use mango::state::Mango;
use mango::client::MangoClient;
use mango::instruction::flash_loan::{FlashLoanBegin, FlashLoanEnd};
use solana_program::system_instruction;

pub fn flash_loan(ctx: Context<FlashLoan>, amount: u64) -> Result<()> {
    // Получаем начальный баланс
    let initial_balance = ctx.accounts.user_account.lamports();

    // Начало Flash Loan
    let flash_loan_begin = FlashLoanBegin {
        mango_group: ctx.accounts.mango_group.key(),
        mango_account: ctx.accounts.mango_account.key(),
        token_account: ctx.accounts.token_account.key(),
        vault: ctx.accounts.vault.key(),
        amount,
    };

    let mango_client = MangoClient::new(ctx.program_id);
    let transaction_id = mango_client.flash_loan_begin(flash_loan_begin)?;

    // Проверяем баланс после начала FlashLoan
    let balance_after_begin = ctx.accounts.user_account.lamports();
    msg!("Баланс после начала FlashLoan: {}", balance_after_begin);
    msg!("ID транзакции: {}", transaction_id);

    // Ваша бизнес-логика с заемными средствами
    // ...

    // Окончание Flash Loan  
    let flash_loan_end = FlashLoanEnd {
        mango_group: ctx.accounts.mango_group.key(),
        mango_account: ctx.accounts.mango_account.key(),
        token_account: ctx.accounts.token_account.key(),
        vault: ctx.accounts.vault.key(),
    };
    
    mango_client.flash_loan_end(flash_loan_end)?;

    // Проверяем баланс после окончания FlashLoan
    let final_balance = ctx.accounts.user_account.lamports();
    msg!("Баланс после окончания FlashLoan: {}", final_balance);

    Ok(())
}


/*
На основе изученной информации из репозитория Mango V4 и документации, вот как вы можете получить и идентифицировать "mango account" и "vault account":

1. Получение "mango account":

Для получения "mango account" вам необходимо создать его, связав с определенной "mango group". Вы можете сделать это программно, используя SDK Mango V4:

use mango::client::MangoClient;

let mangoGroup = client.get_mango_group(mango_group_key); // Получаем mango group
let mangoAccount = client.create_mango_account(mangoGroup, wallet, desiered_account_number); // Создаем mango account

После создания, "mango account" будет иметь публичный ключ, по которому вы сможете его идентифицировать.

2. Получение "vault account": 

"Vault account" - это аккаунт, контролируемый программой Mango, в котором хранятся заемные токены для использования в операциях FlashLoan. Его адрес зависит от конкретной "mango group", с которой вы работаете.

Вы можете получить адрес "vault account" из объекта "mango group":

let vaultAccount = mangoGroup.vaults[tokenIndex]; // tokenIndex - индекс токена в группе

Или, если вы знаете публичный ключ "mango group", вы можете вычислить адрес "vault account" с помощью программы Mango:

let vaultAccount = get_vault_account(&mangoGroup.pubkey, tokenIndex);

Таким образом, для идентификации "mango account" вам нужно создать его и получить публичный ключ, а для "vault account" - получить его адрес из объекта "mango group" или вычислить по публичному ключу группы и индексу токена.[2][3][8]

*/
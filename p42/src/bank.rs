use std::collections::HashMap;
pub enum Error {
    SenderNotFound,
    ReceiverNotFound,
    AttemptedAmountTooLarge,
    SenderOverflow,
    ReceiverOverflow,
    SenderCreditLimitCrossed,
    SenderCreditLimitTooLarge,
}
pub struct User {
    credit_line: u64,
    balance: i64,
}
pub struct Bank {
    users: HashMap<String, User>,
    name: String,
    credit_interest: u64,
    debit_interest: u64,
}

impl Bank {
    pub fn calc_balance(&self) -> (u64, u64) {
        let mut liab: u64 = 0;
        let mut ass: u64 = 0;
        for (_key, value) in self.users.iter() {
            if value.balance < 0 {
                ass += value.balance.unsigned_abs();
            } else {
                liab += value.balance.unsigned_abs();
            }
        }
        (liab, ass)
    }

    pub fn accrue_interest(&mut self) {
        for (_key, value) in self.users.iter_mut() {
            if value.balance > 0 {
                value.balance +=
                    i64::try_from((value.balance.unsigned_abs() * self.debit_interest) / 10000)
                        .unwrap();
            } else {
                value.balance -=
                    i64::try_from((value.balance.unsigned_abs() * self.credit_interest) / 10000)
                        .unwrap();
            }
        }
    }

    pub fn merge_bank(&mut self, new: Bank) {
        if self.name == new.name {
            return;
        }
        for (key_i, value_i) in new.users {
            match self.users.get_mut(&key_i) {
                Some(v) => {
                    v.balance += value_i.balance;
                }
                None => {
                    self.users.insert(key_i, value_i);
                }
            };
        }
    }

    pub fn transfer_funds(
        &mut self,
        user_send: String,
        user_rec: String,
        amount: u64,
    ) -> Result<(), Error> {
        let sender = self.users.get(&user_send).ok_or(Error::SenderNotFound)?;

        let sender_credit_line_i64 =
            i64::try_from(sender.credit_line).map_err(|_| Error::SenderCreditLimitTooLarge)?;

        let amount_i64 = i64::try_from(amount).map_err(|_| Error::AttemptedAmountTooLarge)?;

        let sender_final_balance = sender
            .balance
            .checked_sub(amount_i64)
            .ok_or(Error::SenderOverflow)?;

        if sender_final_balance < -sender_credit_line_i64 {
            return Err(Error::SenderCreditLimitCrossed);
        }

        let receiver = self
            .users
            .get_mut(&user_rec)
            .ok_or(Error::ReceiverNotFound)?;

        let receiver_final_balance = receiver
            .balance
            .checked_add(amount_i64)
            .ok_or(Error::ReceiverOverflow)?;

        self.users.get_mut(&user_send).unwrap().balance = sender_final_balance;
        self.users.get_mut(&user_rec).unwrap().balance = receiver_final_balance;

        Ok(())
    }
}

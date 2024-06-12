pub enum Error {
    AmountTooLarge,
    SenderNotFound,
    ReceiverNotFound,
    CreditLimitOverflow,
    BadCreditLimit,
}
pub struct User {
    name: String,
    credit_line: u64,
    balance: i64,
}
pub struct Bank {
    users: Vec<User>,
    name: String,
    credit_interest: u64,
    debit_interest: u64,
}

impl Bank {
    pub fn calc_balance(&self) -> (u64, u64) {
        let mut liab: u64 = 0;
        let mut ass: u64 = 0;
        for i in self.users.iter() {
            if i.balance < 0 {
                ass += i.balance.unsigned_abs();
            } else {
                liab += i.balance.unsigned_abs();
            }
        }
        (liab, ass)
    }

    pub fn transfer_funds(
        &mut self,
        user1: String,
        user2: String,
        amount: u64,
    ) -> Result<(), Error> {
        let mut user1_index = usize::MAX;
        let mut user2_index = self.users.len();
        let amount_i64 = i64::try_from(amount).map_err(|_| Error::AmountTooLarge)?;
        for (i, user) in self.users.iter().enumerate() {
            if user.name == user1 {
                user1_index = i;
                if user2_index != self.users.len() {
                    break;
                }
            }
            if user.name == user2 {
                user2_index = i;
                if user1_index != usize::MAX {
                    break;
                }
            }
        }
        if user1_index == usize::MAX {
            return Err(Error::SenderNotFound);
        }
        if user2_index == self.users.len() {
            return Err(Error::ReceiverNotFound);
        }
        let user1_balance = self.users[user1_index].balance;
        let user1_credit_i64 = i64::try_from(self.users[user1_index].credit_line)
            .map_err(|_| Error::BadCreditLimit)?;
        if user1_balance - amount_i64 < -user1_credit_i64 {
            return Err(Error::CreditLimitOverflow);
        }
        self.users[user1_index].balance -= amount_i64;
        self.users[user2_index].balance += amount_i64;
        Ok(())
    }

    pub fn accrue_interest(&mut self) {
        for i in self.users.iter_mut() {
            if i.balance > 0 {
                i.balance +=
                    i64::try_from((i.balance.unsigned_abs() * self.debit_interest) / 10000)
                        .unwrap();
            } else {
                i.balance -=
                    i64::try_from((i.balance.unsigned_abs() * self.credit_interest) / 10000)
                        .unwrap();
            }
        }
    }

    pub fn merge_bank(&mut self, new: Bank) {
        if self.name == new.name {
            return;
        }
        'outer_loop: for i in new.users {
            for j in self.users.iter_mut() {
                if i.name == j.name {
                    j.balance += i.balance;
                    continue 'outer_loop;
                }
            }
            self.users.push(i);
        }
    }
}

pub enum Error {
    Err1,
    Err2,
}
pub struct User {
    name: String,
    credit_line: u64,
    balance: i64,
}
pub struct Bank {
    user: Vec<User>,
    name: String,
    credit_interest: u64,
    debit_interest: u64,
}

impl Bank {
    pub fn calc_balance(&self) -> (u64, u64) {
        let mut liab: u64 = 0;
        let mut ass: u64 = 0;
        for i in self.user.iter() {
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
    ) -> Result<bool, Error> {
        let mut user1_index = usize::MAX;
        let mut user2_index = self.user.len();

        for (i, user) in self.user.iter().enumerate() {
            if user.name == user1 {
                user1_index = i;
                if user2_index != self.user.len() {
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
            return Err(Error::Err1);
        }
        if user2_index == self.user.len() {
            return Err(Error::Err1);
        }
        if (self.user[user1_index].balance - i64::try_from(amount).unwrap()).unsigned_abs()
            > self.user[user1_index].credit_line
        {
            return Err(Error::Err2);
        }

        self.user[user1_index].balance -= i64::try_from(amount).unwrap();
        self.user[user2_index].balance += i64::try_from(amount).unwrap();
        Ok(true)
    }

    pub fn accrue_interest(&mut self) {
        for i in self.user.iter_mut() {
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
        'outer_loop: for i in new.user {
            for j in self.user.iter_mut() {
                if i.name == j.name {
                    j.balance += i.balance;
                    continue 'outer_loop;
                }
            }
            self.user.push(i);
        }
    }
}

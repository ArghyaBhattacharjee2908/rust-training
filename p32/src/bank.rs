#[derive(Clone)]
pub struct User {
    pub name: String,
    pub credit_line: u64,
    pub balance: i64,
}
impl Default for User {
    fn default() -> Self {
        User {
            name: "".to_string(),
            credit_line: 0,
            balance: 0,
        }
    }
}
pub struct Bank {
    pub user: Vec<User>,
    pub name: String,
    pub credit_interest: u64,
    pub debit_interest: u64,
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
    pub fn transfer_funds(&mut self, user1: String, user2: String, amount: u64) {
        let mut temp1 = User::default();
        let mut temp2 = User::default();
        let mut flag1: bool = false;
        let mut flag2: bool = false;
        for i in self.user.iter() {
            if i.name == user1 {
                temp1.name.push_str(i.name.as_str());
                temp1.credit_line = i.credit_line;
                temp1.balance = i.balance;
                flag1 = true;
            }
        }
        for i in self.user.iter() {
            if i.name == user2 {
                temp2.name.push_str(i.name.as_str());
                temp2.credit_line = i.credit_line;
                temp2.balance = i.balance;
                flag2 = true;
            }
        }
        if !flag1 || !flag2 {
            println!("Error: (One or More) Invalid User Names.");
            return;
        }
        if temp1.balance < 0 {
            if temp1.balance.unsigned_abs() + amount > temp1.credit_line {
                println!("Error: Invalid Credit Limit.");
                return;
            }
        } else if temp1.balance.unsigned_abs() < amount
            && amount - temp1.balance.unsigned_abs() > temp1.credit_line
        {
            println!("Error: Invalid Credit Limit.");
            return;
        }
        temp1.balance -= i64::try_from(amount).unwrap();
        temp2.balance += i64::try_from(amount).unwrap();

        for i in self.user.iter_mut() {
            if i.name == user1 {
                i.balance = temp1.balance;
            }
        }
        for i in self.user.iter_mut() {
            if i.name == user2 {
                i.balance = temp2.balance;
            }
        }
    }

    pub fn accrue_interest(&mut self) {
        for i in self.user.iter_mut() {
            if i.balance > 0 {
                i.balance = i.balance
                    + i64::try_from((i.balance.unsigned_abs() * self.debit_interest) / 10000)
                        .unwrap();
            } else {
                i.balance = i.balance
                    - i64::try_from((i.balance.unsigned_abs() * self.credit_interest) / 10000)
                        .unwrap();
            }
        }
    }

    pub fn merge_bank(&mut self, new: Bank) {
        'outer_loop: for i in new.user.iter() {
            for j in self.user.iter_mut() {
                if i.name == j.name {
                    j.balance += i.balance;
                    continue 'outer_loop;
                }
            }
            self.user.push(i.clone());
        }
    }
}

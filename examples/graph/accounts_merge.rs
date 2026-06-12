// Given a list of accounts where each element accounts[i] is a list 
// of strings, where the first element accounts[i][0] is a name, and 
// the rest of the elements are emails representing emails of the account.

// Now, we would like to merge these accounts. Two accounts definitely belong 
// to the same person if there is some common email to both accounts. Note 
// that even if two accounts have the same name, they may belong to different 
// people as people could have the same name. A person can have any number of 
// accounts initially, but all of their accounts definitely have the same name.

// After merging the accounts, return the accounts in the following format: the 
// first element of each account is the name, and the rest of the elements are 
// emails in sorted order. The accounts themselves can be returned in any order.

use std::collections::HashMap;

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            let root = self.find(self.parent[i]);
            self.parent[i] = root;
        }

        self.parent[i]
    }

    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);

        if ra != rb {
            self.parent[ra] = rb;
        }
    }
}

fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut email_to_id = HashMap::new();
    let mut email_to_name = HashMap::new();
    let mut next_id = 0;

    // Give every email a unique id and store email to name
    for account in &accounts {
        let name = &account[0];

        for email in account.iter().skip(1) {
            email_to_id.entry(email.clone()).or_insert_with(|| {
                let id = next_id;
                next_id += 1;
                id
            });

            email_to_name.insert(email.clone(), name.clone());
        }
    }

    let mut uf = UnionFind::new(next_id);

    // Union emails that appear in the same accounts
    for account in &accounts {
        let first_email = &account[1];
        let first_id = email_to_id[first_email];

        for email in account.iter().skip(2) {
            let id = email_to_id[email];
            uf.union(first_id, id);
        }
    }

    // Group emails by root
    let mut groups: HashMap<usize, Vec<String>> = HashMap::new();

    for (email, &id) in &email_to_id {
        let root = uf.find(id);

        groups
            .entry(root)
            .or_default()
            .push(email.clone());
    }

    // Build answer
    let mut result = Vec::new();
    for (_, mut emails) in groups {
        emails.sort();

        let name = email_to_name[&emails[0]].clone();

        let mut merged_account = vec![name];
        merged_account.extend(emails);

        result.push(merged_account);
    }

    result
}

fn main() {
    let accounts = vec![
        vec!["John", "johnsmith@mail.com", "john_newyork@mail.com"],
        vec!["John", "johnsmith@mail.com", "john00@mail.com"],
        vec!["Mary", "mary@mail.com"],
        vec!["John", "johnnybravo@mail.com"],
    ]
    .into_iter()
    .map(|v| v.into_iter().map(String::from).collect())
    .collect();

    let merged = accounts_merge(accounts);

    for account in merged {
        println!("{:?}", account);
    }
}
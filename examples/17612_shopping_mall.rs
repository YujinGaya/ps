// 17612 shopping mall
use std::io::{self, Read};

#[derive(Clone)]
struct Customer {
    id: i32,
    items: i8,
}

fn customers_finished(k: usize, waiting: Vec<Customer>) -> Vec<i32> {
    let mut waiting = waiting.into_iter().peekable();
    let mut cashiers: Vec<Option<Customer>> = vec![None; k];
    let mut finished = Vec::new();

    while waiting.peek().is_some() {
        for cashier in cashiers.iter_mut().filter(|c| c.is_none()) {
            *cashier = waiting.next();
        }

        for customer in cashiers.iter_mut().filter_map(|c| c.as_mut()) {
            customer.items -= 1;
        }

        for cashier in cashiers.iter_mut().rev() {
            if let Some(customer) = cashier {
                if customer.items == 0 {
                    finished.push(customer.id);
                    *cashier = None;
                }
            }
        }
    }

    let mut customers: Vec<Customer> = cashiers.into_iter().filter_map(|c| c).rev().collect();
    customers.sort_by_key(|c| c.items);

    [finished, customers.iter().map(|c| c.id).collect()].concat()
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let (k, waiting) = {
        let mut s = s.lines();

        let l = s.next().unwrap().split(' ');
        let k = l.skip(1).next().unwrap().parse().unwrap();

        let customers = s
            .map(|l| {
                let mut l = l.split(' ');
                Customer {
                    id: l.next().unwrap().parse().unwrap(),
                    items: l.next().unwrap().parse().unwrap(),
                }
            })
            .collect();
        (k, customers)
    };

    println!(
        "{}",
        customers_finished(k, waiting)
            .iter()
            .enumerate()
            .fold(0i64, |sum, (i, &id)| sum + (i as i64 + 1) * id as i64)
    );
}

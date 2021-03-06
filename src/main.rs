use blockchain::{
    blockchain::BlockChain,
    account::Account,
};

fn main() {
    let mut a0 = Account::new("a", "a", "a");
    let mut a1 = Account::new("b", "b", "b");
    let mut a2 = Account::new("c", "c", "c");
    let mut a3 = Account::new("d", "d", "d");
    let mut a4 = Account::new("e", "e", "e");
    
    a0.add_money(100.0);
    a2.add_money(100.0);
    a4.add_money(100.0);

    let mut blockchain = BlockChain::new(2);
    blockchain.push_transaction(&mut a0, &mut a1, 2.0, "a");
    blockchain.push_transaction(&mut a2, &mut a3, 1.0, "c");

    println!("{} {} {} {} {}", a0, a1, a2, a3, a4);
}

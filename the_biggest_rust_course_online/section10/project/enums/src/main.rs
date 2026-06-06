#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum,
}

#[derive(Debug)]
enum Subscription {
    Free,

    // Tuple here is for price and months
    Basic(f64, u32),
    Premium(Tier),
}

impl Subscription {
    fn summarize(&self) {
        match self {
            Self::Free => println!("Limited Access"),
            Self::Basic(price, months) => {
                println!("Limited Access for $ {price} and {months} months.")
            }
            Self::Premium(tier) => {
                println!("Full Access. Current Tier is {:#?}", tier);
            }
        }
    }
}

fn main() {
    let free = Subscription::Free;
    let basic = Subscription::Basic(19.9, 3);
    let premium = Subscription::Premium(Tier::Platinum);

    println!("free: {:#?}", free);
    println!("basic: {:#?}", basic);
    println!("premium: {:#?}", premium);
    free.summarize();
    basic.summarize();
    premium.summarize();
}

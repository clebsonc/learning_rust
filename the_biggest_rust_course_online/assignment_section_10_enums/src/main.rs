#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum,
}

#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premium(Tier),
}

impl Subscription {
    fn summarize(&self) -> () {
        match self {
            Subscription::Free => println!("Limited access to the site"),
            Subscription::Basic(value, time) => println!(
                "Limited access to the site premium features for {} during {}",
                value, time
            ),
            Subscription::Premium(tier) => {
                println!(
                    "Full access to the sites premiums features with Tier: {:?}",
                    tier
                );
            }
        }
    }
}

fn main() {
    let sub1 = Subscription::Free;
    sub1.summarize();

    let sub2 = Subscription::Basic(14.99, 12);
    sub2.summarize();

    let sub3 = Subscription::Premium(Tier::Platinum);
    sub3.summarize()
}

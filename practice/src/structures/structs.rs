#[derive(Debug)]
struct ColorRegularStruct {
    // TODO: Add the fields that the test `regular_structs` expects.
    // What types should the fields have? What are the minimum and maximum values for RGB colors?
    red: u8,
    blue: u8,
    green: u8,
}
pub fn structs_one() {
    let color_one = ColorRegularStruct {
        red: 0,
        green: 255,
        blue: 0,
    };

    println!("{:#?}", color_one);
    println!("{}", color_one.green);
    println!("{}", color_one.blue);
    println!("{}", color_one.red)
}

#[derive(Debug)]
pub struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn base_order() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

pub fn strtucts_two() -> Order {
    // TODO: Create your own order using the update syntax and template above!
    // let your_order =
    let base = base_order();

    let new_order = Order {
        name: String::from("Hacker in Rust"),
        count: 1,

        ..base
    };

    println!("{:#?}", new_order);
    println!("{}", new_order.name);
    println!("{}", new_order.year);
    println!("{}", new_order.made_by_phone);
    println!("{}", new_order.made_by_mobile);
    println!("{}", new_order.made_by_email);
    println!("{}", new_order.item_number);
    println!("{}", new_order.count);

    new_order
}

pub struct Package {
    pub sender_country: String,
    pub recipient_country: String,
    pub weight_in_grams: u32,
}

impl Package {
    pub fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Self {
        if weight_in_grams < 10 {
            // This isn't how you should handle errors in Rust, but we will
            // learn about error handling later.
            panic!("Can't ship a package with weight below 10 grams");
        }

        Self {
            sender_country,
            recipient_country,
            weight_in_grams,
        }
    }

    // TODO: Add the correct return type to the function signature.
    pub fn is_international(&self) -> bool {
        // TODO: Read the tests that use this method to find out when a package
        // is considered international.

        if self.recipient_country != "usa" {
            return true;
        }

        false
    }

    // TODO: Add the correct return type to the function signature.
    pub fn get_fees(&self, cents_per_gram: u32) -> u32 {
        // TODO: Calculate the package's fees.
        //

        self.weight_in_grams * cents_per_gram
    }
}

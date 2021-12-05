pub struct Pizza{
    pub topping: String,
    pub inches: u8,
}

impl Pizza{
    pub fn peperoni(inches: u8) -> Self{
        Pizza::bake("peperoni", inches)
    }

    pub fn bake(topping: &str, inches: u8) -> Self{
        Pizza{
            topping: String::from(topping),
            inches,
        }
    }
}
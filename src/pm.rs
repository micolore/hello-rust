
/// project management 
/// crate、package、module
///  

mod nation {
   pub mod goeverment{
        fn govern(){}
    }
    mod congress{
        fn legislate(){}
    }
    mod court{
        fn judicial(){
            super::congress::legislate();
        }
    }
}
// crate::nation::goeverment::govern()


mod back_house{

    pub struct Breakfast{
        pub toast:String,
        seasonal_fruit:String,
    }

    impl Breakfast {

        pub fn summer(toast:&str) -> Breakfast{

            Breakfast{
                toast:String::from(toast),
                seasonal_fruit:String::from("peaches"),
            }
        }
    }
}
pub fn eat_at_restauant(){

    let mut meal = back_house::Breakfast::summer("Rye");
    meal.toast=String::from("wheat");
    println!(" i like",meal.toast);

}


use std::collections::HashMap;

use clap::ValueEnum;
use strum::IntoEnumIterator;
use strum_macros::{EnumString, EnumIter, EnumProperty};
use paste::paste;
// use expanders::enum_with_props;

use crate::util::rand_number;


struct Country {
    code: String,
    name: String,
    flag: String,
    population: String,
    latitude: String,
    longitude: String,
    radius: String,
    population_density: String,
    density: String,

}

pub struct USA{}
impl USA {
    pub fn states() -> Vec<String> {
        State::iter()
            .map(|s| s.to_possible_value()
                .unwrap()
                .get_name()
                .to_string()
            ).collect()
    }
}

type AreaCodes<'a> = &'a [usize];


// macro_rules! create_macro {
//     (
//         $name:ident
//     ) => {
//         macro_rules! $name {
//             () => {

//             }
//         }
//     };
// }

// create_macro!(country);
// country!();

// [<$name Props>]

// #[macro_export]
// macro_rules! enum_with_props {(
//     $vis:vis enum $enum_name:ident (
//         $($prop_name:ident: $prop_type:ty,)*
//     ) {
//         $($variant:ident ($($value:expr),*),)*
//     }
// ) => {
//     paste! {
//         #[derive(Default)]
//         $vis struct [<$enum_name Props>] {
//             $($prop_name: $prop_type),*
//         }
        
//         $vis enum $enum_name {
//             $($variant,)*
//         }

//         impl $enum_name {
//             const ENUM_VALUES: HashMap<String, Vec<[<$enum_name Props>]>> = {
//                 let keys = vec![ $($prop_name),* ];
//                 let values_map = HashMap::new();
//                 $( values_map.insert($variant, vec![$($value),*]); )*

//                 let combined = HashMap::new();
//                 values_map.iter().for_each(|(k, v)| {
//                     let mut i = 0;
//                     let mut value = [<$enum_name Props>] {
//                         $($prop_name: v[i]),*
//                     };
//                     i += 1;
//                 });

//                 combined
//             };

//             pub fn props(&self) -> [<$enum_name Props>] {
//                 // match *self {
//                 //     $($enum_name::$variant => [<$enum_name Props>] {
//                 //         $($prop_name: $value,)*
//                 //     }),*
//                 // }
//                 Default::default()
//             }
//         }
//     }
// }}

// static mut V: Vec<String> = vec![];

pub macro enum_with_props (
    $vis:vis $enum_name:ident (
        $($prop_name:ident: $prop_type:ty,)*
    ) {
        $($variant:ident ($($value:expr),*),)*
    }
) {
    paste! {
        #[derive(Default)]
        $vis struct [<$enum_name Props>] {
            $($prop_name: $prop_type),*
        }
        
        $vis enum $enum_name {
            $($variant),*
        }

        impl $enum_name {
            pub fn props(&self) -> [<$enum_name Props>] {
                // match *self {
                //     $($enum_name::$variant => [<$enum_name Props>] {
                //         $($prop_name: $value,)*
                //     }),*
                // }
                Default::default()
            }
        }
    }
}

enum_with_props! {
    pub StateList (
        name: String,
        age: usize,
    ) {
        A("Dog".to_string(), 3),
        B("Cat".to_string(), 7),
        C("Fish".to_string(), 5),
    }
}

// pub enum Animal {
//     A,
//     B,
// }
// impl Animal {
//     const ENUM_VALUES: Vec<String> = {
//         let vec_tuples: [(String, (String, usize)); 2] = [
//             (String::from("Animal_1"), (String::from("Dog"), 3)),
//             (String::from("Animal_2"), (String::from("Cat"), 7)),
//         ];

//         for (k, (animal, count)) in &vec_tuples {

//         }
//         vec_tuples.iter().for_each(|(k, (animal, count))| {
//             println!("Key: {}, Animal: {}, Count: {}", k, animal, count);
//         });

//         let i = 0;
//         vec![]
//     };
//     // pub const fn is_a(&self) -> String {
//     //     "".to_string()
//     // }
//     pub fn props(&self) -> AnimalProps {
//         match *self {
//             Animal::A => AnimalProps { name: "Dog".to_string(), age: 3 },
//             Animal::B => AnimalProps { name: "Cat".to_string(), age: 7 },
//         }
//     }
// }

// pub struct AnimalProps {
//     name: String,
//     age: usize,
// }




// #[derive(EnumString, EnumIter, ValueEnum, EnumProperty, Clone, Debug, PartialEq)]
// pub enum ValidCountry {
//     Afghanistan  93
//     Albania  355
//     Algeria  213
//     Andorra  376
//     Angola  244
//     AntiguaAndBarbuda  1268
//     Argentina  54
//     Armenia  374
//     Australia  61
//     Austria  43
//     Azerbaijan  994
//     Bahamas  1242
//     Bahrain  973
//     Bangladesh  880
//     Barbados  1246
//     Belarus  375
//     Belgium  32
//     Belize  501
//     Benin  229
//     Bhutan  975
//     Bolivia  591
//     BosniaAndHerzegovina  387
//     Botswana  267
//     Brazil  55
//     Brunei  673
//     Bulgaria  359
//     Burkina Faso  226
//     Burundi  257
//     Cabo Verde  238
//     Cambodia  855
//     Cameroon  237
//     Canada  1
//     CentralAfricanRepublic  236
//     Chad  235
//     Chile  56
//     China  86
//     Colombia  57
//     Comoros  269
//     CongoDRC  243
//     CongoRepublic  242
//     CostaRica  506
//     Croatia  385
//     Cuba  53
//     Cyprus  357
//     CzechRepublic  420
//     Denmark  45
//     Djibouti  253
//     Dominica  1767
//     DominicanRepublic  1809, 1829, 1849
//     EastTimor  670
//     Ecuador  593
//     Egypt  20
//     ElSalvador  503
//     Equatorial Guinea  240
//     Eritrea  291
//     Estonia  372
//     Eswatini  268
//     Ethiopia  251
//     Fiji  679
//     Finland  358
//     France  33
//     Gabon  241
//     Gambia  220
//     Georgia  995
//     Germany  49
//     Ghana  233
//     Greece  30
//     Grenada  1473
//     Guatemala  502
//     Guinea  224
//     GuineaBissau  245
//     Guyana  592
//     Haiti  509
//     Honduras  504
//     Hungary  36
//     Iceland  354
//     India  91
//     Indonesia  62
//     Iran  98
//     Iraq  964
//     Ireland  353
//     Israel  972
//     Italy  39
//     IvoryCoast  225
//     Jamaica  1876
//     Japan  81
//     Jordan  962
//     Kazakhstan  7
//     Kenya  254
// }
// impl ValidCountry {
    
// }



#[derive(EnumString, EnumIter, ValueEnum, EnumProperty, Clone, Debug, PartialEq)]
pub enum State {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WashingtonDc,
    WestVirginia,
    Wisconsin,
    Wyoming,

    Unknown,
}
impl State {
    const NUMBER: usize = 3;
    pub fn area_codes(&self) -> AreaCodes {
        match self {
            State::Alabama => &[205, 251, 256, 334],
            State::Alaska => &[907],
            State::Arizona => &[480, 520, 602],
            State::Arkansas => &[327, 479, 501, 870],
            State::California => &[209, 213, 310, 323, 408, 415, 424, 442, 510, 630, 559, 562, 619, 626, 628, 650, 657, 661, 669, 707, 714, 747, 760, 805, 818, 831, 858, 909, 916, 925, 949, 951],
            State::Colorado => &[303, 719, 720, 970],
            State::Connecticut => &[203, 475, 860, 959],
            State::Delaware => &[302],
            State::Florida => &[329, 305, 321, 352, 386, 407, 561, 727, 754, 772, 786, 813, 850, 863, 904, 941, 954],
            State::Georgia => &[229, 404, 470, 478, 678, 706, 762, 770, 912],
            State::Hawaii => &[808],
            State::Idaho => &[208, 986],
            State::Illinois => &[217, 224, 309, 312, 331, 618, 630, 708, 773, 779, 815, 847, 872],
            State::Indiana => &[219, 260, 317, 463, 574, 765, 812, 930],
            State::Iowa => &[319, 515, 563, 641, 712],
            State::Kansas => &[316, 620, 785, 913],
            State::Kentucky => &[270, 364, 502, 606, 859],
            State::Louisiana => &[225, 318, 337, 504, 985],
            State::Maine => &[207],
            State::Maryland => &[240, 301, 410, 443, 667],
            State::Massachusetts => &[339, 351, 413, 508, 617, 774, 781, 857, 978],
            State::Michigan => &[231, 248, 269, 313, 517, 586, 616, 734, 810, 906, 947, 989],
            State::Minnesota => &[218, 320, 507, 612, 651, 763, 952],
            State::Mississippi => &[228, 601, 662, 664, 769],
            State::Missouri => &[314, 417, 573, 636, 660, 816],
            State::Montana => &[406],
            State::Nebraska => &[308, 402, 531],
            State::Nevada => &[702, 725, 775],
            State::NewHampshire => &[603],
            State::NewJersey => &[201, 551, 609, 732, 848, 856, 862, 908, 973],
            State::NewMexico => &[505, 575],
            State::NewYork => &[212, 315, 332, 347, 516, 518, 585, 607, 631, 646, 680, 716, 718, 838, 845, 914, 917, 929, 934],
            State::NorthCarolina => &[252, 336, 704, 743, 828, 910, 919, 980, 984],
            State::NorthDakota => &[701],
            State::Ohio => &[216, 220, 234, 330, 380, 419, 440, 513, 567, 614, 740, 937],
            State::Oklahoma => &[405, 539, 580, 918],
            State::Oregon => &[458, 503, 541, 971],
            State::Pennsylvania => &[215, 267, 272, 412, 484, 570, 610, 717, 724, 814, 878],
            State::RhodeIsland => &[401],
            State::SouthCarolina => &[803, 843, 854, 864],
            State::SouthDakota => &[605],
            State::Tennessee => &[425, 615, 629, 731, 865, 901, 931],
            State::Texas => &[210, 214, 254, 281, 325, 346, 361, 409, 430, 432, 469, 512, 682, 713, 737, 806, 817, 830, 832, 903, 915, 936, 940, 956, 972, 979],
            State::Utah => &[385, 435, 801],
            State::Vermont => &[802],
            State::Virginia => &[276, 434, 540, 571, 703, 757, 804],
            State::Washington => &[202],
            State::WashingtonDc => &[202, 206, 253, 360, 425, 509, 564],
            State::WestVirginia => &[384, 681],
            State::Wisconsin => &[262, 414, 534, 608, 715, 920],
            State::Wyoming => &[307],
            _ => &[0],
        }
    }

    pub fn random(rng: &mut rand::rngs::ThreadRng) -> Self {
        let states = Self::iter();
        states.get(rand_number(rng, 0, states.len()-1)).unwrap()        
    }
}

impl From<usize> for State {
    fn from(value: usize) -> Self {
        for state in State::iter() {
            for code in state.area_codes() {
                if *code == value {
                    return state;
                }
            }
        }
        State::Unknown
    }
}

struct PhoneNumber([usize; 10]);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_country_code() {
        let us = USA {};
        let states = USA::states();

        println!("{:#?}", states);

    }

    #[test]
    fn test_random_state() {
        let mut rng = rand::thread_rng();
        let state = State::random(&mut rng);

        println!("{:#?}\n{:?}", state, state.area_codes());
    }

    #[test]
    fn test_state_from_code() {
        let s = State::NUMBER;
        let list = StateList::A;
        // let props = list.props();
        // println!("{}", props.name);

        let map: HashMap<String, Vec<String>> = HashMap::new(); 

        // map.iter().for_each(|(k, v)| {
        //     v.iter().for_each(|a|)
        // });
        
        assert_eq!(State::from(406), State::Montana);
        assert_eq!(State::from(720), State::Colorado);
        assert_eq!(State::from(0), State::Unknown);
        assert_eq!(State::from(999), State::Unknown);
    }
}
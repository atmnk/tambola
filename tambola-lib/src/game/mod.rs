use rand::Rng;
use serde::{Serialize,Deserialize};
use uuid::Uuid;
use rand::distributions::{Distribution, Uniform};
pub mod proto;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameSnapshot{
    pub done_numbers:Vec<u8>,
    pub started:bool,
    pub winnings:Vec<Winning>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Winning{
    pub name:String,
    pub verify_by:WinningVerifier,
    pub winner:Option<Uuid>
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WinningVerifier{
    LoveAtFirstSight,
    BullsEye,
    Early(u8),
    Line(u8),
    Range(u8,u8),
    BloodPressure,
    Temprature,
    NthHousie(u8)
}
#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct Winner{
    pub user:String,
    pub win_name:String,
    pub ticket:Ticket
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Draw{
    Random,
    Specific(u8)
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id:Uuid,
    pub name:String,
    pub user_type:UserType,
    pub ticket:Ticket
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticket{
    pub numbers:Vec<PositionedNumber>
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UserType{
    Host,
    NonHost
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionedNumber{
    pub row:u8,
    pub column:u8,
    pub claimed:bool,
    pub number:u8
}
impl WinningVerifier{
    pub fn verify(&self,game:&GameSnapshot,user:Uuid,ticket:&Ticket)->bool{
        if let Some(winner) = game.winnings.iter().find(|winning|{
            if winning.verify_by.clone() == self.clone(){
                if let Some(_) =  winning.winner {
                    false
                } else {
                    true
                }
            } else {
                false
            }
        }) {
            return false;
        }
        match self {
            WinningVerifier::LoveAtFirstSight=>{
                if game.done_numbers.len() != 1{
                    false
                } else {
                    let first_done=game.done_numbers.get(0).unwrap().clone();
                    if let Some(_)=ticket.numbers.iter().find(|pn| pn.number == first_done && pn.claimed) {
                        true
                    } else {
                        false
                    }
                }
            },
            _=>{false}
        }
    }
}
impl GameSnapshot{
    pub fn blank()->Self{
        GameSnapshot{
            done_numbers:vec![],
            started:false,
            winnings:vec![]
        }
    }
    pub fn non_done(&self)->Vec<u8>{
        let range = (1..91 as u8).collect::<Vec<u8>>();
        return range.iter().filter(|num| !self.done_numbers.contains(num.clone())).map(|n|n.clone()).collect()
    }
    pub fn draw_number(&mut self,draw_type:Draw)->Option<u8>{
        let number = match draw_type {
            Draw::Random=>{
                one_uniform_random_from(self.non_done().iter().map(|num| num.clone() as usize).collect()).map(|num| num as u8)
            },
            Draw::Specific(num)=>{
                if self.non_done().contains(&num){
                    Option::Some(num)
                } else {
                    Option::None
                }
            }
        };
        if let Some(num) = number {
            self.done_numbers.push(num)
        }
        number
    }
    pub fn mark_number_done(&mut self,number:u8){
        self.done_numbers.push(number);
    }
    pub fn mark_winner(&mut self,name:String,user:Uuid){
        let index = self.winnings.iter().position(|r| r.name.eq(&name)).unwrap();
        let mut item = self.winnings.get_mut(index).unwrap();
        item.winner = Option::Some(user)
    }
}
impl User{
    pub fn new(name:String,user_type:UserType)->Self {
        User{
            name,
            id:Uuid::new_v4(),
            ticket:Ticket::default(),
            user_type
        }
    }
}

impl Ticket {
    pub fn claim_number(&mut self,number:u8)->bool{
        let opt_index = self.numbers.iter().position(|r| r.number.clone() ==number  && !r.claimed.clone());
        if let Some(index) = opt_index {
            if let Some(mut item) = self.numbers.get_mut(index) {
                item.claimed = true;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl Default for Ticket{
    fn default() -> Self {
        Ticket{
            numbers:get_random_ticket_numbers()
        }
    }
}
pub fn get_random_ticket_numbers()->Vec<PositionedNumber>{
    let mut pn : Vec<PositionedNumber> = vec![];
    let mut cols :Vec<(u8,u8)>=vec![];
    for row in 0..3 as u8 {

        let vals = get_n_random_numbers_between(5,0,9);

        cols.extend(vals.iter().map(|val| (val.clone(),row)))
    }
    for col in 0..10 as u8 {
        let cts:Vec<(u8,u8)> = cols.clone().into_iter().filter(|(c_col,_)| c_col == &col ).collect::<Vec<(u8,u8)>>();
        let add = if col == 0 {
            1
        } else {
            0
        };
        let vals = get_n_random_numbers_between(cts.len() as u8,col * 10 + add , col* 10 + 10);
        let mut i:usize = 0;
        for ct in cts{
            pn.push(PositionedNumber{
                row:ct.1.clone(),
                column:ct.0.clone(),
                claimed:false,
                number:vals.get(i).unwrap().clone()
            });
            i = i + 1
        }
    }
    pn
}
pub fn get_n_random_numbers_between(size:u8,low:u8,high:u8)->Vec<u8>{
    let mut vals: Vec<u8> = vec![];

    for _ in 0..size  {
        while true {
            let val:u8 = get_random_number_between_wasm(low as usize,high as usize) as u8;
            if !vals.contains(&val){
                vals.push(val);
                break;
            }
        }
    }
    vals.sort();
    return vals;
}
pub fn get_random_number_between_wasm(low:usize,high:usize)->usize {
    let mut rng = rand::thread_rng();
    let val = rng.gen_range(low, high);
    val
}
pub fn one_uniform_random_from(numbers:Vec<usize>)->Option<usize> {
    if numbers.len() > 0 {
        let die = Uniform::from(0..numbers.len());
        let index = {
            let mut rng = rand::thread_rng();
            die.sample(&mut rng)
        };
        let number = numbers.get(index).unwrap().clone();
        Option::Some(number)
    } else {
        Option::None
    }
}
pub fn n_uniform_random_from(numbers:Vec<usize>,n:usize)->Option<Vec<usize>> {
    if numbers.len() > n as usize {
        let mut ret_numbers = vec![];
        let mut new_nums = numbers.clone();
        for _ in 0..n {
            if let Some(num) = one_uniform_random_from(new_nums.clone()) {
                ret_numbers.push(num);
                new_nums = new_nums.iter().filter(|i_num| num.clone() != *i_num.clone()).map(|n|n.clone()).collect();
            }
        }
        Option::Some(ret_numbers)
    } else {
        Option::None
    }
}
pub fn n_uniform_random_from_range(low:usize,high:usize,n:usize)->Option<Vec<usize>>{
    let nums = (low..(high + 1)).collect();
    return n_uniform_random_from(nums,n);
}
use std::hash::Hash;
use std::io;

use std::collections::HashMap;
fn main() {
    
    println!("Please enter your address");

   let mut address = String::new(); //buffer -> address of user 
    io::stdin().read_line(&mut address);


     
    let mut balances_of_user = HashMap::new(); // wallet 

     balances_of_user .insert(Token::USDT,1000.0);



// type -> value 
// type -> key -> hashable -> Hash & Eq
//implicitly hashmap    == ,type 

    // create a wallert for user 
      let mut users_wallet = Wallet::create_wallet(address,  balances_of_user);
    // wallet

       
      println!("your wallet created ");
      



loop{
    println!("");
    println!("1-check your balance"); 
    println!("2- see the market"); 
    println!("3- buying token");
    println!("4- sell token");
    let mut  ch = String::new();
    io::stdin().read_line(&mut ch); 

    // 1 -> String -> int 
    // 1 -> i32 
    // trim -> white spaces -> &str 
   // unwrap -> Option or Result -> unwrap to get the value out of it 
    let choice:i32 = ch.trim().parse().unwrap();
      // matching on i32 => 
    match choice {
         1 => {
          users_wallet.check_my_balance();
         },
         2=>{
            
          Token::show_current_market();
           
         },
         3=>{
             println!("which token you want to buy "); 
               Token::show_current_market(); 
             let mut token_name = String::new();
             io::stdin().read_line(&mut token_name);

               

             // string = token_name :Token



             let buying_token = Token::return_token(token_name.trim());

             println!("enter the amount  "); 

             let mut token_amount = String::new();
             io::stdin().read_line(&mut token_amount);


             let parsed_token_amount:f64 = token_amount.trim().parse().unwrap();

             Token::buy_token(buying_token, parsed_token_amount, &mut users_wallet.balances)


         },
         4=> {
          println!("which token you want to sell");

          users_wallet.check_my_balance();
          let mut token_name = String::new();
          io::stdin().read_line(&mut token_name);
          let selling_token = Token::return_token(token_name.trim());
          let selling_token2 = Token::return_token(token_name.trim());
          println!("enter the amount  "); 

          let mut token_amount = String::new();
          io::stdin().read_line(&mut token_amount);


          let parsed_token_amount:f64 = token_amount.trim().parse().unwrap();


          Token::sell_token(selling_token, selling_token2,parsed_token_amount, &mut users_wallet.balances)

         },

         _=>{
          println!("invalid option")
         }

         
    };


}
     



    
      //  check_my_balance -> 







}  

// listing tokens  -> 
#[derive(Debug,PartialEq,Hash,Eq)]
enum Token{

    SOL,
    DOT,
    ETH, 
    BTC, 
    USDT
}



// buy_token(token:Token, amount:f64, balances:Vec<Balances> )



impl Token {

    fn show_current_market() {
   // Self == TOken::
        println!("BTC: price:{}",Self::return_price(&Token::BTC) );
        println!("ETH: price:{}",Self::return_price(&Token::ETH) );
        println!("SOL: price:{}",Self::return_price(&Token::SOL) );
        println!("DOT: price:{}",Self::return_price(&Token::DOT) );
        println!("USDT: price:{}",Self::return_price(&Token::USDT) );
    }


    fn return_token(input:&str) -> Self {

        match input {
            "sol"=> Token::SOL,
            "dot"=> Token::DOT,
            "btc"=> Token::BTC,
            "eth"=> Token::ETH,
            "usdt"=> Token::USDT,
            _=> Token::BTC
        }
    }
    
    fn return_price(&self) -> f64 {
    
        match self{
            Token::SOL=> 34.0,
            Token::BTC=> 30000.0,
            Token::ETH=> 1000.0,
            Token::DOT=> 8.0,
            Token::USDT=> 1.0,
    
        }
    
    }



    fn buy_token(self, amount:f64, balances:&mut  HashMap<Token,f64>) {

          // 1000 usdt 
           // self 
           //1000   
           //usdt >= current price(self)*amount 
           // 10 *8 = 80 dot
           // 40 *34  = 1360 

           // get usdt of balacnce of user 
          let users_usdt_bal = balances.get(&Token::USDT).unwrap();
          // calculate the price 

          let price_of_token = Self::return_price(&self);
          let calculated_price_of_token = price_of_token*amount;

          if users_usdt_bal >= &calculated_price_of_token {
                  
         /// update users wallet 
         /// 
             balances.insert(Token::USDT,users_usdt_bal-&calculated_price_of_token );

             if balances.contains_key(&self) {
                let prev_bal_of_token = balances.get(&self).unwrap();
                balances.insert(self, &amount+prev_bal_of_token);
             }else {
                balances.insert(self, amount);
             }
             
               println!("Transaction is succesfull !!! ");
            

                
          }else {

            println!("insuffiecuent balance");
            println!("Transaction declined ");
          }


            // for =>  usdt 
            // 

           

        }

        fn sell_token(self,sell_token:Token,amount:f64,balances:&mut HashMap<Token,f64> ){

            // let copy_sell_token= &self;
                   let bal_of_token = balances.get(&self).unwrap();
                   if bal_of_token>=&amount {

                    balances.insert(self, bal_of_token-amount);
                    let prev_bal_of_usdt = balances.get(&Token::USDT).unwrap();
                    let price_of_token = Self::return_price(&sell_token);
                   let calculated_price_of_token = price_of_token*amount;
                    let cal_usdt_bal = prev_bal_of_usdt+calculated_price_of_token;
                    balances.insert(Token::USDT,cal_usdt_bal );

                            println!("TRansction is succeasfull ");
                   }else {
                    println!("invalid amount ");
                    println!("transaction declined");
                   }

        }


     

}




// wallet (address:String and balances:) 


// hashmap<Token, f64>  
// look up 

struct Wallet {

    address:String,
    balances:HashMap<Token,f64>,
}


impl Wallet {

// associted function
fn create_wallet(address:String, balances:HashMap<Token, f64>)-> Self {


    Self { address, balances }


}

// method
fn check_my_balance(&self) {
  
    // into_inter() -> ownership of vector 

    println!("{:?}", self.balances); // hashmap-> 

}

}



// rahul'wallet -> usdt:1000 sol:40 , dot:9 Balances


// Balances (token: balance)

struct Balances {

    token:Token,
    balance:f64

}

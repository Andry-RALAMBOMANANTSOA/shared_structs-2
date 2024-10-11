use serde::{Serialize, Deserialize};
use std::{fmt,collections::{BTreeMap,HashMap}, hash::Hash};


#[derive(Debug, Clone, PartialEq,Serialize,Deserialize)]
 pub enum OrderSide {
     Long,
     Short,
     Unspecified,
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderSide::Long => write!(f, "Long"),
            OrderSide::Short => write!(f, "Short"),
            OrderSide::Unspecified => write!(f, ""),
        }
    }
}

#[derive(Debug, Clone, PartialEq,Serialize,Deserialize)]
 pub enum OrderExpiration {
    FOK,  // Fill or Kill
    IOC,  // Immediate or Cancel
    GTC,  // Good Till Cancelled
    GTD,// Good Till Date
    Unspecified,  
}
impl fmt::Display for OrderExpiration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderExpiration::FOK => write!(f, "FOK"),
            OrderExpiration::IOC => write!(f, "IOC"),
            OrderExpiration::GTC => write!(f, "GTC"),
            OrderExpiration::GTD => write!(f, "GTD"),
            OrderExpiration::Unspecified => write!(f, ""),
        }
    }
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct IcebergOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub total_quantity:i32,
    pub visible_quantity:i32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration, //tsisy raha market
    pub price: i32, //tsisy raha market
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct IcebergOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub unix_time:i64,
    pub iceberg_identifier:i64,
    pub total_quantity:i32,
    pub visible_quantity:i32,
    pub resting_quantity:i32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration, //tsisy raha market
    pub price: i32, //tsisy raha market
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct LimitOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub order_identifier: Option<i64>,
    pub order_quantity:i32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration, //tsisy raha market
    pub price: i32, //tsisy raha market
    pub pointing_at: Option<i64>, //position_identifier
    
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MarketOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub order_identifier: Option<i64>,
    pub order_quantity:i32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub pointing_at: Option<i64>, //position_identifier
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct StopOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub order_quantity:i32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub trigger_price: i32, 
    pub pointing_at: Option<i64>, //position_identifier
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct StopLimitOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub order_quantity:i32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration, 
    pub trigger_price: i32, 
    pub price:i32,
    pub pointing_at: Option<i64>, //position_identifier
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ModifyOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub order_identifier: i64, //misy raha avy amin'ny modify,stop,mit,sns
    pub new_quantity:i32,
    
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ModifyIcebergOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub iceberg_identifier: i64, //misy raha avy amin'ny modify,stop,mit,sns
    pub new_quantity:i32,
    pub new_visible_quantity:i32,
    
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct DeleteOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub order_identifier: i64, //misy raha avy amin'ny modify,stop,mit,sns
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct DeleteIcebergOrder {
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub iceberg_identifier: i64, 
}


#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MarketConf {
    pub exchange: String,
    pub market_name: String,
    pub contract : i32,
    pub tick_size: i32,
    pub tick_value: i32,
    pub quotation: String,
    
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct TraderOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub order_quantity: i32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub price: i32,
    pub pointing_at: Option<i64>, //position_identifier
    
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct TraderStopOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub order_quantity: i32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub trigger_price: i32,
    pub pointing_at: Option<i64>, //position_identifier
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct TraderStopLimitOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub order_quantity: i32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub trigger_price: i32,
    pub price: i32,
    pub pointing_at: Option<i64>, //position_identifier
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct DeletedOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub order_quantity: i32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub price: i32,
    pub pointing_at: Option<i64>, //position_identifier
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct DeletedIcebergStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub iceberg_identifier: i64,
    pub total_quantity: i32,
    pub visible_quantity:i32,
    pub resting_quantity:i32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub price: i32,
   
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct DeletedStopOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub order_quantity: i32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub trigger_price: i32,
    pub pointing_at: Option<i64>, //position_identifier
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct DeletedStopLimitOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub order_quantity: i32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub trigger_price: i32,
    pub price :i32,
    pub pointing_at: Option<i64>, //position_identifier
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ModifiedOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub older_order_quantity: i32,
    pub new_order_quantity: i32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub price: i32,
    pub pointing_at: Option<i64>, //position_identifier
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ModifiedIcebergStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub iceberg_identifier: i64,
    pub older_quantity: i32,
    pub new_quantity: i32,
    pub older_visible_quantity: i32,
    pub new_visible_quantity: i32,
    pub older_resting_quantity:i32,
    pub new_resting_quantity:i32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub price: i32,
   
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ModifiedStopOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub older_order_quantity: i32,
    pub new_order_quantity: i32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub trigger_price: i32,
    pub pointing_at: Option<i64>, //position_identifier
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ModifiedStopLimitOrderStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub trader_identifier: i64,
    pub order_identifier: i64,
    pub older_order_quantity: i32,
    pub new_order_quantity: i32,
    pub order_side: OrderSide,
    pub expiration:OrderExpiration,
    pub trigger_price: i32,
    pub price: i32,
    pub pointing_at: Option<i64>, //position_identifier
}


#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MatchStruct {
    pub market: String,
    pub broker_identifier_taker: String,
    pub broker_identifier_maker: String,
    pub unix_time: i64,
    pub match_identifier:i64,
    pub trader_identifier_taker: i64,
    pub order_identifier_taker: i64,
    pub trader_identifier_maker: i64,
    pub order_identifier_maker: i64,
    pub maker_position_side : OrderSide,
    pub taker_position_side : OrderSide,
    pub taker_type:String,
    pub expiration_taker:OrderExpiration,
    pub expiration_maker:OrderExpiration,
    pub order_quantity: i32,
    pub order_side: OrderSide,
    pub price: i32,
    pub pointing_at_maker: Option<i64>, //position_identifier
    pub pointing_at_taker: Option<i64>, //position_identifier
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct PositionStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub position_identifier:i64,
    pub trader_identifier: i64,
    pub position_side: OrderSide,
    pub position_quantity: i32,
    pub price: i32,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ClosePositionStruct {
    pub market: String,
    pub broker_identifier: String,
    pub unix_time: i64,
    pub position_identifier:i64,
    pub position_identifier_closer:i64,
    pub trader_identifier: i64,
    pub position_side: OrderSide,
    pub position_quantity: i32,
    pub initial_price: i32,
    pub closing_price: i32,
}


#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct PostTraderInf {
    pub unix_time:i64,
    pub market: String,
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub position_identifier:i64,
    pub position_identifier_closer:i64,
    pub position_quantity:i32,
    pub trader_calcbalance:i32, //positive if profit, negative if loss
  
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct BBO {
   
    pub unix_time: i64,
    pub market:String,
    pub ask_price: Option<i32>,
    pub bid_price:Option<i32>,
    pub ask_size:Option<i32>,
    pub bid_size:Option<i32>,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct TimeSale {
    
    pub market: String,
    pub exchange:String,
    pub unix_time: i64,
    pub order_quantity: i32,
    pub order_side: OrderSide,
    pub price:i32,
}


#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Last {
    pub unix_time:i64,
    pub market:String,
    pub price:i32,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct OHLCV {
    pub state:String, //progessiong, completed
    pub datetime:i64,
    pub timeframe:String,
    pub market:String,
    pub open: i32,
    pub high: i32,
    pub low: i32,
    pub close: i32,
    pub buy_volume: i32,
    pub sell_volume: i32,
    pub differential_volume: i32,
    pub total_volume: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VolumeData {
    pub buy_volume: i32,
    pub sell_volume: i32,
    pub differential_volume: i32, // buy_volume - sell_volume
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct VP {
    pub state:String, //progessiong, completed
    pub datetime:i64,
    pub timeframe:String,
    pub market:String,
    pub volume_points: BTreeMap<i32, VolumeData>, // i32 is the price point
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MBPData {
   
    pub mbp: BTreeMap<i32, i32>,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct InterestTree {
   
    pub interest: BTreeMap<i32, i32>,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct OB {
    pub market:String,
    pub bid: BTreeMap<i32, i32>,
    pub ask: BTreeMap<i32, i32>,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct FullOB {
    pub unix_time: i64,
    pub market:String,
    pub bid: MBPData,
    pub ask: MBPData,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct FullInterest {
    pub unix_time: i64,
    pub market:String,
    pub long: InterestTree,
    pub short: InterestTree,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MBOData {
  
    pub mbo: BTreeMap<i32, Vec<i32>>,//key is price and value is quantity
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Volume {
   
    pub unix_time: i64,
    pub market:String,
    pub buy_volume:i32,
    pub sell_volume:i32,
    pub price:i32,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MAPData {
    
  
    pub map:BTreeMap<i32, Vec<i64>>,//key is price, value is vec of order identifier
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MAPStopData {
  
    pub map:BTreeMap<i32, Vec<i64>>,//key is price, value is vec of order identifier
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MAPStopLimitData {
    pub map:BTreeMap<i32, Vec<i64>>,//key is price, value is vec of order identifier
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Messaging {
    pub unix_time : i64,
    pub market : String,
    pub broker_identifier : String,
    pub trader_identifier : i64,
    pub message : String,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MBPEvents {
    pub unix_time : i64,
    pub market:String,
    pub side : String,
    pub event_value:i32,
    pub event_price:i32,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct InterestEvents {
    pub unix_time : i64,
    pub market:String,
    pub side : String,
    pub event_value:i32,
    pub event_price:i32,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct TotalInterest {
    pub market:String,
    pub long : i32,
    pub short:i32,
    pub total:i32,
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ExecutedStop {
    pub unix_time : i64,
    pub market : String,
    pub broker_identifier : String,
    pub trader_identifier : i64,
    pub order_identifier : i64,
    pub order_quantity : i32,
    pub order_side : OrderSide,
    pub expiration : OrderExpiration,
    pub trigger_price : i32,
    pub pointing_at: Option<i64>,
       
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ExecutedStopLimit {
    pub unix_time : i64,
    pub market : String,
    pub broker_identifier : String,
    pub trader_identifier : i64,
    pub order_identifier : i64,
    pub order_quantity : i32,
    pub order_side : OrderSide,
    pub expiration : OrderExpiration,
    pub trigger_price : i32,
    pub price : i32,
    pub pointing_at: Option<i64>,

}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct ExecutedIceberg {
    pub unix_time : i64,
    pub market : String,
    pub broker_identifier : String,
    pub trader_identifier : i64,
    pub iceberg_identifier : i64,
    pub executed_quantity : i32,
    pub order_side : OrderSide,
    pub expiration : OrderExpiration,
    pub price : i32,
   
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct StructData {
    pub market:String,
    pub data_struct: HashMap<i64, TraderOrderStruct>,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct StructStopData {
    pub market:String,
   pub data_struct: HashMap<i64, TraderStopOrderStruct>,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct StructStopLimitData {
    pub market:String,
   pub data_struct: HashMap<i64, TraderStopLimitOrderStruct>,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Save {
    pub market:String,
   
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub enum Structs {
    IcebergOrder(IcebergOrder),
    LimitOrder(LimitOrder),
    MarketOrder(MarketOrder),
    StopOrder(StopOrder),
    StopLimitOrder(StopLimitOrder),
    ModifyOrder(ModifyOrder),
    ModifyIcebergOrder(ModifyIcebergOrder),
    DeleteOrder(DeleteOrder),
    DeleteIcebergOrder(DeleteIcebergOrder),
    IcebergOrderStruct(IcebergOrderStruct),
    TraderOrderStruct(TraderOrderStruct),
    TraderStopOrderStruct(TraderStopOrderStruct),
    TraderStopLimitOrderStruct(TraderStopLimitOrderStruct),
    DeletedOrderStruct(DeletedOrderStruct),
    DeletedIcebergStruct(DeletedIcebergStruct),
    DeletedStopOrderStruct(DeletedStopOrderStruct),
    DeletedStopLimitOrderStruct(DeletedStopLimitOrderStruct),
    ModifiedStopOrderStruct(ModifiedStopOrderStruct),
    ModifiedStopLimitOrderStruct(ModifiedStopLimitOrderStruct),
    ModifiedOrderStruct(ModifiedOrderStruct),
    ModifiedIcebergStruct(ModifiedIcebergStruct),
    ExecutedStop(ExecutedStop),
    ExecutedStopLimit(ExecutedStopLimit),
    ExecutedIceberg(ExecutedIceberg),
    Messaging(Messaging),
    MatchStruct(MatchStruct),
    PositionStruct(PositionStruct),
    ClosePositionStruct(ClosePositionStruct),
    PostTraderInf(PostTraderInf),
    BBO(BBO),
    TimeSale(TimeSale),
    Last(Last),
    MBPData(MBPData),
    MBOData(MBOData),
    Volume(Volume),
    MAPData(MAPData),
    MAPStopData(MAPStopData),
    MAPStopLimitData(MAPStopLimitData),
    MBPEvents(MBPEvents),
    MarketConf(MarketConf),
    StructData(StructData),
    StructStopData(StructStopData),
    StructStopLimitData(StructStopLimitData),
    OB(OB),
    FullOB(FullOB),
    FullInterest(FullInterest),
    InterestEvents(InterestEvents),
    Save(Save),
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct MarketConfig {
    pub market_list: Vec<String>,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct BrokerConfig {
    pub broker_list: Vec<String>, // Broker identifier -> HMAC key
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MarketSpec {
    pub tick_size: i32,
    pub min_contract: i32,
    pub max_contract: i32,
    pub iceberg_min_contract:i32,
   
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MarketSpecConfig {
    pub market_specification: HashMap<String, MarketSpec>,
}
#[derive(Serialize)]
pub struct MSResponse {
    pub message: String,
    pub success: bool,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct BrokerConfigDedicaced {
    pub broker_list: HashMap<String, String>, // Broker identifier -> HMAC key
}
//Broker level
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct PaidCommission {
    pub unix_time: i64, 
    pub commission_amount:i32,
    pub c_type:String,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct TraderBalance {
    pub broker_identifier: String,
    pub trader_identifier:i64,
    pub balance:i32,
  
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Number {
    
    pub number:u32,
}

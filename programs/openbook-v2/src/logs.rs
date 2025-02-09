use crate::state::{Market, MarketIndex, Position};
use anchor_lang::prelude::*;
use borsh::BorshSerialize;

pub fn emit_balances(open_orders_acc: Pubkey, p: &Position, _m: &Market) {
    emit!(BalanceLog {
        open_orders_acc,
        base_position: p.base_position_lots(),
        quote_position: p.quote_position_native().to_bits(),
    });
}

#[event]
pub struct BalanceLog {
    pub open_orders_acc: Pubkey,
    pub base_position: i64,
    pub quote_position: i128, // I80F48
}

#[event]
pub struct DepositLog {
    pub open_orders_acc: Pubkey,
    pub signer: Pubkey,
    pub quantity: u64,
}

#[event]
pub struct FillLog {
    pub taker_side: u8, // side from the taker's POV
    pub maker_slot: u8,
    pub maker_out: bool, // true if maker order quantity == 0
    pub timestamp: u64,
    pub seq_num: u64, // note: usize same as u64

    pub maker: Pubkey,
    pub maker_client_order_id: u64,
    pub maker_fee: f32,

    // Timestamp of when the maker order was placed; copied over from the LeafNode
    pub maker_timestamp: u64,

    pub taker: Pubkey,
    pub taker_client_order_id: u64,
    pub taker_fee: f32,

    pub price: i64,
    pub quantity: i64, // number of base lots
}

#[event]
pub struct MarketMetaDataLog {
    pub market: Pubkey,
    pub market_index: MarketIndex,
    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub base_lot_size: i64,
    pub quote_lot_size: i64,
    pub oracle: Pubkey,
}

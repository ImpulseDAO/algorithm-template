#![no_std]

use arena_io::{AttackKind, BattleAction, YourTurn};
use gstd::msg;

const QUICK_ATTAK_ENERGY: u8 = 2;

#[gstd::async_main]
async fn main() {
    let turn: YourTurn = msg::load().expect("unable to decode `YourTurn`");

    if turn.you.position > turn.enemy.position && turn.you.position - turn.enemy.position > 1 {
        msg::reply(BattleAction::MoveLeft, 0).expect("unable to reply");
        return;
    } else if turn.enemy.position > turn.you.position && turn.enemy.position - turn.you.position > 1
    {
        msg::reply(BattleAction::MoveRight, 0).expect("unable to reply");
        return;
    }

    if turn.you.energy >= QUICK_ATTAK_ENERGY {
        msg::reply(
            BattleAction::Attack {
                kind: AttackKind::Quick,
            },
            0,
        )
        .expect("unable to reply");
    } else {
        msg::reply(BattleAction::Rest, 0).expect("unable to reply");
    }
}

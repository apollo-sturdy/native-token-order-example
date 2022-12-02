use cosmwasm_std::Coin;
use native_token_order_example::msg::{ExecuteMsg, InstantiateMsg};
use osmosis_testing::{Module, OsmosisTestApp, Wasm};
use test_case::test_case;

const UATOM: &str = "uatom";
const UOSMO: &str = "uosmo";

// const WASM_FILE: &str = "target/wasm32-unknown-unknown/release/native_token_order_example.wasm";
const WASM_FILE: &str = "artifacts/native_token_order_example.wasm";

#[test_case(vec![Coin::new(1_000_000, UATOM),Coin::new(1_000_000, UOSMO)]; "deposit uatom uosmo")]
#[test_case(vec![Coin::new(1_000_000, UOSMO),Coin::new(1_000_000, UATOM)]; "deposit uosmo uatom")]
fn test_deposit_funds(funds: Vec<Coin>) {
    let runner = OsmosisTestApp::default();
    let accs = runner
        .init_accounts(
            &[
                Coin::new(1_000_000_000_000, UATOM),
                Coin::new(1_000_000_000_000, UOSMO),
            ],
            1,
        )
        .unwrap();
    let acc = &accs[0];
    let wasm = Wasm::new(&runner);

    // Upload the contract
    println!("Uploading contract");
    let wasm_byte_code = std::fs::read(WASM_FILE).unwrap();
    let code_id = wasm
        .store_code(&wasm_byte_code, None, acc)
        .unwrap()
        .data
        .code_id;

    // Instantiate the contract
    let contract_addr = wasm
        .instantiate(code_id, &InstantiateMsg {}, None, None, &[], acc)
        .unwrap()
        .data
        .address;

    // Execute Deposit
    let msg = ExecuteMsg::Deposit {};
    wasm.execute(&contract_addr, &msg, &funds, acc).unwrap();

    // Execute DepositAndRefund
    let msg = ExecuteMsg::DepositAndRefund {};
    wasm.execute(&contract_addr, &msg, &funds, acc).unwrap();

    // Execute DepositAndRefundReverseSorted
    let msg = ExecuteMsg::DepositAndRefundReverseSorted {};
    wasm.execute(&contract_addr, &msg, &funds, acc).unwrap();
}

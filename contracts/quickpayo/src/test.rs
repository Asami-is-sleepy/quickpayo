#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};

use crate::{QuickPayoContract, QuickPayoContractClient, InvoiceError};

// ── helpers ──────────────────────────────────────────────────────────────────

fn setup() -> (Env, QuickPayoContractClient<'static>) {
    let env = Env::default();
    let contract_id = env.register_contract(None, QuickPayoContract);
    let client = QuickPayoContractClient::new(&env, &contract_id);
    (env, client)
}

fn make_parties(env: &Env) -> (Address, Address) {
    (Address::generate(env), Address::generate(env))
}

// ── happy path ────────────────────────────────────────────────────────────────

#[test]
fn test_happy_path_payment() {
    let (env, client) = setup();
    let (client_addr, freelancer) = make_parties(&env);
    let invoice_id = BytesN::from_array(&env, &[1u8; 32]);

    client.create_invoice(&invoice_id, &client_addr, &freelancer, &5000);
    client.pay_invoice(&invoice_id);

    assert!(client.check_status(&invoice_id));
}

#[test]
fn test_state_after_payment() {
    let (env, client) = setup();
    let (client_addr, freelancer) = make_parties(&env);
    let invoice_id = BytesN::from_array(&env, &[3u8; 32]);

    client.create_invoice(&invoice_id, &client_addr, &freelancer, &5000);
    assert!(!client.check_status(&invoice_id), "should be unpaid before payment");

    client.pay_invoice(&invoice_id);
    assert!(client.check_status(&invoice_id), "should be paid after payment");
}

// ── error cases ───────────────────────────────────────────────────────────────

#[test]
#[should_panic]
fn test_double_payment_panics() {
    let (env, client) = setup();
    let (client_addr, freelancer) = make_parties(&env);
    let invoice_id = BytesN::from_array(&env, &[2u8; 32]);

    client.create_invoice(&invoice_id, &client_addr, &freelancer, &5000);
    client.pay_invoice(&invoice_id);
    client.pay_invoice(&invoice_id); // should panic: AlreadyPaid
}

#[test]
#[should_panic]
fn test_duplicate_invoice_panics() {
    let (env, client) = setup();
    let (client_addr, freelancer) = make_parties(&env);
    let invoice_id = BytesN::from_array(&env, &[4u8; 32]);

    client.create_invoice(&invoice_id, &client_addr, &freelancer, &5000);
    client.create_invoice(&invoice_id, &client_addr, &freelancer, &5000); // should panic: InvoiceExists
}

#[test]
#[should_panic]
fn test_pay_nonexistent_invoice_panics() {
    let (env, client) = setup();
    let invoice_id = BytesN::from_array(&env, &[5u8; 32]);

    client.pay_invoice(&invoice_id); // should panic: NotFound
}

#[test]
#[should_panic]
fn test_check_nonexistent_invoice_panics() {
    let (env, client) = setup();
    let invoice_id = BytesN::from_array(&env, &[6u8; 32]);

    client.check_status(&invoice_id); // should panic: NotFound
}

// ── edge cases ────────────────────────────────────────────────────────────────

#[test]
fn test_multiple_independent_invoices() {
    let (env, client) = setup();
    let (client_addr, freelancer) = make_parties(&env);

    let id_a = BytesN::from_array(&env, &[10u8; 32]);
    let id_b = BytesN::from_array(&env, &[11u8; 32]);

    client.create_invoice(&id_a, &client_addr, &freelancer, &1000);
    client.create_invoice(&id_b, &client_addr, &freelancer, &2000);

    // Pay only invoice A
    client.pay_invoice(&id_a);

    assert!(client.check_status(&id_a), "invoice A should be paid");
    assert!(!client.check_status(&id_b), "invoice B should still be unpaid");
}

#[test]
fn test_zero_amount_invoice() {
    let (env, client) = setup();
    let (client_addr, freelancer) = make_parties(&env);
    let invoice_id = BytesN::from_array(&env, &[20u8; 32]);

    // Zero-amount invoices are valid at the contract level
    client.create_invoice(&invoice_id, &client_addr, &freelancer, &0);
    client.pay_invoice(&invoice_id);

    assert!(client.check_status(&invoice_id));
}
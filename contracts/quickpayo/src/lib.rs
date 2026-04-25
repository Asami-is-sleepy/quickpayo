#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    xdr::{ScErrorCode, ScErrorType},
    Address, Env, BytesN, panic_with_error,
};

#[derive(Clone)]
#[contracttype]
pub struct Invoice {
    pub client: Address,
    pub freelancer: Address,
    pub amount: i128,
    pub paid: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Invoice(BytesN<32>),
}

// u32 discriminants required for manual From impl below
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum InvoiceError {
    InvoiceExists = 1,
    NotFound      = 2,
    AlreadyPaid   = 3,
}

// Teach Soroban how to turn InvoiceError into its Error type
impl From<InvoiceError> for soroban_sdk::Error {
    fn from(e: InvoiceError) -> Self {
        soroban_sdk::Error::from((
            ScErrorType::Contract,
            match e {
                InvoiceError::InvoiceExists => ScErrorCode::ExistingValue,
                InvoiceError::NotFound      => ScErrorCode::MissingValue,
                InvoiceError::AlreadyPaid   => ScErrorCode::InvalidInput,
            },
        ))
    }
}

#[contract]
pub struct QuickPayoContract;

#[contractimpl]
impl QuickPayoContract {
    /// Create a new invoice. Panics if invoice_id already exists.
    pub fn create_invoice(
        env: Env,
        invoice_id: BytesN<32>,
        client: Address,
        freelancer: Address,
        amount: i128,
    ) {
        let key = DataKey::Invoice(invoice_id.clone());
        if env.storage().instance().has(&key) {
            panic_with_error!(&env, InvoiceError::InvoiceExists);
        }

        let invoice = Invoice { client, freelancer, amount, paid: false };
        env.storage().instance().set(&key, &invoice);
    }

    /// Pay an existing invoice. Panics if not found or already paid.
    pub fn pay_invoice(env: Env, invoice_id: BytesN<32>) {
        let key = DataKey::Invoice(invoice_id.clone());

        let mut invoice: Invoice = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or_else(|| panic_with_error!(&env, InvoiceError::NotFound));

        if invoice.paid {
            panic_with_error!(&env, InvoiceError::AlreadyPaid);
        }

        invoice.paid = true;
        env.storage().instance().set(&key, &invoice);
    }

    /// Returns true if the invoice has been paid, false otherwise.
    pub fn check_status(env: Env, invoice_id: BytesN<32>) -> bool {
        let key = DataKey::Invoice(invoice_id.clone());

        let invoice: Invoice = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or_else(|| panic_with_error!(&env, InvoiceError::NotFound));

        invoice.paid
    }
}

#[cfg(test)]
mod test;
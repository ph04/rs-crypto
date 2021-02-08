use std::{fmt, error};
use std::convert::{TryInto, TryFrom};
use sha2::{Sha512, Digest};
use chrono::{DateTime, Utc};
use crate::{
	account::Account,
	positive_f64::PositiveF64
};
use ed25519_dalek::{
	Keypair,
	Signature,
	Signer,
};

/// A structure to handle the transactions of the blockchain.
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
	pub sender: Account,
	pub receiver: Account,
	pub amount: f64,
	pub time: DateTime<Utc>,
	hash_sender_password: [u8; 64],
	message: String,
	signature: [u8; 64],
	pub hash: [u8; 64],
}

impl Transaction {
	/// Generates a new `Transaction`.
	/// In order to perform a new transaction, the sender must specify his account, his password,
	/// the amount and the receiver's account.
	/// 
	/// Every transaction contains:
	/// - the sender's `Account`
	/// - the receiver's `Account`
	/// - the amount of the transaction
	/// - the `DateTime<Utc>` time when the block was generated
	/// - the SHA-512 hash of the sender's password
	/// - the message to be signed
	/// - the digital signature of the message
	/// - the hash of the transaction
	pub fn new(sender: Account, receiver: Account, amount: f64, sender_password: &str) -> Self {
		let mut hasher = Sha512::new();

		hasher.update(sender_password.as_bytes());

		let hash_sender_password = hasher
			.finalize()[..]
			.try_into()
			.expect("Error generating the SHA-512 hash of the password.");

		let mut transaction = Self {
			sender,
			receiver,
			amount,
			time: Utc::now(),
			hash_sender_password,
			message: String::new(),
			signature: [0; 64],
			hash: [0; 64],
		};

		transaction.sign();

		transaction.calculate_hash();

		transaction
	}

	/// This method is called when a new transaction is generated,
	/// and it is used to perform the digital signature of the new transaction.
	/// 
	/// The digital signature is generated using the `Keypair` from the sender's account,
	/// using the `ed25519_dalek` crate.
	/// 
	/// The signature is performed on the `message`, generated by using:
	/// - the sender's `Account`
	/// - the receiver's `Account`
	/// - the amount of the transaction
	/// - the `DateTime<Utc>` time when the block was generated
	fn sign(&mut self) {
		let keypair = Keypair::from_bytes(&self.sender.keypair).expect("Error generating the Keypair while signing the transaction.");
		
		self.message = format!("{}{}{}{:?}", self.sender, self.receiver, self.amount, self.time);

		self.signature = keypair.sign(self.message.as_bytes()).to_bytes();
	}

	/// This method is called when a new transacion is generated,
	/// and is is used to calculate the SHA-512 hash of the new transaction.
	///
	/// The hash is calculated by using the `message` and the `signature`,
	/// both fields generated in the `sign()` method.
	fn calculate_hash(&mut self) {
		let mut hasher = Sha512::new();

		let message = format!("{:?}{:?}", self.message, self.signature);

		hasher.update(message.as_bytes());

		self.hash = hasher
			.finalize()[..]
			.try_into()
			.expect("Error generating the SHA-512 hash of the transaction.");
	}

	/// This method checks if the transaction is valid,
	/// and returns a `Err(ValidationError)` if the transaction isn't valid.
	/// 
	/// - If the hash in the input doesn't match with the `hash` of the transaction,
	/// a `ValidationError::Tempered` error is returned.
	/// - If the hash of the sender's password doesn't match with the `hash_sender_password` field,
	/// a `ValidationError::WrongPassword` error is returned.
	/// - If the signature verification doesn't succeed,
	/// a `ValidationError::InvalidSign` error is returned.
	/// - If the amount is zero or negative,
	/// a `ValidationError::InvalidAmount` error is returned. 
	pub fn validate(&self, hash: [u8; 64]) -> Result<(), ValidationError> {
		let signature = Signature::try_from(self.signature).expect("Error generating the Signature while validating the transaction.");

		let keypair = Keypair::from_bytes(&self.sender.keypair).expect("Error generating the Keypair while validating the transaction.");

		if hash != self.hash {
			Err(ValidationError::Tempered)
		} else if self.hash_sender_password != self.sender.hash_password {
			Err(ValidationError::WrongPassword)
		} else if keypair.verify(self.message.as_bytes(), &signature).is_err() {
			Err(ValidationError::InvalidSignature)
		} else if PositiveF64::new(self.amount).is_err() || self.amount == 0.0 || self.sender.balance.0 < self.amount {
			Err(ValidationError::InvalidAmount)
		} else {
			Ok(())
		}
	}
}

/// An enum to handle errors generated while validating `Transaction`s.
#[derive(Debug)]
pub enum ValidationError {
	Tempered,
	WrongPassword,
	InvalidSignature,
	InvalidAmount,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
        	Self::Tempered =>  write!(f, "Tempered transaction."),
        	Self::WrongPassword => write!(f, "Wrong password."),
        	Self::InvalidSignature => write!(f, "Invalid signature."),
		Self::InvalidAmount => write!(f, "Invalid amount.")
        }
    }
}

impl error::Error for ValidationError {}

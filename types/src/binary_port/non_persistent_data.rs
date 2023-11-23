//! The "request for non persistent data" variant of the request to the binary port.

use crate::{
    bytesrepr::{self, FromBytes, ToBytes, U8_SERIALIZED_LENGTH},
    BlockHash, TransactionHash,
};

const BLOCK_HEIGHT_2_HASH_TAG: u8 = 0;
const HIGHEST_BLOCK_TAG: u8 = 1;
const COMPLETED_BLOCK_CONTAINS_TAG: u8 = 2;
const TRANSACTION_HASH_2_BLOCK_HASH_AND_HEIGHT_TAG: u8 = 3;
const PEERS_TAG: u8 = 4;

/// Request for non persistent data
#[derive(Debug)]
pub enum NonPersistedDataRequest {
    /// Returns hash for a given height.
    BlockHeight2Hash {
        /// Block height.
        height: u64,
    },
    /// Returns height&hash for the currently highest block.
    HighestCompleteBlock,
    /// Returns true if `self.completed_blocks.highest_sequence()` contains the given hash
    CompletedBlockContains {
        /// Block hash.
        block_hash: BlockHash,
    },
    /// Returns block hash and height for a given transaction hash.
    TransactionHash2BlockHashAndHeight {
        /// Transaction hash.
        transaction_hash: TransactionHash,
    },
    /// Returns connected peers.
    Peers,
    // TODO:
    // Status requests (effect builders on slack)
    // Uptime
    // Network name
    // GetValidatorChanges
    // GetTrie
}

impl ToBytes for NonPersistedDataRequest {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut buffer = bytesrepr::allocate_buffer(self)?;
        self.write_bytes(&mut buffer)?;
        Ok(buffer)
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), bytesrepr::Error> {
        match self {
            NonPersistedDataRequest::BlockHeight2Hash { height } => {
                BLOCK_HEIGHT_2_HASH_TAG.write_bytes(writer)?;
                height.write_bytes(writer)
            }
            NonPersistedDataRequest::HighestCompleteBlock => HIGHEST_BLOCK_TAG.write_bytes(writer),
            NonPersistedDataRequest::CompletedBlockContains { block_hash } => {
                COMPLETED_BLOCK_CONTAINS_TAG.write_bytes(writer)?;
                block_hash.write_bytes(writer)
            }
            NonPersistedDataRequest::TransactionHash2BlockHashAndHeight { transaction_hash } => {
                TRANSACTION_HASH_2_BLOCK_HASH_AND_HEIGHT_TAG.write_bytes(writer)?;
                transaction_hash.write_bytes(writer)
            }
            NonPersistedDataRequest::Peers => PEERS_TAG.write_bytes(writer),
        }
    }

    fn serialized_length(&self) -> usize {
        U8_SERIALIZED_LENGTH
            + match self {
                NonPersistedDataRequest::BlockHeight2Hash { height } => height.serialized_length(),
                NonPersistedDataRequest::HighestCompleteBlock => 0,
                NonPersistedDataRequest::CompletedBlockContains { block_hash } => {
                    block_hash.serialized_length()
                }
                NonPersistedDataRequest::TransactionHash2BlockHashAndHeight {
                    transaction_hash,
                } => transaction_hash.serialized_length(),
                NonPersistedDataRequest::Peers => 0,
            }
    }
}

impl FromBytes for NonPersistedDataRequest {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (tag, remainder) = u8::from_bytes(bytes)?;
        match tag {
            BLOCK_HEIGHT_2_HASH_TAG => {
                let (height, remainder) = u64::from_bytes(remainder)?;
                Ok((
                    NonPersistedDataRequest::BlockHeight2Hash { height },
                    remainder,
                ))
            }
            HIGHEST_BLOCK_TAG => Ok((NonPersistedDataRequest::HighestCompleteBlock, remainder)),
            COMPLETED_BLOCK_CONTAINS_TAG => {
                let (block_hash, remainder) = BlockHash::from_bytes(remainder)?;
                Ok((
                    NonPersistedDataRequest::CompletedBlockContains { block_hash },
                    remainder,
                ))
            }
            TRANSACTION_HASH_2_BLOCK_HASH_AND_HEIGHT_TAG => {
                let (transaction_hash, remainder) = TransactionHash::from_bytes(remainder)?;
                Ok((
                    NonPersistedDataRequest::TransactionHash2BlockHashAndHeight {
                        transaction_hash,
                    },
                    remainder,
                ))
            }
            PEERS_TAG => Ok((NonPersistedDataRequest::Peers, remainder)),
            _ => Err(bytesrepr::Error::Formatting),
        }
    }
}

/// Response to the request for non persistent data.
#[derive(Debug)]
pub enum NonPersistedDataResponse {
    /// Returns hash for a given height.
    BlockHeight2Hash {
        /// Block hash.
        hash: BlockHash,
    },
    /// Returns height&hash for the currently highest block.
    HighestBlock {
        /// Block hash.
        hash: BlockHash,
        /// Block height.
        height: u64,
    },
    /// Returns true if `self.completed_blocks.highest_sequence()` contains the given hash
    CompletedBlockContains(bool),
    /// Block height and hash for a given transaction.
    TransactionHash2BlockHashAndHeight {
        /// Block hash.
        hash: BlockHash,
        /// Block height.
        height: u64,
    },
}

impl ToBytes for NonPersistedDataResponse {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut buffer = bytesrepr::allocate_buffer(self)?;
        self.write_bytes(&mut buffer)?;
        Ok(buffer)
    }

    fn write_bytes(&self, writer: &mut Vec<u8>) -> Result<(), bytesrepr::Error> {
        match self {
            NonPersistedDataResponse::BlockHeight2Hash { hash } => {
                BLOCK_HEIGHT_2_HASH_TAG.write_bytes(writer)?;
                hash.write_bytes(writer)
            }
            NonPersistedDataResponse::HighestBlock { hash, height } => {
                HIGHEST_BLOCK_TAG.write_bytes(writer)?;
                hash.write_bytes(writer)?;
                height.write_bytes(writer)
            }
            NonPersistedDataResponse::CompletedBlockContains(val) => {
                COMPLETED_BLOCK_CONTAINS_TAG.write_bytes(writer)?;
                val.write_bytes(writer)
            }
            NonPersistedDataResponse::TransactionHash2BlockHashAndHeight { hash, height } => {
                TRANSACTION_HASH_2_BLOCK_HASH_AND_HEIGHT_TAG.write_bytes(writer)?;
                hash.write_bytes(writer)?;
                height.write_bytes(writer)
            }
        }
    }

    fn serialized_length(&self) -> usize {
        U8_SERIALIZED_LENGTH
            + match self {
                NonPersistedDataResponse::BlockHeight2Hash { hash } => hash.serialized_length(),
                NonPersistedDataResponse::HighestBlock { hash, height } => {
                    hash.serialized_length() + height.serialized_length()
                }
                NonPersistedDataResponse::CompletedBlockContains(val) => val.serialized_length(),
                NonPersistedDataResponse::TransactionHash2BlockHashAndHeight { hash, height } => {
                    hash.serialized_length() + height.serialized_length()
                }
            }
    }
}

impl FromBytes for NonPersistedDataResponse {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (tag, remainder) = u8::from_bytes(bytes)?;
        match tag {
            BLOCK_HEIGHT_2_HASH_TAG => {
                let (hash, remainder) = BlockHash::from_bytes(remainder)?;
                Ok((
                    NonPersistedDataResponse::BlockHeight2Hash { hash },
                    remainder,
                ))
            }
            HIGHEST_BLOCK_TAG => {
                let (hash, remainder) = BlockHash::from_bytes(remainder)?;
                let (height, remainder) = u64::from_bytes(remainder)?;
                Ok((
                    NonPersistedDataResponse::HighestBlock { hash, height },
                    remainder,
                ))
            }
            COMPLETED_BLOCK_CONTAINS_TAG => {
                let (val, remainder) = bool::from_bytes(remainder)?;
                Ok((
                    NonPersistedDataResponse::CompletedBlockContains(val),
                    remainder,
                ))
            }
            TRANSACTION_HASH_2_BLOCK_HASH_AND_HEIGHT_TAG => {
                let (hash, remainder) = BlockHash::from_bytes(remainder)?;
                let (height, remainder) = u64::from_bytes(remainder)?;
                Ok((
                    NonPersistedDataResponse::TransactionHash2BlockHashAndHeight { hash, height },
                    remainder,
                ))
            }
            _ => Err(bytesrepr::Error::Formatting),
        }
    }
}

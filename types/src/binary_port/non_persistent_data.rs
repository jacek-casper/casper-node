//! The "request for non persistent data" variant of the request to the binary port.

use crate::{
    bytesrepr::{self, FromBytes, ToBytes, U8_SERIALIZED_LENGTH},
    BlockHash, TransactionHash,
};

const BLOCK_HEIGHT_2_HASH_TAG: u8 = 0;
const HIGHEST_COMPLETE_BLOCK_TAG: u8 = 1;
const COMPLETED_BLOCK_CONTAINS_TAG: u8 = 2;
const TRANSACTION_HASH_2_BLOCK_HASH_AND_HEIGHT_TAG: u8 = 3;
const PEERS_TAG: u8 = 4;
const UPTIME_TAG: u8 = 5;
const LAST_PROGRESS_TAG: u8 = 6;
const REACTOR_STATE_TAG: u8 = 7;
const NETWORK_NAME_TAG: u8 = 8;
const CONSENSUS_VALIDATOR_CHANGES_TAG: u8 = 9;
const BLOCK_SYNCHRONIZER_STATUS_TAG: u8 = 10;
const AVAILABLE_BLOCK_RANGE_TAG: u8 = 11;
const NEXT_UPGRADE_TAG: u8 = 12;
const CONSENSUS_STATUS_TAG: u8 = 13;
const CHAINSPEC_RAW_BYTES: u8 = 14;
const GENESIS_ACCOUNTS_BYTES_TAG: u8 = 15;
const GLOBAL_STATE_BYTES_TAG: u8 = 16;

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
    CompletedBlocksContain {
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
    /// Returns node uptime.
    Uptime,
    /// Returns last progress of the sync process.
    LastProgress,
    /// Returns current state of the main reactor.
    ReactorState,
    /// Returns current network name.
    // TODO[RC]: Consider "generic" get chainspec param? Or just "get_chainspec"?
    NetworkName,
    /// Returns consensus validator changes.
    ConsensusValidatorChanges,
    /// Returns status of the BlockSynchronizer.
    BlockSynchronizerStatus,
    /// Returns the available block range.
    AvailableBlockRange,
    /// Returns info about next upgrade.
    NextUpgrade,
    /// Returns consensus status.
    ConsensusStatus,
    /// Returns chainspec raw bytes.
    ChainspecRawBytes,
    /// Returns genesis accounts raw bytes.
    GenesisAccountsBytes,
    /// Returns global state raw bytes.
    GlobalStateBytes,
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
            NonPersistedDataRequest::HighestCompleteBlock => {
                HIGHEST_COMPLETE_BLOCK_TAG.write_bytes(writer)
            }
            NonPersistedDataRequest::CompletedBlocksContain { block_hash } => {
                COMPLETED_BLOCK_CONTAINS_TAG.write_bytes(writer)?;
                block_hash.write_bytes(writer)
            }
            NonPersistedDataRequest::TransactionHash2BlockHashAndHeight { transaction_hash } => {
                TRANSACTION_HASH_2_BLOCK_HASH_AND_HEIGHT_TAG.write_bytes(writer)?;
                transaction_hash.write_bytes(writer)
            }
            NonPersistedDataRequest::Peers => PEERS_TAG.write_bytes(writer),
            NonPersistedDataRequest::Uptime => UPTIME_TAG.write_bytes(writer),
            NonPersistedDataRequest::LastProgress => LAST_PROGRESS_TAG.write_bytes(writer),
            NonPersistedDataRequest::ReactorState => REACTOR_STATE_TAG.write_bytes(writer),
            NonPersistedDataRequest::NetworkName => NETWORK_NAME_TAG.write_bytes(writer),
            NonPersistedDataRequest::ConsensusValidatorChanges => {
                CONSENSUS_VALIDATOR_CHANGES_TAG.write_bytes(writer)
            }
            NonPersistedDataRequest::BlockSynchronizerStatus => {
                BLOCK_SYNCHRONIZER_STATUS_TAG.write_bytes(writer)
            }
            NonPersistedDataRequest::AvailableBlockRange => {
                AVAILABLE_BLOCK_RANGE_TAG.write_bytes(writer)
            }
            NonPersistedDataRequest::NextUpgrade => NEXT_UPGRADE_TAG.write_bytes(writer),
            NonPersistedDataRequest::ConsensusStatus => CONSENSUS_STATUS_TAG.write_bytes(writer),
            NonPersistedDataRequest::ChainspecRawBytes => CHAINSPEC_RAW_BYTES.write_bytes(writer),
            NonPersistedDataRequest::GenesisAccountsBytes => {
                GENESIS_ACCOUNTS_BYTES_TAG.write_bytes(writer)
            }
            NonPersistedDataRequest::GlobalStateBytes => GLOBAL_STATE_BYTES_TAG.write_bytes(writer),
        }
    }

    fn serialized_length(&self) -> usize {
        U8_SERIALIZED_LENGTH
            + match self {
                NonPersistedDataRequest::BlockHeight2Hash { height } => height.serialized_length(),
                NonPersistedDataRequest::HighestCompleteBlock => 0,
                NonPersistedDataRequest::CompletedBlocksContain { block_hash } => {
                    block_hash.serialized_length()
                }
                NonPersistedDataRequest::TransactionHash2BlockHashAndHeight {
                    transaction_hash,
                } => transaction_hash.serialized_length(),
                NonPersistedDataRequest::Peers => 0,
                NonPersistedDataRequest::Uptime => 0,
                NonPersistedDataRequest::LastProgress => 0,
                NonPersistedDataRequest::ReactorState => 0,
                NonPersistedDataRequest::NetworkName => 0,
                NonPersistedDataRequest::ConsensusValidatorChanges => 0,
                NonPersistedDataRequest::BlockSynchronizerStatus => 0,
                NonPersistedDataRequest::AvailableBlockRange => 0,
                NonPersistedDataRequest::NextUpgrade => 0,
                NonPersistedDataRequest::ConsensusStatus => 0,
                NonPersistedDataRequest::ChainspecRawBytes => 0,
                NonPersistedDataRequest::GenesisAccountsBytes => 0,
                NonPersistedDataRequest::GlobalStateBytes => 0,
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
            HIGHEST_COMPLETE_BLOCK_TAG => {
                Ok((NonPersistedDataRequest::HighestCompleteBlock, remainder))
            }
            COMPLETED_BLOCK_CONTAINS_TAG => {
                let (block_hash, remainder) = BlockHash::from_bytes(remainder)?;
                Ok((
                    NonPersistedDataRequest::CompletedBlocksContain { block_hash },
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
            UPTIME_TAG => Ok((NonPersistedDataRequest::Uptime, remainder)),
            LAST_PROGRESS_TAG => Ok((NonPersistedDataRequest::LastProgress, remainder)),
            REACTOR_STATE_TAG => Ok((NonPersistedDataRequest::ReactorState, remainder)),
            NETWORK_NAME_TAG => Ok((NonPersistedDataRequest::NetworkName, remainder)),
            CONSENSUS_VALIDATOR_CHANGES_TAG => Ok((
                NonPersistedDataRequest::ConsensusValidatorChanges,
                remainder,
            )),
            BLOCK_SYNCHRONIZER_STATUS_TAG => {
                Ok((NonPersistedDataRequest::BlockSynchronizerStatus, remainder))
            }
            AVAILABLE_BLOCK_RANGE_TAG => {
                Ok((NonPersistedDataRequest::AvailableBlockRange, remainder))
            }
            NEXT_UPGRADE_TAG => Ok((NonPersistedDataRequest::NextUpgrade, remainder)),
            CONSENSUS_STATUS_TAG => Ok((NonPersistedDataRequest::ConsensusStatus, remainder)),
            CHAINSPEC_RAW_BYTES => Ok((NonPersistedDataRequest::ChainspecRawBytes, remainder)),
            GENESIS_ACCOUNTS_BYTES_TAG => {
                Ok((NonPersistedDataRequest::GenesisAccountsBytes, remainder))
            }
            GLOBAL_STATE_BYTES_TAG => Ok((NonPersistedDataRequest::GlobalStateBytes, remainder)),
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
                HIGHEST_COMPLETE_BLOCK_TAG.write_bytes(writer)?;
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
            HIGHEST_COMPLETE_BLOCK_TAG => {
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

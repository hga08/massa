// Copyright (c) 2022 MASSA LABS <info@massa.net>

use massa_time::MassaTime;
use serde::Deserialize;
/// Dynamic protocol configuration mix in static settings and constants configurations.
#[derive(Debug, Deserialize, Clone, Copy)]
pub struct ProtocolConfig {
    /// running threads count
    pub thread_count: u8,
    /// after `ask_block_timeout` milliseconds we try to ask a block to another node
    pub ask_block_timeout: MassaTime,
    /// max known blocks of current nodes we keep in memory (by node)
    pub max_known_blocks_size: usize,
    /// max known blocks of foreign nodes we keep in memory (by node)
    pub max_node_known_blocks_size: usize,
    /// max wanted blocks per node kept in memory
    pub max_node_wanted_blocks_size: usize,
    /// max known operations current node kept in memory
    pub max_known_ops_size: usize,
    /// max known operations of foreign nodes we keep in memory (by node)
    pub max_node_known_ops_size: usize,
    /// max known endorsements by our node that we kept in memory
    pub max_known_endorsements_size: usize,
    /// max known endorsements of foreign nodes we keep in memory (by node)
    pub max_node_known_endorsements_size: usize,
    /// we ask for the same block `max_simultaneous_ask_blocks_per_node` times at the same time
    pub max_simultaneous_ask_blocks_per_node: usize,
    /// Max wait time for sending a Network or Node event.
    pub max_send_wait: MassaTime,
    /// Maximum number of batches in the memory buffer.
    /// Dismiss the new batches if overflow
    pub operation_batch_buffer_capacity: usize,
    /// Maximum number of operations in the announcement buffer.
    /// Immediately announce if overflow.
    pub operation_announcement_buffer_capacity: usize,
    /// Start processing batches in the buffer each `operation_batch_proc_period` in millisecond
    pub operation_batch_proc_period: MassaTime,
    /// All operations asked are prune each `operation_asked_pruning_period` millisecond
    pub asked_operations_pruning_period: MassaTime,
    /// Interval at which operations are announced in batches.
    pub operation_announcement_interval: MassaTime,
    /// Maximum of operations sent in one message.
    pub max_operations_per_message: u64,
    /// Maximum size in bytes of all serialized operations size in a block
    pub max_serialized_operations_size_per_block: usize,
    /// Maximum operations in a block
    pub max_operations_per_block: u32,
    /// Controller channel size
    pub controller_channel_size: usize,
    /// Event channel size
    pub event_channel_size: usize,
    /// t0
    pub t0: MassaTime,
    /// Genesis timestamp
    pub genesis_timestamp: MassaTime,
    /// max time we propagate operations
    pub max_operations_propagation_time: MassaTime,
    /// max time we propagate endorsements
    pub max_endorsements_propagation_time: MassaTime,
}

use anyhow::Result;
use futures_util::TryStreamExt;
use universaldb::{
	RangeOption,
	options::StreamingMode,
	utils::{IsolationLevel::Snapshot, end_of_key_range},
};

use crate::conveyer::keys;

use super::plan::{ReadSource, StorageScope};

pub(super) async fn tx_load_delta_blob(
	tx: &universaldb::Transaction,
	delta_prefix: &[u8],
) -> Result<Option<Vec<u8>>> {
	let delta_chunks = super::tx::tx_scan_prefix_values(tx, delta_prefix).await?;
	if delta_chunks.is_empty() {
		return Ok(None);
	}

	let mut delta_blob = Vec::new();
	for (_, chunk) in delta_chunks {
		delta_blob.extend_from_slice(&chunk);
	}

	Ok(Some(delta_blob))
}

pub(super) async fn tx_load_latest_shard_blob(
	tx: &universaldb::Transaction,
	scope: &StorageScope,
	shard_id: u32,
) -> Result<Option<(Vec<u8>, Vec<u8>)>> {
	let sources = match scope {
		StorageScope::Branch(plan) => plan.sources.clone(),
	};

	for source in sources {
		let as_of_txid = match source {
			ReadSource::Branch(source) => source.max_txid,
		};
		let prefix = match source {
			ReadSource::Branch(source) => {
				keys::branch_shard_version_prefix(source.branch_id, shard_id)
			}
		};
		let end_key = match source {
			ReadSource::Branch(source) => {
				keys::branch_shard_key(source.branch_id, shard_id, as_of_txid)
			}
		};
		let end = end_of_key_range(&end_key);
		let informal = tx.informal();
		let mut stream = informal.get_ranges_keyvalues(
			RangeOption {
				mode: StreamingMode::WantAll,
				..(prefix.as_slice(), end.as_slice()).into()
			},
			Snapshot,
		);

		let mut latest = None;
		while let Some(entry) = stream.try_next().await? {
			latest = Some((entry.key().to_vec(), entry.value().to_vec()));
		}

		if latest.is_some() {
			return Ok(latest);
		}
	}

	Ok(None)
}

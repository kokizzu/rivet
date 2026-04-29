use crate::compaction::{
	*,
	companion::{CompanionKind, run_companion_loop},
	shared::*,
};
use crate::workflows::db_manager::branch_record_is_live_at_generation;

#[workflow(DbColdCompacterWorkflow)]
pub async fn db_cold_compacter(ctx: &mut WorkflowCtx, input: &DbColdCompacterInput) -> Result<()> {
	run_companion_loop(ctx, input.database_branch_id, CompanionKind::Cold).await
}

#[activity(UploadColdJob)]
pub async fn upload_cold_job(
	ctx: &ActivityCtx,
	input: &UploadColdJobInput,
) -> Result<UploadColdJobOutput> {
	let input = input.clone();
	let input_for_tx = input.clone();
	let upload = ctx
		.udb()?
		.run(move |tx| {
			let input = input_for_tx.clone();
			async move { prepare_cold_upload_tx(&tx, &input).await }
		})
		.await?;

	let PreparedColdUpload {
		status,
		output_refs,
		objects,
	} = upload;
	if !matches!(status, CompactionJobStatus::Succeeded) {
		return Ok(UploadColdJobOutput {
			status,
			output_refs: Vec::new(),
		});
	}

	let Some(cold_tier) = workflow_cold_tier(ctx.config()).await? else {
		return Ok(UploadColdJobOutput {
			status: CompactionJobStatus::Rejected {
				reason: "cold storage is disabled".to_string(),
			},
			output_refs: Vec::new(),
		});
	};
	for object in objects {
		if let Err(err) = cold_tier.put_object(&object.object_key, &object.bytes).await {
			tracing::warn!(
				?input.database_branch_id,
				?input.job_id,
				object_key = object.object_key.as_str(),
				error = ?err,
				"sqlite workflow cold shard upload failed"
			);
			return Ok(UploadColdJobOutput {
				status: CompactionJobStatus::Rejected {
					reason: "cold shard upload failed".to_string(),
				},
				output_refs: Vec::new(),
			});
		}
	}

	Ok(UploadColdJobOutput {
		status,
		output_refs,
	})
}

#[derive(Debug)]
struct PreparedColdUpload {
	status: CompactionJobStatus,
	output_refs: Vec<ColdShardRef>,
	objects: Vec<ColdUploadObject>,
}

#[derive(Debug)]
struct ColdUploadObject {
	object_key: String,
	bytes: Vec<u8>,
}

async fn prepare_cold_upload_tx(
	tx: &universaldb::Transaction,
	input: &UploadColdJobInput,
) -> Result<PreparedColdUpload> {
	if input.job_kind != CompactionJobKind::Cold {
		return Ok(rejected_cold_upload("cold compacter received a non-cold job"));
	}

	let branch_record = tx_get_value(
		tx,
		&keys::branches_list_key(input.database_branch_id),
		Serializable,
	)
	.await?
	.as_deref()
	.map(decode_database_branch_record)
	.transpose()
	.context("decode sqlite database branch record for cold upload")?;
	if !branch_record_is_live_at_generation(
		branch_record.as_ref(),
		input.base_lifecycle_generation,
	) {
		return Ok(rejected_cold_upload("database branch lifecycle changed"));
	}

	let root = tx_get_value(
		tx,
		&keys::branch_compaction_root_key(input.database_branch_id),
		Serializable,
	)
	.await?
	.as_deref()
	.map(decode_compaction_root)
	.transpose()
	.context("decode sqlite compaction root for cold upload")?
	.unwrap_or(CompactionRoot {
		schema_version: 1,
		manifest_generation: 0,
		hot_watermark_txid: 0,
		cold_watermark_txid: 0,
		cold_watermark_versionstamp: [0; 16],
	});
	if root.manifest_generation != input.base_manifest_generation {
		return Ok(rejected_cold_upload("base manifest generation changed"));
	}

	let cold_inputs =
		read_cold_input_snapshot(tx, input.database_branch_id, &root, Serializable).await?;
	if cold_inputs.shard_blobs.is_empty() {
		return Ok(rejected_cold_upload("no cold shard input is available"));
	}
	if cold_inputs.min_versionstamp != input.input_range.min_versionstamp
		|| cold_inputs.max_versionstamp != input.input_range.max_versionstamp
	{
		return Ok(rejected_cold_upload("cold compaction versionstamp bounds changed"));
	}
	let input_fingerprint =
		fingerprint_cold_inputs(input.database_branch_id, &root, &cold_inputs);
	if input_fingerprint != input.input_fingerprint {
		return Ok(rejected_cold_upload("cold compaction input fingerprint changed"));
	}

	let mut output_refs = Vec::with_capacity(cold_inputs.shard_blobs.len());
	let mut objects = Vec::with_capacity(cold_inputs.shard_blobs.len());
	let publish_generation = root.manifest_generation.saturating_add(1);
	for blob in cold_inputs.shard_blobs {
		if blob.as_of_txid != input.input_range.txids.max_txid {
			return Ok(rejected_cold_upload("cold shard txid does not match planned range"));
		}
		let content_hash = content_hash(&blob.bytes);
		let object_key = cold_shard_object_key(
			input.database_branch_id,
			blob.shard_id,
			blob.as_of_txid,
			input.job_id,
			content_hash,
		);
		output_refs.push(ColdShardRef {
			object_key: object_key.clone(),
			object_generation_id: input.job_id,
			shard_id: blob.shard_id,
			as_of_txid: blob.as_of_txid,
			min_txid: input.input_range.txids.min_txid,
			max_txid: blob.as_of_txid,
			min_versionstamp: input.input_range.min_versionstamp,
			max_versionstamp: input.input_range.max_versionstamp,
			size_bytes: u64::try_from(blob.bytes.len()).unwrap_or(u64::MAX),
			content_hash,
			publish_generation,
		});
		objects.push(ColdUploadObject {
			object_key,
			bytes: blob.bytes,
		});
	}

	Ok(PreparedColdUpload {
		status: CompactionJobStatus::Succeeded,
		output_refs,
		objects,
	})
}

fn rejected_cold_upload(reason: impl Into<String>) -> PreparedColdUpload {
	PreparedColdUpload {
		status: CompactionJobStatus::Rejected {
			reason: reason.into(),
		},
		output_refs: Vec::new(),
		objects: Vec::new(),
	}
}

#[activity(PublishColdJob)]
pub async fn publish_cold_job(
	ctx: &ActivityCtx,
	input: &PublishColdJobInput,
) -> Result<PublishColdJobOutput> {
	let input = input.clone();

	ctx.udb()?
		.run(move |tx| {
			let input = input.clone();
			async move { publish_cold_job_tx(&tx, &input).await }
		})
		.await
}

async fn publish_cold_job_tx(
	tx: &universaldb::Transaction,
	input: &PublishColdJobInput,
) -> Result<PublishColdJobOutput> {
	if input.job_kind != CompactionJobKind::Cold {
		return Ok(rejected_cold_publish("manager received a non-cold job"));
	}

	let branch_record = tx_get_value(
		tx,
		&keys::branches_list_key(input.database_branch_id),
		Serializable,
	)
	.await?
	.as_deref()
	.map(decode_database_branch_record)
	.transpose()
	.context("decode sqlite database branch record for cold publish")?;
	if !branch_record_is_live_at_generation(
		branch_record.as_ref(),
		input.base_lifecycle_generation,
	) {
		return Ok(rejected_cold_publish("database branch lifecycle changed"));
	}

	let root = tx_get_value(
		tx,
		&keys::branch_compaction_root_key(input.database_branch_id),
		Serializable,
	)
	.await?
	.as_deref()
	.map(decode_compaction_root)
	.transpose()
	.context("decode sqlite compaction root for cold publish")?
	.unwrap_or(CompactionRoot {
		schema_version: 1,
		manifest_generation: 0,
		hot_watermark_txid: 0,
		cold_watermark_txid: 0,
		cold_watermark_versionstamp: [0; 16],
	});
	if root.manifest_generation != input.base_manifest_generation {
		return Ok(rejected_cold_publish("base manifest generation changed"));
	}

	let mut db_pins =
		history_pin::read_db_history_pins(tx, input.database_branch_id, Serializable).await?;
	if resolve_bucket_fork_pins(tx, input.database_branch_id, &mut db_pins).await? {
		return Ok(rejected_cold_publish("bucket fork proof is ambiguous"));
	}

	let cold_inputs =
		read_cold_input_snapshot(tx, input.database_branch_id, &root, Serializable).await?;
	if cold_inputs.shard_blobs.is_empty() {
		return Ok(rejected_cold_publish("no cold shard input is available"));
	}
	if cold_inputs.min_versionstamp != input.input_range.min_versionstamp
		|| cold_inputs.max_versionstamp != input.input_range.max_versionstamp
	{
		return Ok(rejected_cold_publish("cold compaction versionstamp bounds changed"));
	}
	let input_fingerprint =
		fingerprint_cold_inputs(input.database_branch_id, &root, &cold_inputs);
	if input_fingerprint != input.input_fingerprint {
		return Ok(rejected_cold_publish("cold compaction input fingerprint changed"));
	}

	let publish_generation = root.manifest_generation.saturating_add(1);
	let expected_outputs = expected_cold_output_refs(input, &cold_inputs, publish_generation);
	if expected_outputs != input.output_refs {
		return Ok(rejected_cold_publish("cold output refs do not match planned inputs"));
	}

	let mut seen_outputs = BTreeSet::new();
	for output_ref in &input.output_refs {
		if !seen_outputs.insert((output_ref.shard_id, output_ref.as_of_txid)) {
			return Ok(rejected_cold_publish("duplicate cold shard output ref"));
		}
		if tx_get_value(
			tx,
			&keys::branch_compaction_retired_cold_object_key(
				input.database_branch_id,
				content_hash(output_ref.object_key.as_bytes()),
			),
			Serializable,
		)
		.await?
		.is_some()
		{
			return Ok(rejected_cold_publish("cold object was already retired"));
		}
	}

	for output_ref in &input.output_refs {
		tx.informal().set(
			&keys::branch_compaction_cold_shard_key(
				input.database_branch_id,
				output_ref.shard_id,
				output_ref.as_of_txid,
			),
			&encode_cold_shard_ref(output_ref.clone())
				.context("encode sqlite cold shard ref for cold publish")?,
		);
	}

	let next_root = CompactionRoot {
		schema_version: root.schema_version,
		manifest_generation: publish_generation,
		hot_watermark_txid: root.hot_watermark_txid,
		cold_watermark_txid: root
			.cold_watermark_txid
			.max(input.input_range.txids.max_txid),
		cold_watermark_versionstamp: root
			.cold_watermark_versionstamp
			.max(input.input_range.max_versionstamp),
	};
	tx.informal().set(
		&keys::branch_compaction_root_key(input.database_branch_id),
		&encode_compaction_root(next_root)
			.context("encode sqlite compaction root for cold publish")?,
	);

	Ok(PublishColdJobOutput {
		status: CompactionJobStatus::Succeeded,
		output_refs: input.output_refs.clone(),
	})
}

fn rejected_cold_publish(reason: impl Into<String>) -> PublishColdJobOutput {
	PublishColdJobOutput {
		status: CompactionJobStatus::Rejected {
			reason: reason.into(),
		},
		output_refs: Vec::new(),
	}
}

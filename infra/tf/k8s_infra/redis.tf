locals {
	redis_k8s = var.redis_provider == "kubernetes"
	service_redis = lookup(var.services, "redis", {
		count = 3
		resources = {
			cpu = 1000
			memory = 2000
		}
	})

	redis_svcs = local.redis_k8s ? {
		"persistent" = {
			persistent = true
			node_config = {
				priorityClassName = kubernetes_priority_class.stateful_priority.metadata.0.name
				resources = var.limit_resources ? {
					limits = {
						memory = "${local.service_redis.resources.memory}Mi"
						cpu = "${local.service_redis.resources.cpu}m"
					}
				} : null
				persistence = {
					enabled = true
				}
			}
		}
		"ephemeral" = {
			persistent = false
			node_config = {
				priorityClassName = kubernetes_priority_class.stateful_priority.metadata.0.name
				resources = var.limit_resources ? {
					limits = {
						memory = "${local.service_redis.resources.memory}Mi"
						cpu = "${local.service_redis.resources.cpu}m"
					}
				} : null
				persistence = {
					enabled = true
				}
			}
		}
	} : {}
}

module "redis_secrets" {
	count = local.redis_k8s ? 1 : 0

	source = "../modules/secrets"

	keys = [
		for k, v in local.redis_svcs: "redis/${k}/password"
	]
}

resource "kubernetes_namespace" "redis" {
	depends_on = [helm_release.prometheus]
	for_each = local.redis_svcs

	metadata {
		name = "redis-${each.key}"
	}
}

resource "helm_release" "redis" {
	depends_on = [null_resource.daemons, null_resource.wait_for_service_monitors]
	for_each = local.redis_svcs

	name = "redis"
	namespace = kubernetes_namespace.redis[each.key].metadata.0.name
	chart = "../../helm/redis"
	# repository = "https://charts.bitnami.com/bitnami"
	# chart = "redis"
	# version = "18.4.0"
	values = [yamlencode({
		architecture = "replication"

		commonConfiguration = <<EOF
		# Enable AOF https://redis.io/topics/persistence#append-only-file
		appendonly ${each.value.persistent ? "yes" : "no"}

		# Disable RDB persistence. AOF persistence already enabled on persistent,
		# not needed on ephemeral.
		save ""

		# Use allkeys-lru instead of volatile-lru because we don't want the cache nodes to crash if they run out of memory
		maxmemory-policy ${each.value.persistent ? "noeviction" : "allkeys-lru"}
		EOF

		global = {
			storageClass = var.k8s_storage_class
			redis = {
				password = module.redis_secrets[0].values["redis/${each.key}/password"]
			}
		}

		tls = {
			enabled = true
			authClients = false
			autoGenerated = true
		}

		master = merge(each.value.node_config, {
			count = 1
		})
		replica = merge(each.value.node_config, {
			replicaCount = 1
		})
		sentinel = {
			enabled = true
		}

		metrics = {
			enabled = true
			serviceMonitor = {
				enabled = var.prometheus_enabled
				namespace = kubernetes_namespace.redis[each.key].metadata.0.name
			}
			extraArgs = each.key == "chirp" ? {
				"check-streams" = "'{topic:*}:topic'"
			} : {}

			# TODO:
			# prometheusRule = {
			# 	enabled = true
			# 	namespace = kubernetes_namespace.redis[each.key].metadata.0.name
			# }
		}
	})]
}

data "kubernetes_secret" "redis_ca" {
	for_each = local.redis_k8s ? local.redis_svcs : {}

	depends_on = [helm_release.redis]

	metadata {
		name = "redis-crt"
		namespace = kubernetes_namespace.redis[each.key].metadata.0.name
	}
}

resource "kubernetes_config_map" "redis_ca" {
	for_each = local.redis_k8s ? merge([
		for ns in ["rivet-service", "bolt"]: {
			for k, v in local.redis_svcs:
				"${k}-${ns}" => {
				db = k
				namespace = ns
			}
		}
	]...) : {}

	metadata {
		name = "redis-${each.value.db}-ca"
		namespace = each.value.namespace
	}

	data = {
		"ca.crt" = data.kubernetes_secret.redis_ca[each.value.db].data["ca.crt"]
	}
}


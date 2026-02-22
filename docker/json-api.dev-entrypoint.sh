#!/usr/bin/env bash
set -euo pipefail

admin_url="${DATABASE_ADMIN_URL_DOCKER:-${DATABASE_ADMIN_URL:-postgresql://${POSTGRES_USER:-lattice_user}:${POSTGRES_PASSWORD:-lattice_password}@postgres:5432/${POSTGRES_DB:-lattice_db}}}"
app_url="${DATABASE_URL_DOCKER:-${DATABASE_URL:-postgresql://${APP_DB_USER:-lattice_app}:${APP_DB_PASSWORD:-lattice_app_password}@postgres:5432/${POSTGRES_DB:-lattice_db}}}"
app_role="${APP_DB_USER:-lattice_app}"
app_password="${APP_DB_PASSWORD:-lattice_app_password}"
openbao_addr="${OPENBAO_ADDR:-}"
openbao_token="${OPENBAO_TOKEN:-}"
openbao_transit_key="${OPENBAO_TRANSIT_KEY:-lattice-api-tokens}"
dev_tenant_name="${DEV_TENANT_NAME:-Dev Tenant}"
dev_tenant_uuid="${DEV_TENANT_UUID:-00000000-0000-0000-0000-000000000001}"
dev_api_token_file="${DEV_API_TOKEN_FILE:-/app/.dev-api-token}"
dev_api_token=""

wait_for_openbao() {
  local retries=60
  local attempt=0

  while [[ ${attempt} -lt ${retries} ]]; do
    if curl --silent --show-error --output /dev/null "${openbao_addr}/v1/sys/health"; then
      return 0
    fi

    attempt=$((attempt + 1))
    sleep 1
  done

  return 1
}

openbao_enable_transit() {
  curl \
    --silent \
    --output /dev/null \
    --header "X-Bao-Token: ${openbao_token}" \
    --header "X-Vault-Token: ${openbao_token}" \
    --header "Content-Type: application/json" \
    --request POST \
    --data '{"type":"transit"}' \
    "${openbao_addr}/v1/sys/mounts/transit" || true
}

openbao_ensure_transit_key() {
  curl \
    --silent \
    --output /dev/null \
    --header "X-Bao-Token: ${openbao_token}" \
    --header "X-Vault-Token: ${openbao_token}" \
    --header "Content-Type: application/json" \
    --request POST \
    --data '{}' \
    "${openbao_addr}/v1/transit/keys/${openbao_transit_key}" || true
}

if [[ -n "${openbao_addr}" && -n "${openbao_token}" ]]; then
  echo "Waiting for OpenBao at ${openbao_addr}..."

  if ! wait_for_openbao; then
    echo "OpenBao did not become ready in time." >&2
    exit 1
  fi

  echo "Enabling transit secrets engine..."
  openbao_enable_transit

  echo "Ensuring transit key '${openbao_transit_key}' exists..."
  openbao_ensure_transit_key
else
  echo "OpenBao not configured (OPENBAO_ADDR or OPENBAO_TOKEN is unset)." >&2
  exit 1
fi

echo "Running database migrations..."
DATABASE_URL="${admin_url}" sqlx migrate run --ignore-missing

echo "Ensuring runtime role '${app_role}' exists with RLS-safe privileges..."
cargo run --quiet --package lattice-app -- \
  db ensure-app-role \
  --database-url "${admin_url}" \
  --role-name "${app_role}" \
  --password "${app_password}"

echo "Ensuring default dev tenant/token exists..."
set +e
tenant_create_output="$(
  cargo run --quiet --package lattice-app -- \
    tenant create \
    --database-url "${app_url}" \
    --name "${dev_tenant_name}" \
    --tenant-uuid "${dev_tenant_uuid}" 2>&1
)"
tenant_create_status=$?
set -e

if [[ ${tenant_create_status} -eq 0 ]]; then
  echo "${tenant_create_output}"
elif echo "${tenant_create_output}" | grep -qi "tenant already exists"; then
  echo "default dev tenant already exists."
else
  echo "${tenant_create_output}" >&2
  exit "${tenant_create_status}"
fi

if [[ -f "${dev_api_token_file}" ]]; then
  dev_api_token="$(head -n 1 "${dev_api_token_file}")"
  echo "reusing cached dev API token from ${dev_api_token_file}."
else
  set +e
  token_create_output="$(
    cargo run --quiet --package lattice-app -- \
      token create \
      --database-url "${app_url}" \
      --tenant-uuid "${dev_tenant_uuid}" \
      --openbao-addr "${openbao_addr}" \
      --openbao-token "${openbao_token}" \
      --openbao-transit-key "${openbao_transit_key}" 2>&1
  )"
  token_create_status=$?
  set -e

  if [[ ${token_create_status} -ne 0 ]]; then
    echo "${token_create_output}" >&2
    exit "${token_create_status}"
  fi

  echo "${token_create_output}"
  dev_api_token="$(printf '%s\n' "${token_create_output}" | sed -n 's/^api_token: //p' | tail -n 1)"

  if [[ -n "${dev_api_token}" ]]; then
    printf '%s\n' "${dev_api_token}" > "${dev_api_token_file}"
    chmod 600 "${dev_api_token_file}" || true
  fi
fi

echo "Starting json-api dev watcher..."
exec watchexec \
  --watch crates/core \
  --watch crates/json-api \
  --watch crates/app \
  --watch crates/json-api \
  --watch Cargo.toml \
  --watch Cargo.lock \
  --exts rs,toml,sql \
  --restart -- \
  cargo run --package lattice-json

echo "Swagger docs: http://localhost:8698/docs"
if [[ -n "${dev_api_token}" ]]; then
  echo "Use Authorization header: Bearer ${dev_api_token}"
fi

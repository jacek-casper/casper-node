#!/usr/bin/env bash
set -e
shopt -s expand_aliases

DRONE_ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." >/dev/null 2>&1 && pwd)"
SCENARIOS_DIR="$DRONE_ROOT_DIR/utils/nctl/sh/scenarios"
SCENARIOS_CHAINSPEC_DIR="$SCENARIOS_DIR/chainspecs"
SCENARIOS_ACCOUNTS_DIR="$SCENARIOS_DIR/accounts_toml"
SCENARIOS_CONFIGS_DIR="$SCENARIOS_DIR/configs"

NCTL_CLIENT_BRANCH="${DRONE_BRANCH:='dev'}"

# Activate Environment
pushd "$DRONE_ROOT_DIR"
source "$(pwd)"/utils/nctl/activate

# Call compile wrapper for client, launcher, and nctl-compile
bash -c "$DRONE_ROOT_DIR/ci/nctl_compile.sh"

function start_run_teardown() {
    local RUN_CMD=$1
    local TEST_NAME
    local STAGE_TOML_DIR
    local SETUP_ARGS
    local CONFIG_TOML
    local CHAINSPEC_TOML
    local ACCOUNTS_TOML

    # Capture test prefix for custom file checks
    TEST_NAME="$(echo $RUN_CMD | awk -F'.sh' '{ print $1 }')"
    STAGE_TOML_DIR="$NCTL/overrides"
    CONFIG_TOML="$STAGE_TOML_DIR/$TEST_NAME.config.toml"
    CHAINSPEC_TOML="$STAGE_TOML_DIR/$TEST_NAME.chainspec.toml.in"
    ACCOUNTS_TOML="$STAGE_TOML_DIR/$TEST_NAME.accounts.toml"

    # Really-really make sure nothing is leftover
    nctl-assets-teardown

    # Overrides chainspec.toml
    if [ -f "$CHAINSPEC_TOML" ]; then
        SETUP_ARGS+=("chainspec_path=$CHAINSPEC_TOML")
    fi

    # Overrides accounts.toml
    if [ -f "$ACCOUNTS_TOML" ]; then
        SETUP_ARGS+=("accounts_path=$ACCOUNTS_TOML")
    fi

    # Overrides config.toml
    if [ -f "$CONFIG_TOML" ]; then
        SETUP_ARGS+=("config_path=$CONFIG_TOML")
    fi

    # Setup nctl files for test
    echo "Setting up network: nctl-assets-setup ${SETUP_ARGS[@]}"
    nctl-assets-setup "${SETUP_ARGS[@]}"
    sleep 1

    # Start nctl network
    nctl-start
    echo "Sleeping 10s to allow network startup"
    sleep 10

    # Run passed in test
    pushd "$SCENARIOS_DIR"
    echo "Starting scenario: $RUN_CMD"
    # Don't qoute the cmd
    source $RUN_CMD

    # Cleanup after test completion
    popd
    nctl-assets-teardown
    sleep 1
}

function run_nightly_upgrade_test() {
    # setup only needed the first time
    bash -c "./ci/nctl_upgrade.sh test_id=4"
    bash -c "./ci/nctl_upgrade.sh test_id=5 skip_setup=true"
    bash -c "./ci/nctl_upgrade.sh test_id=6 skip_setup=true"
    bash -c "./ci/nctl_upgrade.sh test_id=7 skip_setup=true"
    bash -c "./ci/nctl_upgrade.sh test_id=8 skip_setup=true"
    bash -c "./ci/nctl_upgrade.sh test_id=9 skip_setup=true"
    bash -c "./ci/nctl_upgrade.sh test_id=10"
    bash -c "./ci/nctl_upgrade.sh test_id=11"
    bash -c "./ci/nctl_upgrade.sh test_id=12"
    bash -c "./ci/nctl_upgrade.sh test_id=13"
    bash -c "./ci/nctl_upgrade.sh test_id=14"
}

function run_soundness_test() {
    echo "Starting network soundness test"

    # Really-really make sure nothing is leftover
    nctl-assets-teardown

    $NCTL/sh/scenarios/network_soundness.py

    # Clean up after the test
    nctl-assets-teardown
}

source "$NCTL/sh/staging/set_override_tomls.sh"
start_run_teardown "client.sh"
start_run_teardown "itst01.sh"
start_run_teardown "itst02.sh"
start_run_teardown "itst11.sh"
start_run_teardown "itst13.sh"
start_run_teardown "itst14.sh"
start_run_teardown "itst01_private_chain.sh"
start_run_teardown "itst02_private_chain.sh"
start_run_teardown "itst06_private_chain.sh"
start_run_teardown "itst07_private_chain.sh"
start_run_teardown "itst11_private_chain.sh"
start_run_teardown "bond_its.sh"
start_run_teardown "emergency_upgrade_test.sh"
start_run_teardown "emergency_upgrade_test_balances.sh"
start_run_teardown "upgrade_after_emergency_upgrade_test.sh"
start_run_teardown "sync_test.sh timeout=500"
start_run_teardown "swap_validator_set.sh"
start_run_teardown "sync_upgrade_test.sh node=6 era=5 timeout=500"
start_run_teardown "validators_disconnect.sh"
# Without start_run_teardown - these ones perform their own assets setup, network start and teardown
source "$SCENARIOS_DIR/upgrade_after_emergency_upgrade_test_pre_1.5.sh"
source "$SCENARIOS_DIR/regression_3976.sh"

run_nightly_upgrade_test

run_soundness_test

# Run these last as they occasionally fail (see https://github.com/casper-network/casper-node/issues/2973)
start_run_teardown "itst06.sh"
start_run_teardown "itst07.sh"

. integration_tests/.env
. integration_tests/lib.sh

# SETUP

# Build the latest darwin executable
make build-x86_64_darwin

# Move the binary to the template project folder
cp target/x86_64-apple-darwin/release/bs \
	"$TEST_RUNNER_PROJECT_TEMPLATE"

# Move the project folder to the client machine
scp -r -i integration_tests/.ssh/id_rsa \
	"$TEST_RUNNER_PROJECT_TEMPLATE"/* \
	"$MACOS_CLIENT_USER"@"$MACOS_CL IENT_IP":"$MACOS_CLIENT_DIR"

# TESTS

# Check that help works
runOnMacOSClient "./bs --help"

# Send project from client to server
runOnMacOSClient "./bs \
	--client-ip $MACOS_CLIENT_IP \
	--client-user $MACOS_CLIENT_USER \
	--client-paths $MACOS_CLIENT_DIR \
	--server-ip $LINUX_SERVER_IP \
	--server-user $LINUX_SERVER_USER \
	--server-paths $LINUX_SERVER_DIR"

ls integration_tests/it_project

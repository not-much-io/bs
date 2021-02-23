. integration_tests/.env

runOnMacOSClient() {
	ssh -i integration_tests/.ssh/id_rsa \
		"$MACOS_CLIENT_USER"@"$MACOS_CLIENT_IP" \
		"cd $MACOS_CLIENT_DIR && $1"
}

make build-x86_64_darwin

scp -i integration_tests/.ssh/id_rsa \
	target/x86_64-apple-darwin/release/bs \
	"$MACOS_CLIENT_USER"@"$MACOS_CLIENT_IP":"$MACOS_CLIENT_DIR"

runOnMacOSClient "./bs --help"
runOnMacOSClient "./bs \
	--client-ip $MACOS_CLIENT_IP \
	--client-user $MACOS_CLIENT_USER \
	--client-paths $MACOS_CLIENT_DIR \
	--server-ip $LINUX_SERVER_IP \
	--server-user $LINUX_SERVER_USER \
	--server-paths $LINUX_SERVER_DIR"

ls integration_tests/test_env

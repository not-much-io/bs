runOnMacOSClient() {
	ssh -i integration_tests/.ssh/id_rsa \
		"$MACOS_CLIENT_USER"@"$MACOS_CLIENT_IP" \
		"cd $MACOS_CLIENT_DIR && $1"
}
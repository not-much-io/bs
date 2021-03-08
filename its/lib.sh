. its/.env

getHostValue() {
	HOST=$1
	VALUE=$2

	case $HOST in
		DARWIN1)
			case $VALUE in
				user)
					echo "$DARWIN1_USER"
					;;
				ip)
					echo "$DARWIN1_IP"
					;;
				wd)
					echo "$DARWIN1_WD"
					;;
				*)
					echo "Unknown value: $VALUE"
					exit 1
					;;
			esac
			;;
		LINUX1)
			case $VALUE in
				user)
					echo "$LINUX1_USER"
					;;
				ip)
					echo "$LINUX1_IP"
					;;
				wd)
					echo "$LINUX1_WD"
					;;
				*)
					echo "Unknown value: $VALUE"
					exit 1
					;;
			esac
			;;
		LINUX2)
			case $VALUE in
				user)
					echo "$LINUX2_USER"
					;;
				ip)
					echo "$LINUX2_IP"
					;;
				wd)
					echo "$LINUX2_WD"
					;;
				*)
					echo "Unknown value: $VALUE"
					exit 1
					;;
			esac
			;;
		*)
			echo "Unknown host: $HOST"
			exit 1
			;;
	esac
}

runOn() {
	HOST=$1
	CMD=$2

	user=$(getHostValue "$HOST" "user")
	ip=$(getHostValue "$HOST" "ip")
	wd=$(getHostValue "$HOST" "wd")

	ssh -i "$IT_DIR"/.ssh/id_rsa \
		"$user"@"$ip" \
		"cd $wd && $CMD"
}

runOnInRoot() {
	HOST=$1
	CMD=$2

	user=$(getHostValue "$HOST" "user")
	ip=$(getHostValue "$HOST" "ip")
	wd=$(getHostValue "$HOST" "wd")
	
	ssh -i "$IT_DIR"/.ssh/id_rsa \
		"$user"@"$ip" \
		"$CMD"
}

setupProjectOnHost() {
	HOST=$1
	PROJECT=$2
	
	user=$(getHostValue "$HOST" "user")
	ip=$(getHostValue "$HOST" "ip")
	wd=$(getHostValue "$HOST" "wd")

	# Clean if anything in the way WD should never contain anything non nukable
	runOnInRoot "$HOST" "rm -rf $wd && mkdir $wd"
	
	scp -r -i "$IT_DIR"/.ssh/id_rsa \
		"$PROJECT"/* \
		"$user"@"$ip":"$wd"
}

test_local() {
	HOST="DARWIN1"
	PRJ="$IT_PROJECTS/generic"

	# Build the binary
	make build-x86_64_darwin

	# Add binary to project
	cp "$BUILT_DARWIN_BIN" "$PRJ"

	# Setup project on host
	setupProjectOnHost "$HOST" "$PRJ"

	# Check if binary will run at all
	runOn "$HOST" "./distrunner --help"

	# Check if actually works
	runOn "$HOST" "./distrunner"
}

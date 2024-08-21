

dry-run:
	@RUST_BACKTRACE=full npx spin-m4 dry-run --path provable_game_logic/ --zkwasm ../../../../g2/zkwasm/target/debug/ --public 0 --public 0 --public 0 --public 0 --public 0 \
	--public 24 --public 0 --public 0 --public 0 --public 0 --public 0 \
	--private 0 --private 0 --private 0 --private 0 --private 0 --private 0

build-image:
	@npx spin-m4 build-image --path provable_game_logic/
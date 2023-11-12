deploy:
	cargo build --release
	mv target/armv7a-none-eabi/release/rustberry_pi deploy/rustberry_pi

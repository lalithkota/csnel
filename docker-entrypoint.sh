
cp -rf /mnt-dir/* /output-dir/

if [ -f /mnt-dir/Cargo.toml ];then
  echo "Log: Cargo.toml already present."
fi
if [ -d /mnt-dir/.cargo ];then
  echo "Log: .cargo already present."
fi
if [ -f /mnt-dir/x86_64-req.json ];then
  echo "Log: x86_64-req.json already present."
fi
if [ -f /mnt-dir/src/main.rs ];then
  echo "Log: main.rs already present."
fi

cd /output-dir ; /root/.cargo/bin/cargo build; /root/.cargo/bin/cargo bootimage
cp /output-dir/target/x86_64-req/debug/bootimage-*.bin /mnt-dir/csnel-outputimage.bin


/bin/bash

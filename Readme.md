# Csnel

A http unikernel implementation from scratch. In Rust.

Qemu is the chosen hypervisor, for running the unikernel. U are free to try any other, but that is out of scope for this project.

Now available on a [docker image](https://hub.docker.com/r/lalithkota/csnel).

## 1. Before jumping in.

- A unikernel is a standalone OS (kind of), which is tailor-fitted, only for the application that is to be deployed (Similar to a container, but totally different).
- So there is no distinction of kernel space or user space, in a unikernel. All the system functionalities have to "hand-made".
- As there is no concept of an underlying OS, there is no concept of an executable as well. So the binary, that is built, has to be built for a custom target.
- `main` function also can't be used. Our bootloader has to be told which function to load as the kernel start.
- Similarly, no `std` functionalities can be used. Because those have to be built ourselves, according to our application's need.
- What is Csnel now? It aims to provide a user-friendly
mechanism to build & run rust-based unikernels. (also this primarily aims to be a http/web server unikernel maker) (Also please note "aims to". WIP.)
  - Also, as previously said, no std functionalities can be used. So csnel tries to provide for those functionalities. (Ex: print/println) (See Section 3 in this)

## 2. How to Use

Two methods.

1. A shell script is made. Which builds and runs the unikernels for the user, all by itself, with little to no intervention.
  - This only requires docker and qemu to be installed.  But the csnel docker image size is huge, at >1.6GB.
2. You can directly clone this repo. Use this crate in your dependency.
  - Use the `enduser` dir as a template for your project directory.
  - This, obviously, requires rust and cargo (nightly only) to be setup. (Which is fairly easy. See rustup.)

### 2.1. Using the csnel.sh & docker Image method

(See [Releases](https://github.com/lalithkota/csnel/releases)):
  - `./csnel.sh build` will build a unikernel from the current directory.
  - `./csnel.sh runq` will run the built unikernel in qemu.
  - For building, the current directory needs to have an `src` dir. And the `src` needs to have a `mymain.rs`. (Not `main.rs`, which is different.)
```
    --proj_dir
      |--src
         |--mymain.rs
```
  - `mymain.rs` must have a `pub fn starter()`. This can now be used as your regular main function.
  - An example `mymain.rs` looks kind of like this.
```
    use csnel::println;

    pub fn starter(){
      println!("Hello World");
    }
```
  - The docker-run and the script will take care of gathering all the required files and building the unikernel.
  - Once built, the unikernel will appear in your proj_dir at `csnel-outputimage.bin`, which can then be used, to run using qemu manually or; simply use `./csnel.sh runq`.

### 2.2. Regular/cargo method.

Take a brief look at the above method just to get a idea.

- This method is also going to be similar to the first one. But here your proj_dir is a separate cargo crate. So apart from `src/mymain.rs`, it additionally needs `Cargo.toml` (obviously), `.cargo/config.toml`, `x86_64-req.json` (the target file), `src/main.rs`.
- Get all these files from the `enduser` dir of the repo (NOT from the csnel crate itself). Basically copy-paste the the whole `enduser` folder, as is. Then rename it to your liking. (You can also rename the crate and add author inside this Cargo.toml file)
- Also install the `llvm-tools-preview`, `rust-src`, using the command `rustup component add llvm-tools-preview rust-src`
- Also make sure your rust setup is nightly. You don't need to install anything else, the dependencies in the `Cargo.toml` will take care of it.
- Put your code in `src/mymain.rs` in the "starter" function.
- `cargo build` command will build your crate.
- `cargo bootimage` command will create the unikernel-bin. (In `target/x86_64-req/debug` folder)
- `bootimage runner` will launch the unikernel in qemu for us.
- Additionally `cargo run` has been configured in the `.cargo/config.toml`. Which will run all the above steps, also launching qemu.
- The above build commands require target file to be specified. This should be done manually for each of the above commands. Or it can be configured in `.cargo/config.toml`. Which has been taken care of.
- Done, you should see the qemu already.
- You can now delete the `src/mymain.rs`, instead simply use the main.rs, if you are convenient.
- If you are interested in details, follow [phil-os blog](https://os.phil-opp.com) and start everthing from the beginning.

## 3. Whats working

- Done: Print interface. Interrupts. Hardware faults. PIC. Keyboard inputs. PCI configuration. Ethernet drivers.
- RTL interrupts are not working, but when i poll the isr it is working. So mostly the second-PIC is not initialized correctly.
- On the receive side, even after getting the RxOK interrupt (on polling), the rx_buffer is fully empty. All zeros. Have to figure out why.
- [No netowork stack nor web framework is built, yet. WIP. (So yeah. Cant yet deploy a web server unikernel.) (General unikernels can be made though.)]
- Experimental virtualbox run is also implemented. `./csnel.sh runvb` . But it is having some problem with the image/binary type of the unikernel. The final output image is raw format image. So it cant be mounted as cd/dvd. When mounted as raw floppy, it gives no errors. But it cant start the vm.  
- Aiming to compress the docker image size, by using a different base image.

## 4. Credits

Huge credits to [Philipp Oppermann's blog posts](https://os.phil-opp.com) for getting us started.

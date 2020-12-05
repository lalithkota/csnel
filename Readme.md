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
  - Also, as previously said, no std functionalities can be used. So csnel tries to provide for those functionalities. (Ex: print/println) (See "Whats working" section in this)

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

    pub fn starter(bootinfo : &'static csnel::BootInfo){
    	  println!("Hello World{}","!");
	  let (_,mapper) = csnel::init(bootinfo);
	  csnel::net::sample_deal_all(&mapper);
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
- In your/enduser's Cargo.toml, also make sure you set the path to csnel crate itself properly (which you get by cloning).
- Put your code in `src/mymain.rs` in the "starter" function. (Try using example code from 2.1)
- `cargo run` command will create the unikernel, and launch it in qemu. Additionally,
  - `cargo build` command will only build your code.
  - `cargo bootimage` command will build code. Then create the unikernel-bin. (In `target/x86_64-req/debug` folder)
  - `bootimage runner` command will build code, create unikernel and launch it. `cargo run` is configured to be same as this.
- The above build commands require target file to be specified. This should be done manually for each of the above commands. Or it can be configured in `.cargo/config.toml`. Which has been taken care of.
- Done, you should see the qemu already.
- You can now delete the `src/mymain.rs`, instead simply use the main.rs, if you are convenient.
- If you are interested in details, follow [phil-os blog](https://os.phil-opp.com) and start everthing from the beginning.

## 3. Brief Explanation (What the 2.1 example code does)

csnel::init() will initialise everything required for the higher layers of our unikernel to work, like interrupts related stuff, memory related stuff, pci stuff, some drivers, some other network stuff etc.
csnel::net::sample_deal_all() will try to deal with the incoming traffic.
It primarily deals with http requests .. but it requires the mapper argument .. which you can get from the second return value of csnel::init()

## 4. Whats working

- Done: Print interface. Interrupts. Hardware faults. PIC. Keyboard inputs. PCI configuration. Ethernet drivers.
  - Memory: paging and address translation done. No dynamic memory allocation as of now.
- RTL interrupts are not working, but when the isr is polled it is working. So mostly the second-PIC is not initialized correctly.
  - Update: after unmasking this interrupt index in the pic, the interrupts started working as well. But it is left masked for now, because polling works alright. Maybe will change in future.
  - Update: unable to use the interrupt handler. because when trying to translate VirtAddr to PhysAddr, a page fault is occuring. but the same wont happen in regular execution. So this idea is pretty much halted. (for now)
    - Update: a new branch is made where this is dealt with, but still not 100% ok. See branches.
- ~~On the receive side, even after getting the RxOK interrupt (on polling), the rx_buffer is fully empty. All zeros. Have to figure out why.~~
  - Update: Our doubts were true. RTL's rb_start needs buffer's physical address (previously simply the virtual address was passed). So, once simple paging & address conversion is implemented, and when the buffer's phys_addr is updated, it started working as well.
- ~~No netowork stack nor web framework is built, yet. WIP. (So yeah. Cant yet deploy a web server unikernel.) (General unikernels can be made though.)~~
  - Update: Lot of progress made. A pseudo/dummy network stack is built. It can even make simple HTTP transactions. YAY.
  - Update: working UDP, Working DHCP also done. (Should work with bridged networking also now. TODO.)
    - Update: When using bridged networking (qemu's tap mode), dhcp doesnt work as expected. Guessing there is something wrong with udp checksum. And also mabe Qemu slirp doesnt care about udp checksum, because DHCP does work on slirp, but doesnt work outside/real network.
  - Update: No filesystem yet. (So have to hardcode html file into string.) Also no tcp exit mechanism yet.
- No congestion control nor flow control on any layer.. have to work on that.
- Experimental virtualbox run is also implemented. `./csnel.sh runvb` . But it is having some problem with the image/binary type of the unikernel. The final output image is raw format image. So it cant be mounted as cd/dvd. When mounted as raw floppy, it gives no errors. But it cant start the vm.
  - Update: will try to mount it as disk drive only. By using clonevdi, and creating a raw disk vdi image. ~~TODO~~.
  - Update: Apparently the above doesnt work. Will try to use vmdk file descriptor, and use raw image. TODO.
- Aiming to compress the docker image size, by using a different base image.
  - Update: After changing to rust:alpine base, the filesize actually increased (~2.0GiB). Will try to make modifications. Or will go back to using, previous. ~~TODO~~
  - Going back to oiginal because, final image size with rust:alpine base is slighter higher than that of regular alpine base, and rustup installes image.

## 5. Credits

Huge credits to [Philipp Oppermann's blog posts](https://os.phil-opp.com) for getting us started.

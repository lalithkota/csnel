# Csnel

net with interrupts, handling the lower level network traffic.

- master branch's net::sample_deal_all() is fine but, it takes away the primary execution space and keeps looping until it gets interrupts.
- In other words, once it is called .. we cannot do other things.
- On the contrary, if the Etherent-NIC interrupts are enabled, and made such that the traffic is handled according to the received packets all inside the interrupt handler...
- then it doesnt take away from the primary execution context, (because the interrupt handling will run in a different context).
- In other words, we can parellelly do other stuff, apart from traffic handling, all without implementing multi-threaded/multi-process nature (which is not implemented mainly).
- So this branch tries to take care of this situation/problem with main branch..
- But i cannot seem to get it work as efficiently as the main on though.. the receive interrupts fire up twice everytime ... and there is somehow an overall performance overhead ...
- And mainly when translating virtaddr to physaddr .. a page fault occurs ... Mostly because of the mapper object ... (because mapper object is created in the main thread .. and is needed inside the interrupt handler so .. i am storing the addr of that mapper object as static .. but maybe on the interrupt handler side it is getting a garbage mapper upon accessing from the stored addr)
- So ya... this has to be worked upon

- Update: So the Mapper_addr is being stored in the csnel::net::init_net().. and that is being called in the csnel::init() ... but outisde csnel::init(), the mapper doesnt exist .. so the address stored in the init_net() is obsolete
  - Update: So need to update the Mapper_addr outside init function... in either main (or starter function) : Maybe this will solve the issue : ~~TODO~~
  - Update: Now changing MAPPER_ADDR to be part of csnel::memory instead of csnel::net. : ~~TODO~~
  - Update: Ok so the aim is to make the mapper object never get deconstructed..
    - As control reaches end of `src/mymain.rs::starter()` (it then goes to `src/main.rs::kernel_main()`), but the mapper object is deleted after the starter()..
	- So if the starter function never ends, mapper wont be destroyed.. anyway after the starter() .. the kernel_main() goes to halt_loop()..
	- So if an empty loop{} is called prematurely at the end of starter() itself .. it should solve our issue : ~~TODO~~
- Update: so the above thing solved it... but it still receives two rec_interrupts everytime ... : TODO
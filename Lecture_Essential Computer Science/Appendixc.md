 APPENDIX C

ACPI System States

Power optimization of computer systems has become very important. 
There are many governing bodies (like the California Energy Commission) 
that mandate a certain level of power efficiency in computing devices. In a 
computer system, there are multiple pieces of hardware and software that 
all need to be in sync. Therefore, a mechanism is needed for these pieces 
to pass information around. The Advanced Configuration and Power 
Interface Special Interest Group (ACPI SIG) developed such a standard, 
named after the group, ACPI.

ACPI provides an open standard that system firmware (BIOS) and 

operating systems use to discover, configure, and carry out system-specific 
operations. ACPI replaces the multiple earlier standards like Advanced 
Power Management (APM), MultiProcessor Specification, and the Plug 
and Play (PnP) BIOS Specification. ACPI defines a hardware abstraction 
interface across system firmware, computer hardware components, and 
operating systems. ACPI is the key element in operating system–directed 
configuration and power management (OSPM). In 2013, the ACPI SIG 
agreed to transfer the specification to the UEFI Forum, which now owns 
the specification.

ACPI defines standard operating states for systems, devices, and 
processors, among other things. Figure C-1 shows the various states 
defined by ACPI and transitions between them. In the following sections, 
we talk about these states and explain what they all mean.

© Paul D. Crutcher, Neeraj Kumar Singh, and Peter Tiegs 2021 
P. D. Crutcher et al., Essential Computer Science,  
https://doi.org/10.1007/978-1-4842-7107-0

269

.

d
e
v
r
e
s
e
r
 
s
t

h
g
i
r
 
l
l

A

 
.
.

P

 
.

L

 
s
s
e
r
p
A

 
.

1
2
0
2
©

 

 
t

h
g
i
r
y
p
o
C

Crutcher, Paul D., et al. Essential Computer Science : A Programmer's Guide to Foundational Concepts, Apress L. P., 2021.
         ProQuest Ebook Central, http://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=6640815.
Created from th-deggendorf on 2022-03-07 21:18:35.

Appendix C 
ACpi SyStem StAteS

 ACpi SyStem StAteS

(cid:87)(cid:381)(cid:449)(cid:286)(cid:396)(cid:3)(cid:38)(cid:258)(cid:349)(cid:367)(cid:437)(cid:396)(cid:286)(cid:3)(cid:876)(cid:3)
(cid:87)(cid:381)(cid:449)(cid:286)(cid:396)(cid:882)(cid:75)(cid:299)

(cid:39)(cid:1007)(cid:882)(cid:68)(cid:286)(cid:272)(cid:346)(cid:3)
(cid:75)(cid:299)

(cid:68)(cid:381)(cid:282)(cid:286)(cid:373) (cid:44)(cid:24)(cid:24)

(cid:24)(cid:1007)
(cid:24)(cid:1006)
(cid:24)(cid:1005)
(cid:24)(cid:1004)
(cid:24)(cid:1004)

(cid:24)(cid:1007)
(cid:24)(cid:1006)
(cid:24)(cid:1005)
(cid:24)(cid:1004)

(cid:62)(cid:286)(cid:336)(cid:258)(cid:272)(cid:455)

(cid:39)(cid:1004)(cid:894)(cid:94)(cid:1004)(cid:895)(cid:882)
(cid:116)(cid:381)(cid:396)(cid:364)(cid:349)(cid:374)(cid:336)

(cid:116)(cid:258)(cid:364)(cid:286)(cid:3)
(cid:28)(cid:448)(cid:286)(cid:374)(cid:410)

(cid:18)(cid:24)(cid:90)(cid:75)(cid:68)

(cid:24)(cid:1007)
(cid:24)(cid:1006)
(cid:24)(cid:1005)
(cid:24)(cid:1004)

(cid:17)(cid:47)(cid:75)(cid:94)(cid:3)(cid:90)(cid:381)(cid:437)(cid:410)(cid:349)(cid:374)(cid:286)

(cid:94)(cid:1008)

(cid:94)(cid:1007)

(cid:94)(cid:1006)

(cid:94)(cid:1005)
(cid:39)(cid:1005)(cid:882)
(cid:94)(cid:367)(cid:286)(cid:286)(cid:393)(cid:349)(cid:374)(cid:336)

(cid:39)(cid:1006)(cid:894)(cid:94)(cid:1009)(cid:895)(cid:882)
(cid:94)(cid:381)(cid:332)(cid:3)(cid:75)(cid:299)

(cid:87)(cid:286)(cid:396)(cid:296)(cid:381)(cid:396)(cid:373)(cid:258)(cid:374)(cid:272)(cid:286)(cid:3)
(cid:94)(cid:410)(cid:258)(cid:410)(cid:286)(cid:3)(cid:87)(cid:121)

(cid:100)(cid:346)(cid:396)(cid:381)(cid:427)(cid:374)(cid:336)

(cid:18)(cid:1004)

(cid:18)(cid:1005)

(cid:18)(cid:1006)

(cid:18)(cid:374)

(cid:18)(cid:87)(cid:104)

Figure C-1.  Global and System Power States and Transitions

 Global and System States
ACPI defines four global states and six system states. The global states are 
marked G0–G3, while the system states are marked as S0–S5. It must be 
noted, however, that some motherboard documents reference S6, which is 
not an ACPI-defined state. If you come across this, you can safely map this 
to G3.

ACPI defines a mechanism to transition the system between the 

working state (G0) and the sleeping state (G1) or the soft-off state (G2). 
During transitions between the working and sleeping states, the operating 
system will maintain your context, so you don’t lose information on such 
transitions. ACPI defines the level of the G1 sleeping state by defining the 
system attributes of four types of ACPI sleeping states (S1, S2, S3, and S4). 
Each sleeping state is defined to allow implementations to trade-off cost, 
power, and wake latencies:

270

.

d
e
v
r
e
s
e
r
 
s
t

h
g
i
r
 
l
l

A

 
.
.

P

 
.

L

 
s
s
e
r
p
A

 
.

1
2
0
2
©

 

 
t

h
g
i
r
y
p
o
C

Crutcher, Paul D., et al. Essential Computer Science : A Programmer's Guide to Foundational Concepts, Apress L. P., 2021.
         ProQuest Ebook Central, http://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=6640815.
Created from th-deggendorf on 2022-03-07 21:18:35.

Appendix C 

 ACpi SyStem StAteS

•  G0/S0: In the G0 state, work is being performed by the 
OS/application software and the hardware. The CPU or 
any particular hardware device could be in any one of 
the defined power states (more on the device and CPU 
power states in a later section); however, some work 
will be taking place in the system.

  a.  S0: System is in a fully working state.

•  G1: In the G1 state, the system is assumed to be doing 

no work. Prior to entering the G1 state, OSPM will 
place devices in a device power state compatible with 
the system sleeping state to be entered; if a device is 
enabled to wake the system, then OSPM will place 
these devices into the lowest Dx state from which the 
device supports wake.

  a.  S1: The S1 state is defined as a low wake latency 

sleeping state. In this state, the entire system 
context is preserved with the exception of CPU 
caches. Before entering S1, OSPM will flush the 
system caches.

  b.  S2: The S2 state is defined as a low wake 

latency sleep state. This state is similar to the 
S1 sleeping state where any context except for 
system memory may be lost.

  c.  S3: Commonly referred to as Standby, Sleep, or 
Suspend to RAM (STR). The S3 state is defined 
as a low wake latency sleep state. From the 
software viewpoint, this state is functionally the 
same as the S2 state. The operational difference 
is that some power resources that may have 
been left ON in the S2 state may not be available 

271

.

d
e
v
r
e
s
e
r
 
s
t

h
g
i
r
 
l
l

A

 
.
.

P

 
.

L

 
s
s
e
r
p
A

 
.

1
2
0
2
©

 

 
t

h
g
i
r
y
p
o
C

Crutcher, Paul D., et al. Essential Computer Science : A Programmer's Guide to Foundational Concepts, Apress L. P., 2021.
         ProQuest Ebook Central, http://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=6640815.
Created from th-deggendorf on 2022-03-07 21:18:35.

Appendix C 

 ACpi SyStem StAteS

to the S3 state. As such, some devices may be in 
a lower-power state when the system is in the 
S3 state than when the system is in the S2 state. 
Similarly, some device wake events can function 
in S2 but not S3.

  d.  S4: Also known as Hibernation or Suspend to 

Disk. The S4 sleeping state is the lowest-power, 
longest wake latency sleeping state supported by 
ACPI. In order to reduce power to a minimum, 
it is assumed that the hardware platform has 
powered off all devices. Because this is a sleeping 
state, the platform context is maintained. 
Depending on how the transition into the S4 
sleeping state occurs, the responsibility for 
maintaining system context changes between 
OSPM and BIOS. To preserve context, in this 
state all content of the main memory is saved 
to non-volatile memory such as a hard drive 
and is powered down. The contents of RAM are 
restored on resume. All hardware is in the off 
state and maintains no context.

•  G2/S5: Also referred to as Soft Off. In G2/S5, all 

hardware is in the off state and maintains no context. 
OSPM places the platform in the S5, soft-off, state to 
achieve a logical off. The S5 state is not a sleeping state 
(it is a G2 state), and no context is saved by OSPM or 
hardware, but power may still be applied to parts of the 
platform in this state, and as such, it is not safe to take 
the system apart. Also, from a hardware perspective, the 
S4 and S5 states are nearly identical. When initiated, 
the hardware will sequence the system to a state similar 

.

d
e
v
r
e
s
e
r
 
s
t

h
g
i
r
 
l
l

A

 
.
.

P

 
.

L

 
s
s
e
r
p
A

 
.

1
2
0
2
©

 

 
t

h
g
i
r
y
p
o
C

272

Crutcher, Paul D., et al. Essential Computer Science : A Programmer's Guide to Foundational Concepts, Apress L. P., 2021.
         ProQuest Ebook Central, http://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=6640815.
Created from th-deggendorf on 2022-03-07 21:18:35.

Appendix C 

 ACpi SyStem StAteS

to the off state. The hardware has no responsibility 
for maintaining any system context (memory or I/O); 
however, it does allow a transition to the S0 state due to 
a power button press or a remote start.

•  G3: Mechanical Off. Same as S5. Additionally, the 

power supply is isolated. The computer's power has 
been totally removed via a mechanical switch, and 
no electrical current is running through. This is the 
only state that the system can be worked on without 
damaging the hardware.

 Device States
In addition to global and system states, ACPI defines various device states 
ranging from D0 to D3. The exact definition or meaning of specific device 
states depends on the device class. A device class describes a type of 
device – for example, audio, storage, network, and so on:

•  D0: This state is assumed to be the highest level of 

functionality and power consumption. The device is 
completely active and responsive and is expected to 
remember all relevant contexts.

•  D1: Many device classes may not support D1. In 

general, D1 is expected to save less power and preserve 
more device context than D2. D1 may cause the device 
to lose some context.

•  D2: Many device classes may not support D2. In 
general, D2 is expected to save more power and 
preserve less device context than D1 or D0. D2 may 
cause the device to lose some context.

273

.

d
e
v
r
e
s
e
r
 
s
t

h
g
i
r
 
l
l

A

 
.
.

P

 
.

L

 
s
s
e
r
p
A

 
.

1
2
0
2
©

 

 
t

h
g
i
r
y
p
o
C

Crutcher, Paul D., et al. Essential Computer Science : A Programmer's Guide to Foundational Concepts, Apress L. P., 2021.
         ProQuest Ebook Central, http://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=6640815.
Created from th-deggendorf on 2022-03-07 21:18:35.

Appendix C 

 ACpi SyStem StAteS

•  D3 Hot: Devices in the D3 Hot state are required to be 

software enumerable. In general, D3 Hot is expected 
to save more power and optionally preserve device 
context. If device context is lost when this state is 
entered, the OS software will reinitialize the device 
when transitioning back to D0.

•  D3 Cold: Power has been fully removed from the 

device. The device context is lost when this state is 
entered, so the OS software will have to fully reinitialize 
the device when powering it back on. Devices in this 
state have the longest restore times.

 Processor States
ACPI defines the power state of system processors while in the G0 working 
state as being either active (executing) or sleeping (not executing). 
Processor power states are designated as C0, C1, C2, C3, … Cn. The C0 
power state is an active power state where the CPU executes instructions. 
The C1–Cn power states are processor sleeping states where the processor 
consumes less power and dissipates less heat than leaving the processor in 
the C0 state. While in a sleeping state, the processor does not execute any 
instructions. Each processor sleeping state has a latency associated with 
entering and exiting that corresponds to the power savings. In general, 
the longer the entry/exit latency, the greater the power savings is for 
the state. To conserve power, OSPM places the processor into one of its 
supported sleeping states when idle. While in the C0 state, ACPI allows the 
performance of the processor to be altered through a defined “throttling” 
process and through transitions into multiple performance states (P 
states). A diagram of processor power states (not to be confused with 
performance states) is provided in Figure C-2.

274

.

d
e
v
r
e
s
e
r
 
s
t

h
g
i
r
 
l
l

A

 
.
.

P

 
.

L

 
s
s
e
r
p
A

 
.

1
2
0
2
©

 

 
t

h
g
i
r
y
p
o
C

Crutcher, Paul D., et al. Essential Computer Science : A Programmer's Guide to Foundational Concepts, Apress L. P., 2021.
         ProQuest Ebook Central, http://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=6640815.
Created from th-deggendorf on 2022-03-07 21:18:35.

Appendix C 

 ACpi SyStem StAteS

(cid:100)(cid:346)(cid:396)(cid:381)(cid:410)(cid:410)(cid:367)(cid:349)(cid:374)(cid:336)

(cid:87)(cid:890)(cid:62)(cid:115)(cid:62)(cid:1007)
(cid:4)(cid:90)(cid:17)(cid:890)(cid:24)(cid:47)(cid:94)(cid:1089)(cid:1005)

(cid:47)(cid:374)(cid:410)(cid:286)(cid:396)(cid:396)(cid:437)(cid:393)(cid:410)(cid:3)(cid:381)(cid:396)
(cid:17)(cid:68)(cid:3)(cid:4)(cid:272)(cid:272)(cid:286)(cid:400)(cid:400)

(cid:18)(cid:1007)

(cid:87)(cid:286)(cid:396)(cid:296)(cid:381)(cid:396)(cid:373)(cid:258)(cid:374)(cid:272)(cid:286)(cid:3)
(cid:94)(cid:410)(cid:258)(cid:410)(cid:286)(cid:3)(cid:87)(cid:121)

(cid:47)(cid:374)(cid:410)(cid:286)(cid:396)(cid:396)(cid:437)(cid:393)(cid:410)

(cid:44)(cid:62)(cid:100)

(cid:18)(cid:1005)

(cid:100)(cid:44)(cid:100)(cid:890)(cid:28)(cid:69)(cid:882)(cid:1005)(cid:3)(cid:3)(cid:3)(cid:3)(cid:3)

(cid:258)(cid:374)(cid:282)
(cid:24)(cid:47)(cid:122)(cid:3)(cid:448)(cid:258)(cid:367)(cid:437)(cid:286)

(cid:18)(cid:1004)

(cid:100)(cid:44)(cid:100)(cid:890)(cid:28)(cid:69)(cid:882)(cid:1004)

(cid:87)(cid:890)(cid:62)(cid:115)(cid:62)(cid:1006)

(cid:47)(cid:374)(cid:410)(cid:286)(cid:396)(cid:396)(cid:437)(cid:393)(cid:410)

(cid:18)(cid:1006)

(cid:39)(cid:1004)(cid:3)

(cid:116)(cid:381)(cid:396)(cid:364)(cid:349)(cid:374)(cid:336)

Figure C-2.  Processor Power States

In summary, one of the main goals of OSPM is to save power/energy 

when the workload allows it, and detecting inactivity and putting the 
devices and the system (if possible) in their low-power states forms the 
heart of power management software.

.

d
e
v
r
e
s
e
r
 
s
t

h
g
i
r
 
l
l

A

 
.
.

P

 
.

L

 
s
s
e
r
p
A

 
.

1
2
0
2
©

 

 
t

h
g
i
r
y
p
o
C

275

Crutcher, Paul D., et al. Essential Computer Science : A Programmer's Guide to Foundational Concepts, Apress L. P., 2021.
         ProQuest Ebook Central, http://ebookcentral.proquest.com/lib/th-deggendorf/detail.action?docID=6640815.
Created from th-deggendorf on 2022-03-07 21:18:35.


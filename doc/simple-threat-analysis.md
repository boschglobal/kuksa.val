# Simple Security and Threat Analysis

When deploying KUKSA inside a Vehicle you need to consider potential security threats. 

While specific threats and counter-measure will depend on a your use case and deployment environment  ([see deployment blueprints](./deployment.md) for examples), this document intends to give you some direction on generic threats and measures you need to consider when deploying a KUKSA based system.

## The Four-Step Framework and the STRIDE Method
Threat modeling consists of multiple steps, introducing goals that have to be accomplished, rather than seeing it as a single activity. The four-step framework describes a method that is concerned about four questions that need to be addressed in the system with regard to security security:

 - **What are you building?**: You need to understand what you are building and what  the important assets in your system are.
 - **What can go wrong with it once it`s build?**: During design/development time, such mindset helps to mitigate problems already instead of "patching" solutions later on.
 -  **What can you you do about those things that can go wrong?**: It is about risks and its assessment to address the most important threats first.
 - **Do you do a decent job of analysis?**: It is about validation of the mitigation or threats, and did you miss something?

The Figure below illustrates the Four-Step Framework. TBD Picture

## Finding Threats Using STRIDE
After the identification of what you are building and what your assets are that need to be protected, you can start finding threats in your system. 

Having the mindset of "what can go wrong" in your system is a good start to find threats. A common framework for this is **STRIDE**. It has  been developed [by Loren Kohnfelder and Praerit Garg](https://www.microsoft.com/security/blog/2009/08/27/the-threats-to-our-products/). **STRIDE** is a mnemonic for things that can go wrong in a system form a security perspective:

| # | Threat                  | Property Violated | Threat Definition                                              | Typical Victims                       | Mitigation Options                   | Examples                                                                                                                                          |
|---|-------------------------|-------------------|----------------------------------------------------------------|---------------------------------------|--------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------|
| S | Spoofing                | Authentication    | Pretending to be something or someone other than yourself      | Processes, external entities, persons | Identification and Authentication    | Falsely claiming to be the President of the United States.                                                                                        |
| T | Tampering               | Integrity         | Modify some data at rest or during transmission                | Data flows (messages), processes      | Cryptographic, or Anti-pattern       | Modifying, adding, removing packets from a network, either local or far across the Internet; wired or wireless.                                   |
| R | Repudiation             | Non-Repudiation   | Claiming that you didn´t do something, or were not responsible | Processes                             | Logs                                 | Process or System: "I didn´t hit the big red button".                                                                                             |
| I | Information Disclosure  | Confidentiality   | Providing information to someone who is not authorized to see  | Processes, data, data flows           | Cryptographic, or Encryption         | Allowing access to files, to packets (e.g., content) in the network (e.g., by forwarding them)                                                    |
| D | Denial of Service       | Availability      | Absorbing resources needed to provide a service                | Processes, data, data flows           | Avoid multipliers, or careful design | A program that can be tricked to use all resources, or so many connections/traffic that the real traffic can not get through                      |
| E | Elevation of Privileges | Authorization     | Allowing someone to do something he is not authorized to do    | Process                               | Identity Management, Sandbox, etc.   | Allow a normal user to execute code as administrator; allow a remote entity to interact with a program or a system without having any privileges. |

(adapted from Adam Shostack, "Threat Modeling: Designing for Security", John Wiley & Sons, Inc., ISBN: 978-1-118-80999-0 (2014))

STRIDE can be a very useful method when identifying threats, but it also depends how it use actually used. There are different options to apply STRIDE dependent on the assets to protect and the overall system design. There are two popular options to apply STRIDE:
 - **STRIDE-per-Element:** Applying STRIDE to each element/component/entity in the network to identify threats for each of those elements. Examples include physical devices, software components, users, etc.
 - **STRIDE-per-Interaction:** Applying STRIDE focusing on interaction threats within the system. Examples include software component interactions, data flows, protocol interactions, etc.

As KUKSA is all about a (network) based API to interact with a vehicle via VSS signals, **STRIDE-per-Interaction** will be used in this document.

## System Architecture: What are we building?

We have a VSS server that interacts with several VSS consumers and several VSS providers.


For more information regarding the basic KUKSA system architecture refer to the [terminology](./terminology.md) and [system architecture](./system-architecture.md) documentation.


